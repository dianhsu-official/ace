use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Display;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;

use crate::constants::ProgramLanguage;
#[derive(Debug, PartialEq)]
pub enum Verdict {
    Waiting = 0,  // Waiting for judge or judge in progress
    Resulted = 1, // Judge finished
}
#[derive(Debug)]
pub struct PostSubmissionInfo {
    pub submission_id: String,
    pub problem_identifier: String,
    pub verdict: Verdict,
    pub verdict_info: String,
    pub execute_time: String,
    pub execute_memory: String,
}
impl PostSubmissionInfo {
    pub fn new() -> PostSubmissionInfo {
        PostSubmissionInfo {
            submission_id: String::new(),
            problem_identifier: String::new(),
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

#[derive(Serialize, Deserialize, Clone, Copy, Display, Debug, EnumString, EnumIter)]
pub enum Platform {
    Codeforces,
    AtCoder,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LanguageSubmitConfig {
    pub alias: String,
    pub suffix: String,
    pub platform: Platform,
    pub identifier: ProgramLanguage,
    pub submit_id: String,
    pub submit_description: String,
}

impl Display for LanguageSubmitConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}, {}, {})",
            self.alias, self.suffix, self.identifier, self.submit_description
        )
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LanguageConfig {
    pub id: i64,
    pub alias: String,
    pub suffix: String,
    pub platform: Platform,
    pub identifier: ProgramLanguage,
    pub submit_id: String,
    pub submit_description: String,
    pub template_path: String,
    pub compile_command: String,
    pub execute_command: String,
    pub clear_command: String,
}
impl Display for LanguageConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}, {}, {}, {})",
            self.alias, self.suffix, self.identifier, self.submit_description, self.platform
        )
    }
}

pub struct AccountInfo {
    pub username: String,
    pub password: String,
    pub cookies: String,
    pub current: i64,
    pub last_use: String,
}
#[derive(Debug)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformLanguage {
    pub language: ProgramLanguage,
    pub platform: Platform,
    pub id: String,
    pub description: String,
}

impl Display for PlatformLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
