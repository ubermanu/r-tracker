# r-tracker

A command line tool to track your time, written in Rust.

## Environment

- `RTRACKERFILE`: Path to the file where the data is stored. Defaults to `~/.local/share/rtracker`.

## Commands

### `start`

Starts a new task. If a task is already running, it will be stopped and a new one will be started.

* `--task` - The name of the task
* `--project` - The project the task belongs to

### `stop`

Stops the current task.

### `continue`

Starts the last task again.

### `status`

Prints the current task.

* `--json` - Prints the current task as JSON

### `report`

Prints a list of all tasks.

* `--from` - The start date of the list
* `--to` - The end date of the list

* `--today` - Only show tasks from today
* `--yesterday` - Only show tasks from yesterday
* `--week` - Only show tasks from the last 7 days
* `--month` - Only show tasks from the given month


* `--json` - Prints the task list as JSON
* `--csv` - Prints the task list as CSV
