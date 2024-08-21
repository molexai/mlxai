use dotenv::from_path;
use reqwest::Client;
use serde_json::json;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mlxai")]
struct Opt {
    personality: String,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let personality = &opt.personality;
    let text = &opt.text;

    request(personality, text).await?;
    Ok(())
}

async fn request(personality: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let env_path = env::var("GITHUB_TOKEN").unwrap_or_else(|_| String::from("C:/Users/User/RustroverProjects/mlxai/src/.env"));
    from_path(&env_path).expect("Failed to read .env file");

    let github_token = env::var("GITHUB_TOKEN")?;

    let client = Client::new();
    let url = "https://models.inference.ai.azure.com/chat/completions";

    let request_body = json!({
        "messages": [
            {
                "role": "system",
                "content": personality
            },
            {
                "role": "user",
                "content": text
            }
        ],
        "model": "Phi-3-medium-128k-instruct"
    });

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", github_token))
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("{}", response_text);

    Ok(())
}
