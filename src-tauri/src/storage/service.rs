use crate::calculations::types::{CalculationHistory, CalculationStep, HistorySummary};
use crate::data_manager::types::ColumnEntry;
use log::{debug, error, info};
use serde_json;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct CalculationHistoryService {
    pub base_dir: PathBuf,
    current_history: Arc<Mutex<Option<CalculationHistory>>>,
    file_writer: Arc<Mutex<Option<BufWriter<File>>>>,
}

impl CalculationHistoryService {
    pub fn new(base_dir: impl AsRef<Path>) -> io::Result<Self> {
        let base_dir = base_dir.as_ref().to_path_buf();
        fs::create_dir_all(&base_dir)?;

        Ok(Self {
            base_dir,
            current_history: Arc::new(Mutex::new(None)),
            file_writer: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn get_current_session_data(&self) -> io::Result<Vec<Arc<ColumnEntry>>> {
        let history_guard = self.current_history.lock().await;
        let history = match &*history_guard {
            Some(h) => h,
            None => return Ok(Vec::new()),
        };

        let entries = history
            .steps
            .iter()
            .map(|step| {
                Arc::new(ColumnEntry {
                    timestamp: step.timestamp,
                    temperatures: step.temperatures.clone(),
                    compositions: step.compositions.clone(),
                    percentage_complete: 0.0,
                    distilled_mass: step.distilled_mass.unwrap_or(0.0) as f64,
                })
            })
            .collect();
        Ok(entries)
    }

    pub async fn _export_to_excel(
        &self,
        history_id: Option<String>,
        _path: &str,
    ) -> io::Result<()> {
        let _history = if let Some(id) = history_id {
            self.load_history(&self.base_dir.join(id))?
        } else {
            let history_guard = self.current_history.lock().await;
            match &*history_guard {
                Some(h) => h.clone(),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "No current history found",
                    ))
                }
            }
        };

        Ok(())
    }

    pub async fn start_new_history(
        &self,
        number_plates: usize,
        initial_mass: f64,
        initial_concentration: f64,
    ) -> io::Result<()> {
        let history = CalculationHistory::new(number_plates, initial_mass, initial_concentration);

        let filename = format!("distillation_history_{}.json", history.start_time);
        let file_path = self.base_dir.join(filename);

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)?;

        let mut writer = BufWriter::new(file);

        let header = serde_json::json!({
            "id": history.id,
            "initial_mass": history.initial_mass,
            "initial_concentration": history.initial_concentration,
            "number_plates": history.number_plates,
            "start_time": history.start_time,
            "end_time": history.end_time,
            "file_size": history.file_size,
        });

        writeln!(writer, "# HEADER: {}", serde_json::to_string(&header)?)?;
        writer.flush()?;

        {
            let mut file_writer = self.file_writer.lock().await;
            *file_writer = Some(writer);
        }

        {
            let mut current_history = self.current_history.lock().await;
            *current_history = Some(history);
        }

