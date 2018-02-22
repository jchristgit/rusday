extern crate ansi_term;
extern crate rusqlite;

use self::ansi_term::Style;
use rusqlite::Connection;

pub fn remove_entry(conn: &Connection, name: &str, color: bool) -> Result<String, String> {
    match conn.execute("DELETE FROM person WHERE name = ?1", &[&name]) {
        Ok(changed_rows) => {
            if changed_rows == 0 {
                Err(format!(
                    "Failed to find anyone named `{}` in the database...",
                    if color {
                        Style::new().bold().paint(name).to_string()
                    } else {
                        name.to_string()
                    }
                ))
            } else {
                Ok(format!(
                    "Successfully removed `{}` from the database.",
                    if color {
                        Style::new().bold().paint(name).to_string()
                    } else {
                        name.to_string()
                    }
                ))
            }
        }
        Err(e) => Err(format!(
            "Failed to remove `{}`: {}",
            if color {
                Style::new().bold().paint(name).to_string()
            } else {
                name.to_string()
            },
            if color {
                Style::new().italic().paint(e.to_string()).to_string()
            } else {
                e.to_string()
            }
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use add::add_entry;
    use common::get_db_conn;

    #[test]
    fn returns_err_with_no_entries() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(remove_entry(&conn, "Marc", false).is_err());

        env::remove_var("RUSDAY_DB_PATH");
    }

    #[test]
    fn returns_err_with_no_matching_entry() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(add_entry(&conn, "01-01-1900", "Marc", false).is_ok());
        assert!(remove_entry(&conn, "John", false).is_err());

        env::remove_var("RUSDAY_DB_PATH");
    }

    #[test]
    fn returns_ok_with_matching_entry() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(add_entry(&conn, "01-01-1900", "Marc", false).is_ok());
        assert!(remove_entry(&conn, "Marc", false).is_ok());

        env::remove_var("RUSDAY_DB_PATH");
    }
}
