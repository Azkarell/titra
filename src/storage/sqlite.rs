use std::path::PathBuf;

use chrono::Local;
use log::info;
use rusqlite::{Connection, OpenFlags, ToSql};

use crate::TitraConfig;

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
        info!("Create sqlite storage");
        let connection = Connection::open_with_flags(root_dir.clone().join("./db.sqlite"), OpenFlags::SQLITE_OPEN_CREATE)?;
        info!("Create table");
        connection.execute("CREATE TABLE times (
                                    id      INTEGER PRIMARY KEY,
                                    start   TEXT NOT NULL,
                                    end     TEXT NOT NULL,
                                    remark  TEXT
                                    )", ())?;   
        Ok(Self { root_dir, connection })
    }
}

impl TimeStorage for SqliteStorage {
    fn add_entry(&mut self, entry: TimeEntryData) -> Result<super::TimeEntryId, DataStorageError> {
        
        let mut statment = self.connection.prepare("insert into times (start, end, remark) values (?1, ?2, ?3) returning id")?;
        let res = statment.insert((&entry.start.to_sql().unwrap(), &entry.end.to_sql().unwrap(), &entry.remark))?;

        Ok(res)
    }

    fn remove_entry(&mut self, entry_id: super::TimeEntryId) -> Result<(), DataStorageError> {
        todo!()
    }

    fn get_in_range(&self, start: chrono::DateTime<Local>, end: chrono::DateTime<Local>) -> Result<Vec<TimeEntry>, DataStorageError> {
        todo!()
    }
}

