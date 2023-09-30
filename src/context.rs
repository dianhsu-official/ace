use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::model::Platform;

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
}
lazy_static! {
    pub static ref CONTEXT: Mutex<Context> = Mutex::new(Context::new());
}
