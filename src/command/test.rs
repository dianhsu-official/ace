use super::model::TestArgs;
use crate::context::CONTEXT;
use crate::database::CONFIG_DB;
use crate::snippet::Snippet;
use crate::utility::diff::Difference;
use crate::utility::Utility;
use colored::Colorize;
use inquire::Select;
use std::env::current_dir;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use tokio::fs;
pub struct TestCommand {}

impl TestCommand {
    pub async fn handle(args: TestArgs) -> Result<String, String> {
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
                        let filename = match Select::new("Select file to test(you can only select file which filename startwith `code`): ", files).prompt()
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
        if let Some(current_path_str) = current_dir.to_str() {
            if let Ok(mut context) = CONTEXT.lock() {
                context.update(current_path_str);
            }
        }
        let absolute_path = current_dir.join(filename.clone());
        let absolute_path_str = match absolute_path.to_str() {
            Some(absolute_path_str) => absolute_path_str,
            None => {
                return Err("Cannot get absolute path".to_string());
            }
        };
        if let Ok(mut context) = CONTEXT.lock() {
            let path = Path::new(&filename);
            context.filename_without_extension = match path.file_stem() {
                Some(filename) => match filename.to_str() {
                    Some(filename) => Some(filename.to_string()),
                    None => None,
                },
                None => None,
            };
            context.filename_with_extension = match path.file_name() {
                Some(filename) => match filename.to_str() {
                    Some(filename) => Some(filename.to_string()),
                    None => None,
                },
                None => None,
            };
        }
        let workspace = match CONFIG_DB.get_config("workspace") {
            Ok(workspace) => workspace,
            Err(info) => {
                return Err(info);
            }
        };
        let (platform, _, _) =
            match Utility::get_identifiers_from_currrent_location(absolute_path_str, &workspace) {
                Ok(resp) => resp,
                Err(info) => {
                    return Err(info);
                }
            };
        let language_configs =
            match Utility::get_language_config_by_filename_and_platform(&filename, platform) {
                Ok(configs) => configs,
                Err(info) => {
                    return Err(info);
                }
            };
        let mut language_config = match language_configs.len() {
            0 => {
                return Err("Cannot find language config".to_string());
            }
            1 => language_configs[0].clone(),
            _ => match Select::new("Select language config", language_configs).prompt() {
                Ok(language_config) => language_config,
                Err(info) => {
                    return Err(info.to_string());
                }
            },
        };
        println!("Test with language config: {}", language_config);
        if let Ok(context) = CONTEXT.lock() {
            language_config.compile_command =
                Snippet::replace(&context, &language_config.compile_command);
            language_config.execute_command =
                Snippet::replace(&context, &language_config.execute_command);
            language_config.clear_command =
                Snippet::replace(&context, &language_config.clear_command);
        }
        return Self::run_test_commands(
            &language_config.compile_command,
            &language_config.execute_command,
            &language_config.clear_command,
        )
        .await;
    }
    fn run_no_input_command(single_command: &str) -> Result<String, String> {
        let mut command = match cfg!(target_os = "windows") {
            true => Command::new("powershell"),
            false => Command::new("sh"),
        };
        let output = command
            .args(["-c", single_command])
            .stdin(Stdio::null())
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            let stderr = match String::from_utf8(output.stderr) {
                Ok(stderr) => stderr,
                Err(_) => String::from("Cannot get stderr"),
            };
            return Err(stderr);
        } else {
            return Ok("Execute success".to_string());
        }
    }
    async fn run_test_commands(
        compile_command: &str,
        execute_command: &str,
        clear_command: &str,
    ) -> Result<String, String> {
        // Run compile command
        log::info!("Compile with command: {}", compile_command.bright_blue());
        if let Err(info) = Self::run_no_input_command(compile_command) {
            return Err(info);
        }
        let test_cases = match Utility::get_test_cases_filename_from_current_location() {
            Ok(test_cases) => test_cases,
            Err(info) => {
                return Err(info);
            }
        };

        // Run test command
        log::info!("Test with command: {}", execute_command.bright_blue());
        for case in test_cases {
            let input_file = case[0].clone();
            let output_file = case[1].clone();
            let file_in = match fs::read_to_string(input_file.clone()).await {
                Ok(stdin) => stdin,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let file_out = match fs::read_to_string(output_file.clone()).await {
                Ok(stdout) => stdout,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let mut command = match cfg!(target_os = "windows") {
                true => Command::new("powershell"),
                false => Command::new("sh"),
            };
            let child = command
                .args(["-c", execute_command])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute process");
            match child.stdin {
                Some(mut stdin) => match stdin.write_all(file_in.as_bytes()) {
                    Ok(_) => {}
                    Err(info) => {
                        return Err(info.to_string());
                    }
                },
                None => {
                    return Err("Cannot get stdin".to_string());
                }
            }
            match child.stdout {
                Some(mut stdout) => {
                    let mut stdout_str = String::new();
                    match stdout.read_to_string(&mut stdout_str) {
                        Ok(_) => {}
                        Err(info) => {
                            return Err(info.to_string());
                        }
                    }
                    let same = Difference::is_same(&file_out, &stdout_str);
                    if same {
                        println!("Test success with input file: {}", input_file.bright_blue());
                    } else {
                        println!("Test failed with input file: {}", input_file.red());
                        return Err(format!(
                            "Test failed with input file: {}",
                            input_file.bright_blue()
                        ));
                    }
                }
                None => {
                    return Err("Cannot get stdout".to_string());
                }
            }
        }

        // Run clear command
        log::info!("Clear with command: {}", clear_command.bright_blue());
        if let Err(info) = Self::run_no_input_command(clear_command) {
            return Err(info);
        }
        return Ok("Test success".to_string());
    }
}
