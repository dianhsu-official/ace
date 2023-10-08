use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::{database::CONFIG_DB, utility::Utility, model::Platform};

#[derive(Debug, Clone)]
pub struct Context {
    pub platform: Option<Platform>,
    pub problem_identifier: Option<String>,
    pub contest_identifier: Option<String>,
    pub workspace_directory: Option<String>,
    /// The full file name of the source file, including the extension.
    pub filename_with_extension: Option<String>,
    /// The name of the source file, without the extension.
    pub filename_without_extension: Option<String>,
}
impl Context {
    pub fn new() -> Context {
        Context {
            platform: None,
            problem_identifier: None,
            contest_identifier: None,
            workspace_directory: None,
            filename_with_extension: None,
            filename_without_extension: None,
        }
    }
    pub fn update(&mut self, cur_path: &str) {
        if let Ok(workspace) = CONFIG_DB.get_config("workspace") {
            match Utility::get_identifiers_from_currrent_location(cur_path, &workspace) {
                Ok(res) => {
                    self.platform = Some(res.0);
                    self.contest_identifier = Some(res.1);
                    self.problem_identifier = Some(res.2);
                    self.workspace_directory = Some(cur_path.to_string());
                }
                Err(_) => {}
            };
        }
    }
}
lazy_static! {
    pub static ref CONTEXT: Mutex<Context> = Mutex::new(Context::new());
}
