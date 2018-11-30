extern crate ansi_term;
extern crate chrono;
extern crate isatty;
extern crate rusqlite;

use self::ansi_term::Colour::Red;
use self::ansi_term::Style;
use chrono::NaiveDate;
use common::Person;
use rusqlite::Connection;

pub fn add_entry(
    conn: &Connection,
    date: &str,
    name: &str,
    color: bool,
    date_fmt: &str,
) -> Result<String, String> {
    if let Ok(naive_date) = NaiveDate::parse_from_str(date, date_fmt) {
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
    } else {
        if color {
            Err(format!(
                    "{}: Failed to add `{}` due to a date parsing error. Are you sure it's formatted correctly?",
                    Red.paint("error"),
                    Style::new().bold().paint(name)
           ))
        } else {
            Err(format!(
                    "error: Failed to add `{}` due to a date parsing error. Are you sure it's formatted correctly?",
                    name
               ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::get_db_conn;
    use std::env;

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

        assert!(add_entry(&conn, "01-01-1990", "Marc", false, "%d-%m-%Y").is_ok());

        env::remove_var("RUSDAY_DB_PATH");
    }

    #[test]
    fn invalid_date_returns_err() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(add_entry(&conn, "invalid-date", "Mike", false, "%d-%m-%Y").is_err());

        env::remove_var("RUSDAY_DB_PATH");
    }
}
