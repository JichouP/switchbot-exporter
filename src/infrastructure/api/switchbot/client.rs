use crate::infrastructure::env::switchbot::SwitchBotEnv;
use chrono::Utc;
use reqwest::{
    self,
    header::{HeaderMap, AUTHORIZATION},
};
use ring::hmac;
use serde::Deserialize;

pub struct SwitchBotClient {
    base_url: String,
    client: reqwest::Client,
}

impl SwitchBotClient {
    pub fn new() -> Self {
        let SwitchBotEnv { token, secret } = SwitchBotEnv::new();

        let t = Utc::now().timestamp_millis();
        let nonce = uuid::Uuid::new_v4().to_string();
        let sign = {
            let key = hmac::Key::new(ring::hmac::HMAC_SHA256, secret.as_bytes());
            let data = format!("{}{}{}", token, t, nonce);
            let sign = hmac::sign(&key, data.as_bytes());
            base64::encode(sign)
        };

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, token.parse().unwrap());
        headers.insert("sign", sign.parse().unwrap());
        headers.insert("t", t.to_string().parse().unwrap());
        headers.insert("nonce", nonce.parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            base_url: "https://api.switch-bot.com/v1.1".to_string(),
            client,
        }
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> anyhow::Result<T> {
        Ok(self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?
            .json::<T>()
            .await?)
    }
}
