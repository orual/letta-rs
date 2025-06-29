//! Simple test to isolate the issue.

use letta_rs::{ClientConfig, LettaClient};

#[tokio::test]
async fn test_simple_request() {
    // Create client for local server (no auth required)
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Make a simple request first
    let response = client
        .http()
        .get("http://localhost:8283/v1/agents/")
        .send()
        .await
        .unwrap();

    println!("Status: {}", response.status());

    let text = response.text().await.unwrap();
    println!("Response first 500 chars: {}", &text[..text.len().min(500)]);

    // Try to parse it manually
    let agents: serde_json::Value = serde_json::from_str(&text).unwrap();
    println!("Successfully parsed JSON. Is array: {}", agents.is_array());

    if let Some(array) = agents.as_array() {
        println!("Got array with {} items", array.len());
        if !array.is_empty() {
            println!(
                "First agent keys: {:?}",
                array[0].as_object().unwrap().keys().collect::<Vec<_>>()
            );
        }
    }
}
