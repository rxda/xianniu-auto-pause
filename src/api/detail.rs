use reqwest::header;

use super::{XianNiuClient, login::{LoginResp, UserDetail}};

impl XianNiuClient {
    pub async fn detail(&self) -> anyhow::Result<UserDetail> {
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
        headers.insert("user-agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1".parse()?);

        let res = self.client
            .post("https://webapi.xianniu.com/api/member/details")
            .headers(headers)
            .body(format!(r#"{{"access_token":"{}"}}"#, self.token.clone().unwrap()))
            .send()
            .await?
            .json::<LoginResp>()
            .await?;
        Ok(res.data)
    }
}
