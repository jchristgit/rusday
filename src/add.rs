extern crate chrono;
extern crate rusqlite;

use chrono::NaiveDate;
use rusqlite::Connection;
use std::env;



#[derive(Debug)]
pub struct Person {
    pub id: i32,
    pub date: NaiveDate,
    pub name: String
}


pub fn add_entry(date: &str, name: &str) {
    let mut db_path_buf = env::home_dir().unwrap();
    db_path_buf.push(".local/share/rusday");
    let conn = Connection::open(db_path_buf.as_path()).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS person (
        id      INTEGER PRIMARY KEY,
        date    DATE NOT NULL,
        name    TEXT NOT NULL
        )", &[]).unwrap();
    let new_entry = Person {
        id: 0,
        date: NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap(),
        name: String::from(name)
    };
    conn.execute("INSERT INTO person (date, name) VALUES (?1, ?2)", &[&new_entry.date, &new_entry.name]).unwrap();
    let _ = conn.close();
}
