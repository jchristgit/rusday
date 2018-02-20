extern crate rusqlite;

use std::env;
use std::process;
use rusqlite::Connection;


pub fn get_db_conn() -> rusqlite::Connection {
    let mut db_path_buf = env::home_dir().unwrap();
    db_path_buf.push(".local/share/rusday.db");
    match Connection::open(db_path_buf.as_path()) {
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
