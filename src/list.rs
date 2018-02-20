extern crate chrono;
extern crate rusqlite;


use add::Person;
use rusqlite::Connection;


pub fn list_entries(conn: &Connection) -> Result<String, String> {
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

    Ok(format!(""))
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::get_db_conn;
    use std::env;

    #[test]
    fn returns_ok_with_no_records() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(list_entries(&conn).is_ok());

        env::remove_var("RUSDAY_DB_PATH");
        let _ = conn.close();
    }
}
