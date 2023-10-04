use std::fs::create_dir_all;
use std::path;

use super::model::ParseArgs;
use crate::constants::PLATFORM_MAP;
use crate::database::CONFIG_DB;
use crate::model::{ContestStatus, Platform};
use crate::platform::{atcoder::AtCoder, codeforces::Codeforces};
use crate::traits::OnlineJudge;

pub struct ParseCommand {}

impl ParseCommand {
    pub fn handle(args: ParseArgs) -> Result<String, String> {
        let real_platform = match PLATFORM_MAP.get(args.platform.as_str()) {
            Some(platform) => *platform,
            None => {
                return Err(format!("Platform {} not found", args.platform));
            }
        };
        let contest_test_cases = match real_platform {
            Platform::Codeforces => {
                let mut cf = match Codeforces::new() {
                    Ok(cf) => cf,
                    Err(info) => {
                        return Err(info);
                    }
                };
                let contest = match cf.get_contest(&args.contest_identifier) {
                    Ok(contest) => contest,
                    Err(info) => {
                        return Err(info);
                    }
                };
                let mut contest_test_cases = Vec::new();
                if contest.status != ContestStatus::NotStarted {
                    let problem_infos = match cf.get_problems(&args.contest_identifier) {
                        Ok(problem_infos) => problem_infos,
                        Err(info) => {
                            return Err(info);
                        }
                    };
                    for problem_info in problem_infos {
                        let test_cases = match cf.get_test_cases(&problem_info[1]) {
                            Ok(test_cases) => test_cases,
                            Err(info) => {
                                return Err(info);
                            }
                        };
                        let problem_identifier = problem_info[0].clone();
                        contest_test_cases.push((problem_identifier, test_cases));
                    }
                    contest_test_cases
                } else {
                    return Err(format!("Contest {} not started", args.contest_identifier));
                }
            }
            Platform::AtCoder => {
                let mut atc = match AtCoder::new() {
                    Ok(atc) => atc,
                    Err(info) => return Err(info),
                };
                let contest = match atc.get_contest(&args.contest_identifier) {
                    Ok(contest) => contest,
                    Err(info) => {
                        return Err(info);
                    }
                };
                let mut contest_test_cases = Vec::new();
                if contest.status != ContestStatus::NotStarted {
                    let problem_infos = match atc.get_problems(&args.contest_identifier) {
                        Ok(problem_infos) => problem_infos,
                        Err(info) => {
                            return Err(info);
                        }
                    };
                    for problem_info in problem_infos {
                        let test_cases = match atc.get_test_cases(&problem_info[1]) {
                            Ok(test_cases) => test_cases,
                            Err(info) => {
                                return Err(info);
                            }
                        };
                        let problem_identifier = problem_info[0].clone();
                        contest_test_cases.push((problem_identifier, test_cases));
                    }
                    contest_test_cases
                } else {
                    return Err(format!("Contest {} not started", args.contest_identifier));
                }
            }
        };
        let platform_str = real_platform.to_string();
        let workspace = match CONFIG_DB.get_config("workspace") {
            Ok(workspace) => workspace,
            Err(info) => {
                return Err(info);
            }
        };
        let contest_path = path::Path::new(workspace.as_str())
            .join(platform_str)
            .join(args.contest_identifier.to_lowercase());
        match create_dir_all(contest_path.clone()) {
            Ok(_) => {}
            Err(_) => {
                return Err(String::from("Create contest directory failed"));
            }
        }
        for (problem_identifier, test_cases) in contest_test_cases {
            let vec = problem_identifier.split("_").collect::<Vec<_>>();
            if vec.len() != 2 {
                return Err(String::from("Invalid problem identifier."));
            }
            let contest_problem_identifier = vec[1];
            let problem_path = contest_path.join(contest_problem_identifier.to_lowercase());
            match create_dir_all(problem_path.clone()) {
                Ok(_) => {}
                Err(_) => {
                    return Err(String::from("Create problem directory failed"));
                }
            }
            for (index, test_case) in test_cases.iter().enumerate() {
                let input_path = problem_path.clone().join(format!("{:03}i.txt", index + 1));
                let output_path = problem_path.clone().join(format!("{:03}o.txt", index + 1));
                match std::fs::write(input_path, test_case.input.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(String::from("Write input file failed"));
                    }
                }
                match std::fs::write(output_path, test_case.output.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(String::from("Write output file failed"));
                    }
                }
            }
        }
        return Ok(String::from("Parse command success"));
    }
}
