extern crate ansi_term;
extern crate chrono;
extern crate rusqlite;

use self::ansi_term::Style;
use common::Person;
use rusqlite::Connection;

pub fn list_entries(conn: &Connection, color: bool, date_fmt: &str) -> Result<String, String> {
    let mut stmt = conn.prepare("SELECT id, date, name FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| Person {
        id: row.get(0),
        date: row.get(1),
        name: row.get(2),
    }).unwrap();
    let mut persons: Vec<_> = person_iter.map(|r| r.unwrap()).collect();
    persons.sort_by_key(|p| p.date);
    for person in persons {
        if color {
            println!(
                "{}{:>30}{}: {}",
                Style::new().italic().prefix(),
                person.name,
                Style::new().italic().suffix(),
                person.date.format(date_fmt)
            );
        } else {
            println!("{:>30}: {}", person.name, person.date.format(date_fmt));
        }
    }

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

        assert!(list_entries(&conn, false, "%Y-%m-%d").is_ok());

        env::remove_var("RUSDAY_DB_PATH");
    }
}
