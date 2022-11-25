use csv;

use std::env;
use std::fs::File;
use std::option::Option;
use std::path::PathBuf;
use task::Task;

mod cli;
mod task;

// Returns the path to the CSV file that contains the time entries
// The environment variable `RTRACKERFILE` can be used to override the default path
// The default path is `~/.local/share/rtracker

fn get_file_path() -> String {
    if let Ok(path) = env::var("RTRACKERFILE") {
        path
    } else {
        let mut path = PathBuf::from(env::var("HOME").unwrap());
        path.push(".local/share/rtracker");
        path.to_str().unwrap().to_string()
    }
}

// Get the last line of the CSV file
// And parse it into a `Task` struct

fn get_last_entry() -> Option<Task> {
    let path = get_file_path();
    let file = File::open(path);
    if let Ok(file) = file {
        let mut reader = csv::Reader::from_reader(file);
        let mut last_entry = None;
        for entry in reader.deserialize() {
            last_entry = Some(entry.unwrap());
        }
        last_entry
    } else {
        None
    }
}

// Write a new entry to the CSV file
// The entry is serialized into a string and written to the file

fn write_entry(entry: &Task) {
    let path = get_file_path();
    let file = File::create(path);
    if let Ok(file) = file {
        let mut writer = csv::Writer::from_writer(file);
        writer.serialize(entry).unwrap();
    }
}

fn main() {
    let matches = cli::build_cli().get_matches();

    // If the subcommand is "start", then we want to start a new task.
    // The clap library will ensure that the required arguments are present.
    if let Some(matches) = matches.subcommand_matches("start") {
        let task_name = matches.get_one::<String>("task").unwrap().to_string();
        let project_name = matches.get_one::<String>("project");
        let entry = Task::new(task_name, project_name.cloned());
        write_entry(&entry);
        println!("Started task \"{}\" in project \"{:?}\"", entry.task, entry.project);
    }

    // If the subcommand is "stop", then we want to stop the in-progress task.
    if let Some(_) = matches.subcommand_matches("stop") {
        let last_entry = get_last_entry();
        if let Some(mut entry) = last_entry {
            if entry.in_progress() {
                entry.stop_task();
                write_entry(&entry);
                println!("Stopped task \"{}\" in project \"{:?}\"", entry.task, entry.project);
                println!("Duration: {}", entry.duration_str());
            } else {
                println!("The last task is already stopped");
            }
        } else {
            println!("There are no tasks to stop");
        }
    }

    // If the subcommand is "continue", then we want to continue the last task.
    if let Some(_) = matches.subcommand_matches("continue") {
        let last_entry = get_last_entry();
        if let Some(mut entry) = last_entry {
            entry.continue_task();
            write_entry(&entry);
            println!("Continuing task \"{}\" in project \"{:?}\"", entry.task, entry.project);
        } else {
            println!("There were not task to continue");
        }
    }

    // If the subcommand is "status", then we want to print the in-progress task information.
    if let Some(matches) = matches.subcommand_matches("status") {
        let last_entry = get_last_entry();
        if let Some(entry) = last_entry {
            if entry.in_progress() {
                if let Some(json) = matches.get_one::<bool>("json") {
                    match json {
                        true => println!("{}", entry.to_json()),
                        false => {
                            println!("Task: {}", entry.task);
                            println!("Project: {:?}", entry.project);
                            println!("Started: {}", entry.start_date().format("%Y-%m-%d %H:%M:%S"));
                            println!("Duration: {}", entry.duration_str());
                        }
                    }
                }
            } else {
                println!("There is no in-progress task");
            }
        } else {
            println!("There is no in-progress task");
        }
    }

    // If the subcommand is "report", then we want to print the list of all the tasks for the given date range.
    if let Some(matches) = matches.subcommand_matches("report") {
        println!("Printing report");

        if let Some(json) = matches.get_one::<bool>("json") {
            println!("JSON: {}", json);
        }

        if let Some(csv) = matches.get_one::<bool>("csv") {
            println!("CSV: {}", csv);
        }

        if let Some(from) = matches.get_one::<String>("from") {
            println!("From: {}", from);
        }

        if let Some(to) = matches.get_one::<String>("to") {
            println!("To: {}", to);
        }
    }

    // If no subcommand is given, then we want to print the help message.
    if let None = matches.subcommand_name() {
        cli::build_cli().print_help().unwrap();
    }
}
