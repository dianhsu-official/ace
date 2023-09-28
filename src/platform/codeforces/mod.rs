use std::collections::HashMap;
mod builder;
mod config;
use crate::library::OnlineJudge;
use crate::misc::http_client::HttpClient;
use crate::model::Contest;
use crate::model::ContestStatus::{Ended, NotStarted, Running};
use crate::model::SubmissionInfo;
use crate::model::Verdict;
use builder::UrlBuilder;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use chrono::{DateTime, Duration, TimeZone, Utc};
use regex::Regex;
use soup::prelude::*;
use soup::Soup;
pub struct Codeforces {
    pub client: HttpClient,
}

impl OnlineJudge for Codeforces {
    /// Submit code to the platform.  
    ///
    /// problem_identifier: the identifier of the problem.  
    ///             For example, the identifier of the problem https://codeforces.com/problemset/problem/4/A is 4_A.  
    ///
    /// code: the code to submit.  
    ///
    /// lang_id: the language id of the code.  
    ///         For example, the language id of C++ is 73.  
    ///        You can get the language id from the submit page.  
    ///
    /// Return the submit id of the submit request.  
    fn submit(
        &mut self,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
    ) -> Result<String, String> {
        let info: Vec<&str> = problem_identifier.split("_").collect();
        if info.len() != 2 {
            return Err(String::from("Invalid identifier."));
        }
        let contest_id = info[0];
        let problem_id = info[1];
        let submit_page_url = UrlBuilder::build_submit_page_url(contest_id);
        let submit_page = match self.client.get(&submit_page_url) {
            Ok(page) => page,
            Err(err) => {
                return Err(String::from("unable to get submit page, ") + err.as_str());
            }
        };
        let mut params: HashMap<&str, &str> = HashMap::new();
        let csrf_token = match Self::get_csrf(&submit_page) {
            Ok(csrf_token) => csrf_token,
            Err(info) => {
                return Err(String::from("Submit failed, ") + info.as_str());
            }
        };
        let ftaa = Self::get_ftaa();
        let bfaa = Self::get_bfaa();
        params.insert("csrf_token", &csrf_token);
        params.insert("ftaa", &ftaa);
        params.insert("bfaa", &bfaa);
        params.insert("action", "submitSolutionFormSubmitted");
        params.insert("submittedProblemIndex", problem_id);
        params.insert("programTypeId", lang_id);
        params.insert("source", code);
        params.insert("tabSize", "4");
        params.insert("_tta", "176");
        let submit_url = UrlBuilder::build_submit_url(contest_id, &params["csrf_token"]);
        let resp = match self.client.post_form(&submit_url, &params) {
            Ok(resp) => resp,
            Err(err) => {
                return Err(String::from("Submit failed, ") + err.as_str());
            }
        };
        if resp.contains("You have submitted exactly the same code before") {
            return Err(String::from(
                "Submit failed, you have submitted exactly the same code before.",
            ));
        }
        return match Self::parse_recent_submit_id(&resp) {
            Ok(submit_id) => Ok(submit_id),
            Err(info) => Err(info),
        };
    }

    /// Check if the user is logged in.
    fn is_login(&mut self) -> Result<String, String> {
        let main_page = self.client.get("https://codeforces.com").unwrap();
        let re = match Regex::new(r#"handle = "([\s\S]+?)""#) {
            Ok(re) => re,
            Err(_) => return Err(String::from("Create regex failed.")),
        };
        let caps = match re.captures(main_page.as_str()) {
            Some(caps) => caps,
            None => return Err(String::from("Can't find handle.")),
        };
        return Ok(caps[1].to_string());
    }

