use chrono::{DateTime, Utc, NaiveTime, NaiveDate, FixedOffset, TimeZone};

pub struct Utility {}

impl Utility {
    pub fn get_datetime_from_href(href: &str) -> Result<DateTime<Utc>, String> {
        let re = match regex::Regex::new(r#"iso=([\d]+T[\d]+)"#) {
            Ok(re) => re,
            Err(_) => return Err(String::from("Failed to create regex.")),
        };
        let caps = match re.captures(&href) {
            Some(caps) => caps,
            None => {
                return Err(String::from("Failed to find start time caps."));
            }
        };
        let datetime_str = match caps.get(1) {
            Some(datetime_str) => datetime_str.as_str(),
            None => {
                return Err(String::from("Failed to find time."));
            }
        };
        let naive_time = match NaiveTime::parse_from_str(datetime_str, "%Y%m%dT%H%M") {
            Ok(naive_time) => naive_time,
            Err(info) => {
                return Err(format!("Failed to parse time, {}", info));
            }
        };
        let naive_date = match NaiveDate::parse_from_str(datetime_str, "%Y%m%dT%H%M") {
            Ok(naive_date) => naive_date,
            Err(info) => {
                return Err(format!("Failed to parse time, {}", info));
            }
        };
        let tz_offset = match FixedOffset::east_opt(9 * 3600) {
            Some(tz_offset) => tz_offset,
            None => {
                return Err(String::from("Failed to get timezone offset."));
            }
        };
        let naive_dt = naive_date.and_time(naive_time);
        let dt_with_tz = tz_offset.from_local_datetime(&naive_dt).unwrap();
        let parsed_time = Utc.from_utc_datetime(&dt_with_tz.naive_utc());
        return Ok(parsed_time);
    }
    pub fn get_csrf(resp: &str) -> Result<String, String> {
        let re = match regex::Regex::new(r#"var csrfToken = "([\S]+)""#) {
            Ok(re) => re,
            Err(_) => {
                return Err(String::from("Failed to create regex."));
            },
        };
        let caps = match re.captures(resp) {
            Some(caps) => caps,
            None => {
                return Err(String::from("Failed to find csrf token."));
            },
        };
        return Ok(String::from(&caps[1]));
    }
}
