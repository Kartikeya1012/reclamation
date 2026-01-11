use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

pub async fn summarize_files(files: &[PathBuf]) -> Result<String, String> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| "ANTHROPIC_API_KEY environment variable not set".to_string())?;

    if files.is_empty() {
        return Ok("No files need review.".to_string());
    }

    let file_list = files.iter()
        .filter_map(|p| p.to_str())
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "You are analyzing files in a folder that need review before deletion.\nHere are the file paths:\n\n{}\n\nProvide a concise summary:\n1. What types of files are present?\n2. Are there any files that seem important?\n3. Overall safety assessment for deletion\n\nKeep response under 200 words.",
        file_list
    );

    let request_body = AnthropicRequest {
        model: "claude-sonnet-4-5-20250929".to_string(),
        max_tokens: 1024,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    let api_response: AnthropicResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(api_response.content
        .first()
        .map(|c| c.text.clone())
        .unwrap_or_else(|| "No summary generated.".to_string()))
}