        info!("Started new calculation history at {}", file_path.display());
        Ok(())
    }

    pub async fn record_step(&self, step: CalculationStep) -> io::Result<()> {
        let mut history_guard = self.current_history.lock().await;
        let history = match history_guard.as_mut() {
            Some(h) => h,
            None => {
                error!("No current history found, cannot record step.");
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "No current history found",
                ));
            }
        };

        history.steps.push(step.clone());
        history.end_time = step.timestamp;

        self.write_step_to_file(&step).await?;

        Ok(())
    }

    async fn write_step_to_file(&self, step: &CalculationStep) -> io::Result<()> {
        let mut writer_guard = self.file_writer.lock().await;

        if let Some(writer) = writer_guard.as_mut() {
            let json = serde_json::to_string(step)?;

            writeln!(writer, "{}", json)?;
            writer.flush()?;

            debug!("Recorded step saved: {:?}", step);
            Ok(())
        } else {
            error!("File writer is not initialized, cannot write step.");
            Err(io::Error::new(
                io::ErrorKind::Other,
                "File writer not initialized",
            ))
        }
    }

    pub async fn finish_current_history(&self) -> io::Result<Option<PathBuf>> {
        let file_path = {
            let mut writer_guard = self.file_writer.lock().await;
            if let Some(mut writer) = writer_guard.take() {
                writer.flush()?;
                debug!("Finished writing current history to file.");
            }

            let history_guard = self.current_history.lock().await;
            match &*history_guard {
                Some(history) => {
                    let filename = format!("distillation_history_{}.json", history.start_time);
                    Some(self.base_dir.join(filename))
                }
                None => None,
            }
        };

        {
            let mut history_guard = self.current_history.lock().await;
            *history_guard = None;
        }

        info!("Finished current calculation history.");
        Ok(file_path)
    }

    pub fn load_history(&self, file_path: &Path) -> io::Result<CalculationHistory> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut lines = io::BufRead::lines(reader);

        let header_line = match lines.next() {
            Some(Ok(line)) => line,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Empty history file",
                ))
            }
        };

        let header_json = if header_line.starts_with("# HEADER: ") {
            &header_line[9..]
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid history file format",
            ));
        };
        let header: serde_json::Value = serde_json::from_str(header_json)?;

        let id = header["id"]
            .as_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing id in header"))?
            .to_string();
        let initial_mass = header["initial_mass"].as_f64().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Missing initial_mass in header")
        })?;
        let initial_concentration = header["initial_concentration"].as_f64().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing initial_concentration in header",
            )
        })?;
        let number_plates = header["number_plates"].as_u64().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing number_plates in header",
            )
        })? as u32;
        let start_time = header["start_time"].as_u64().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Missing start_time in header")
        })? as u32;
        let mut end_time = start_time;

        let mut steps = Vec::new();
        for (i, line_result) in lines.enumerate() {
            let line = line_result?;
            let step: CalculationStep = match serde_json::from_str(&line) {
                Ok(step) => step,
                Err(e) => {
                    error!("Failed to parse step {}: {}", i + 1, e);
                    continue;
                }
            };

            if step.timestamp > end_time {
                end_time = step.timestamp;
            }

            steps.push(step);
        }

        let history = CalculationHistory {
            id,
            initial_mass,
            initial_concentration,
            number_plates,
            steps,
            start_time,
            end_time,
            file_size: Some(std::fs::metadata(file_path)?.len() as u32),
            description: None,
        };

        Ok(history)
    }

    pub fn list_history_files(&self) -> io::Result<Vec<PathBuf>> {
        let entries = fs::read_dir(&self.base_dir)?;

        let mut history_files = Vec::new();
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && path.extension().map_or(false, |ext| ext == "json")
                && path.file_name().map_or(false, |name| {
                    name.to_string_lossy().starts_with("distillation_history")
                })
            {
                history_files.push(path);
            }
        }

        history_files.sort_by(|a, b| {
            let a_name = a.file_name().unwrap_or_default().to_string_lossy();
            let b_name = b.file_name().unwrap_or_default().to_string_lossy();
            b_name.cmp(&a_name)
        });

        Ok(history_files)
    }

    pub fn load_history_summary(&self, file_path: &Path) -> io::Result<HistorySummary> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(&file);
        let mut lines = io::BufRead::lines(reader);

        let id = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let first_line = match lines.next() {
            Some(Ok(line)) => line,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Empty history file",
                ));
            }
        };

        let mut steps_count = 0;

        for _ in lines {
            steps_count += 1;
        }

        if first_line.starts_with("# HEADER: ") {
            let header_json = &first_line[9..];
            let header: serde_json::Value = serde_json::from_str(header_json)?;

            return Ok(HistorySummary {
                id,
                initial_mass: header["initial_mass"].as_f64().unwrap_or(0.0),
                initial_concentration: header["initial_concentration"].as_f64().unwrap_or(0.0),
                start_time: header["start_time"].as_u64().unwrap_or(0) as u32,
                end_time: header["end_time"].as_u64().unwrap_or(0) as u32,
                steps_count,
                number_plates: header["number_plates"].as_u64().unwrap_or(0) as u32,
                file_size: Some(file.metadata()?.len() as u32),
            });
        } else {
            error!("Invalid history file format: {}", file_path.display());
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid history file format",
            ));
        }
    }
    pub fn convert_history_to_entries(
        &self,
        history: &CalculationHistory,
    ) -> Vec<Arc<ColumnEntry>> {
        history
            .steps
            .iter()
            .map(|step| {
                Arc::new(ColumnEntry {
                    timestamp: step.timestamp,
                    temperatures: step.temperatures.clone(),
                    compositions: step.compositions.clone(),
                    percentage_complete: step.step_index as f64 / history.steps.len() as f64
                        * 100.0,
                    distilled_mass: step.distilled_mass.unwrap_or(0.0) as f64,
                })
            })
            .collect()
    }
}
