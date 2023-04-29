use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
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
            log::info!("{}", path.display());
            let mut file = match File::open(&path) {
                Err(err) => {
                    log::error!("Open file failed. {}", err);
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
    pub fn save(&mut self, path: &Path) -> Result<(), io::Error>{
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => {
                log::error!("Save file failed. {}", err);
                process::exit(101);
            }
        };
        let write_str = match serde_yaml::to_string(&self) {
            Ok(config) => config,
            Err(err) => {
                log::error!("Save file failed. {}", err);
                process::exit(101);
            }
        };
        match file.write_all(write_str.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
