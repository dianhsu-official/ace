use colored::Colorize;

pub struct Utility;
impl Utility {
    #[allow(unused)]
    pub fn get_input(prompt: &str) -> String {
        let input = dialoguer::Input::<String>::new()
            .with_prompt(prompt)
            .interact();
        return input.unwrap();
    }
    #[allow(unused)]
    pub fn get_password_input(prompt: &str) -> String {
        let password = dialoguer::Password::new().with_prompt(prompt).interact();
        return password.unwrap();
    }
    #[allow(unused)]
    pub fn choose_index(max_size: i32) -> i32 {
        let prompt = format!("Choose index from [0, {}]", max_size - 1);
        let re_prompt = format!(
            "Input index invalid. Choose index from [0, {}]",
            max_size - 1
        );
        let mut idx = -1;
        let mut res = Self::get_input(&prompt.green());
        while idx >= max_size || idx < 0 {
            idx = match res.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    res = Self::get_input(&re_prompt.green());
                    -1
                }
            }
        }
        return idx;
    }
}
