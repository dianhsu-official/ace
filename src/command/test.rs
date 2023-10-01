use inquire::Select;

use crate::context::CONTEXT;
use crate::database::CONFIG_DB;
use crate::logger::LOGGER;
use crate::misc::utility::Utility;
use crate::snippet::Snippet;

use super::model::TestArgs;
use std::env::current_dir;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
pub struct TestCommand {}

impl TestCommand {
    pub fn handle(args: TestArgs) -> Result<String, String> {
        let current_path = match current_dir() {
            Ok(current_path) => current_path,
            Err(_) => {
                return Err("Cannot get current path".to_string());
            }
        };
        let filename = match args.filename {
            Some(filename) => filename,
            None => {
                let files = match fs::read_dir(current_path.clone()) {
                    Ok(files) => files
                        .into_iter()
                        .filter_map(|x| match x {
                            Ok(file) => match file.file_name().to_str() {
                                Some(filename) => Some(filename.to_string()),
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
                    Err(_) => {
                        return Err("Cannot get current path".to_string());
                    }
                };
                filename
            }
        };
        if let Some(current_path_str) = current_path.to_str() {
            if let Ok(mut context) = CONTEXT.lock() {
                context.update(current_path_str);
            }
        }
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
        let language = match Utility::get_program_language_from_filename(&filename) {
            Ok(language) => language,
            Err(info) => {
                return Err(info);
            }
        };
        let mut language_config = match CONFIG_DB.get_language_config(language) {
            Ok(config) => config,
            Err(info) => {
                return Err(info);
            }
        };
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
        );
    }
    fn run_no_input_command(single_command: &str) -> Result<String, String> {
        if cfg!(target_os = "windows") {
            println!("run command: powershell -c {}", single_command);
            let output = Command::new("powershell")
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
        } else {
            println!("run command: sh -c {}", single_command);
            let output = Command::new("sh")
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
    }
    fn run_test_commands(
        compile_command: &str,
        execute_command: &str,
        clear_command: &str,
    ) -> Result<String, String> {
        if let Err(info) = Self::run_no_input_command(compile_command) {
            return Err(info);
        }
        let test_cases = match Utility::get_test_cases_filename_from_current_dir() {
            Ok(test_cases) => test_cases,
            Err(info) => {
                return Err(info);
            }
        };

        for case in test_cases {
            let input_file = case[0].clone();
            let output_file = case[1].clone();
            let file_in = match fs::read_to_string(input_file.clone()) {
                Ok(stdin) => stdin,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let file_out = match fs::read_to_string(output_file.clone()) {
                Ok(stdout) => stdout,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            if cfg!(target_os = "windows") {
                LOGGER.info(&format!("run command: powershell -c {}", execute_command));
                let command = Command::new("powershell")
                    .args(["-c", execute_command])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process");
                match command.stdin {
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
                match command.stdout {
                    Some(mut stdout) => {
                        let mut stdout_str = String::new();
                        match stdout.read_to_string(&mut stdout_str) {
                            Ok(_) => {}
                            Err(info) => {
                                return Err(info.to_string());
                            }
                        }
                        if stdout_str != file_out {
                            return Err(format!("Test failed: \noutput:\n---------------\n{}----------\nexpect:\n--------------------\n{}", stdout_str, file_out));
                        }
                    }
                    None => {
                        return Err("Cannot get stdout".to_string());
                    }
                }
            } else {
                LOGGER.info(&format!("run command: sh -c {}", execute_command));
                let command = Command::new("sh")
                    .args(["-c", execute_command])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process");
                match command.stdin {
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
                match command.stdout {
                    Some(mut stdout) => {
                        let mut stdout_str = String::new();
                        match stdout.read_to_string(&mut stdout_str) {
                            Ok(_) => {}
                            Err(info) => {
                                return Err(info.to_string());
                            }
                        }
                        if stdout_str != file_out {
                            return Err(format!("Test failed: \noutput:\n---------------\n{}----------\nexpect:\n--------------------\n{}", stdout_str, file_out));
                        }
                    }
                    None => {
                        return Err("Cannot get stdout".to_string());
                    }
                }
            }
        }
        if let Err(info) = Self::run_no_input_command(clear_command) {
            return Err(info);
        }
        return Ok("Test success".to_string());
    }
}
