extern crate chrono;
extern crate rusqlite;


use add::Person;
use chrono::prelude::*;
use std::env;
use rusqlite::Connection;


pub fn show_dashboard() {
    let mut db_path_buf = env::home_dir().unwrap();
    db_path_buf.push(".local/share/rusday");
    let conn = Connection::open(db_path_buf.as_path()).unwrap();
    let dt = Local::now();
    let mut stmt = conn.prepare("SELECT id, date, name FROM person WHERE strftime('%d', date) = strftime('%d', ?1) AND strftime('%m', date) = strftime('%m', ?1)").unwrap();
    let person_iter = stmt.query_map(&[&dt], |row| {
        Person {
            id: row.get(0),
            date: row.get(1),
            name: row.get(2)
        }
    }).unwrap();
    for person in person_iter {
        let unwrapped = person.unwrap();
        println!("Today is {}'s {}. birthday.", unwrapped.name, dt.year() - unwrapped.date.year());
    }
}
