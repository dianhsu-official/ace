use chrono::{DateTime, Utc};

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
