use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeDelta};
use error::DataStorageError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::{date_range::DateRange, time_entry::{TimeEntry, TimeEntryData, TimeEntryId}};



pub mod sqlite;
pub mod cache;
pub mod migrate;
pub mod error;
pub mod null;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum StorageImplementation {
    Sqlite,
}




pub trait TimeStorage {
    fn add_entry(&mut self, entry: TimeEntryData) -> Result<TimeEntryId, DataStorageError>;
    fn remove_entry(&mut self, entry_id: TimeEntryId) -> Result<(), DataStorageError>;
    fn update_entry(&mut self, entry_id: TimeEntryId, data: TimeEntryData) -> Result<(), DataStorageError>; 
    fn get_in_range(&self, range: DateRange) -> Result<Vec<TimeEntry>, DataStorageError>;
    fn dyn_clone(&self) -> Box<dyn TimeStorage + Send>;
}


pub trait PlannedHoursStorage {
    fn set(&mut self, date: NaiveDate, duration: TimeDelta) -> Result<(), DataStorageError>;
    fn get(&self, date: NaiveDate) -> Result<TimeDelta, DataStorageError>;
    fn get_range(&self, range: DateRange) -> Result<HashMap<NaiveDate,TimeDelta>, DataStorageError>;
    fn dyn_clone(&self) -> Box<dyn PlannedHoursStorage + Send>;
}

impl Clone for Box<dyn PlannedHoursStorage + Send> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}


impl Clone for Box<dyn TimeStorage + Send> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}