use crate::errors::{Error, FileError, Result};
use log::{error, info};
use serde::Deserialize;
use std::fs;
use std::path::Path;

pub fn read_json_file<T>(path: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    // Ensure the file is a JSON file
    if !path.ends_with(".json") {
        error!("File is not a JSON file: {}", path);
        return Err(Error::FileError(FileError::InvalidFileType));
    }

    // Ensure the file exists
    ensure_parent_directory(path)?;

    // Read the file
    let raw_file = fs::read_to_string(path).map_err(|e| {
        error!("Failed to read file: {}", e);
        FileError::ReadError(e.to_string())
    })?;

    // Parse the JSON file
    let json_file: T = serde_json::from_str(&raw_file).map_err(|e| {
        error!("Failed to parse JSON file: {}", e);
        FileError::ParseJsonError(e.to_string())
    })?;

    Ok(json_file)
}

pub fn write_json_file<T>(path: &str, data: &T) -> Result<()>
where
    T: serde::Serialize,
{
    // Ensure the file is a JSON file
    if !path.ends_with(".json") {
        error!("File is not a JSON file: {}", path);
        return Err(Error::FileError(FileError::InvalidFileType));
    }

    // Ensure the file exists
    ensure_parent_directory(path)?;

    // Serialize the data
    let json_data = serde_json::to_string_pretty(data).map_err(|e| {
        error!("Failed to serialize data: {}", e);
        FileError::SerializeError(e.to_string())
    })?;

    // Write the JSON data to the file
    fs::write(path, json_data).map_err(|e| {
        error!("Failed to write JSON data to file: {}", e);
        FileError::WriteError(e.to_string())
    })?;

    Ok(())
}

fn ensure_parent_directory(path: &str) -> Result<()> {
    let path_dir = Path::new(path).parent().ok_or_else(|| {
        error!("Path has no parent directory: {}", path);
        FileError::InvalidPathError(path.to_string())
    })?;

    // Ensure the directory exists
    if !path_dir.exists() {
        info!("Creating directory: {}", path_dir.display());

        // Create the directory
        fs::create_dir_all(path_dir).map_err(|e| {
            error!("Failed to create directory: {}", e);
            FileError::CreateDirError(e.to_string())
        })?;
    }
    Ok(())
}
