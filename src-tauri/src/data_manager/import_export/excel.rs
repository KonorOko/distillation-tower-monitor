use crate::calculations::service::CalculationService;
use crate::calculations::types::CompositionResult;
use crate::data_manager::types::{ColumnEntry, ColumnStructure};
use crate::errors::{Error, ExportError, ImportError, Result};
use calamine::{open_workbook, Data, DataType, Range, Reader, Xlsx};
use log::info;
use rust_xlsxwriter::{Workbook, XlsxError};
use std::sync::Arc;

pub struct ExcelDataImporter {
    calculation_service: CalculationService,
}

impl ExcelDataImporter {
    pub fn new(calculation_service: CalculationService) -> Self {
        Self {
            calculation_service,
        }
    }

    pub async fn import(
        &self,
        path: &str,
    ) -> Result<(usize, Vec<Arc<ColumnEntry>>, Option<f64>, Option<f64>)> {
        let mut workbook: Xlsx<_> = open_workbook(path)
            .map_err(|_| ImportError::InvalidFormat("Unable to open workbook".into()))?;
        let worksheet_name = workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or(ImportError::InvalidFormat("No sheets found".into()))?;

        let range = workbook
            .worksheet_range(&worksheet_name)
            .map_err(|_| ImportError::InvalidFormat("Cannot read sheet".into()))?;

        let mut initial_mass: Option<f64> = None;

        let mut initial_composition: Option<f64> = None;

        if let Some(cell) = range.get((0, 0)) {
            if let Data::String(label) = cell {
                if label.contains("Initial Mass") {
                    if let Some(Data::Float(value)) = range.get((1, 0)) {
                        initial_mass = Some(*value);
                    } else if let Some(Data::Int(value)) = range.get((1, 0)) {
                        initial_mass = Some(*value as f64);
                    }
                }
            }
        }

        if let Some(cell) = range.get((0, 1)) {
            if let Data::String(label) = cell {
                if label.contains("Initial Composition") {
                    if let Some(Data::Float(value)) = range.get((1, 1)) {
                        initial_composition = Some(*value);
                    } else if let Some(Data::Int(value)) = range.get((1, 1)) {
                        initial_composition = Some(*value as f64);
                    }
                }
            }
        }

        println!("Initial Mass: {:?}", initial_mass);
        println!("Initial Composition: {:?}", initial_composition);

        let header_row = if initial_mass.is_some() || initial_composition.is_some() {
            4
        } else {
            1
        };

        let column_structure = self.parse_headers(&range, header_row)?;

        let entries = self.process_rows(&range, &column_structure, header_row)?;

        Ok((
            column_structure.number_plates,
            entries,
            initial_mass,
            initial_composition,
        ))
    }

    fn parse_headers(&self, range: &Range<Data>, header_row: usize) -> Result<ColumnStructure> {
        let headers = range
            .rows()
            .nth(header_row - 1)
            .ok_or_else(|| ImportError::InvalidFormat("No headers found".into()))?;
        if headers.len() < 2 {
            return Err(ImportError::InvalidFormat("Insufficient columns".into()).into());
        }

        let mut has_compositions = false;
        let mut number_plates = 0;

        for header in headers.iter().skip(1) {
            if let Data::String(s) = header {
                if s.starts_with("Temperature") {
                    number_plates += 1;
                } else if s.starts_with("Composition x_1") {
                    has_compositions = true;
                    break;
                }
            }
        }

        if has_compositions && number_plates == 0 {
            number_plates = (headers.len() - 1) / 3;
        } else if number_plates == 0 {
            number_plates = headers.len() - 1;
        }

        info!("Detected {} plates in Excel file", number_plates);

        let compositions_x_start = if has_compositions {
            Some(1 + number_plates)
        } else {
            None
        };

        let compositions_y_start = if has_compositions {
            Some(1 + number_plates * 2)
        } else {
            None
        };

        Ok(ColumnStructure {
            number_plates,
            has_compositions,
            timestamp_column: 0,
            temperatures_start: 1,
            compositions_x_start,
            compositions_y_start,
        })
    }

