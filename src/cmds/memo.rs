use crate::Result;
use serde::Serialize;

const MEMO_URL: &str = "https://memo.azusachino.icu/api/memo?openId=";

#[derive(Serialize)]
struct MemoReq {
    content: String,
}

impl MemoReq {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

pub async fn handle_memo(msg: String) -> Result<()> {
    let open_id = std::env::var("MEMO_OPENID")?;

    let api_url = format!("{}{}", MEMO_URL, open_id);
    let req = MemoReq::new(msg);
    let r = reqwest::ClientBuilder::new()
        .build()?
        .post(api_url)
        .json(&req)
        .send()
        .await?;
    if r.status().is_success() {
        Ok(())
    } else {
        anyhow::bail!("fail to send memo request")
    }
}
