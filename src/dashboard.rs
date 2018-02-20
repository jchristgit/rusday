extern crate chrono;
extern crate rusqlite;


use add::Person;
use chrono::prelude::*;
use rusqlite::Connection;


pub fn show_dashboard(conn: &Connection) -> Result<String, String> {
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
    };

    Ok(format!(""))
}



#[cfg(test)]
mod tests {
    use common::get_db_conn;
    use std::env;
    use super::*;

    #[test]
    fn returns_ok_with_no_records() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(show_dashboard(&conn).is_ok());

        env::remove_var("RUSDAY_DB_PATH");
        let _ = conn.close();
    }
}
