use crate::{
    constants::PLATFORM_MAP,
    model::Platform,
    platform::{atcoder::AtCoder, codeforces::Codeforces},
};

use super::model::RaceArgs;

pub struct RaceCommand {}

impl RaceCommand {
    pub async fn handle(args: RaceArgs) -> Result<String, String> {
        let real_platform = match PLATFORM_MAP.get(args.platform.as_str()) {
            Some(platform) => *platform,
            None => {
                return Err(format!("Platform {} not found", args.platform));
            }
        };
        let _ = match real_platform {
            Platform::Codeforces => {
                match Codeforces::new() {
                    Ok(cf) => cf,
                    Err(info) => {
                        return Err(info);
                    }
                };
            }
            Platform::AtCoder => {
                match AtCoder::new() {
                    Ok(atc) => atc,
                    Err(info) => {
                        return Err(info);
                    }
                };
            }
        };
        return Ok("".to_string());
    }
}
