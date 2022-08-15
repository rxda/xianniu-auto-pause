use reqwest::header;

use crate::api::login::UserDetail;

use super::XianNiuClient;

impl XianNiuClient {
    pub async fn pause(&self) -> anyhow::Result<bool> {
        let mut headers = header::HeaderMap::new();
        headers.insert("authority", "webapi.xianniu.com".parse().unwrap());
        headers.insert(
            "accept",
            "application/json, text/plain, */*".parse().unwrap(),
        );
        headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
        headers.insert("cache-control", "no-cache".parse().unwrap());
        headers.insert(
            "content-type",
            "application/json;charset=UTF-8".parse().unwrap(),
        );
        headers.insert("dnt", "1".parse().unwrap());
        headers.insert("origin", "https://wap.xianniu.com".parse().unwrap());
        headers.insert("pragma", "no-cache".parse().unwrap());
        headers.insert("referer", "https://wap.xianniu.com/".parse().unwrap());
        headers.insert("sec-fetch-dest", "empty".parse().unwrap());
        headers.insert("sec-fetch-mode", "cors".parse().unwrap());
        headers.insert("sec-fetch-site", "same-site".parse().unwrap());
        headers.insert("user-agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1".parse().unwrap());


        let res = self
            .client
            .post("https://webapi.xianniu.com/api/member/suspend")
            .headers(headers)
            .body(format!(r#"{{"access_token":"{}"}}"#, self.token.clone().unwrap()))
            .send()
            .await?
            .json::<UserDetail>()
            .await?;
        Ok(res.pause_state == 0)
    }
}
