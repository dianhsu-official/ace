use chrono::{Utc, Duration, DateTime, TimeZone};
use regex::Regex;


pub struct Utility{}

impl Utility{
    pub fn get_end_time(start_time: DateTime<Utc>, duration: &str) -> Result<DateTime<Utc>, String> {
        let time_vec = duration.split(":").collect::<Vec<_>>();
        let mut end_time = start_time;
        if time_vec.len() == 2 {
            let hour = match time_vec[0].parse::<i64>() {
                Ok(hour) => hour,
                Err(_) => {
                    return Err(String::from("Parse end time failed."));
                }
            };
            let min = match time_vec[1].parse::<i64>() {
                Ok(min) => min,
                Err(_) => {
                    return Err(String::from("Parse end time failed."));
                }
            };
            end_time = match end_time.checked_add_signed(Duration::hours(hour)) {
                Some(end_time) => end_time,
                None => {
                    return Err(String::from("Parse end time failed."));
                }
            };
            end_time = match end_time.checked_add_signed(Duration::minutes(min)) {
                Some(end_time) => end_time,
                None => {
                    return Err(String::from("Parse end time failed."));
                }
            };
        } else if time_vec.len() == 3 {
            let day = match time_vec[0].parse::<i64>() {
                Ok(day) => day,
                Err(_) => {
                    return Err(String::from("Parse end time failed."));
                }
            };
            let hour = match time_vec[1].parse::<i64>() {
                Ok(hour) => hour,
                Err(_) => {
                    return Err(String::from("Parse end time failed."));
                }
            };
            let min = match time_vec[2].parse::<i64>() {
                Ok(min) => min,
                Err(_) => {
                    return Err(String::from("Parse end time failed."));
                }
            };
            end_time = match end_time.checked_add_signed(Duration::days(day)) {
                Some(end_time) => end_time,
                None => {
                    return Err(String::from("Parse end time failed."));
                }
            };
            end_time = match end_time.checked_add_signed(Duration::hours(hour)) {
                Some(end_time) => end_time,
                None => {
                    return Err(String::from("Parse end time failed."));
                }
            };
            end_time = match end_time.checked_add_signed(Duration::minutes(min)) {
                Some(end_time) => end_time,
                None => {
                    return Err(String::from("Parse end time failed."));
                }
            };
        } else {
            return Err(String::from("Parse end time failed."));
        }
        return Ok(end_time);
    }
    pub fn get_start_time(start_time_href: &str) -> Result<DateTime<Utc>, String> {
        let re = match Regex::new(
            r#"\?day=(\d+)&month=(\d+)&year=(\d+)&hour=(\d+)&min=(\d+)&sec=(\d+)&"#,
        ) {
            Ok(re) => re,
            Err(_) => {
                return Err(String::from("Create regex failed."));
            }
        };
        let caps = match re.captures(start_time_href) {
            Some(caps) => caps,
            None => {
                return Err(String::from("Parse start time failed."));
            }
        };
        let day = match caps[1].parse::<u32>() {
            Ok(day) => day,
            Err(_) => {
                return Err(String::from("Parse start time failed."));
            }
        };
        let month = match caps[2].parse::<u32>() {
            Ok(month) => month,
            Err(_) => {
                return Err(String::from("Parse start time failed."));
            }
        };
        let year = match caps[3].parse::<i32>() {
            Ok(year) => year,
            Err(_) => {
                return Err(String::from("Parse start time failed."));
            }
        };
        let hour = match caps[4].parse::<u32>() {
            Ok(hour) => hour,
            Err(_) => {
                return Err(String::from("Parse start time failed."));
            }
        };
        let min = match caps[5].parse::<u32>() {
            Ok(min) => min,
            Err(_) => {
                return Err(String::from("Parse start time failed."));
            }
        };
        let sec = match caps[6].parse::<u32>() {
            Ok(sec) => sec,
            Err(_) => {
                return Err(String::from("Parse start time failed."));
            }
        };
        let start_time = match Utc
            .with_ymd_and_hms(year, month, day, hour, min, sec)
            .single()
        {
            Some(parsed_time) => parsed_time,
            None => {
                return Err(String::from("Parse start time failed."));
            }
        };
        return Ok(start_time);
    }
    pub fn get_bfaa() -> String {
        String::from("f1b3f18c715565b589b7823cda7448ce")
    }
    pub fn get_ftaa() -> String {
        random_str::get_string(18, true, false, true, false)
    }

    pub fn get_csrf(body: &str) -> Result<String, String> {
        let re = match Regex::new(r#"csrf='(.+?)'"#) {
            Ok(re) => re,
            Err(_) => {
                return Err(String::from("Create regex failed."));
            }
        };
        let csrf = match re.captures(body) {
            Some(caps) => caps[1].to_string(),
            None => {
                return Err(String::from("Parse csrf failed."));
            }
        };
        return Ok(csrf);
    }
}