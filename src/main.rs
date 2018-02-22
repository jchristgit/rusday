extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate isatty;
extern crate rusday;

use clap::{App, Arg, SubCommand};
use isatty::stdout_isatty;
use rusday::{add_entry, get_db_conn, is_valid_date, list_entries, remove_entry, show_dashboard};

fn main() {
    let matches = App::new("rusday")
        .about("A CLI tool to help you remember your friends' birthdays.")
        .version(crate_version!())
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .help("when to use colored / formatted output")
                .possible_values(&["auto", "always", "never"])
                .default_value("auto"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds someone to the database.")
                .arg(
                    Arg::with_name("date")
                        .help("a date string in the format dd-mm-yyyy")
                        .required(true)
                        .empty_values(false)
                        .validator(is_valid_date),
                )
                .arg(
                    Arg::with_name("name")
                        .help("the name of the person to add")
                        .required(true)
                        .empty_values(false),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Shows a list of people in the database."))
        .subcommand(
            SubCommand::with_name("dashboard")
                .about("Shows a dashboard with the most relevant information."),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove someone from the database.")
                .arg(
                    Arg::with_name("name")
                        .help("the name of the person to remove")
                        .required(true)
                        .empty_values(false),
                ),
        )
        .get_matches();

    let conn = get_db_conn();
    let color = match matches.value_of("color").unwrap() {
        "always" => true,
        "auto" => stdout_isatty(),
        "never" => false,
        _ => unreachable!(),
    };
    let cmd_result = match matches.subcommand_name() {
        Some("add") => {
            let matches = matches.subcommand_matches("add").unwrap();
            let date = matches.value_of("date").unwrap();
            let name = matches.value_of("name").unwrap();
            add_entry(&conn, date, name, color)
        }
        Some("dashboard") => show_dashboard(&conn, color),
        Some("list") => list_entries(&conn, color),
        Some("remove") => {
            let matches = matches.subcommand_matches("remove").unwrap();
            remove_entry(&conn, matches.value_of("name").unwrap(), color)
        }
        None => Err("No subcommand was used.".to_string()),
        _ => unreachable!(),
    };

    if let Ok(msg) = cmd_result {
        if !msg.is_empty() {
            println!("{}", msg)
        }
    } else {
        eprintln!("{}", cmd_result.err().unwrap())
    }
}
