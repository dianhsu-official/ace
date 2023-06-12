use serde_derive::Deserialize;
use serde_derive::Serialize;
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

