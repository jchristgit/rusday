extern crate rusqlite;

use rusqlite::Connection;


pub fn remove_entry(conn: &Connection, name: &str) -> Result<String, String> {
    match conn.execute("DELETE FROM person WHERE name = ?1", &[&name]) {
        Ok(changed_rows) => {
            if changed_rows == 0 {
                Err(format!("Failed to find anyone named `{}` in the database...", name))
            } else {
                Ok(format!("Successfully removed `{}` from the database.", name))
            }
        },
        Err(e) => {
            Err(format!("Failed to remove `{}`: {}", name, e))
        }
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

        assert!(remove_entry(&conn, "Marc").is_err());

        env::remove_var("RUSDAY_DB_PATH");
    }

    #[test]
    fn returns_err_with_no_matching_entry() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(add_entry(&conn, "01-01-1900", "Marc").is_ok());
        assert!(remove_entry(&conn, "John").is_err());

        env::remove_var("RUSDAY_DB_PATH");
    }

    #[test]
    fn returns_ok_with_matching_entry() {
        env::set_var("RUSDAY_DB_PATH", ":memory:");
        let conn = get_db_conn();

        assert!(add_entry(&conn, "01-01-1900", "Marc").is_ok());
        assert!(remove_entry(&conn, "Marc").is_ok());

        env::remove_var("RUSDAY_DB_PATH");
    }
}
