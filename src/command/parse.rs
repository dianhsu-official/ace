use std::path;
use tokio::fs;

use colored::Colorize;

use super::model::ParseArgs;
use crate::constants::PLATFORM_MAP;
use crate::database::CONFIG_DB;
use crate::platform::OnlineJudge;

pub struct ParseCommand {}

impl ParseCommand {
    pub async fn handle(args: ParseArgs) -> Result<String, String> {
        let real_platform = match PLATFORM_MAP.get(args.platform.as_str()) {
            Some(platform) => *platform,
            None => {
                return Err(format!("Platform {} not found", args.platform));
            }
        };
        let account_info = match CONFIG_DB.get_default_account(real_platform) {
            Ok(account_info) => account_info,
            Err(info) => {
                return Err(info);
            }
        };
        let mut oj = OnlineJudge::new(account_info, real_platform);
        let contest_test_cases = match oj.get_contest_test_cases(&args.contest_identifier).await {
            Ok(test_cases) => test_cases,
            Err(info) => return Err(info),
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
        match fs::create_dir_all(contest_path.clone()).await {
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
            match fs::create_dir_all(problem_path.clone()).await {
                Ok(_) => {}
                Err(_) => {
                    return Err(String::from("Create problem directory failed"));
                }
            }
            for (index, test_case) in test_cases.iter().enumerate() {
                let input_path = problem_path.clone().join(format!("{:03}i.txt", index + 1));
                let output_path = problem_path.clone().join(format!("{:03}o.txt", index + 1));
                match fs::write(input_path, test_case.input.as_bytes()).await {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(String::from("Write input file failed"));
                    }
                }
                match fs::write(output_path, test_case.output.as_bytes()).await {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(String::from("Write output file failed"));
                    }
                }
            }
            println!(
                "Save test case for {} success.",
                problem_identifier.bright_blue()
            );
        }
        return Ok(String::from("Parse command success"));
    }
}