    fn process_rows(
        &self,
        range: &Range<Data>,
        structure: &ColumnStructure,
        start_row: usize,
    ) -> Result<Vec<Arc<ColumnEntry>>> {
        let mut imported_data: Vec<Arc<ColumnEntry>> = Vec::new();
        let row_count = range.height();
        let total_rows = if row_count > start_row {
            row_count - start_row
        } else {
            0
        };

        for (index, row_index) in (start_row..row_count).enumerate() {
            let row_data: Vec<&Data> = range
                .rows()
                .nth(row_index)
                .map(|row| row.iter().collect())
                .unwrap_or_default();

            if row_data.is_empty()
                || row_data.len() < structure.temperatures_start + structure.number_plates
            {
                continue;
            }

            let percentage_complete = if total_rows > 0 {
                ((index as f64 + 1.0) / total_rows as f64 * 100.0) as f64
            } else {
                0.0
            };

            let timestamp = match row_data
                .get(structure.timestamp_column)
                .and_then(|cell| cell.as_f64())
            {
                Some(ts) => ts as u64,
                None => continue,
            };

            let temperatures: Vec<f64> = row_data
                .iter()
                .skip(structure.temperatures_start)
                .take(structure.number_plates)
                .filter_map(|cell| cell.as_f64())
                .collect();

            if temperatures.len() != structure.number_plates {
                info!("Row {} has incomplete temperature, skipping", index + 1);
                continue;
            }

            let compositions: Vec<CompositionResult> = if structure.has_compositions {
                let comp_x: Vec<Option<f64>> = match structure.compositions_x_start {
                    Some(start) => row_data
                        .iter()
                        .skip(start)
                        .take(structure.number_plates)
                        .map(|cell| cell.as_f64())
                        .collect(),
                    None => vec![None; structure.number_plates],
                };

                let comp_y: Vec<Option<f64>> = match structure.compositions_y_start {
                    Some(start) => row_data
                        .iter()
                        .skip(start)
                        .take(structure.number_plates)
                        .map(|cell| cell.as_f64())
                        .collect(),
                    None => vec![None; structure.number_plates],
                };

                comp_x
                    .into_iter()
                    .zip(comp_y.into_iter())
                    .map(|(x, y)| CompositionResult { x_1: x, y_1: y })
                    .collect()
            } else {
                temperatures
                    .iter()
                    .map(|&temp| {
                        self.calculation_service
                            .calculate_composition(None, temp, None, None)
                            .unwrap_or_else(|_| CompositionResult {
                                x_1: None,
                                y_1: None,
                            })
                    })
                    .collect()
            };

            let distilled_mass = self
                .calculation_service
                .calculate_distilled_mass(1000.0, imported_data.clone());

            imported_data.push(Arc::new(ColumnEntry {
                timestamp,
                temperatures,
                compositions,
                percentage_complete,
                distilled_mass,
            }));
        }

        if imported_data.is_empty() {
            return Err(ImportError::InvalidFormat("No valid data rows found".into()).into());
        }

        Ok(imported_data)
    }
}

pub struct ExcelDataExporter {
    calculation_service: CalculationService,
}

impl ExcelDataExporter {
    pub fn new(calculation_service: CalculationService) -> Self {
        ExcelDataExporter {
            calculation_service,
        }
    }

    pub fn export_data(
        &self,
        column_data: &Vec<Arc<ColumnEntry>>,
        initial_mass: f64,
        initial_composition: f64,
        path: &str,
    ) -> Result<()> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        self.write_initial_config(worksheet, initial_mass, initial_composition)?;

        let start_row = 3;
        let Some(first) = column_data.first() else {
            return Err(Error::ExportError(ExportError::NoDataError));
        };

