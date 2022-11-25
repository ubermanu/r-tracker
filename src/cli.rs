use clap::{arg, command, Command};

pub fn build_cli() -> Command {
    command!()
        .about("A command line interface that tracks your time")
        .subcommand(
            Command::new("start")
                .about("Start a new task")
                .arg(arg!(-t --task <TASK> "The name of the task").required(true))
                .arg(arg!(-p --project <PROJECT> "The name of the project"))
        )
        .subcommand(
            Command::new("stop")
                .about("Stop the in-progress task")
        )
        .subcommand(
            Command::new("continue")
                .about("Start the last task again")
        )
        .subcommand(
            Command::new("status")
                .about("Print the in-progress task information")
                .arg(arg!(--json "Prints in JSON format"))
        )
        .subcommand(
            Command::new("report")
                .about("Print the list of all the tasks for the given date range")
                .arg(arg!(--json "Prints in JSON format"))
                .arg(arg!(--csv "Prints in CSV format"))
                .arg(arg!(--from <FROM> "The start date of the report"))
                .arg(arg!(--to <TO> "The end date of the report"))
        )
}
