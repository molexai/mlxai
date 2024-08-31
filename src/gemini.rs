use reqwest::Client;
use serde_json::Value;
use regex::Regex;

pub(crate) async fn request(key: &str, model: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", model, key);

    let request_body = serde_json::json!({
        "contents": [{
            "parts": [{"text": text}]
        }]
    });

    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;

    let response_json: Value = serde_json::from_str(&response_text)?;
    if let Some(text) = response_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        let re = Regex::new(r"(\*\*|\*|```|`)").unwrap();
        println!("{}", re.replace_all(&text, ""));
    } else {
        println!("molexAI Error: No response text found.");
    }

    Ok(())
}