extern crate ansi_term;
extern crate chrono;
extern crate isatty;
extern crate rusqlite;

use self::ansi_term::Style;
use chrono::NaiveDate;
use common::Person;
use rusqlite::Connection;

pub fn add_entry(conn: &Connection, date: &str, name: &str, color: bool) -> Result<String, String> {
    let naive_date = NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap();
    let new_entry = Person::from_args(naive_date, name);
    conn.execute(
        "INSERT INTO person (date, name) VALUES (?1, ?2)",
        &[&new_entry.date, &new_entry.name],
    ).unwrap();
    if color {
        Ok(format!(
            "Successfully added `{}` to the database.",
            Style::new().bold().paint(name)
        ))
    } else {
        Ok(format!("Successfully added `{}` to the database.", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use common::get_db_conn;

    #[test]
    fn person_from_args_as_expected() {
        let birth_date = NaiveDate::from_ymd(1990, 01, 01);
        let person = Person::from_args(birth_date, "Marc");
        assert_eq!(person.id, 0);
        assert_eq!(person.date, birth_date);
        assert_eq!(person.name, "Marc");
    }

    #[test]
    fn valid_insert_returns_ok() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(add_entry(&conn, "01-01-1990", "Marc", false).is_ok());

        env::remove_var("RUSDAY_DB_PATH");
    }
}
