mod api;

use std::time::Duration;

use api::XianNiuClient;
use crossbeam_channel::tick;


pub async fn auto_pause(phone: &str, password: &str) {
    let ticker = tick(Duration::from_secs(1800));
    loop {
        ticker.recv().unwrap();
        check_and_pause(phone, password).await;
    }
}

async fn check_and_pause(phone: &str, password: &str) {
    let mut client = XianNiuClient::new();
    let detail = match client.detail().await {
        Ok(d) => d,
        Err(_) => client.login(phone, password).await.unwrap(),
    };
    if detail.pause_state == 1 {
        let status = client.status().await.unwrap();
        match status {
            api::pasue_log::Status::PAUSED => (),
            api::pasue_log::Status::RUNNING(minute) => {
                if minute > 300 {
                    client.pause().await.unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::api::{pasue_log::Status, XianNiuClient};

    #[tokio::test]
    async fn it_works() {
        let mut client = XianNiuClient::new();
        client
            .login("phone", "password")
            .await
            .unwrap();
        let running_time = match client.status().await.unwrap() {
            Status::PAUSED => 0,
            Status::RUNNING(min) => min,
        };
        println!("{running_time}");
        client.pause().await.unwrap();
    }
}
