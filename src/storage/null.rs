use crate::model::{date_range::DateRange, time_entry::{TimeEntry, TimeEntryData, TimeEntryId}};

use super::{error::DataStorageError, PlannedHoursStorage, TimeStorage};

pub struct NullService;

impl PlannedHoursStorage for NullService {
    fn set(&mut self, date: chrono::NaiveDate, duration: chrono::TimeDelta) -> Result<(), DataStorageError> {
        todo!()
    }

    fn get(&self, date: chrono::NaiveDate) -> Result<chrono::TimeDelta, DataStorageError> {
        todo!()
    }

    fn get_range(&self, range: DateRange) -> Result<std::collections::HashMap<chrono::NaiveDate,chrono::TimeDelta>, DataStorageError> {
        todo!()
    }

    fn dyn_clone(&self) -> Box<dyn PlannedHoursStorage + Send> {
        todo!()
    }
}

impl TimeStorage for NullService {
    fn add_entry(&mut self, entry: TimeEntryData) -> Result<TimeEntryId, DataStorageError> {
        todo!()
    }

    fn remove_entry(&mut self, entry_id: TimeEntryId) -> Result<(), DataStorageError> {
        todo!()
    }

    fn update_entry(&mut self, entry_id: TimeEntryId, data: TimeEntryData) -> Result<(), DataStorageError> {
        todo!()
    }

    fn get_in_range(&self, range: DateRange) -> Result<Vec<TimeEntry>, DataStorageError> {
        todo!()
    }

    fn dyn_clone(&self) -> Box<dyn TimeStorage + Send> {
        todo!()
    }
}
