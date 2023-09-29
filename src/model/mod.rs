use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Display;
#[derive(Debug)]
pub enum Verdict {
    Waiting = 0,  // Waiting for judge or judge in progress
    Resulted = 1, // Judge finished
}
#[derive(Debug)]
pub struct SubmissionInfo {
    pub submission_id: String,
    pub identifier: String,
    pub verdict: Verdict,
    pub verdict_info: String,
    pub execute_time: String,
    pub execute_memory: String,
}
impl SubmissionInfo {
    pub fn new() -> SubmissionInfo {
        SubmissionInfo {
            submission_id: String::new(),
            identifier: String::new(),
            verdict: Verdict::Waiting,
            verdict_info: String::new(),
            execute_time: String::new(),
            execute_memory: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum ContestStatus {
    NotStarted = 0,
    Running = 1,
    Ended = 2,
}
#[derive(Debug)]
pub struct Contest {
    pub identifier: String,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: ContestStatus,
}

#[derive(Serialize, Deserialize)]
pub struct CodeTemplate {
    pub alias: String,
    pub lang: String,
    pub path: String,
    pub suffix: String,
    pub before_script: String,
    pub script: String,
    pub after_script: String,
}
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Platform {
    Codeforces,
    Atcoder,
}
impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Platform::Codeforces => "codeforces",
            Platform::Atcoder => "atcoder",
        })
    }
}
