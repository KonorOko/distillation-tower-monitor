use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Settings error")]
    SettingsError(#[from] SettingsError),
    #[error("File error")]
    FileError(#[from] FileError),
    #[error("Modbus error")]
    ModbusError(#[from] ModbusError),
    #[error("Math error")]
    RootError(#[from] RootError),
    #[error("Data error")]
    DataError(#[from] DataError),
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum SettingsError {
    #[error("Failed to load settings: {0}")]
    LoadError(String),
    #[error("Failed to save settings: {0}")]
    SaveError(String),
    #[error("Failed to get usb devices")]
    GetUsbDevicesError(String),
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum FileError {
    #[error("Failed to read file: {0}")]
    ReadError(String),
    #[error("Failed to write file: {0}")]
    WriteError(String),
    #[error("Failed to ensure file: {0}")]
    EnsureFileError(String),
    #[error("Invalid file type, expect JSON")]
    InvalidFileType,
    #[error("Parse Json error: {0}")]
    ParseJsonError(String),
    #[error("Create directory error: {0}")]
    CreateDirError(String),
    #[error("Serialize error: {0}")]
    SerializeError(String),
    #[error("Invalid path error: {0}")]
    InvalidPathError(String),
}

#[derive(Error, Serialize, Debug, Deserialize)]
pub enum ModbusError {
    #[error("Failed to connect modbus {0}")]
    ConnectionError(String),
    #[error("Failed to read coils {0}")]
    ReadCoilsError(String),
    #[error("Failed to write coils {0}")]
    WriteCoilsError(String),
    #[error("Failed to read holding registers {0}")]
    ReadHoldingRegistersError(String),
    #[error("Failed to write holding registers {0}")]
    WriteHoldingRegistersError(String),
    #[error("Failed to write single coil {0}")]
    WriteSingleCoilError(String),
    #[error("Failed to write single register {0}")]
    WriteSingleRegisterError(String),
}

#[derive(Error, Serialize, Debug, Deserialize)]
pub enum RootError {
    #[error("Not founded root")]
    NotFoundedRoot,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Negative root")]
    NegativeRootError,
}

#[derive(Error, Serialize, Debug, Deserialize)]
pub enum DataError {
    #[error("Data empty")]
    EmptyDataError,
    #[error("No more data")]
    NoMoreDataError,
}

impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}
