// nlp.rs - NLP-based tag suggestion for QuietWins
// This is a placeholder for future NLP logic. For now, it uses simple keyword matching.
// You can later replace this with a real NLP library (e.g., rust-bert, nlprule, etc.)

pub fn suggest_tags(text: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let lower = text.to_lowercase();
    // Example: simple keyword-based fallback
    if lower.contains("essay") || lower.contains("draft") || lower.contains("write") {
        tags.push("writing".to_string());
    }
    if lower.contains("walk") {
        tags.push("casual recreation".to_string());
    }
    if lower.contains("family") {
        tags.push("family bonding".to_string());
    }
    // ...add more rules or call NLP library here...
    tags
}

use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::mock_data::{get_mock_wins, MockWin};

#[derive(Debug, Serialize, Deserialize)]
pub struct NlpResult {
    pub sentiment: serde_json::Value,
    pub entities: Vec<Entity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub text: String,
    pub label: String,
}

pub async fn analyze_mock_wins() -> Result<Vec<(MockWin<'static>, NlpResult)>, reqwest::Error> {
    let wins = get_mock_wins();
    let texts: Vec<&str> = wins.iter().map(|w| w.text).collect();
    let client = Client::new();
    let resp = client
        .post("http://127.0.0.1:8000/analyze_batch")
        .json(&serde_json::json!({"texts": texts}))
        .send()
        .await?;
    let json: serde_json::Value = resp.json().await?;
    let results = json["results"].as_array().unwrap();
    let mut enriched = Vec::new();
    for (i, win) in wins.into_iter().enumerate() {
        let result = &results[i];
        let sentiment = result["sentiment"].clone();
        let entities: Vec<Entity> = serde_json::from_value(result["entities"].clone()).unwrap();
        enriched.push((win, NlpResult { sentiment, entities }));
    }
    Ok(enriched)
}

#[cfg(debug_assertions)]
#[tokio::main]
pub async fn run_nlp_on_mock_data() {
    match crate::nlp::analyze_mock_wins().await {
        Ok(results) => {
            for (win, nlp) in results {
                println!("[MOCK NLP] {} | Sentiment: {:?} | Entities: {:?}", win.text, nlp.sentiment, nlp.entities);
            }
        }
        Err(e) => {
            eprintln!("[MOCK NLP ERROR] {e}");
        }
    }
}
