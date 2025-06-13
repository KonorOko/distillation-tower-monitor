use crate::data_manager::types::ColumnEntry;
use crate::storage::types::{CalculationHistory, CalculationStep};
use log::{debug, error, info, warn};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;

pub struct CalculationHistoryService {
    base_dir: PathBuf,
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

    pub async fn start_new_history(
        &self,
        initial_mass: f32,
        initial_concentration: f32,
    ) -> io::Result<()> {
        let history = CalculationHistory::new(initial_mass, initial_concentration);

        let filename = format!("distillation_history_{}.json", history.start_time);
        let file_path = self.base_dir.join(filename);

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)?;

        let writer = BufWriter::new(file);

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

    pub async fn record_calculation_step(
        &self,
        prev_entry: &Arc<ColumnEntry>,
        current_entry: &Arc<ColumnEntry>,
        initial_mass: f32,
    ) -> io::Result<()> {
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

        let step_index = history.steps.len();
        let x_re = prev_entry
            .compositions
            .first()
            .and_then(|c| c.x_1)
            .unwrap_or(0.0) as f32;
        let x_do = prev_entry
            .compositions
            .last()
            .and_then(|c| c.y_1)
            .unwrap_or(0.0) as f32;
        let next_x_re = current_entry
            .compositions
            .first()
            .and_then(|c| c.x_1)
            .unwrap_or(0.0) as f32;
        let next_x_do = current_entry
            .compositions
            .last()
            .and_then(|c| c.y_1)
            .unwrap_or(0.0) as f32;

        let delta_x = next_x_re - x_re;
        let f_0 = Self::calculate_function(x_re, x_do);
        let f_1 = Self::calculate_function(next_x_re, next_x_do);

        let partial_integral = 0.5 * (f_0 + f_1) * delta_x;

        let accumulated_integral = if let Some(last_step) = history.steps.last() {
            last_step.accumulated_integral + partial_integral
        } else {
            partial_integral
        };

        let remaining_mass = accumulated_integral.exp() * initial_mass;
        let distilled_mass = initial_mass - remaining_mass;

        let step = CalculationStep {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as u32,
            step_index,
            x_re,
            x_do,
            temp_re: current_entry.temperatures.first().copied().unwrap_or(0.0) as f32,
            temp_do: current_entry.temperatures.last().copied().unwrap_or(0.0) as f32,
            delta_x,
            f_0,
            f_1,
            partial_integral,
            accumulated_integral,
            remaining_mass,
            distilled_mass,
        };

        history.steps.push(step.clone());
        history.end_time = step.timestamp;

        self.write_step_to_file(&step).await?;

        Ok(())
    }

    fn calculate_function(x_re: f32, x_do: f32) -> f32 {
        if (x_re - x_do).abs() < 1e-6 {
            return 0.0;
        }

        1.0 / (x_re + x_do)
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

    pub fn load_history(file_path: &Path) -> io::Result<CalculationHistory> {
        let content = fs::read_to_string(file_path)?;
        let history: CalculationHistory = serde_json::from_str(&content)?;
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
}