    /// Login to the platform.
    fn login(&mut self, username: &str, password: &str) -> Result<String, String> {
        let login_page = match self.client.get(&UrlBuilder::build_index_url()) {
            Ok(login_page) => login_page,
            Err(info) => {
                return Err(info);
            }
        };
        let mut params: HashMap<&str, &str> = HashMap::new();
        let csrf_token = match Self::get_csrf(&login_page) {
            Ok(csrf_token) => csrf_token,
            Err(info) => {
                return Err(String::from("Login failed, ") + info.as_str());
            }
        };
        let ftaa = Self::get_ftaa();
        let bfaa = Self::get_bfaa();

        params.insert("csrf_token", &csrf_token);
        params.insert("action", "enter");
        params.insert("ftaa", &ftaa);
        params.insert("bfaa", &bfaa);
        params.insert("handleOrEmail", username);
        params.insert("password", password);
        params.insert("_tta", "176");
        params.insert("remember", "on");
        return match self
            .client
            .post_form(&UrlBuilder::build_login_url(), &params)
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(err),
        };
    }
    /// Get test cases
    fn get_test_cases(&mut self, problem_identifier: &str) -> Result<Vec<[String; 2]>, String> {
        let info: Vec<&str> = problem_identifier.split("_").collect();
        if info.len() != 2 {
            return Err(String::from("Invalid identifier."));
        }
        let contest_id = info[0];
        let problem_id = info[1];
        let resp = match self
            .client
            .get(&UrlBuilder::build_problem_url(contest_id, problem_id))
        {
            Ok(resp) => resp,
            Err(err) => {
                return Err(String::from("Get problem page failed, ") + err.as_str());
            }
        };
        return match Self::parse_test_cases(&resp) {
            Ok(test_cases) => Ok(test_cases),
            Err(info) => {
                return Err(String::from("Parse test cases failed, ") + info.as_str());
            }
        };
    }

    fn retrive_result(
        &mut self,
        problem_identifier: &str,
        submission_id: &str,
    ) -> Result<SubmissionInfo, String> {
        let info: Vec<&str> = problem_identifier.split("_").collect();
        if info.len() != 2 {
            return Err(String::from("Invalid identifier."));
        }
        let contest_id = info[0];
        let problem_id = info[1];
        let url = UrlBuilder::build_submission_url(contest_id, submission_id);
        let resp = match self.client.get(&url) {
            Ok(resp) => resp,
            Err(info) => {
                return Err(info);
            }
        };
        let soup = Soup::new(&resp);
        let mut submission_info = SubmissionInfo::new();
        let table_node = match soup.tag("table").find() {
            Some(table_node) => table_node,
            None => {
                return Err(String::from("Parse submission info failed."));
            }
        };
        let vec = table_node.tag("td").find_all().collect::<Vec<_>>();
        if vec.len() != 10 {
            return Err(String::from("Parse submission info failed."));
        }
        submission_info.submission_id = submission_id.to_string();
        submission_info.identifier = format!("{}{}", contest_id, problem_id);
        submission_info.verdict_info = vec[4].text().trim().to_string();
        submission_info.verdict = Codeforces::parse_verdict(&submission_info.verdict_info);
        submission_info.execute_time = vec[5].text().trim().to_string();
        submission_info.execute_memory = vec[6].text().trim().to_string();
        return Ok(submission_info);
    }

    fn get_problems(&mut self, contest_identifier: &str) -> Result<Vec<String>, String> {
        let problem_list_url = UrlBuilder::build_problem_list_url(contest_identifier);
        let resp = match self.client.get(&problem_list_url) {
            Ok(resp) => resp,
            Err(info) => {
                return Err(info);
            }
        };

        let soup = Soup::new(&resp);
        let table = match soup.tag("table").attr("class", "problems").find() {
            Some(table) => table,
            None => {
                return Err(String::from("Parse problem list failed."));
            }
        };
        let trs = table.tag("tr").find_all();
        let mut problems = Vec::new();
        for tr in trs {
            let tds = tr.tag("td").find_all().collect::<Vec<_>>();
            if tds.len() != 4 {
                continue;
            }
            let problem_key = tds[0].text();
            if problem_key == "#" {
                continue;
            }
            problems.push(format!("{}_{}", contest_identifier, problem_key.trim()));
        }
        return Ok(problems);
    }

    fn get_contest(&mut self, contest_identifier: &str) -> Result<Contest, String> {
        let contest_url = UrlBuilder::build_contest_url(contest_identifier);
        let resp = match self.client.get(&contest_url) {
            Ok(resp) => resp,
            Err(info) => {
                return Err(info);
            }
        };
        let soup = Soup::new(&resp);
        let tr = match soup
            .tag("tr")
            .attr("data-contestid", contest_identifier)
            .find()
        {
            Some(tr) => tr,
            None => {
                return Err(String::from("Parse contest failed."));
            }
        };
        let tds = tr.tag("td").find_all().collect::<Vec<_>>();
        if tds.len() != 6 {
            return Err(String::from("Parse contest failed."));
        }
        let mut title = String::from("");
        for ele in tds[0].children() {
            if ele.is_text() {
                title = ele.text();
                break;
            }
        }
        let start_time_archor = match tds[2].tag("a").find() {
            Some(start_time_archor) => start_time_archor,
            None => {
                return Err(String::from("Parse contest failed."));
            }
        };
        let start_time_href = match start_time_archor.get("href") {
            Some(start_time_href) => start_time_href,
            None => {
                return Err(String::from("Parse contest failed."));
            }
        };
        let duration = tds[3].text().trim().to_string();
        let start_time = match Self::get_start_time(&start_time_href) {
            Ok(start_time) => start_time,
            Err(info) => {
                return Err(info);
            }
        };
        let end_time = match Self::get_end_time(start_time, &duration) {
            Ok(end_time) => end_time,
            Err(info) => {
                return Err(info);
            }
        };
        let mut contest = Contest {
            identifier: contest_identifier.to_string(),
            title: title.trim().to_string(),
            start_time,
            end_time,
            status: NotStarted,
        };
        if start_time > Utc::now() {
            contest.status = NotStarted;
        } else if Utc::now() <= end_time {
            contest.status = Running;
        } else {
            contest.status = Ended;
        }
        return Ok(contest);
    }
}

