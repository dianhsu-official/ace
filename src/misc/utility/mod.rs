use std::path;

use inquire::Select;

use crate::{
    constants::{ProgramLanguage, PLATFORM_MAP},
    database::CONFIG_DB,
    model::Platform, platform,
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
        if path_vec.len() < 4 {
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
    pub fn get_program_language_from_filename(filename: &str) -> Result<ProgramLanguage, String> {
        let suffix = match filename.split(".").last() {
            Some(suffix) => suffix,
            None => {
                return Err("invalid filename".to_string());
            }
        };
        let vec = match CONFIG_DB.get_language_submit_config_from_suffix(&suffix) {
            Ok(vec) => vec,
            Err(info) => {
                return Err(info);
            }
        };
        match vec.len() {
            0 => {
                return Err(format!("cannot find language from suffix: {}", suffix));
            }
            1 => {
                return Ok(vec[0].identifier.clone());
            }
            _ => {
                let item = Select::new("Select language", vec).prompt().unwrap();
                return Ok(item.identifier);
            }
        }
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
