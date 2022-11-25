use csv;

use std::env;
use std::fs::File;
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

fn get_all_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();
    let path = get_file_path();
    let file = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let task: Task = result.unwrap();
        tasks.push(task);
    }
    tasks
}

// Write tasks to the CSV file

fn write_all_tasks(tasks: Vec<Task>) {
    let path = get_file_path();
    let file = File::create(path).unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    for task in tasks {
        wtr.serialize(task).unwrap();
    }
    wtr.flush().unwrap();
}

fn main() {
    let matches = cli::build_cli().get_matches();

    // If the subcommand is "start", then we want to start a new task.
    // The clap library will ensure that the required arguments are present.
    if let Some(matches) = matches.subcommand_matches("start") {
        let task_name = matches.get_one::<String>("task").unwrap().to_string();
        let project_name = matches.get_one::<String>("project");
        let entry = Task::new(task_name, project_name.cloned());
        let mut tasks = get_all_tasks();
        tasks.push(entry.clone());
        write_all_tasks(tasks);
        println!("Started task \"{}\" in project \"{:?}\"", entry.task, entry.project);
    }

    // If the subcommand is "stop", then we want to stop the in-progress task.
    if let Some(_) = matches.subcommand_matches("stop") {
        let mut tasks = get_all_tasks();
        let last_entry = tasks.pop();
        if let Some(mut entry) = last_entry {
            if entry.in_progress() {
                entry.stop_task();
                tasks.push(entry.clone());
                write_all_tasks(tasks);
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
        let mut tasks = get_all_tasks();
        let last_entry = tasks.pop();
        if let Some(mut entry) = last_entry {
            entry.continue_task();
            tasks.push(entry.clone());
            write_all_tasks(tasks);
            println!("Continuing task \"{}\" in project \"{:?}\"", entry.task, entry.project);
        } else {
            println!("There were not task to continue");
        }
    }

    // If the subcommand is "status", then we want to print the in-progress task information.
    if let Some(matches) = matches.subcommand_matches("status") {
        let mut tasks = get_all_tasks();
        let last_entry = tasks.pop();
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
