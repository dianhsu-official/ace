use chrono::Utc;
use scraper::{Html, Selector};

use crate::model::{Contest, ContestStatus, PostSubmissionInfo, TestCase, Verdict};

use super::utility::Utility;

pub struct HtmlParser {}

impl HtmlParser {
    pub fn parse_submission_page(
        contest_identifier: &str,
        problem_identifier: &str,
        submission_id: &str,
        resp: &str,
    ) -> Result<PostSubmissionInfo, String> {
        let document = Html::parse_document(&resp);
        let tbody_selector = match Selector::parse("tbody") {
            Ok(tbody_selector) => tbody_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let tbody = match document.select(&tbody_selector).next() {
            Some(tbody) => tbody,
            None => {
                return Err(String::from("Failed to find tbody."));
            }
        };
        let tr_selector = match Selector::parse("tr") {
            Ok(tr_selector) => tr_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let td_selector = match Selector::parse("td") {
            Ok(td_selector) => td_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let tds = tbody
            .select(&tr_selector)
            .map(|tr| tr.select(&td_selector).next())
            .filter(|x| x.is_some())
            .collect::<Vec<_>>();
        if tds.len() != 9 && tds.len() != 7 {
            return Err(String::from("Failed to find tds."));
        }
        let status = tds[6].unwrap().text().collect::<String>();
        let mut execute_time = String::new();
        let mut execute_memory = String::new();
        if tds.len() == 9 {
            execute_time = tds[7].unwrap().text().collect::<String>();
            execute_memory = tds[8].unwrap().text().collect::<String>();
        }
        let mut submission_info = PostSubmissionInfo::new();
        submission_info.submission_id = String::from(submission_id);
        submission_info.contest_identifier = String::from(contest_identifier);
        submission_info.problem_identifier = String::from(problem_identifier);
        submission_info.verdict_info = status;
        submission_info.execute_time = execute_time;
        submission_info.execute_memory = execute_memory;
        submission_info.verdict = match submission_info.verdict_info.as_str() {
            "Judging" => Verdict::Waiting,
            "WJ" => Verdict::Waiting,
            _ => Verdict::Resulted,
        };
        return Ok(submission_info);
    }
    pub fn parse_problem_list(
        contest_identifier: &str,
        resp: &str,
    ) -> Result<Vec<[String; 2]>, String> {
        let document = Html::parse_document(&resp);
        let tbody_selector = match Selector::parse("tbody") {
            Ok(tbody_selector) => tbody_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let tbody = match document.select(&tbody_selector).next() {
            Some(tbody) => tbody,
            None => {
                return Err(String::from("Failed to find tbody."));
            }
        };
        let trs_selector = match Selector::parse("tr") {
            Ok(trs_selector) => trs_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let td_selector = match Selector::parse("td:first-child") {
            Ok(td_selector) => td_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let a_selector = match Selector::parse("a") {
            Ok(a_selector) => a_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let tds = tbody
            .select(&trs_selector)
            .map(|x| x.select(&td_selector).next())
            .collect::<Vec<_>>();

        let mut problems = Vec::new();
        for td in tds {
            match td {
                Some(td) => {
                    let problem_key = td.text().collect::<String>();
                    let problem_href = match td.select(&a_selector).next() {
                        Some(a) => match a.value().attr("href") {
                            Some(href) => href.to_string(),
                            None => {
                                continue;
                            }
                        },
                        None => {
                            continue;
                        }
                    };
                    problems.push([format!(
                        "{}_{}",
                        contest_identifier,
                        problem_key.to_lowercase()
                    ), format!("https://atcoder.jp{}", problem_href)]);
                }
                None => {
                    continue;
                }
            }
        }
        return Ok(problems);
    }
    pub fn parse_contest(contest_identifier: &str, resp: &str) -> Result<Contest, String> {
        let document = Html::parse_document(&resp);
        let title_selector = match Selector::parse(r#"h1[class="text-center"]"#) {
            Ok(title_selector) => title_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let title = match document.select(&title_selector).next() {
            Some(title) => title.text().collect::<String>(),
            None => {
                return Err(String::from("Failed to find title."));
            }
        };
        let contest_duration_selector =
            match Selector::parse(r#"small[class="contest-duration"] a"#) {
                Ok(contest_duration_selector) => contest_duration_selector,
                Err(info) => {
                    return Err(format!("Failed to parse selector, {}", info));
                }
            };
        let contest_duration_nodes = document
            .select(&contest_duration_selector)
            .filter_map(|x| x.value().attr("href"))
            .collect::<Vec<_>>();
        if contest_duration_nodes.len() != 2 {
            return Err(String::from("Failed to find contest duration nodes."));
        }
        let start_duration_href = &contest_duration_nodes[0];
        let end_duration_href = &contest_duration_nodes[1];
        let start_time = match Utility::get_datetime_from_href(&start_duration_href) {
            Ok(start_time) => start_time,
            Err(info) => return Err(info),
        };
        let end_time = match Utility::get_datetime_from_href(&end_duration_href) {
            Ok(end_time) => end_time,
            Err(info) => return Err(info),
        };
        let mut contest = Contest {
            identifier: String::from(contest_identifier),
            title,
            start_time,
            end_time,
            status: ContestStatus::NotStarted,
        };
        if start_time > Utc::now() {
            contest.status = ContestStatus::NotStarted;
        } else if end_time >= Utc::now() {
            contest.status = ContestStatus::Running;
        } else {
            contest.status = ContestStatus::Ended;
        }
        return Ok(contest);
    }
    pub fn parse_recent_submission_id(resp: &str) -> Result<String, String> {
        let document = Html::parse_document(resp);
        let td_selector = match Selector::parse(r#"td[class*="submission-score"]"#) {
            Ok(td_selector) => td_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let ele = document
            .select(&td_selector)
            .filter_map(|x| match x.value().attr("data-id") {
                Some(id) => Some(id),
                None => None,
            })
            .next();
        if let Some(ele) = ele {
            return Ok(String::from(ele));
        }
        return Err(String::from("Failed to get recent submit id."));
    }

    pub fn parse_test_cases(resp: &str) -> Result<Vec<TestCase>, String> {
        let document = Html::parse_document(&resp);
        let mut inputs_ja = vec![];
        let mut outputs_ja = vec![];
        let mut inputs_en = vec![];
        let mut outputs_en = vec![];
        let section_selector = match Selector::parse("section") {
            Ok(section_selector) => section_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let h3_selector = match Selector::parse("h3") {
            Ok(h3_selector) => h3_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        let pre_selector = match Selector::parse("pre") {
            Ok(pre_selector) => pre_selector,
            Err(info) => {
                return Err(format!("Failed to parse selector, {}", info));
            }
        };
        for section in document.select(&section_selector) {
            let h3 = match section.select(&h3_selector).next() {
                Some(h3) => h3,
                None => continue,
            };
            let pre = match section.select(&pre_selector).next() {
                Some(pre) => pre,
                None => continue,
            };
            let h3_text = h3.text().collect::<String>();
            let pre_text = pre.text().collect::<String>();
            if h3_text.contains("入力例") {
                inputs_ja.push(pre_text);
            } else if h3_text.contains("出力例") {
                outputs_ja.push(pre_text);
            } else if h3_text.contains("Sample Input") {
                inputs_en.push(pre_text);
            } else if h3_text.contains("Sample Output") {
                outputs_en.push(pre_text);
            }
        }
        let mut sample_vec = Vec::new();
        if !inputs_en.is_empty() && inputs_en.len() == outputs_en.len() {
            for i in 0..inputs_en.len() {
                sample_vec.push(TestCase {
                    input: inputs_en[i].clone(),
                    output: outputs_en[i].clone(),
                });
            }
        } else if !inputs_ja.is_empty() && inputs_ja.len() == outputs_ja.len() {
            for i in 0..inputs_ja.len() {
                sample_vec.push(TestCase {
                    input: inputs_ja[i].clone(),
                    output: outputs_ja[i].clone(),
                });
            }
        } else {
            return Err(String::from("Failed to get test cases."));
        }
        return Ok(sample_vec);
    }
}

#[test]
fn test_parse_submission_page() {
    let content = std::fs::read_to_string("assets/atcoder/submission_page.html").unwrap();
    let submission_info =
        HtmlParser::parse_submission_page("abc321", "b", "46033672", &content).unwrap();
    assert_eq!(submission_info.submission_id, "46033672");
    assert_eq!(submission_info.contest_identifier, "abc321");
    assert_eq!(submission_info.problem_identifier, "b");
    assert_eq!(submission_info.verdict_info, "WA");
}

#[test]
fn test_parse_problem_list() {
    let content = std::fs::read_to_string("assets/atcoder/problem_list.html").unwrap();
    let problems = HtmlParser::parse_problem_list("abc321", &content).unwrap();
    assert_eq!(problems.len(), 7);
}

#[test]
fn test_parse_contest() {
    let content = std::fs::read_to_string("assets/atcoder/contest.html").unwrap();
    let contest = HtmlParser::parse_contest("abc321", &content).unwrap();
    assert_eq!(contest.identifier, "abc321");
    assert_eq!(contest.title.contains("AtCoder Beginner Contest 321"), true);
}

#[test]
fn test_parse_recent_submission_id() {
    let content = std::fs::read_to_string("assets/atcoder/recent_submission.html").unwrap();
    let submission_id = HtmlParser::parse_recent_submission_id(&content).unwrap();
    assert_eq!(submission_id, "46003634");
}

#[test]
fn test_parse_test_cases() {
    let content = std::fs::read_to_string("assets/atcoder/test_cases.html").unwrap();
    let test_cases = HtmlParser::parse_test_cases(&content).unwrap();
    println!("{:?}", test_cases)
}
