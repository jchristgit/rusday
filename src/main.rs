#[macro_use]
extern crate clap;
extern crate rusday;

use clap::{App, Arg, SubCommand};
use rusday::{add_entry, remove_entry, get_db_conn, show_dashboard, list_entries};


fn main() {
    let matches = App::new("rusday")
        .about("A CLI tool to help you remember your friends' birthdays.")
        .version(crate_version!())
        .subcommand(SubCommand::with_name("add")
                    .about("Adds someone to the database.")
                    .arg(Arg::with_name("date")
                         .help("a date string in the format dd-mm-yyyy")
                         .required(true)
                         .empty_values(false))
                    .arg(Arg::with_name("name")
                         .help("the name of the person to add")
                         .required(true)
                         .empty_values(false)))
        .subcommand(SubCommand::with_name("list")
                    .about("Shows a list of people in the database."))
        .subcommand(SubCommand::with_name("dashboard")
                    .about("Shows a dashboard with the most relevant information."))
        .subcommand(SubCommand::with_name("remove")
                    .about("Remove someone from the database.")
                    .arg(Arg::with_name("name")
                         .help("the name of the person to remove")
                         .required(true)
                         .empty_values(false)))
        .get_matches();

    let conn = get_db_conn();
    let cmd_result = match matches.subcommand_name() {
        Some("add") => {
            let matches = matches.subcommand_matches("add").unwrap();
            add_entry(&conn, matches.value_of("date").unwrap(), matches.value_of("name").unwrap())
        },
        Some("dashboard") => show_dashboard(&conn),
        Some("list") => list_entries(&conn),
        Some("remove") => {
            let matches = matches.subcommand_matches("remove").unwrap();
            remove_entry(&conn, matches.value_of("name").unwrap())
        },
        None => Err("No subcommand was used.".to_string()),
        _ => unreachable!()
    };

    if let Ok(msg) = cmd_result {
        println!("{}", msg)
    } else {
        eprintln!("{}", cmd_result.err().unwrap())
    }
}
