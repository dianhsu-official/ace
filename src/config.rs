use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;

use crate::codeforces::Codeforces;
#[derive(Serialize, Deserialize)]
pub struct CodeTemplate {
    pub alias: String,
    pub lang: String,
    pub path: String,
    pub suffix: String,
    pub before_script: String,
    pub script: String,
    pub after_script: String,
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub cf: Codeforces,
    pub templates: Vec<CodeTemplate>,
    pub generate_template: bool,
}

impl Config {
    pub fn new(path: &Path) -> Config {
        if !path.exists() {
            Config {
                cf: Codeforces::new(),
                templates: Vec::new(),
                generate_template: false,
            }
        } else {
            let mut file = match File::open(&path) {
                Err(_) => {
                    process::exit(101);
                }
                Ok(file) => file,
            };
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => {
                    log::error!("couldn't read from {}: {:?}", path.display(), why);
                    process::exit(102);
                }
                Ok(_) => {
                    log::debug!("successfully read from {}", path.display());
                }
            }
            serde_yaml::from_str(s.as_str()).unwrap()
        }
    }
}
