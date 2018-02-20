extern crate chrono;
extern crate clap;
extern crate rusqlite;

use clap::{App, Arg, SubCommand};
use common::get_db_conn;

mod add;
mod common;
mod dashboard;
mod list;
mod remove;


fn main() {
    let matches = App::new("rusday")
        .about("A CLI tool to help you remember your friends' birthdays.")
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
            add::add_entry(&conn, matches.value_of("date").unwrap(), matches.value_of("name").unwrap())
        },
        Some("dashboard") => dashboard::show_dashboard(&conn),
        Some("list") => list::list_entries(&conn),
        Some("remove") => {
            let matches = matches.subcommand_matches("remove").unwrap();
            remove::remove_entry(&conn, matches.value_of("name").unwrap())
        },
        None => Err(format!("No subcommand was used.")),
        _ => unreachable!()
    };

    if let Ok(msg) = cmd_result {
        println!("{}", msg)
    } else {
        eprintln!("{}", cmd_result.err().unwrap())
    }
}
