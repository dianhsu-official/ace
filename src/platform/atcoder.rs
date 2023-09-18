use crate::misc::http_client::HttpClient;
pub struct AtCoder {
    pub client: HttpClient,
    pub host: String,
}

impl AtCoder {
    #[allow(unused)]
    pub fn new(cookies: &str) -> Self {
        return Self {
            client: HttpClient::new(cookies, "https://atcoder.jp"),
            host: String::from("https://atcoder.jp"),
        };
    }
}
