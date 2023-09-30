use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use strum_macros::Display;
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

#[derive(Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Display, Debug)]
pub enum Platform {
    Codeforces,
    AtCoder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub suffix: String,
    pub template_path: String,
    pub compile_command: String,
    pub execute_command: String,
    pub clear_command: String,
}

pub struct AccountInfo {
    pub username: String,
    pub password: String,
    pub cookies: String,
    pub current: i64,
    pub last_use: String,
}
#[derive(Debug)]
pub struct TestCase{
    pub input: String,
    pub output: String,
}