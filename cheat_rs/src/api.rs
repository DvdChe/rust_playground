use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::process;
use std::time::Duration;

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

pub async fn query_gemini(
    model: &str,
    api_key: &str,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let spinner = ProgressBar::new_spinner();

    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Searching command..."));
    spinner.enable_steady_tick(Duration::from_millis(100));
    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let client = reqwest::Client::new();
    let response = client.post(&url).json(&request_body).send().await?;

    spinner.finish_and_clear();

    if !response.status().is_success() {
        eprintln!("Erreur API: Status: {}", response.status());
        process::exit(1);
    }

    let response_json: GeminiResponse = response.json().await?;

    if let Some(candidate) = response_json.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            return Ok(part.text.trim().to_string());
        }
    }

    Err("The ai did not found any parts".into())
}
