extern crate rusqlite;

use common::get_db_conn;


pub fn remove_entry(name: &str) {
    let conn = get_db_conn();
    match conn.execute("DELETE FROM person WHERE name = ?1", &[&name]) {
        Ok(changed_rows) => {
            if changed_rows == 0 {
                eprintln!("Failed to find anyone named `{}` in the database...", name);
            } else {
                println!("Successfully removed `{}` from the database.", name);
            }
        },
        Err(e) => {
            eprintln!("Failed to remove `{}`: {}", name, e);
        }
    }
    let _ = conn.close();
}
