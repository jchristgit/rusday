extern crate ansi_term;
extern crate chrono;
extern crate rusqlite;


use self::ansi_term::Style;
use common::Person;
use chrono::prelude::*;
use rusqlite::Connection;


pub fn show_dashboard(conn: &Connection, color: bool) -> Result<String, String> {
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
        println!(
            "Today is {}'s {}. birthday.",
            if color { Style::new().bold().paint(unwrapped.name).to_string() } else { unwrapped.name },
            dt.year() - unwrapped.date.year()
        );
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

        assert!(show_dashboard(&conn, false).is_ok());

        env::remove_var("RUSDAY_DB_PATH");
    }
}
