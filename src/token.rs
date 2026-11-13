use std::collections::HashMap;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
    message: Option<String>,
}
pub async fn get_refresh() {
    let mut body = HashMap::new();
    body.insert("client_id", "00000000402b5328");
    body.insert("scope", "service::user.auth.xboxlive.com::MBI_SSL");
    let client = reqwest::Client::new();
    let res = client.post("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
    .form(&body)
    .send()
    .await;
    println!("0
    ßkj+")
}