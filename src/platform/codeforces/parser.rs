use chrono::Utc;
use regex::Regex;
use scraper::Html;
use scraper::Selector;

use crate::model::Contest;
use crate::model::ContestStatus;
use crate::model::PostSubmissionInfo;
use crate::model::TestCase;
use crate::model::Verdict;

use super::utility::Utility;

pub struct HtmlParser {}

impl HtmlParser {
    pub fn parse_contest(contest_identifier: &str, resp: &str) -> Result<Contest, String> {
        let document = Html::parse_document(&resp);

        let tr_selector_str = format!("table tr[data-contestid=\"{}\"]", contest_identifier);
        let tr_selector = match Selector::parse(&tr_selector_str) {
            Ok(tr_selector) => tr_selector,
            Err(_) => {
                return Err(String::from("Build tr selector failed."));
            }
        };
        let tr = match document.select(&tr_selector).next() {
            Some(tr) => tr,
            None => {
                return Err(String::from("Select tr failed."));
            }
        };
        let td_selector = match Selector::parse("td") {
            Ok(td_selector) => td_selector,
            Err(_) => {
                return Err(String::from("Build td selector failed."));
            }
        };
        let tds = tr.select(&td_selector).collect::<Vec<_>>();
        if tds.len() != 6 {
            return Err(String::from("Td count is not 6."));
        }
        let mut title = String::from("");
        for ele in tds[0].children() {
            if ele.value().is_text() {
                title = match ele.value().as_text() {
                    Some(title) => title.to_string(),
                    None => {
                        return Err(String::from("Can't get contest title."));
                    }
                };
                break;
            }
        }
        let anchor_selector = match Selector::parse("a") {
            Ok(anchor_selector) => anchor_selector,
            Err(_) => {
                return Err(String::from("Build anchor selector failed."));
            }
        };
        let start_time_href = match tds[2].select(&anchor_selector).next() {
            Some(start_time_anchor) => match start_time_anchor.value().attr("href") {
                Some(start_time_href) => start_time_href,
                None => {
                    return Err(String::from("Select start time href failed."));
                }
            },
            None => {
                return Err(String::from("Select start time anchor failed."));
            }
        };
        let duration = tds[3].text().collect::<String>().trim().to_string();
        let start_time = match Utility::get_start_time(&start_time_href) {
            Ok(start_time) => start_time,
            Err(info) => {
                return Err(info);
            }
        };
        let end_time = match Utility::get_end_time(start_time, &duration) {
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
            status: ContestStatus::NotStarted,
        };
        if start_time > Utc::now() {
            contest.status = ContestStatus::NotStarted;
        } else if Utc::now() <= end_time {
            contest.status = ContestStatus::Running;
        } else {
            contest.status = ContestStatus::Ended;
        }
        return Ok(contest);
    }
    pub fn parse_problem_list(
        contest_identifier: &str,
        resp: &str,
    ) -> Result<Vec<[String; 2]>, String> {
        let mut problems = Vec::new();
        let tr_selector = match Selector::parse(r#"table[class="problems"] tr"#) {
            Ok(tr_selector) => tr_selector,
            Err(_) => {
                return Err(String::from("Parse problem list failed."));
            }
        };
        let document = Html::parse_document(&resp);
        let td_selector = match Selector::parse("td") {
            Ok(td_selector) => td_selector,
            Err(_) => {
                return Err(String::from("Parse problem list failed."));
            }
        };
        let a_selector = match Selector::parse("a") {
            Ok(a_selector) => a_selector,
            Err(_) => {
                return Err(String::from("Parse problem list failed."));
            }
        };
        for td in document
            .select(&tr_selector)
            .filter_map(|x| x.select(&td_selector).next())
        {
            let problem_key = td.text().collect::<String>();
            let problem_url = match td.select(&a_selector).next() {
                Some(problem_anchor) => match problem_anchor.value().attr("href") {
                    Some(problem_url) => problem_url.to_string(),
                    None => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            };
            problems.push([
                format!("{}_{}", contest_identifier, problem_key.trim()),
                format!("https://codeforces.com{}", problem_url),
            ]);
        }
        return Ok(problems);
    }
    pub fn parse_recent_submission(resp: &str) -> Result<String, String> {
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
    pub fn parse_submission_page(
        submission_id: &str,
        contest_id: &str,
        problem_id: &str,
        resp: &str,
    ) -> Result<PostSubmissionInfo, String> {
        let document = Html::parse_document(&resp);
        let mut post_submission_info = PostSubmissionInfo::new();
        let table_selector = match Selector::parse("table") {
            Ok(table_selector) => table_selector,
            Err(_) => {
                return Err(String::from("Build table selector failed."));
            }
        };
        let td_selector = match Selector::parse("td") {
            Ok(td_selector) => td_selector,
            Err(_) => {
                return Err(String::from("Build td selector failed."));
            }
        };
        let table = match document.select(&table_selector).next() {
            Some(table) => table,
            None => {
                return Err(String::from("Select table failed."));
            }
        };
        let vec = table.select(&td_selector).collect::<Vec<_>>();
        if vec.len() != 11 {
            return Err(format!("Td count is not 11, but {}", vec.len()));
        }
        post_submission_info.submission_id = submission_id.to_string();
        post_submission_info.problem_identifier = format!("{}{}", contest_id, problem_id);
        post_submission_info.verdict_info = vec[4].text().collect::<String>().trim().to_string();
        if post_submission_info.verdict_info.contains("Running")
            || post_submission_info.verdict_info.contains("queue")
        {
            post_submission_info.verdict = Verdict::Waiting;
        } else {
            post_submission_info.verdict = Verdict::Resulted;
        }
        post_submission_info.execute_time = vec[5].text().collect::<String>().trim().to_string();
        post_submission_info.execute_memory = vec[6].text().collect::<String>().trim().to_string();
        return Ok(post_submission_info);
    }
    pub fn parse_test_cases(resp: &str) -> Result<Vec<TestCase>, String> {
        let document = Html::parse_document(resp);
        let mut res = Vec::new();
        let input_selector =
            match Selector::parse("div[class=\"sample-test\"] div[class=\"input\"] pre") {
                Ok(input_selector) => input_selector,
                Err(_) => {
                    return Err(String::from("Build input selector failed."));
                }
            };
        let output_selector =
            match Selector::parse("div[class=\"sample-test\"] div[class=\"output\"] pre") {
                Ok(output_selector) => output_selector,
                Err(_) => {
                    return Err(String::from("Build output selector failed."));
                }
            };
        let div_selector = match Selector::parse("div") {
            Ok(div_selector) => div_selector,
            Err(_) => {
                return Err(String::from("Build div selector failed."));
            }
        };
        let mut input_vec = Vec::new();
        let mut output_vec = Vec::new();
        for input_test_case in document.select(&input_selector) {
            let mut input = String::new();
            for div in input_test_case.select(&div_selector) {
                if input.len() != 0 {
                    input.push('\n');
                }
                let cur_str = div.text().collect::<String>();
                input.push_str(&cur_str);
            }
            if input.len() == 0 {
                input.push_str(input_test_case.text().collect::<String>().as_str());
            } else {
                input.push('\n');
            }
            input_vec.push(input);
        }
        for output_test_case in document.select(&output_selector) {
            let mut output = String::new();
            for div in output_test_case.select(&div_selector) {
                if output.len() != 0 {
                    output.push('\n');
                }
                let cur_str = div.text().collect::<String>();
                output.push_str(&cur_str);
            }
            if output.len() == 0 {
                output.push_str(output_test_case.text().collect::<String>().as_str());
            } else {
                output.push('\n');
            }
            output_vec.push(output);
        }
        if input_vec.len() != output_vec.len() {
            return Err(format!(
                "Input count is not equal to output count. {} != {}",
                input_vec.len(),
                output_vec.len()
            ));
        }
        for i in 0..input_vec.len() {
            res.push(TestCase {
                input: input_vec[i].clone(),
                output: output_vec[i].clone(),
            });
        }
        return Ok(res);
    }
}

#[test]
fn test_parse_contest() {
    let content = std::fs::read_to_string("assets/codeforces/contest.html").unwrap();
    let contest = HtmlParser::parse_contest("1868", &content).unwrap();
    assert_eq!(contest.identifier, "1868");
}
#[test]
fn test_parse_problem_list() {
    let content = std::fs::read_to_string("assets/codeforces/problem_list.html").unwrap();
    let problems = HtmlParser::parse_problem_list("1868", &content).unwrap();
    assert_eq!(problems.len(), 7);
}
#[test]
fn test_parse_recent_submission() {
    let content = std::fs::read_to_string("assets/codeforces/recent_submission.html").unwrap();
    let submission_id = HtmlParser::parse_recent_submission(&content).unwrap();
    assert_eq!(submission_id, "219682893");
}
#[test]
fn test_parse_submission_page() {
    let content = std::fs::read_to_string("assets/codeforces/submission_page.html").unwrap();
    let submission_info =
        HtmlParser::parse_submission_page("219682893", "1860", "C", &content).unwrap();
    assert_eq!(submission_info.submission_id, "219682893");
}
#[test]
fn test_parse_test_cases() {
    let content = std::fs::read_to_string("assets/codeforces/test_cases.html").unwrap();
    let test_cases = HtmlParser::parse_test_cases(&content).unwrap();
    assert_eq!(test_cases.len(), 1);
}
