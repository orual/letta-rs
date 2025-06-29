//! Test agent creation specifically.

use letta_rs::types::{AgentType, CreateAgentRequest};
use letta_rs::{ClientConfig, LettaClient};

#[tokio::test]
async fn test_agent_creation_debug() {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    let create_request = CreateAgentRequest {
        name: Some("Rust SDK Test Agent".to_string()),
        system: Some("You are a test agent created by the Rust SDK integration test.".to_string()),
        agent_type: Some(AgentType::MemGPT),
        model: Some("letta/letta-free".to_string()),
        embedding: Some("letta/letta-free".to_string()),
        tags: Some(vec![
            "rust-sdk-test".to_string(),
            "integration-test".to_string(),
        ]),
        ..Default::default()
    };

    println!(
        "Request JSON: {}",
        serde_json::to_string_pretty(&create_request).unwrap()
    );

    match client.agents().create(create_request).await {
        Ok(agent) => {
            println!("✅ Successfully created agent: {}", agent.id);
            // Clean up
            client.agents().delete(&agent.id).await.ok();
        }
        Err(e) => {
            println!("❌ Failed to create agent: {}", e);
        }
    }
}
