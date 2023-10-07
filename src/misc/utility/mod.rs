use std::path;

use regex::Regex;

use crate::{
    constants::{ProgramLanguage, PLATFORM_MAP},
    database::CONFIG_DB,
    model::Platform,
};

pub mod account;

pub struct Utility {}

impl Utility {
    /// Get platform, contest identifier and problem identifier from current path.
    /// # Arguments
    /// * `cur_path` - Current path.
    /// # Returns
    /// * `Ok((Platform, String, String))` - Platform, contest identifier and contest problem identifier(e.g. [a, b, c, d, e]).
    pub fn get_identifiers_from_currrent_location(
        cur_path: &str,
        workspace: &str,
    ) -> Result<(Platform, String, String), String> {
        if !cur_path.starts_with(workspace) {
            return Err("current path is not in workspace".to_string());
        }
        let relative_path = match cur_path.strip_prefix(&workspace) {
            Some(path) => path,
            None => {
                return Err("cannot get relative path from workspace".to_string());
            }
        };
        let path_vec = relative_path
            .split(path::MAIN_SEPARATOR)
            .filter_map(|x| {
                if x.len() == 0 {
                    return None;
                }
                return Some(x);
            })
            .collect::<Vec<_>>();
        if path_vec.len() < 2 {
            return Err("invalid path".to_string());
        } else {
            let platform = match PLATFORM_MAP.get(path_vec[0].to_lowercase().as_str()) {
                Some(platform) => platform,
                None => {
                    return Err("invalid platform".to_string());
                }
            };
            let contest_identifier = path_vec[1].to_string();
            let problem_identifier = format!("{}_{}", path_vec[1], path_vec[2]);
            return Ok((*platform, contest_identifier, problem_identifier));
        }
    }
    pub fn get_program_language_from_filename(filename: &str) -> Result<ProgramLanguage, String> {
        let suffix = match filename.split(".").last() {
            Some(suffix) => suffix,
            None => {
                return Err("invalid filename".to_string());
            }
        };
        return CONFIG_DB.get_program_language_from_suffix(&suffix);
    }
    pub fn get_test_cases_filename_from_current_location() -> Result<Vec<[String; 2]>, String> {
        let current_path = match std::env::current_dir() {
            Ok(current_path) => current_path,
            Err(_) => {
                return Err("Cannot get current path".to_string());
            }
        };
        let re = Regex::new(r"^(\d+)i.txt$").unwrap();
        let files = match std::fs::read_dir(current_path.clone()) {
            Ok(files) => files
                .into_iter()
                .filter_map(|x| match x {
                    Ok(file) => match file.file_name().to_str() {
                        Some(filename) => {
                            if re.is_match(filename) {
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
        let mut test_cases = Vec::new();
        for input_file in files {
            let output_file = input_file.replace("i.txt", "o.txt");
            match std::fs::metadata(output_file.clone()) {
                Ok(_) => {
                    test_cases.push([input_file, output_file]);
                }
                Err(_) => {
                    return Err(format!("{} not found", output_file));
                }
            }
        }
        return Ok(test_cases);
    }
}

#[test]
fn test_get_indentifiers() {
    #[cfg(target_os = "windows")]
    let cur_path = r#"C:\Users\dianhsu\workspace\Atcoder\abc321\abc321_g"#;
    #[cfg(not(target_os = "windows"))]
    let cur_path = r#"/home/dianhsu/workspace/Atcoder/abc321/abc321_g"#;
    #[cfg(target_os = "windows")]
    let workspace = r#"C:\Users\dianhsu\workspace"#;
    #[cfg(not(target_os = "windows"))]
    let workspace = r#"/home/dianhsu/workspace"#;

    let res = Utility::get_identifiers_from_currrent_location(cur_path, workspace);
    assert_eq!(res.is_ok(), true);
}
