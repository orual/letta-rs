//! Integration tests for the cloud Letta API endpoints.

use letta_rs::auth::AuthConfig;
use letta_rs::{types::AgentsSearchRequest, ClientConfig, LettaClient};
use std::env;

#[tokio::test]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
async fn test_cloud_agent_search() {
    // Load API key from environment
    let api_key = match env::var("LETTA_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("⚠️  LETTA_API_KEY not set, skipping cloud API test");
            return;
        }
    };

    // Create client for cloud API
    let config = ClientConfig::new("https://api.letta.com")
        .unwrap()
        .auth(AuthConfig::bearer(api_key));
    let client = LettaClient::new(config).unwrap();

    // Test search functionality
    println!("Testing agent search on cloud API...");
    let search_request = AgentsSearchRequest {
        project_id: Some("0b0ed44d-eb63-4f63-acc9-b220aa523438".to_string()),
        limit: Some(5),
        ..Default::default()
    };

    let search_results = client.agents().search(search_request).await.unwrap();
    println!("✅ Search returned {} agents", search_results.agents.len());

    // Search results can be empty if user has no agents, so just verify we got a response
    println!("✅ Cloud agent search test completed successfully");
}

#[tokio::test]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
async fn test_cloud_agent_count() {
    // Load API key from environment
    let api_key = match env::var("LETTA_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("⚠️  LETTA_API_KEY not set, skipping cloud API test");
            return;
        }
    };

    // Create client for cloud API
    let config = ClientConfig::new("https://api.letta.com")
        .unwrap()
        .auth(AuthConfig::bearer(api_key));
    let client = LettaClient::new(config).unwrap();

    // Test count functionality
    println!("Testing agent count on cloud API...");
    let count = client.agents().count().await.unwrap();
    println!("✅ Agent count: {}", count);

    // Count can be 0 if user has no agents
    println!("✅ Cloud agent count test completed successfully");
}

#[tokio::test]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
async fn test_cloud_agent_list() {
    // Load API key from environment
    let api_key = match env::var("LETTA_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("⚠️  LETTA_API_KEY not set, skipping cloud API test");
            return;
        }
    };

    // Create client for cloud API
    let config = ClientConfig::new("https://api.letta.com")
        .unwrap()
        .auth(AuthConfig::bearer(api_key));
    let client = LettaClient::new(config).unwrap();

    // Test list functionality
    println!("Testing agent list on cloud API...");
    let agents = client.agents().list(None).await.unwrap();
    println!("✅ Found {} agents", agents.len());

    // Print first few agent names if any exist
    for (i, agent) in agents.iter().take(3).enumerate() {
        println!("  Agent {}: {} ({})", i + 1, agent.name, agent.id);
    }

    println!("✅ Cloud agent list test completed successfully");
}
