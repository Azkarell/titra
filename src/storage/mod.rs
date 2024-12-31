use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use thiserror::Error;


pub mod sqlite;
pub mod cache;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum StorageImplementation {
    Sqlite,
}

pub type TimeEntryId = i64;

#[derive(Debug, Clone)]
pub struct TimeEntryData {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub remark: Option<String>
}

pub type TimeEntry = (TimeEntryId, TimeEntryData);

#[derive(Error, Debug, Clone)]
pub enum DataStorageError {
    #[error("Unknown error occured: {0}")]
    Unknown(String)
}

pub trait TimeStorage {
    fn add_entry(&mut self, entry: TimeEntryData) -> Result<TimeEntryId, DataStorageError>;
    fn remove_entry(&mut self, entry_id: TimeEntryId) -> Result<(), DataStorageError>;

    fn get_in_range(&self, start: chrono::DateTime<Local>, end: chrono::DateTime<Local>) -> Result<Vec<TimeEntry>, DataStorageError>;
    fn dyn_clone(&self) -> Box<dyn TimeStorage + Send>;
}



impl Clone for Box<dyn TimeStorage + Send> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}