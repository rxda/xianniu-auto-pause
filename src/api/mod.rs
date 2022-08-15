use std::time::Duration;

mod login;
pub mod pasue_log;
mod pause;
mod detail;


pub struct XianNiuClient {
    client: reqwest::Client,
    token: Option<String>,
}

impl XianNiuClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        Self {
            client,
            token: None,
        }
    }
}


