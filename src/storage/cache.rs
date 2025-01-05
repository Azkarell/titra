use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Local, NaiveDate, TimeDelta};
use egui::mutex::RwLock;

use crate::model::{date_range::DateRange, time_entry::{TimeEntryData, TimeEntryId}};

use super::{DataStorageError, PlannedHoursStorage, TimeEntry, TimeStorage};

#[derive(Clone)]
pub struct SharedQueryResult<T> {
    last_query: Arc<RwLock<Option<DateRange>>>,
    last_result: Arc<RwLock<Option<Result<T, DataStorageError>>>>,
}

impl<T: Clone> SharedQueryResult<T> {
    pub fn new() -> Self {
        Self {
            last_query: Arc::new(RwLock::new(None)),
            last_result: Arc::new(RwLock::new(None))
        }
    }
    pub fn invalidate(&self) {
        let mut g = self.last_query.write();
        *g = None;
    }
    pub fn set_result(
        &self,
        query: DateRange,
        result: Result<T, DataStorageError>,
    ) {
        let mut g = self.last_query.write();
        *g = Some(query);
        let mut g = self.last_result.write();
        *g = Some(result);
    }

    pub fn get_cached(
        &self,
        query: DateRange,
    ) -> Option<Result<T, DataStorageError>> {
        let r = self.last_query.read();
        if let Some((start, end)) = *r {
            if start == query.0 && end == query.1 {
                let r2 = self.last_result.read();
                return r2.clone();
            } 
        } 
        None
    }
}


pub struct CachedStorage<S, T> {
    imp: S,
    last_query: SharedQueryResult<T>,
}

impl<S: TimeStorage + Clone + Send + 'static> TimeStorage for CachedStorage<S, Vec<TimeEntry>> {
    fn add_entry(
        &mut self,
        entry: super::TimeEntryData,
    ) -> Result<super::TimeEntryId, super::DataStorageError> {
        self.last_query.invalidate();
        self.imp.add_entry(entry)
    }

    fn remove_entry(
        &mut self,
        entry_id: super::TimeEntryId,
    ) -> Result<(), super::DataStorageError> {
        self.last_query.invalidate();
        self.imp.remove_entry(entry_id)
    }

    fn get_in_range(
        &self,
        range: DateRange
    ) -> Result<Vec<TimeEntry>, super::DataStorageError> {
        let cached = self.last_query.get_cached(range);
        if let Some(val) = cached {
            return val;
        }
        self.do_query(range)
    }

    fn dyn_clone(&self) -> Box<dyn TimeStorage + Send> {
        Box::new(Self{
            imp: self.imp.clone(),
            last_query: self.last_query.clone()
        })
    }
    
    fn update_entry(&mut self, entry_id: TimeEntryId, data: TimeEntryData) -> Result<(), DataStorageError> {
        self.imp.update_entry(entry_id, data)?;
        self.last_query.invalidate();
        Ok(())
    }
}

impl<S: TimeStorage> CachedStorage<S, Vec<TimeEntry>> {
    pub fn new_time(imp: S) -> Self {
        Self {
            imp,
            last_query: SharedQueryResult::new()
        }
    }

    pub fn do_query(
        &self,
        range: DateRange
    ) -> Result<Vec<TimeEntry>, super::DataStorageError> {
        let res = self.imp.get_in_range(range)?;
        self.last_query.set_result(range ,Ok(res.clone()));
        Ok(res)
    }
}

impl<S: PlannedHoursStorage> CachedStorage<S, HashMap<NaiveDate,TimeDelta>> {
    pub fn new_hours(imp: S) -> Self {
        Self {
            imp,
            last_query: SharedQueryResult::new()
        }
    }

    pub fn do_query(
        &self,
        range: DateRange
    ) -> Result<HashMap<NaiveDate,super::TimeDelta>, super::DataStorageError> {
        let res = self.imp.get_range(range)?;
        self.last_query.set_result(range ,Ok(res.clone()));
        Ok(res)
    }
}


impl<S: PlannedHoursStorage + Clone + Send + 'static> PlannedHoursStorage for CachedStorage<S, HashMap<NaiveDate,TimeDelta>> {
    fn set(&mut self, date: chrono::NaiveDate, duration: TimeDelta) -> Result<(), DataStorageError> {
        self.last_query.invalidate();
        self.imp.set(date, duration)
    }

    fn get(&self, date: chrono::NaiveDate) -> Result<TimeDelta, DataStorageError> {
        self.imp.get(date)
    }

    fn get_range(&self, range: DateRange) -> Result<HashMap<NaiveDate,TimeDelta>, DataStorageError> {
        let cached = self.last_query.get_cached(range);
        if let Some(val) = cached {
            return val;
        }
        self.do_query(range)
    }

    fn dyn_clone(&self) -> Box<dyn PlannedHoursStorage + Send> {
        Box::new(Self{
            imp: self.imp.clone(),
            last_query: self.last_query.clone()
        })
    }
}