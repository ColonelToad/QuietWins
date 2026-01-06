// nlp.rs - NLP-based tag suggestion for QuietWins
// Now integrates rule-based, NER, and sentiment tags.

pub fn suggest_tags(text: &str) -> Vec<String> {
    // Synchronous fallback for compatibility (rule-based only)
    let mut tags = Vec::new();
    let lower = text.to_lowercase();
    if lower.contains("essay") || lower.contains("draft") || lower.contains("write") {
        tags.push("writing".to_string());
    }
    if lower.contains("walk") {
        tags.push("casual recreation".to_string());
    }
    if lower.contains("family") {
        tags.push("family bonding".to_string());
    }
    tags
}

// Async: combines rule-based, NER, and sentiment tags
pub async fn suggest_tags_async(text: &str) -> Vec<String> {
    let mut tags = suggest_tags(text); // Start with rule-based

    // Call Python NLP service for NER and sentiment
    let client = Client::new();
    let resp = client
        .post("http://127.0.0.1:8000/analyze_batch")
        .json(&serde_json::json!({"texts": [text]}))
        .send()
        .await;
    if let Ok(resp) = resp {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(results) = json["results"].as_array() {
                if let Some(result) = results.get(0) {
                    // Sentiment
                    if let Some(sentiment) = result["sentiment"]["compound"].as_f64() {
                        if sentiment > 0.3 {
                            tags.push("positive".to_string());
                        } else if sentiment < -0.3 {
                            tags.push("negative".to_string());
                        } else {
                            tags.push("neutral".to_string());
                        }
                    }
                    // Entities
                    if let Some(entities) = result["entities"].as_array() {
                        for ent in entities {
                            if let (Some(text), Some(label)) =
                                (ent["text"].as_str(), ent["label"].as_str())
                            {
                                // Use label as tag, or combine
                                tags.push(label.to_lowercase());
                                // Optionally, add entity text as tag for PERSON/ORG/GPE
                                if ["PERSON", "ORG", "GPE"].contains(&label) {
                                    tags.push(text.to_lowercase());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Remove duplicates
    tags.sort();
    tags.dedup();
    tags
}

use crate::mock_data::{get_mock_wins, MockWin};
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
        enriched.push((
            win,
            NlpResult {
                sentiment,
                entities,
            },
        ));
    }
    Ok(enriched)
}

#[cfg(debug_assertions)]
#[tokio::main]
pub async fn run_nlp_on_mock_data() {
    match crate::nlp::analyze_mock_wins().await {
        Ok(results) => {
            for (win, nlp) in results {
                println!(
                    "[MOCK NLP] {} | Sentiment: {:?} | Entities: {:?}",
                    win.text, nlp.sentiment, nlp.entities
                );
            }
        }
        Err(e) => {
            // In dev, the Python service may not be running; log once and continue
            eprintln!("[MOCK NLP] Skipping mock analysis (service unreachable: {e})");
        }
    }
}