impl Codeforces {
    #[allow(unused)]
    fn new(cookies: &str) -> Self {
        let endpoint = String::from("https://codeforces.com");
        Self {
            client: HttpClient::new(cookies, &endpoint),
        }
    }
    fn get_end_time(start_time: DateTime<Utc>, duration: &str) -> Result<DateTime<Utc>, String> {
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
    fn get_start_time(start_time_href: &str) -> Result<DateTime<Utc>, String> {
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
    fn get_bfaa() -> String {
        String::from("f1b3f18c715565b589b7823cda7448ce")
    }
    fn get_ftaa() -> String {
        random_str::get_string(18, true, false, true, false)
    }

    fn get_csrf(body: &str) -> Result<String, String> {
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
    fn to_hex_bytes(input: &str) -> [u8; 16] {
        let mut arr = [0; 32];
        for (i, c) in input.chars().enumerate() {
            arr[i] = c as u8;
        }
        let bytes = hex::decode(arr).unwrap();
        let mut output = [0u8; 16];
        output.copy_from_slice(&bytes);
        return output;
    }
    #[allow(unused)]
    fn get_rcpc(body: &str) -> String {
        if body.contains("Redirecting... Please, wait.") {
            return String::from("");
        }
        let re = match Regex::new(
            r#"var a=toNumbers\("([0-9a-f]*)"\),b=toNumbers\("([0-9a-f]*)"\),c=toNumbers\("([0-9a-f]*)"\);"#,
        ) {
            Ok(re) => re,
            Err(_) => return String::from(""),
        };
        let caps = match re.captures(body) {
            Some(caps) => caps,
            None => return String::from(""),
        };
        let key = Self::to_hex_bytes(caps[1].to_string().as_str());
        let iv = Self::to_hex_bytes(caps[2].to_string().as_str());
        let mut blk = Self::to_hex_bytes(caps[3].to_string().as_str()).into();
        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
        Aes128CbcDec::new(&key.into(), &iv.into()).decrypt_block_mut(&mut blk);
        return hex::encode(blk);
    }

    fn parse_verdict(verdict_info: &str) -> Verdict {
        if verdict_info.contains("Running") || verdict_info.contains("queue") {
            return Verdict::Waiting;
        } else {
            return Verdict::Resulted;
        }
    }
    /// Check if the contest is a regular contest.
    /// distinguish regular contest and gym contest.
    #[allow(unused)]
    fn is_regular_contest(identifier: &str) -> bool {
        return false;
    }
    #[allow(unused)]
    fn parse_recent_submit_id(resp: &str) -> Result<String, String> {
        let re = match Regex::new(r#"submissionId="(\d+)"#) {
            Ok(re) => re,
            Err(_) => return Err(String::from("Create regex failed.")),
        };
        let caps = match re.captures(resp) {
            Some(caps) => caps,
            None => return Err(String::from("Can't find submission id.")),
        };
        return Ok(caps[1].to_string());
    }
    #[allow(unused)]
    fn parse_test_cases(resp: &str) -> Result<Vec<[String; 2]>, String> {
        let soup = Soup::new(resp);
        let mut res = Vec::new();
        soup.tag("div")
            .attr("class", "sample-test")
            .find_all()
            .enumerate()
            .for_each(|(i, node)| {
                let mut input = String::new();
                let mut output = String::new();
                let input_div_node = match node.tag("div").attr("class", "input").find() {
                    Some(input_div_node) => input_div_node,
                    None => return,
                };
                let input_pre_node = match input_div_node.tag("pre").find() {
                    Some(input_pre_node) => input_pre_node,
                    None => return,
                };
                input_pre_node
                    .tag("div")
                    .find_all()
                    .enumerate()
                    .for_each(|(i, node)| {
                        if i != 0 {
                            input.push('\n');
                        }
                        input.push_str(node.text().as_str());
                    });
                if input.is_empty() {
                    input = input_pre_node.text();
                }
                let output_div_node = match node.tag("div").attr("class", "output").find() {
                    Some(output_div_node) => output_div_node,
                    None => return,
                };
                let output_pre_node = match output_div_node.tag("pre").find() {
                    Some(output_pre_node) => output_pre_node,
                    None => return,
                };
                output_pre_node
                    .tag("div")
                    .find_all()
                    .enumerate()
                    .for_each(|(i, node)| {
                        if i != 0 {
                            output.push('\n');
                        }
                        output.push_str(node.text().as_str());
                    });
                if output.is_empty() {
                    output = output_pre_node.text();
                }
                while input.ends_with("\n") {
                    input.pop();
                }
                while output.ends_with("\n") {
                    output.pop();
                }
                res.push([input, output]);
            });
        return Ok(res);
    }
}
#[test]
#[ignore = "local test"]
fn test_parse_test_cases() {
    let content = match std::fs::read_to_string("assets/codeforces/problem_1873A.html") {
        Ok(content) => content,
        Err(info) => {
            panic!("{}", info);
        }
    };
    let _ = Codeforces::parse_test_cases(&content);
}

#[test]
#[ignore = "local test"]
fn test_parse_recent_submit_id() {
    let content = match std::fs::read_to_string("assets/codeforces/submit_resp.html") {
        Ok(content) => content,
        Err(info) => {
            panic!("{}", info);
        }
    };
    let res = Codeforces::parse_recent_submit_id(&content);
    assert_eq!(res.is_ok(), true, "{}", res.err().unwrap());
    assert_eq!(res.unwrap(), "224642350");
}

#[test]
#[ignore = "local test"]
fn test_submit_code() {
    dotenv::dotenv().ok();
    let mut cf = Codeforces::new("");
    let username = match dotenv::var("CODEFORCES_USERNAME") {
        Ok(username) => username,
        Err(_) => {
            panic!(
                "Please set CODEFORCES_USERNAME in .env file or set it in the environment variable"
            );
        }
    };
    let password = match dotenv::var("CODEFORCES_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            panic!(
                "Please set CODEFORCES_PASSWORD in .env file or set it in the environment variable"
            );
        }
    };
    let login_res = cf.login(&username, &password);
    assert_eq!(login_res.is_ok(), true, "{}", login_res.err().unwrap());
    let is_login_res = cf.is_login();
    assert_eq!(
        is_login_res.is_ok(),
        true,
        "{}",
        is_login_res.err().unwrap()
    );
    let code = r#"
// 1
#include <iostream>
using namespace std;
int main() {
    int w;
    cin >> w;
    if (w % 2 == 0 && w > 2) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
}
    "#;
    let submit_res = cf.submit("4_A", code, "73");
    assert_eq!(submit_res.is_ok(), true, "{}", submit_res.err().unwrap());
    print!("{}", submit_res.unwrap());
}
#[test]
#[ignore = "local test"]
fn test_login() {
    dotenv::dotenv().ok();
    let mut cf = Codeforces::new("");
    let username = match dotenv::var("CODEFORCES_USERNAME") {
        Ok(username) => username,
        Err(_) => {
            panic!(
                "Please set CODEFORCES_USERNAME in .env file or set it in the environment variable"
            );
        }
    };
    let password = match dotenv::var("CODEFORCES_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            panic!(
                "Please set CODEFORCES_PASSWORD in .env file or set it in the environment variable"
            );
        }
    };
    match cf.login(&username, &password) {
        Ok(_) => {}
        Err(info) => {
            panic!("{}", info);
        }
    };
    match cf.is_login() {
        Ok(_) => {
            println!("Login successfully.");
        }
        Err(info) => {
            panic!("{}", info);
        }
    };
}

