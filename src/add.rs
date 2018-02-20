extern crate chrono;
extern crate rusqlite;

use chrono::NaiveDate;
use rusqlite::Connection;


#[derive(Debug)]
pub struct Person {
    pub id: i32,
    pub date: NaiveDate,
    pub name: String
}

impl Person {
    fn from_args(date: NaiveDate, name: &str) -> Person {
        Person {
            id: 0,
            date: date,
            name: String::from(name)
        }
    }
}


pub fn add_entry(conn: &Connection, date: &str, name: &str) -> Result<String, String> {
    if let Ok(naive_date) = NaiveDate::parse_from_str(date, "%d-%m-%Y") {
        let new_entry = Person::from_args(naive_date, name);
        conn.execute("INSERT INTO person (date, name) VALUES (?1, ?2)", &[&new_entry.date, &new_entry.name]).unwrap();
        Ok(format!("Successfully added `{}` to the database.", name))
    } else {
        Err(format!("Failed to parse a date from `{}`. Are you sure it's formatted correctly?", date))
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

        assert!(add_entry(&conn, "01-01-1990", "Marc").is_ok());

        env::remove_var("RUSDAY_DB_PATH");
        let _ = conn.close();
    }

    #[test]
    fn invalid_date_returns_err() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(add_entry(&conn, "invalid", "Marc").is_err());

        env::remove_var("RUSDAY_DB_PATH");
        let _ = conn.close();
    }
}
