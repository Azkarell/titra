use std::path::PathBuf;

use chrono::{DateTime, Local};
use log::debug;
use rusqlite::{types::FromSql, Connection, ToSql};
use fallible_iterator::FallibleIterator;

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
                                    remark  TEXT
                                    )", ());   
                                if let Err(err) = res {
                                    debug!("Error: {}", err.to_string());
                                }
        Ok(Self { root_dir, connection })
    }
}

impl TimeStorage for SqliteStorage {
    fn add_entry(&mut self, entry: TimeEntryData) -> Result<super::TimeEntryId, DataStorageError> {
        
        let mut statment = self.connection.prepare("insert into times (start, end, remark) values (?1, ?2, ?3)")?;
        let res = statment.insert((&entry.start.to_sql().unwrap(), &entry.end.to_sql().unwrap(), &entry.remark))?;

        Ok(res)
    }

    fn remove_entry(&mut self, entry_id: super::TimeEntryId) -> Result<(), DataStorageError> {
        todo!()
    }

    fn get_in_range(&self, start: chrono::DateTime<Local>, end: chrono::DateTime<Local>) -> Result<Vec<TimeEntry>, DataStorageError> {
        let mut statement = self.connection.prepare("SELECT id, start, end, remark from times where datetime(start) >= ?1 and datetime(end) <= ?2")?;
        let res = statement.query((start.to_sql()?, end.to_sql()?))?;

        let mapped = res.map(|e| Ok((e.get(0)?, TimeEntryData{
            end: e.get(2)?,
            start: e.get(1)?,
            remark: e.get(3)?
        })));
        Ok(mapped.collect()?)
    }
}

