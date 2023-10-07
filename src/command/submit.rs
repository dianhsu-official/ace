use inquire::Select;

use super::model::SubmitArgs;
use crate::{
    constants::ProgramLanguage,
    database::CONFIG_DB,
    misc::utility::Utility,
    model::{Platform, Verdict},
    platform::atcoder::AtCoder,
    platform::codeforces::Codeforces,
    traits::OnlineJudge,
};
use std::{env::current_dir, fs, thread, time::Duration};

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
        let current_dir = match current_dir() {
            Ok(current_dir) => current_dir,
            Err(_) => {
                return Err("Cannot get current path".to_string());
            }
        };
        let filename = match args.filename {
            Some(filename) => filename,
            None => {
                let files = match fs::read_dir(current_dir.clone()) {
                    Ok(files) => files
                        .into_iter()
                        .filter_map(|x| match x {
                            Ok(file) => match file.file_name().to_str() {
                                Some(filename) => {
                                    if filename.starts_with("code.") {
                                        Some(filename.to_string())
                                    } else {
                                        None
                                    }
                                }
                                None => None,
                            },
                            Err(_) => None,
                        })
                        .collect::<Vec<_>>(),
                    Err(_) => {
                        return Err("Cannot get current path".to_string());
                    }
                };
                let filename = match Select::new("Select file to ", files).prompt() {
                    Ok(filename) => filename,
                    Err(info) => {
                        log::error!("{}", info);
                        return Err("Cannot get current path".to_string());
                    }
                };
                filename
            }
        };
        let submit_info = match current_dir.join(filename.clone()).to_str() {
            Some(file_path) => match Self::get_submit_info(&filename, file_path) {
                Ok(submit_info) => Some(submit_info),
                Err(info) => {
                    log::error!("{}", info);
                    return Err(info);
                }
            },
            None => None,
        };
        match submit_info {
            Some(submit_info) => match submit_info.platform {
                Platform::Codeforces => {
                    let mut cf = match Codeforces::new() {
                        Ok(cf) => cf,
                        Err(info) => {
                            return Err(info);
                        }
                    };
                    let submission_id = match cf.submit(
                        &submit_info.problem_identifier,
                        &submit_info.code,
                        &submit_info.language_id,
                    ) {
                        Ok(submission_id) => submission_id,
                        Err(info) => {
                            return Err(info);
                        }
                    };
                    let mut submission_info =
                        match cf.retrive_result(&submit_info.problem_identifier, &submission_id) {
                            Ok(submission_info) => submission_info,
                            Err(_) => {
                                return Err("Cannot get submission info".to_string());
                            }
                        };
                    let mut retry_times = 100;
                    while submission_info.verdict == Verdict::Waiting && retry_times > 0 {
                        retry_times -= 1;
                        thread::sleep(Duration::from_secs(1));
                        submission_info = match cf
                            .retrive_result(&submit_info.problem_identifier, &submission_id)
                        {
                            Ok(submission_info) => submission_info,
                            Err(_) => {
                                return Err("Cannot get submission info".to_string());
                            }
                        };
                    }
                    if submission_info.verdict == Verdict::Waiting {
                        return Err("Cannot get submission info".to_string());
                    } else {
                        return Ok(format!("Submission result: {:?}", submission_info));
                    }
                }
                Platform::AtCoder => {
                    let mut atc = match AtCoder::new() {
                        Ok(atc) => atc,
                        Err(info) => {
                            return Err(info);
                        }
                    };
                    let submisson_id = match atc.submit(
                        &submit_info.problem_identifier,
                        &submit_info.code,
                        &submit_info.language_id,
                    ) {
                        Ok(submission_id) => submission_id,
                        Err(info) => {
                            log::error!("{}", info);
                            return Err(info);
                        }
                    };
                    let mut submission_info =
                        match atc.retrive_result(&submit_info.problem_identifier, &submisson_id) {
                            Ok(submission_info) => submission_info,
                            Err(_) => {
                                return Err("Cannot get submission info".to_string());
                            }
                        };
                    let mut retry_times = 100;
                    while submission_info.verdict == Verdict::Waiting && retry_times > 0 {
                        retry_times -= 1;
                        thread::sleep(Duration::from_secs(1));
                        submission_info = match atc
                            .retrive_result(&submit_info.problem_identifier, &submisson_id)
                        {
                            Ok(submission_info) => submission_info,
                            Err(_) => {
                                return Err("Cannot get submission info".to_string());
                            }
                        };
                    }
                    if submission_info.verdict == Verdict::Waiting {
                        return Err("Cannot get submission info".to_string());
                    } else {
                        return Ok(format!("Submission result: {:?}", submission_info));
                    }
                }
            },
            None => {
                return Err("Cannot get language id".to_string());
            }
        }
    }
}
impl SubmitCommand {
    fn get_submit_info(filename: &str, file_path: &str) -> Result<SubmitInfo, String> {
        let workspace = match CONFIG_DB.get_config("workspace") {
            Ok(workspace) => workspace,
            Err(info) => {
                return Err(info);
            }
        };
        let (platform, problem_identifier, contest_identifier) =
            match Utility::get_identifiers_from_currrent_location(file_path, &workspace) {
                Ok(resp) => resp,
                Err(info) => {
                    return Err(info);
                }
            };
        let language = match Utility::get_program_language_from_filename(filename) {
            Ok(language) => language,
            Err(info) => {
                return Err(info);
            }
        };
        let language_id = match SubmitCommand::get_submit_language_id(language, platform) {
            Ok(language_id) => language_id,
            Err(info) => {
                return Err(info);
            }
        };
        let code = match fs::read_to_string(file_path) {
            Ok(code) => code,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        return Ok(SubmitInfo {
            platform,
            language_id,
            code,
            problem_identifier,
            contest_identifier,
        });
    }
    fn get_submit_language_id(
        language: ProgramLanguage,
        platform: Platform,
    ) -> Result<String, String> {
        let language_configs = match CONFIG_DB.get_language_platform_config(language, platform) {
            Ok(language_configs) => language_configs,
            Err(info) => return Err(info),
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
