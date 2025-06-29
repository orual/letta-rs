//! Simple message API test.

use letta_rs::{ClientConfig, LettaClient};

#[tokio::test]
async fn test_simple_message_list() {
    // Copy exact logic from working agent test
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Test 1: List agents (exactly like working test)
    println!("Testing agent list...");
    let agents = client.agents().list(None).await.unwrap();
    println!("Found {} agents", agents.len());
    assert!(!agents.is_empty(), "Server should have at least one agent");

    println!("✅ Agent list works! Now trying messages...");

    // Use a specific working agent ID instead of first agent
    let agent_id = "agent-44283816-340a-4ec7-939b-5d972085e490";
    let messages = client.messages().list(agent_id, None).await.unwrap();
    println!("Found {} messages for agent {}", messages.len(), agent_id);

    // Print first few messages
    for (i, message) in messages.iter().take(3).enumerate() {
        use letta_rs::types::LettaMessageUnion;
        let type_str = match message {
            LettaMessageUnion::SystemMessage(_) => "System",
            LettaMessageUnion::UserMessage(_) => "User",
            LettaMessageUnion::AssistantMessage(_) => "Assistant",
            LettaMessageUnion::ReasoningMessage(_) => "Reasoning",
            LettaMessageUnion::HiddenReasoningMessage(_) => "HiddenReasoning",
            LettaMessageUnion::ToolCallMessage(_) => "ToolCall",
            LettaMessageUnion::ToolReturnMessage(_) => "ToolReturn",
        };
        println!("  Message {}: Type={}", i + 1, type_str);
    }

    // Test message creation
    println!("Testing message creation...");
    let message_request = letta_rs::types::CreateMessagesRequest {
        messages: vec![letta_rs::types::MessageCreate {
            role: letta_rs::types::MessageRole::User,
            content: letta_rs::types::MessageCreateContent::String(
                "Hello from Rust SDK!".to_string(),
            ),
            name: None,
            otid: None,
            sender_id: None,
            batch_item_id: None,
            group_id: None,
        }],
        max_steps: Some(2),
        ..Default::default()
    };

    let response = client
        .messages()
        .create(agent_id, message_request)
        .await
        .unwrap();
    println!("✅ Message creation successful!");
    println!("  Stop reason: {:?}", response.stop_reason);
    println!(
        "  Usage: {} total tokens",
        response.usage.total_tokens.unwrap_or(0)
    );
    println!("  Received {} messages", response.messages.len());

    println!("✅ Message API works too!");
}
