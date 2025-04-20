use crate::calculations::service::CalculationService;
use crate::calculations::types::CompositionResult;
use crate::data_manager::types::{ColumnEntry, ColumnStructure};
use crate::errors::{ImportError, Result};
use calamine::{open_workbook, Data, DataType, Range, Reader, Xlsx};
use log::info;
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

    pub async fn import(&self, path: &str) -> Result<(usize, Vec<Arc<ColumnEntry>>)> {
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

        let column_structure = self.parse_headers(&range)?;

        let entries = self.process_rows(&range, &column_structure)?;

        Ok((column_structure.number_plates, entries))
    }

    fn parse_headers(&self, range: &Range<Data>) -> Result<ColumnStructure> {
        let headers = range
            .rows()
            .next()
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
    ) -> Result<Vec<Arc<ColumnEntry>>> {
        let mut imported_data: Vec<Arc<ColumnEntry>> = Vec::new();
        let total_rows = range.rows().count().saturating_sub(1);

        let mut x_b0 = 0.0;
        let initial_mass = 1000.0;

        for (index, row) in range.rows().skip(1).enumerate() {
            let percentage_complete = (index as f64 + 1.0) / total_rows as f64 * 100.0;

            if row.is_empty() || row.len() < structure.temperatures_start + structure.number_plates
            {
                continue;
            }

            let timestamp = match row
                .get(structure.timestamp_column)
                .and_then(|cell| cell.as_f64())
            {
                Some(ts) => ts as u64,
                None => continue,
            };

            let temperatures: Vec<f64> = row
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
                    Some(start) => row
                        .iter()
                        .skip(start)
                        .take(structure.number_plates)
                        .map(|cell| cell.as_f64())
                        .collect(),
                    None => vec![None; structure.number_plates],
                };

                let comp_y: Vec<Option<f64>> = match structure.compositions_y_start {
                    Some(start) => row
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

            if compositions.first().and_then(|c| c.x_1).is_some() && x_b0 == 0.0 {
                x_b0 = compositions.first().unwrap().x_1.unwrap();
            }

            let mut distilled_mass = 0.0;
            if imported_data.len() > 0 && x_b0 > 0.0 {
                if let Some(first_comp) = compositions.first() {
                    if let Some(last_comp) = compositions.last() {
                        if let (Some(x_bf), Some(x_d)) = (first_comp.x_1, last_comp.y_1) {
                            println!("\ncount: {}", index);
                            println!("xb0: {}, x_bf: {}, x_d: {}", x_b0, x_bf, x_d);
                            distilled_mass = self.calculation_service.calculate_distilled_mass(
                                initial_mass,
                                0.69,
                                x_bf,
                                x_d,
                            );
                            println!("distilled_mass: {}", distilled_mass);
                        }
                    }
                }
            }

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
