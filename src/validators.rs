extern crate chrono;

use chrono::NaiveDate;

pub fn is_valid_date(s: String) -> Result<(), String> {
    if NaiveDate::parse_from_str(s.as_str(), "%d-%m-%Y").is_ok() {
        Ok(())
    } else {
        Err(format!(
            "Failed to parse a date from `{}`. Are you sure it's formatted correctly?",
            s
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_ok_with_valid_date() {
        assert!(is_valid_date("12-12-2018".to_string()).is_ok());
        assert!(is_valid_date("01-01-2005".to_string()).is_ok());
        assert!(is_valid_date("02-03-1904".to_string()).is_ok());
    }

    #[test]
    fn returns_err_with_invalid_date() {
        assert!(is_valid_date("a name".to_string()).is_err());
        assert!(is_valid_date("12. march".to_string()).is_err());
        assert!(is_valid_date("12-13-1920".to_string()).is_err());
    }
}
