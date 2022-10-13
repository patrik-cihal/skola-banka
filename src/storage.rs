use std::path::Path;
use serde::{Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum StorageError {
    WrongPath,
    WrongFileFormat
}

pub trait Storage {
    fn save<T: Serialize>(obj: &T, path: &Path) -> Result<(), StorageError>;
    fn load<T: DeserializeOwned>(path: &Path) -> Result<T, StorageError>;
}

pub struct FileStorage;

impl Storage for FileStorage {
    fn save<T: Serialize>(obj: &T, path: &Path) -> Result<(), StorageError> {
        let serialized = serde_json::to_string_pretty(obj).map_err(|e| {
            StorageError::WrongFileFormat
        })?;
        std::fs::write(path, serialized).map_err(|_| StorageError::WrongPath)
    }
    fn load<T: DeserializeOwned>(path: &Path) -> Result<T, StorageError> {
        let serialized = std::fs::read_to_string(path).map_err(|_| StorageError::WrongPath)?;

        serde_json::from_str::<T>(&serialized).map_err(|_| StorageError::WrongFileFormat)
    }
}