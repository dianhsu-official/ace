use inquire::Select;
use prettytable::table;

use super::model::SubmitArgs;
use crate::{
    database::CONFIG_DB,
    model::{Platform, PostSubmissionInfo, Verdict},
    platform::OnlineJudge,
    utility::Utility,
};
use std::{env::current_dir, vec};
use tokio::fs;
pub struct SubmitCommand {}
#[derive(Debug)]
pub struct PreSubmissionInfo {
    pub platform: Platform,
    pub language_id: String,
    pub code: String,
    pub problem_identifier: String,
    pub contest_identifier: String,
}
impl SubmitCommand {
    async fn show_result(submission_info: &PostSubmissionInfo, reties: u32) {
        let mut table = match submission_info.verdict {
            Verdict::Waiting => table!(
                ["submission id", submission_info.submission_id],
                ["contest", submission_info.contest_identifier],
                ["problem", submission_info.problem_identifier],
                ["verdict", submission_info.verdict_info],
                ["execute time", submission_info.execute_time],
                ["execute memory", submission_info.execute_memory]
            ),
            _ => table!(
                ["submission id", submission_info.submission_id],
                ["contest", submission_info.contest_identifier],
                ["problem", submission_info.problem_identifier],
                ["verdict", b -> submission_info.verdict_info],
                ["execute time", submission_info.execute_time],
                ["execute memory", submission_info.execute_memory]
            ),
        };
        table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
        let output = table.to_string();
        let lines = output.lines().count() + 1;
        if reties > 0 {
            print!("{}", ansi_escapes::EraseLines(lines as u16));
        }
        table.printstd();
    }
    async fn submit(
        mut oj: OnlineJudge,
        submit_info: &PreSubmissionInfo,
    ) -> Result<String, String> {
        let submission_id = match oj
            .submit(
                &submit_info.problem_identifier,
                &submit_info.code,
                &submit_info.language_id,
            )
            .await
        {
            Ok(submission_id) => submission_id,
            Err(info) => {
                return Err(info);
            }
        };
        let mut retries: u32 = 0;
        while let Ok(post_submission_info) = oj
            .retrive_result(&submit_info.problem_identifier, &submission_id)
            .await
        {
            Self::show_result(&post_submission_info, retries).await;
            if post_submission_info.verdict != Verdict::Waiting {
                return Ok("Submit success".to_string());
            } else if retries > 100 {
                break;
            }
            retries += 1;
        }
        return Err("Cannot get submission info".to_string());
    }
    pub async fn handle(args: SubmitArgs) -> Result<String, String> {
        let current_dir = match current_dir() {
            Ok(current_dir) => current_dir,
            Err(_) => {
                return Err("Cannot get current path".to_string());
            }
        };
        let current_dir_str = match current_dir.to_str() {
            Some(current_dir_str) => current_dir_str,
            None => {
                return Err("Can't get current path".to_string());
            }
        };

        let filename = match args.filename {
            Some(filename) => filename,
            None => {
                let files = Utility::find_source_code_filename_from_directory(current_dir_str);
                match files.len() {
                    0 => {
                        return Err("No code file found".to_string());
                    }
                    1 => files[0].clone(),
                    _ => {
                        let filename = match Select::new("Select file to submit: ", files).prompt()
                        {
                            Ok(filename) => filename,
                            Err(info) => {
                                log::error!("{}", info);
                                return Err(info.to_string());
                            }
                        };
                        filename
                    }
                }
            }
        };
        let pre_submission_info = match current_dir.join(filename.clone()).to_str() {
            Some(file_path) => match Self::get_submit_info(&filename, file_path).await {
                Ok(pre_submission_info) => pre_submission_info,
                Err(info) => {
                    log::error!("{}", info);
                    return Err(info);
                }
            },
            None => {
                return Err("Can't get current path".to_string());
            }
        };
        log::info!("{:?}", pre_submission_info);
        let account_info = match CONFIG_DB.get_default_account(pre_submission_info.platform) {
            Ok(account_info) => account_info,
            Err(info) => {
                return Err(info);
            }
        };

        let oj = OnlineJudge::new(account_info, pre_submission_info.platform);
        return Self::submit(oj, &pre_submission_info).await;
    }
}
impl SubmitCommand {
    async fn get_submit_info(filename: &str, file_path: &str) -> Result<PreSubmissionInfo, String> {
        let workspace = match CONFIG_DB.get_config("workspace") {
            Ok(workspace) => workspace,
            Err(info) => {
                return Err(info);
            }
        };
        let (platform, contest_identifier, problem_identifier) =
            match Utility::get_identifiers_from_currrent_location(file_path, &workspace) {
                Ok(resp) => resp,
                Err(info) => {
                    return Err(info);
                }
            };
        let language_id = match Self::get_submit_language_id(filename, platform) {
            Ok(language_id) => language_id,
            Err(info) => {
                return Err(info);
            }
        };
        let code = match fs::read_to_string(file_path).await {
            Ok(code) => code,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        return Ok(PreSubmissionInfo {
            platform,
            language_id,
            code,
            problem_identifier,
            contest_identifier,
        });
    }
    fn get_submit_language_id(filename: &str, platform: Platform) -> Result<String, String> {
        let language_configs =
            match Utility::get_language_config_by_filename_and_platform(filename, platform) {
                Ok(language) => language,
                Err(info) => {
                    return Err(info);
                }
            };
        match language_configs.len() {
            0 => {
                return Err(format!(
                    "No language config set for {}, please set language first.",
                    platform
                ));
            }
            1 => {
                return Ok(language_configs[0].submit_id.clone());
            }
            _ => {
                let language_ids = language_configs
                    .iter()
                    .map(|x| x.submit_id.clone())
                    .collect::<Vec<_>>();
                let language_id = match Select::new("Select language id", language_ids).prompt() {
                    Ok(language_id) => language_id,
                    Err(info) => {
                        return Err(info.to_string());
                    }
                };
                return Ok(language_id);
            }
        }
    }
}

#[tokio::test]
async fn test_show_result() {
    for idx in 0..20 {
        let submission_info = PostSubmissionInfo {
            submission_id: random_str::get_string(10, true, true, true, true),
            contest_identifier: "123789".to_string(),
            problem_identifier: "A".to_string(),
            verdict: Verdict::Waiting,
            verdict_info: "Accepted".to_string(),
            execute_time: "100ms".to_string(),
            execute_memory: "100MB".to_string(),
        };
        SubmitCommand::show_result(&submission_info, idx as u32).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
