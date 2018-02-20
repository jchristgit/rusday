# rusday
`rusday` is a command line application intended to help you remember your friends' birthdays.
I primarly made this project to learn Rust, but also because I've got to known a couple of awesome
people through programming over the past couple months and wanted to ensure that I don't forget their birthdays.

### Installation
Installation is easy thanks to `cargo`:
```sh
$ cargo install --git https://github.com/Volcyy/rusday
```
Now, you're free to use `rusday`, given that you have `cargo`'s
`bin` directory in your `$PATH`.


### Usage
`rusday help` should give you a basic idea of how to use it.
The basic workflow is simple: Use `rusday add` and `rusday remove`
to add and remove people, then use `rusday list` to get an
overview for all the entries. You can use `rusday dashboard` to only
be informed about who's birthday it is. I sticked that into my `~/.bash_login`
to ensure I get informed about anyone's birthday when I open my Terminal.

```sh
$ rusday help
rusday
A CLI tool to help you remember your friends' birthdays.

USAGE:
    rusday [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add          Adds someone to the database.
    dashboard    Shows a dashboard with the most relevant information.
    help         Prints this message or the help of the given subcommand(s)
    list         Shows a list of people in the database.
    remove       Remove someone from the database.
```


### Configuration
It's possible to configure the database path that `rusday` uses by setting the
environment variable `RUSDAY_DB_PATH`. In case this isn't set, `rusday` will
check for `XDG_DATA_HOME`, and store it in a file named `rusday.db` if present.
By ´default, the database is stored in `~/.local/share/rusday.db`.
