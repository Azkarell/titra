use std::sync::Arc;

use chrono::{DateTime, Local};
use egui::mutex::RwLock;

use crate::views::overview_table::DateRange;

use super::{DataStorageError, TimeEntry, TimeStorage};

#[derive(Clone)]
pub struct SharedQueryResult {
    last_query: Arc<RwLock<Option<DateRange>>>,
    last_result: Arc<RwLock<Option<Result<Vec<TimeEntry>, DataStorageError>>>>,
}

impl SharedQueryResult {
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
        result: Result<Vec<TimeEntry>, DataStorageError>,
    ) {
        let mut g = self.last_query.write();
        *g = Some(query);
        let mut g = self.last_result.write();
        *g = Some(result);
    }

    pub fn get_cached(
        &self,
        query: DateRange,
    ) -> Option<Result<Vec<TimeEntry>, DataStorageError>> {
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

pub struct CachedStorage<S: TimeStorage> {
    imp: S,
    last_query: SharedQueryResult,
}

impl<S: TimeStorage + Clone + Send + 'static> TimeStorage for CachedStorage<S> {
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
    ) -> Result<Vec<super::TimeEntry>, super::DataStorageError> {
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
}

impl<S: TimeStorage> CachedStorage<S> {
    pub fn new(imp: S) -> Self {
        Self {
            imp,
            last_query: SharedQueryResult::new()
        }
    }

    pub fn do_query(
        &self,
        range: DateRange
    ) -> Result<Vec<super::TimeEntry>, super::DataStorageError> {
        let res = self.imp.get_in_range(range)?;
        self.last_query.set_result(range ,Ok(res.clone()));
        Ok(res)
    }
}
