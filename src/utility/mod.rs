use std::path;

use crate::{
    constants::PLATFORM_MAP,
    database::CONFIG_DB,
    model::{LanguageConfig, Platform},
};

pub mod account;
pub mod http_client;
pub struct Utility {}

impl Utility {
    /// Get platform, contest identifier and problem identifier from current path.
    /// # Arguments
    /// * `cur_path` - Current path of code.
    /// # Returns
    /// * `Ok((Platform, String, String))` - Platform, contest identifier and contest problem identifier(e.g. [a, b, c, d, e]).
    pub fn get_identifiers_from_currrent_location(
        cur_path: &str,
        workspace: &str,
    ) -> Result<(Platform, String, String), String> {
        if !cur_path.starts_with(workspace) {
            return Err(format!(
                "current path <{}> is not in workspace <{}>",
                cur_path, workspace
            ));
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
        if path_vec.len() < 3 {
            return Err(format!(
                "invalid path: {}, current_path: {}, workspace: {}",
                relative_path, cur_path, workspace
            ));
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
    pub fn get_language_config_by_filename_and_platform(
        filename: &str,
        platform: Platform,
    ) -> Result<Vec<LanguageConfig>, String> {
        let suffix = match filename.split(".").last() {
            Some(suffix) => suffix,
            None => {
                return Err("invalid filename".to_string());
            }
        };
        let vec = match CONFIG_DB.get_language_config_by_suffix_and_platform(&suffix, platform) {
            Ok(vec) => vec,
            Err(info) => {
                return Err(info);
            }
        };
        return Ok(vec);
    }
    pub fn get_test_cases_filename_from_current_location() -> Result<Vec<[String; 2]>, String> {
        let current_path = match std::env::current_dir() {
            Ok(current_path) => current_path,
            Err(_) => {
                return Err("Cannot get current path".to_string());
            }
        };
        let mut idx = 1;
        let mut test_cases = Vec::new();
        loop {
            let input_file = format!("{:03}i.txt", idx);
            let output_file = format!("{:03}o.txt", idx);
            if !current_path.join(input_file.clone()).exists()
                || !current_path.join(output_file.clone()).exists()
            {
                break;
            }
            test_cases.push([input_file, output_file]);
            idx += 1;
        }
        return Ok(test_cases);
    }

    pub fn find_source_code_filename_from_directory(directory: &str) -> Vec<String> {
        let res = match std::fs::read_dir(directory) {
            Ok(files) => files
                .into_iter()
                .filter_map(|x| match x {
                    Ok(file) => match file.file_name().to_str() {
                        Some(filename) => {
                            if filename.starts_with("code") {
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
            Err(_) => Vec::new(),
        };
        return res;
    }
}

#[test]
fn test_get_indentifiers() {
    #[cfg(target_os = "windows")]
    let cur_path = r#"C:\Users\dianhsu\workspace\Atcoder\abc321\g"#;
    #[cfg(not(target_os = "windows"))]
    let cur_path = r#"/home/dianhsu/workspace/Atcoder/abc321/g"#;
    #[cfg(target_os = "windows")]
    let workspace = r#"C:\Users\dianhsu\workspace"#;
    #[cfg(not(target_os = "windows"))]
    let workspace = r#"/home/dianhsu/workspace"#;

    let res = Utility::get_identifiers_from_currrent_location(cur_path, workspace);
    assert_eq!(res.is_ok(), true);
}
