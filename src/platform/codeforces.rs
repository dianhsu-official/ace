use crate::platform::lib::OnlineJudge;
pub struct Codeforces;

impl OnlineJudge for Codeforces {
    fn submit() -> String {
        String::from("Codeforces submit")
    }

    fn login() -> String {
        String::from("Codeforces login")
    }

    fn get_test_cases() -> String {
        String::from("Codeforces get_test_cases")
    }
}
