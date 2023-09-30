use std::path;

use crate::{constants::PLATFORM_MAP, model::Platform};

pub mod account;

pub struct Utility {}

impl Utility {
    /// Get platform, contest identifier and problem identifier from current path.
    /// # Arguments
    /// * `cur_path` - Current path.
    /// # Returns
    /// * `Ok((Platform, String, String))` - Platform, contest identifier and contest problem identifier(e.g. [a, b, c, d, e]).
    pub fn get_indentifiers(
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

    let res = Utility::get_indentifiers(cur_path, workspace);
    assert_eq!(res.is_ok(), true);
}
