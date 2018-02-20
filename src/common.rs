extern crate rusqlite;

use std::env;
use std::path::PathBuf;
use std::process;
use rusqlite::Connection;


pub fn get_db_conn() -> rusqlite::Connection {
    let db_path = match env::var_os("RUSDAY_DB_PATH") {
        Some(val) => PathBuf::from(val),
        None => {
            match env::var_os("XDG_DATA_HOME") {
                Some(data_home) => {
                    let mut data_home_buf = PathBuf::from(data_home);
                    data_home_buf.push("rusday.db");
                    data_home_buf
                },
                None => {
                    let mut db_path_buf = env::home_dir().unwrap();
                    db_path_buf.push(".local/share/rusday.db");
                    db_path_buf
                } 
            }
        }
    };

    match Connection::open(db_path.as_path()) {
        Ok(conn) => {
            conn.execute("CREATE TABLE IF NOT EXISTS person (
                id      INTEGER PRIMARY KEY,
                date    DATE NOT NULL,
                name    TEXT NOT NULL
            )", &[]).unwrap();
            conn
        },
        Err(e) => {
            eprintln!("Failed to establish a database connection: {}", e);
            process::exit(1);
        }
    }
}
