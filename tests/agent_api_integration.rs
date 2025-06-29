//! Integration tests for agent API against real Letta endpoints.

use letta_rs::types::agent::*;
use letta_rs::types::memory::MemoryBlock;
use std::env;

/// Test that our CreateAgentRequest serialization works with the real API.
/// Run with: cargo test test_create_agent_request_real_api -- --nocapture --ignored
#[test]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
fn test_create_agent_request_real_api() {
    // Load API key from environment
    let api_key = env::var("LETTA_API_KEY").expect("LETTA_API_KEY must be set");

    // Create a minimal agent request
    let request = CreateAgentRequest::builder()
        .name("Test Agent from Rust SDK")
        .system("You are a helpful assistant for testing the Rust SDK.")
        .agent_type(AgentType::MemGPT)
        .memory_block(MemoryBlock {
            id: None,
            label: "human".to_string(),
            value: "The human is testing the Rust SDK.".to_string(),
            limit: Some(2000),
            is_template: false,
            preserve_on_migration: false,
            read_only: false,
            description: None,
            metadata: None,
            name: todo!(),
            organization_id: todo!(),
            created_by_id: todo!(),
            last_updated_by_id: todo!(),
            created_at: todo!(),
            updated_at: todo!(),
        })
        .memory_block(MemoryBlock {
            id: None,
            label: "persona".to_string(),
            value: "I am a test agent created by the Rust SDK.".to_string(),
            limit: Some(2000),
            is_template: false,
            preserve_on_migration: false,
            read_only: false,
            description: None,
            metadata: None,
            name: todo!(),
            organization_id: todo!(),
            created_by_id: todo!(),
            last_updated_by_id: todo!(),
            created_at: todo!(),
            updated_at: todo!(),
        })
        .tags(vec!["rust-sdk-test".to_string()])
        .build();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&request).unwrap();
    println!("Generated JSON:\n{}", json);

    // Write JSON to a temp file for curl
    let temp_file = "/tmp/letta_agent_request.json";
    std::fs::write(temp_file, &json).unwrap();

    // Execute curl command
    let output = std::process::Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("https://api.letta.com/v1/agents")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", api_key))
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(format!("@{}", temp_file))
        .arg("-v")
        .output()
        .expect("Failed to execute curl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    // Check if request was successful
    assert!(
        output.status.success() || stderr.contains("HTTP/2 201") || stderr.contains("HTTP/2 200"),
        "API request failed. Status: {:?}\nSTDERR: {}",
        output.status,
        stderr
    );

    // Parse response and verify agent was created
    if !stdout.is_empty() {
        let response: serde_json::Value =
            serde_json::from_str(&stdout).expect("Failed to parse API response as JSON");

        println!(
            "Parsed response:\n{}",
            serde_json::to_string_pretty(&response).unwrap()
        );

        // Verify essential fields
        assert_eq!(response["name"].as_str(), Some("Test Agent from Rust SDK"));
        assert_eq!(response["agent_type"].as_str(), Some("memgpt_agent"));

        // Clean up - delete the agent if ID is present
        if let Some(agent_id) = response["id"].as_str() {
            println!("Created agent with ID: {}", agent_id);

            // Delete the test agent
            let delete_output = std::process::Command::new("curl")
                .arg("-X")
                .arg("DELETE")
                .arg(format!("https://api.letta.com/v1/agents/{}", agent_id))
                .arg("-H")
                .arg(format!("Authorization: Bearer {}", api_key))
                .output()
                .expect("Failed to execute delete curl");

            if delete_output.status.success() {
                println!("Successfully deleted test agent");
            }
        }
    }
}

/// Test creating agent with shorthand fields
#[test]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
fn test_create_agent_shorthand_real_api() {
    let api_key = env::var("LETTA_API_KEY").expect("LETTA_API_KEY must be set");

    // Use shorthand fields
    let request = CreateAgentRequest {
        name: Some("Shorthand Test Agent".to_string()),
        model: Some("gpt-4o-mini".to_string()),
        embedding: Some("text-embedding-3-small".to_string()),
        context_window_limit: Some(16000),
        embedding_chunk_size: Some(300),
        tags: Some(vec!["rust-sdk-test".to_string(), "shorthand".to_string()]),
        ..Default::default()
    };

    let json = serde_json::to_string_pretty(&request).unwrap();
    println!("Generated shorthand JSON:\n{}", json);

    let temp_file = "/tmp/letta_agent_shorthand.json";
    std::fs::write(temp_file, &json).unwrap();

    let output = std::process::Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("https://api.letta.com/v1/agents")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", api_key))
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(format!("@{}", temp_file))
        .arg("-v")
        .output()
        .expect("Failed to execute curl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    // Clean up any created agent
    if !stdout.is_empty() {
        if let Ok(response) = serde_json::from_str::<serde_json::Value>(&stdout) {
            if let Some(agent_id) = response["id"].as_str() {
                std::process::Command::new("curl")
                    .arg("-X")
                    .arg("DELETE")
                    .arg(format!("https://api.letta.com/v1/agents/{}", agent_id))
                    .arg("-H")
                    .arg(format!("Authorization: Bearer {}", api_key))
                    .output()
                    .ok();
            }
        }
    }
}

/// Test listing agents and deserializing response
#[test]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
fn test_list_agents_real_api() {
    let api_key = env::var("LETTA_API_KEY").expect("LETTA_API_KEY must be set");

    // First, list agents
    let output = std::process::Command::new("curl")
        .arg("-X")
        .arg("GET")
        .arg("https://api.letta.com/v1/agents?limit=5")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", api_key))
        .output()
        .expect("Failed to execute curl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("List agents response:\n{}", stdout);

    // Try to deserialize into our Agent type
    if !stdout.is_empty() {
        let response: serde_json::Value =
            serde_json::from_str(&stdout).expect("Failed to parse list response as JSON");

        // The response should have an 'agents' array
        if let Some(agents) = response.as_array() {
            println!("Found {} agents", agents.len());

            // Try to deserialize the first agent
            if let Some(first_agent) = agents.first() {
                println!(
                    "First agent JSON:\n{}",
                    serde_json::to_string_pretty(first_agent).unwrap()
                );

                // This will help us identify any missing fields
                match serde_json::from_value::<Agent>(first_agent.clone()) {
                    Ok(agent) => {
                        println!("Successfully deserialized agent: {}", agent.name);
                        println!("Agent type: {:?}", agent.agent_type);
                    }
                    Err(e) => {
                        println!("Failed to deserialize agent: {}", e);
                        // This helps us identify what fields we might be missing
                    }
                }
            }
        }
    }
}
