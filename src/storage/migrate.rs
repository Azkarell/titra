use log::debug;
use rusqlite::{Connection, Error};

pub fn migrate_db(connection: &mut Connection) {
    debug!("Create tables");
    let res = connection.execute(
        "CREATE TABLE times (
                                id      INTEGER PRIMARY KEY,
                                start   TEXT NOT NULL,
                                end     TEXT NOT NULL,
                                date    TEXT NOT NULL,
                                remark  TEXT
                                )",
        (),
    );
    if let Err(err) = res {
        debug!("Error: {}", err.to_string());
    }

    let res = connection.execute("CREATE TABLE user_data (
                                                                                    id          INTEGER PRIMARY KEY,
                                                                                    name        TEXT NOT NULL,
                                                                                    street      TEXT,
                                                                                    citycode    TEXT
                                                                                    )", ());
    if let Err(err) = res {
        debug!("Error: {}", err.to_string());
    }

    let res = connection.execute("CREATE TABLE planned_hours (
        date        TEXT PRIMARY KEY,
        hours       TEXT NOT NULL
        )", ());
if let Err(err) = res {
debug!("Error: {}", err.to_string());
}
}
