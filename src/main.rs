extern crate chrono;
extern crate clap;
extern crate rusqlite;

use clap::{App, Arg, SubCommand};

mod add;
mod dashboard;
mod list;


fn main() {
    let matches = App::new("rusday")
        .subcommand(SubCommand::with_name("add")
                    .about("Adds someone to the database.")
                    .arg(Arg::with_name("date")
                         .help("a date string in the format dd-mm-yyyy")
                         .index(1)
                         .required(true))
                    .arg(Arg::with_name("name")
                         .help("the name of the person to add")
                         .index(2)
                         .required(true)))
        .subcommand(SubCommand::with_name("list")
                    .about("Shows a list of people in the database."))
        .subcommand(SubCommand::with_name("dashboard")
                    .about("Shows a dashboard with the most relevant information."))
        .get_matches();

    match matches.subcommand_name() {
        Some("add") => {
            if let Some(ref matches) = matches.subcommand_matches("add") {
                add::add_entry(matches.value_of("date").unwrap(), matches.value_of("name").unwrap());
            }
        },
        Some("dashboard") => {
            dashboard::show_dashboard();
        }
        Some("list") => {
            list::list_entries();
        },
        None => println!("No subcommand was used."),
        _ => println!("Some other subcommand was used.")
    }
}
