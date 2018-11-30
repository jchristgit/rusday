extern crate dirs;
extern crate rusqlite;

use chrono::NaiveDate;
use rusqlite::Connection;
use std::env;
use std::path::PathBuf;
use std::process;

pub struct Person {
    pub id: i32,
    pub date: NaiveDate,
    pub name: String,
}

impl Person {
    pub fn from_args(date: NaiveDate, name: &str) -> Person {
        Person {
            id: 0,
            date: date,
            name: String::from(name),
        }
    }
}

fn get_db_path() -> PathBuf {
    match env::var_os("RUSDAY_DB_PATH") {
        Some(val) => PathBuf::from(val),
        None => dirs::data_dir().expect(
            "Could not determine data directory. \
             Set the `RUSDAY_DB_PATH` environment variable.",
        ),
    }
}

pub fn get_db_conn() -> rusqlite::Connection {
    let db_path = get_db_path();
    match Connection::open(db_path.as_path()) {
        Ok(conn) => {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS person (
                id      INTEGER PRIMARY KEY,
                date    DATE NOT NULL,
                name    TEXT NOT NULL
            )",
                &[],
            ).unwrap();
            conn
        }
        Err(e) => {
            eprintln!("Failed to establish a database connection: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_db_path_with_set_rusday_db_path() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        assert_eq!(get_db_path(), PathBuf::from(":memory:"));

        env::set_var("RUSDAY_DB_PATH", "/home/user/data/rusday.db");
        assert_eq!(get_db_path(), PathBuf::from("/home/user/data/rusday.db"));

        env::remove_var("RUSDAY_DB_PATH");
    }
}
