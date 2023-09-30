use crate::{
    model::Platform, platform::atcoder::AtCoder, platform::codeforces::Codeforces,
    traits::OnlineJudge,
};

use super::model::SubmitArgs;
use std::env::current_dir;

pub struct SubmitCommand {}
pub struct SubmitInfo {
    pub platform: Platform,
    pub language_id: String,
    pub code: String,
    pub problem_identifier: String,
    pub contest_identifier: String,
}
impl SubmitCommand {
    pub fn handle(args: SubmitArgs) -> Result<String, String> {
        let _ = args;
        let submit_info: Option<SubmitInfo> = match args.filename {
            Some(filename) => match current_dir() {
                Ok(current_path) => match current_path.join(filename.clone()).to_str() {
                    Some(file_path) => {
                        match SubmitCommand::get_language_id(&filename, &file_path) {
                            Ok(submit_info) => Some(submit_info),
                            Err(_) => None,
                        }
                    }
                    None => None,
                },
                Err(_) => None,
            },
            None => None,
        };
        match submit_info {
            Some(submit_info) => match submit_info.platform {
                Platform::Codeforces => {
                    let mut cf = Codeforces::new();
                    return cf.submit(
                        &submit_info.problem_identifier,
                        &submit_info.code,
                        &submit_info.language_id,
                    );
                }
                Platform::AtCoder => {
                    let mut ac = AtCoder::new();
                    return ac.submit(
                        &submit_info.problem_identifier,
                        &submit_info.code,
                        &submit_info.language_id,
                    );
                }
            },
            None => {
                return Err("Cannot get language id".to_string());
            }
        }
    }
}
impl SubmitCommand {
    fn get_language_id(filename: &str, file_path: &str) -> Result<SubmitInfo, String> {
        let _ = filename;
        let _ = file_path;
        todo!()
    }
}
