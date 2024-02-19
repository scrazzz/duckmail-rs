use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct APIResponse {
    address: String,
}

pub fn create_email(token: String) -> anyhow::Result<String> {
    let response = ureq::post("https://quack.duckduckgo.com/api/email/addresses")
        .set("authorization", format!("Bearer {}", token).as_str())
        .set("origin", "https://duckduckgo.com")
        .set("referer", "https://duckduckgo.com/")
        .call()
        .with_context(|| "Failed to create email".to_string())?;
    // println!("[DEBUG] {}", response.into_string()?);
    Ok(response.into_json::<APIResponse>()?.address)
}
