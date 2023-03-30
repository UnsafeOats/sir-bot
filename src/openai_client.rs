use reqwest::Client;
use serde_json::json;
use std::env;

// TODO: once credentials struct is created, read OPENAI_API_KEY below from it instead of the
// environment
// 
// TODO: create enum for different engines with their specific api urls
//
// TODO: create chatgpt client struct and use the below function as an impl method
async fn fetch_openai_completion(prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let openai_api_key = env::var("OPENAI_API_KEY")?;
    let api_url = "https://api.openai.com/v1/engines/davinci-codex/completions";
    let client = Client::new();

    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", openai_api_key))
        // TODO: add some way to configure the extra parameters below
        .json(&json!({
            "prompt": prompt,
            "max_tokens": 1000,
            "n": 1,
            "stop": ["\n"],
            "temperature": 0.7,
        }))
        .send()
        .await?;

    let json_response = response.json::<serde_json::Value>().await?;
    let completion = json_response["choices"][0]["text"]
        .as_str()
        .ok_or("Failed to parse completion text")?
        .trim()
        .to_string();

    Ok(completion)
}

