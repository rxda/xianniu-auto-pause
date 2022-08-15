
use chrono::{NaiveTime, Local};
use reqwest::header;
use serde::{Deserialize, Serialize};

use super::{XianNiuClient};

#[derive(Serialize, Deserialize)]
pub struct PauseLogResp {
    #[serde(rename = "code")]
    code: i64,

    #[serde(rename = "msg")]
    msg: String,

    #[serde(rename = "data")]
    data: Vec<PauseLogItem>,
}

#[derive(Serialize, Deserialize)]
pub struct PauseLogItem {
    // 0暂停，1启动
    #[serde(rename = "option_type")]
    option_type: i64,

    #[serde(rename = "option_ip")]
    option_ip: String,

    #[serde(rename = "option_date")]
    option_date: String,

    #[serde(rename = "option_time")]
    option_time: String,
}

pub enum Status {
    PAUSED,
    RUNNING(i64),
}

impl XianNiuClient {
    pub async fn pause_log(&self) -> anyhow::Result<Vec<PauseLogItem>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("authority", "webapi.xianniu.com".parse()?);
        headers.insert("accept", "application/json, text/plain, */*".parse()?);
        headers.insert("accept-language", "zh-CN,zh;q=0.9".parse()?);
        headers.insert("cache-control", "no-cache".parse()?);
        headers.insert("content-type", "application/json;charset=UTF-8".parse()?);
        headers.insert("dnt", "1".parse()?);
        headers.insert("origin", "https://wap.xianniu.com".parse()?);
        headers.insert("pragma", "no-cache".parse()?);
        headers.insert("referer", "https://wap.xianniu.com/".parse()?);
        headers.insert("sec-fetch-dest", "empty".parse()?);
        headers.insert("sec-fetch-mode", "cors".parse()?);
        headers.insert("sec-fetch-site", "same-site".parse()?);
        headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36".parse()?);

        let res = self
            .client
            .post("https://webapi.xianniu.com/api/member/pause_log")
            .headers(headers)
            .body(format!(r#"{{"access_token":"{}"}}"#, self.token.clone().unwrap()))
            .send()
            .await?
            .json::<PauseLogResp>()
            .await?;
        Ok(res.data)
    }

    pub async fn status(&self) -> anyhow::Result<Status> {
        let logs = self.pause_log().await?;
        let last_log = logs.get(0).unwrap();
        if last_log.option_type == 0 {
            return Ok(Status::PAUSED);
        }
        let last_op_time_str = last_log.option_date.clone() + &last_log.option_time;
        let last_op_time = NaiveTime::parse_from_str(&last_op_time_str, "%Y-%m-%d %H:%M:%S")?;
        let now = Local::now().time();
        let duration = now - last_op_time;
        Ok(Status::RUNNING(duration.num_minutes()))
    }
}
