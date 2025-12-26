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
