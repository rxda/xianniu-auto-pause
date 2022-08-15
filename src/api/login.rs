use md5::Digest;
use reqwest::header;
use serde::{Deserialize, Serialize};

use super::XianNiuClient;

#[derive(Serialize, Deserialize)]
pub struct LoginResp {
    #[serde(rename = "code")]
    code: i64,

    #[serde(rename = "msg")]
    msg: String,

    #[serde(rename = "data")]
    pub data: UserDetail,
}

#[derive(Serialize, Deserialize)]
pub struct UserDetail {
    #[serde(rename = "access_token")]
    access_token: String,

    #[serde(rename = "nickname")]
    nickname: String,

    #[serde(rename = "mobile")]
    mobile: String,

    #[serde(rename = "mobile_code")]
    mobile_code: i64,

    #[serde(rename = "state")]
    state: i64,

    #[serde(rename = "sex")]
    sex: i64,

    #[serde(rename = "address")]
    address: String,

    #[serde(rename = "age")]
    age: String,

    #[serde(rename = "avatar")]
    avatar: String,

    #[serde(rename = "package_id")]
    package_id: i64,

    #[serde(rename = "package_title")]
    package_title: String,

    #[serde(rename = "package_connects")]
    package_connects: i64,

    #[serde(rename = "is_master")]
    is_master: i64,

    #[serde(rename = "master_id")]
    master_id: i64,

    #[serde(rename = "is_payment")]
    is_payment: i64,

    #[serde(rename = "vip_pxperience")]
    vip_pxperience: String,

    #[serde(rename = "vip_level")]
    vip_level: i64,

    #[serde(rename = "is_payment_order")]
    is_payment_order: i64,

    #[serde(rename = "is_realname")]
    is_realname: i64,

    #[serde(rename = "realname_age")]
    realname_age: i64,

    #[serde(rename = "allow_age")]
    allow_age: i64,

    #[serde(rename = "billing_type")]
    billing_type: i64,

    // 暂停状态 0-暂停，1-启用
    #[serde(rename = "pause_state")]
    pub pause_state: i64,

    #[serde(rename = "buy_datetime")]
    buy_datetime: i64,

    #[serde(rename = "free_datetime")]
    free_datetime: i64,

    #[serde(rename = "expired_datetime")]
    expired_datetime: String,

    #[serde(rename = "total_datetime")]
    total_datetime: i64,

    #[serde(rename = "weibo_datetime")]
    weibo_datetime: i64,

    #[serde(rename = "weixinweb_datetime")]
    weixinweb_datetime: i64,

    #[serde(rename = "weixin_datetime")]
    weixin_datetime: i64,

    #[serde(rename = "qq_datetime")]
    qq_datetime: i64,

    #[serde(rename = "register_datetime")]
    register_datetime: String,

    #[serde(rename = "isset_pass")]
    isset_pass: i64,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "mobile_code")]
    mobile_code: String,

    #[serde(rename = "mobile")]
    mobile: String,

    #[serde(rename = "password")]
    password: String,

    #[serde(rename = "channel")]
    channel: String,

    #[serde(rename = "client_type")]
    client_type: i64,
}

impl Default for LoginRequest {
    fn default() -> Self {
        Self {
            mobile_code: "86".to_string(),
            mobile: "".to_string(),
            password: "".to_string(),
            channel: "guanwang".to_string(),
            client_type: 6,
        }
    }
}

impl XianNiuClient {
    pub async fn login(&mut self, phone: &str, password: &str) -> anyhow::Result<UserDetail> {
        self.login_with_md5_password(phone,format!("{:x}", md5::Md5::digest(password)).as_str()).await
    }

    pub async fn login_with_md5_password(
        &mut self,
        phone: &str,
        md5_password: &str,
    ) -> anyhow::Result<UserDetail> {
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

        let req = LoginRequest {
            mobile: phone.to_string(),
            password: md5_password.to_string(),
            ..Default::default()
        };

        let res = self
            .client
            .post("https://webapi.xianniu.com/api/auth/login")
            .headers(headers)
            .body(serde_json::to_string(&req)?)
            .send()
            .await?
            .json::<LoginResp>()
            .await?;
        self.token = Some(res.data.access_token.clone());
        Ok(res.data)
    }
}