#[test]
#[ignore = "local test"]
fn test_get_problems() {
    dotenv::dotenv().ok();
    let mut cf = Codeforces::new("");
    let username = match dotenv::var("CODEFORCES_USERNAME") {
        Ok(username) => username,
        Err(_) => {
            panic!(
                "Please set CODEFORCES_USERNAME in .env file or set it in the environment variable"
            );
        }
    };
    let password = match dotenv::var("CODEFORCES_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            panic!(
                "Please set CODEFORCES_PASSWORD in .env file or set it in the environment variable"
            );
        }
    };
    match cf.login(&username, &password) {
        Ok(_) => {}
        Err(info) => {
            panic!("{}", info);
        }
    };
    match cf.is_login() {
        Ok(_) => {
            println!("Login successfully.");
        }
        Err(info) => {
            panic!("{}", info);
        }
    };
    match cf.get_problems("1878") {
        Ok(problems) => {
            println!("{:?}", problems);
        }
        Err(info) => {
            panic!("{}", info);
        }
    }
}
#[test]
fn test_get_verdict() {
    let mut cf = Codeforces::new("");
    match cf.retrive_result("1872_A", "223223698") {
        Ok(submission_info) => {
            println!("{:?}", submission_info);
        }
        Err(info) => {
            panic!("{}", info);
        }
    }
}

#[test]
#[ignore = "local test"]
fn test_get_contest() {
    let mut cf = Codeforces::new("");
    match cf.get_contest("1881") {
        Ok(contest) => {
            println!("{:?}", contest);
        }
        Err(info) => {
            panic!("{}", info);
        }
    }
}

#[test]
fn test_get_csrf_token() {
    let content = match std::fs::read_to_string("assets/codeforces/login_page.html") {
        Ok(content) => content,
        Err(info) => {
            panic!("{}", info);
        }
    };
    let csrf = match Codeforces::get_csrf(&content) {
        Ok(csrf) => csrf,
        Err(info) => {
            panic!("{}", info);
        }
    };
    println!("{}", csrf);
}
