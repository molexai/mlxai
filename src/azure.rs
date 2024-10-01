use reqwest::Client;
use serde_json::Value;
use regex::Regex;

pub(crate) async fn request(model: &str, github_token: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://models.inference.ai.azure.com/chat/completions";

    let request_body = serde_json::json!({
        "messages": [
            {
                "role": "system",
                "content": "Respond with only the response no additional comments."
            },
            {
                "role": "user",
                "content": text
            }
        ],
        "temperature": 1.0,
        "top_p": 1.0,
        "max_tokens": 5000,
        "model": model
    });

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", github_token))
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;

    let response_json: Value = serde_json::from_str(&response_text)?;
    if let Some(text) = response_json["choices"][0]["message"]["content"].as_str() {
        let re = Regex::new(r"(\*\*|\*|```|`)").unwrap();
        println!("{}", re.replace_all(text, ""));
    } else {
        println!("Error: No response text found.");
    }

    Ok(())
}