        let num_plates = first.temperatures.len();

        self.write_headers(worksheet, num_plates)?;

        self.write_data(worksheet, column_data, start_row, num_plates)?;

        info!("Saving excel");
        workbook
            .save(path)
            .map_err(|e: XlsxError| ExportError::ExportDataError(format!("Xlsx error: {}", e)))?;
        info!("Excel saved");

        Ok(())
    }

    fn write_initial_config(
        &self,
        worksheet: &mut rust_xlsxwriter::Worksheet,
        initial_mass: f64,
        initial_composition: f64,
    ) -> Result<()> {
        worksheet
            .write_string(0, 0, "Initial Mass (g)")
            .map_err(|e| ExportError::ExportDataError(format!("Xlsx error: {}", e)))?;
        worksheet
            .write_number(1, 0, initial_mass)
            .map_err(|e| ExportError::ExportDataError(format!("Xlsx error: {}", e)))?;

        worksheet
            .write_string(0, 1, "Initial Composition (x_1)")
            .map_err(|e| ExportError::ExportDataError(format!("Xlsx error: {}", e)))?;
        worksheet
            .write_number(1, 1, initial_composition)
            .map_err(|e| ExportError::ExportDataError(format!("Xlsx error: {}", e)))?;

        Ok(())
    }

    fn write_headers(
        &self,
        worksheet: &mut rust_xlsxwriter::Worksheet,
        num_plates: usize,
    ) -> Result<()> {
        worksheet
            .write(2, 0, "Timestamp")
            .map_err(|e: XlsxError| ExportError::ExportDataError(format!("Xlsx error: {}", e)))?;

        for i in 0..num_plates {
            worksheet
                .write_string(2, (i + 1) as u16, format!("Temperature {}", i + 1))
                .map_err(|e: XlsxError| {
                    ExportError::ExportDataError(format!("Xlsx error: {}", e))
                })?;

            worksheet
                .write_string(
                    2,
                    (num_plates + i + 1) as u16,
                    format!("Composition x_1 {}", i + 1),
                )
                .map_err(|e: XlsxError| {
                    ExportError::ExportDataError(format!("Xlsx error: {}", e))
                })?;

            worksheet
                .write_string(
                    2,
                    (num_plates * 2 + i + 1) as u16,
                    format!("Composition y_1 {}", i + 1),
                )
                .map_err(|e: XlsxError| {
                    ExportError::ExportDataError(format!("Xlsx error: {}", e))
                })?;
        }
        Ok(())
    }

    fn write_data(
        &self,
        worksheet: &mut rust_xlsxwriter::Worksheet,
        column_data: &Vec<Arc<ColumnEntry>>,
        start_row: usize,
        num_plates: usize,
    ) -> Result<()> {
        for (idx, value) in column_data.iter().enumerate() {
            let row = (idx + start_row) as u32;
            worksheet
                .write(row, 0, value.timestamp)
                .map_err(|e: XlsxError| {
                    ExportError::ExportDataError(format!("Xlsx error: {}", e))
                })?;

            for (i, &temp) in value.temperatures.iter().enumerate() {
                worksheet
                    .write(row, (i + 1) as u16, temp)
                    .map_err(|e: XlsxError| {
                        ExportError::ExportDataError(format!("Xlsx error: {}", e))
                    })?;

                worksheet
                    .write(row, (num_plates + i + 1) as u16, value.compositions[i].x_1)
                    .map_err(|e: XlsxError| {
                        ExportError::ExportDataError(format!("Xlsx error: {}", e))
                    })?;
                worksheet
                    .write(
                        row,
                        (num_plates * 2 + i + 1) as u16,
                        value.compositions[i].y_1,
                    )
                    .map_err(|e: XlsxError| {
                        ExportError::ExportDataError(format!("Xlsx error: {}", e))
                    })?;
            }
        }
        Ok(())
    }
}
