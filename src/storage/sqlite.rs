use std::path::PathBuf;

use chrono::Local;
use log::debug;
use rusqlite::{Connection, ToSql};
use fallible_iterator::FallibleIterator;

use crate::views::overview_table::DateRange;

use super::{DataStorageError, TimeEntry, TimeEntryData, TimeStorage};


impl From<rusqlite::Error> for DataStorageError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Unknown(value.to_string())
    }
}

pub struct SqliteStorage {
    root_dir: PathBuf,
    connection: rusqlite::Connection
}

impl SqliteStorage {
    pub fn new(root_dir: PathBuf) -> Result<Self,DataStorageError> {
        debug!("Create sqlite storage");
        let path = std::path::absolute(root_dir.clone().join("db.sqlite")).expect("get path");
        debug!("Path: {:?}", path);
        let connection = Connection::open(path)?;

        debug!("Create table");
        let res = connection.execute("CREATE TABLE times (
                                    id      INTEGER PRIMARY KEY,
                                    start   TEXT NOT NULL,
                                    end     TEXT NOT NULL,
                                    date    TEXT NOT NULL,
                                    remark  TEXT
                                    )", ());   
                                if let Err(err) = res {
                                    debug!("Error: {}", err.to_string());
                                }
        Ok(Self { root_dir, connection })
    }
}

impl Clone for SqliteStorage {
    fn clone(&self) -> Self {
        Self::new(self.root_dir.clone()).unwrap()
    }
}

impl TimeStorage for SqliteStorage {
    fn add_entry(&mut self, entry: TimeEntryData) -> Result<super::TimeEntryId, DataStorageError> {
        
        debug!("Inserting: {:?}", entry);
        let mut statment = self.connection.prepare("insert into times (start, end, date, remark) values (?1, ?2, ?3, ?4)")?;
        let res = statment.insert((&entry.start.to_sql().unwrap(), &entry.end.to_sql().unwrap(), &entry.date.to_sql().unwrap(),  &entry.remark))?;

        Ok(res)
    }

    fn remove_entry(&mut self, entry_id: super::TimeEntryId) -> Result<(), DataStorageError> {
        debug!("Deleting entry: {}", entry_id);
        let mut statement = self.connection.prepare("Delete from times where id = ?1")?;
        let _res = statement.execute([entry_id])?;
        Ok(())
    }

    fn get_in_range(&self, range: DateRange) -> Result<Vec<TimeEntry>, DataStorageError> {
        debug!("query data: {:?}", range);
        let mut statement = self.connection.prepare("SELECT id, start, end, date, remark from times where date(date) >= ?1 and date(date) <= ?2 order by date asc")?;
        let res = statement.query((range.0.to_sql()?, range.1.to_sql()?))?;

        let mapped = res.map(|e| Ok((e.get(0)?, TimeEntryData{
            end: e.get(2)?,
            start: e.get(1)?,
            date: e.get(3)?,
            remark: e.get(4)?
        })));
        Ok(mapped.collect()?)
    }
    
    fn dyn_clone(&self) -> Box<dyn TimeStorage + Send> {
        Box::new(self.clone())
    }
}

