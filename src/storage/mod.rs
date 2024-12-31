use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::views::overview_table::DateRange;


pub mod sqlite;
pub mod cache;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum StorageImplementation {
    Sqlite,
}

pub type TimeEntryId = i64;

#[derive(Debug, Clone)]
pub struct TimeEntryData {
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub date: NaiveDate,
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

    fn get_in_range(&self, range: DateRange) -> Result<Vec<TimeEntry>, DataStorageError>;
    fn dyn_clone(&self) -> Box<dyn TimeStorage + Send>;
}



impl Clone for Box<dyn TimeStorage + Send> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}