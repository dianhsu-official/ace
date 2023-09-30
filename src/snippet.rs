use crate::context::Context;


pub struct Snippet {}

impl Snippet {
    #[allow(dead_code)]
    pub fn get_list() -> Vec<String> {
        vec![
            "platform",
            "pid",
            "cid",
            "workspace",
            "full",
            "file",
            "rand",
            "Y",
            "M",
            "D",
            "h",
            "m",
            "s",
        ]
        .iter()
        .map(|s| format!("%${}$%", s))
        .collect()
    }
    #[allow(dead_code)]
    pub fn get_value(context: &Context, name: &str) -> Option<String> {
        match name {
            "%$platform$%" => match context.platform {
                Some(platform) => Some(platform.to_string()),
                None => None,
            },
            "%$pid$%" => match &context.problem_identifier {
                Some(pid) => Some(pid.clone()),
                None => None,
            },
            "%$cid$%" => match &context.contest_identifier {
                Some(cid) => Some(cid.clone()),
                None => None,
            },
            "%$workspace$%" => match &context.workspace_directory {
                Some(workspace) => Some(workspace.clone()),
                None => None,
            },
            "%$full$%" => match &context.filename_with_extension {
                Some(filename) => Some(filename.clone()),
                None => None,
            },
            "%$file$%" => match &context.filename_without_extension {
                Some(filename) => Some(filename.clone()),
                None => None,
            },
            "%$rand$%" => Some(random_str::get_string(8, true, false, true, false)),
            "%$Y$%" => Some(chrono::Local::now().format("%Y").to_string()),
            "%$M$%" => Some(chrono::Local::now().format("%m").to_string()),
            "%$D$%" => Some(chrono::Local::now().format("%d").to_string()),
            "%$h$%" => Some(chrono::Local::now().format("%H").to_string()),
            "%$m$%" => Some(chrono::Local::now().format("%M").to_string()),
            "%$s$%" => Some(chrono::Local::now().format("%S").to_string()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn replace(context: &Context, text: &str) -> String {
        let mut result = String::from(text);
        for placeholder in Snippet::get_list() {
            let value = match Snippet::get_value(&context, &placeholder) {
                Some(value) => value,
                None => continue,
            };
            result = result.replace(placeholder.as_str(), value.as_str());
        }
        return result;
    }
}

#[test]
fn test_template_replace() {
    let mut context = Context::new();
    context.filename_with_extension = Some("test.cpp".to_string());
    context.filename_without_extension = Some("test".to_string());
    let res = Snippet::replace(
        &context,
        "%$file$%_%$rand$%_%$Y$%_%$M$%_%$D$%_%$h$%_%$m$%_%$s$%",
    );
    assert_eq!(res.len(), 33);
}