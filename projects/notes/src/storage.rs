use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::model::AppData;

#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    Serde(serde_json::Error),
    NoDataDir,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::Io(e) => write!(f, "I/O error: {}", e),
            StorageError::Serde(e) => write!(f, "serialization error: {}", e),
            StorageError::NoDataDir => write!(f, "could not determine data directory"),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<io::Error> for StorageError {
    fn from(e: io::Error) -> Self {
        StorageError::Io(e)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(e: serde_json::Error) -> Self {
        StorageError::Serde(e)
    }
}

pub fn data_path() -> Result<PathBuf, StorageError> {
    if let Ok(custom) = std::env::var("NOTES_DATA_FILE") {
        return Ok(PathBuf::from(custom));
    }
    let dir = dirs::data_dir().ok_or(StorageError::NoDataDir)?;
    Ok(dir.join("notes").join("notes.json"))
}

pub fn load(path: &Path) -> Result<AppData, StorageError> {
    if !path.exists() {
        return Ok(AppData::default());
    }
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(AppData::default());
    }
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn save(path: &Path, data: &AppData) -> Result<(), StorageError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("json.tmp");
    let json = serde_json::to_string_pretty(data)?;
    fs::write(&tmp, json)?;
    if let Err(e) = fs::rename(&tmp, path) {
        fs::write(path, serde_json::to_string_pretty(data)?)?;
        let _ = fs::remove_file(&tmp);
        return Err(StorageError::Io(e));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Note, Reminder, Schedule};
    use chrono::{Local, TimeZone};

    #[test]
    fn save_load_roundtrip() {
        let tmp = std::env::temp_dir().join(format!("notes_test_{}.json", uuid::Uuid::new_v4()));
        let mut data = AppData::default();
        let mut note = Note::new("Hello".into(), "world".into());
        note.reminders.push(Reminder::new(
            "ping".into(),
            Schedule::OneTime {
                at: Local.with_ymd_and_hms(2099, 1, 1, 0, 0, 0).unwrap(),
            },
        ));
        data.notes.push(note);

        save(&tmp, &data).unwrap();
        let loaded = load(&tmp).unwrap();
        assert_eq!(loaded.notes.len(), 1);
        assert_eq!(loaded.notes[0].title, "Hello");
        assert_eq!(loaded.notes[0].reminders.len(), 1);

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn load_missing_returns_default() {
        let tmp = std::env::temp_dir().join(format!("notes_missing_{}.json", uuid::Uuid::new_v4()));
        let loaded = load(&tmp).unwrap();
        assert!(loaded.notes.is_empty());
    }
}
