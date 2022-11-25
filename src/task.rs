use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

// The `Entry` struct represents a single task entry in the CSV file
// It contains the start time, end time, task name, and project name

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub task: String,
    pub project: Option<String>,
    pub start: String,
    pub end: Option<String>,
}

// The `serialize_datetime` function is used to serialize the `DateTime` struct
// into a string that can be written to the CSV file
// This should be able to serialiaze both `DateTime<chrono::Local>` and `Option<DateTime<chrono::Local>>`

impl Task {
    // Creates a new `TimeEntry` with the given task name and project name
    // The start time is set to the current time
    pub fn new(task: String, project: Option<String>) -> Task {
        Task {
            task,
            project,
            start: Utc::now().to_rfc3339(),
            end: None,
        }
    }

    // Returns the start datetime as a `NaiveDateTime`
    pub fn start_date(&self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.start, "%Y-%m-%dT%H:%M:%S%.f%z").unwrap()
    }

    // Returns the end datetime as a `NaiveDateTime`
    pub fn end_date(&self) -> Option<NaiveDateTime> {
        match self.end {
            Some(ref end) => Some(NaiveDateTime::parse_from_str(end, "%Y-%m-%dT%H:%M:%S%.f%z").unwrap()),
            None => None,
        }
    }

    // Returns the duration of the task in seconds
    pub fn duration(&self) -> i64 {
        match self.end_date() {
            Some(ref end) => end.signed_duration_since(self.start_date()).num_seconds(),
            None => Utc::now().timestamp() - self.start_date().timestamp(),
        }
    }

    // Returns the duration of the task in a human-readable format
    pub fn duration_str(&self) -> String {
        let duration = self.duration().clone();
        let hours = duration / 3600;
        let minutes = (duration % 3600) / 60;
        let seconds = duration % 60;
        format!("{}h {}m {}s", hours, minutes, seconds)
    }

    pub fn in_progress(&self) -> bool {
        self.end_date().is_none()
    }

    pub fn continue_task(&mut self) {
        self.end = None;
    }

    pub fn stop_task(&mut self) {
        self.end = Some(Utc::now().to_rfc3339());
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
