extern crate chrono;
extern crate rusqlite;


use add::Person;
use std::env;
use rusqlite::Connection;


pub fn list_entries() {
    let mut db_path_buf = env::home_dir().unwrap();
    db_path_buf.push(".local/share/rusday");
    let conn = Connection::open(db_path_buf.as_path()).unwrap();
    let mut stmt = conn.prepare("SELECT id, date, name FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            date: row.get(1),
            name: row.get(2)
        }
    }).unwrap();
    let mut persons: Vec<_> = person_iter.map(|r| r.unwrap()).collect();
    persons.sort_by_key(|p| p.date);
    for person in persons {
        println!("{:>20}: {}", person.name, person.date);
    };
}
