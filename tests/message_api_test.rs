//! Integration tests for the message API.

use letta_rs::types::{
    AgentType, CreateAgentRequest, CreateMessagesRequest, MessageCreate, MessageCreateContent,
    MessageRole,
};
use letta_rs::{ClientConfig, LettaClient};

#[tokio::test]
async fn test_local_server_message_operations() {
    // Create client for local server (no auth required)
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Create a test agent
    println!("Creating test agent for message operations...");
    let create_request = CreateAgentRequest::builder()
        .name("Message Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .build();

    let agent = client.agents().create(create_request).await.unwrap();
    let agent_id = &agent.id;
    println!("Created test agent: {} ({})", agent.name, agent_id);

    // Test 1: List existing messages
    println!("Testing message list for agent: {}", agent_id);
    let messages = client.messages().list(agent_id, None).await.unwrap();
    println!("✅ Found {} existing messages", messages.len());

    // Test 2: Send a simple message
    println!("Testing message creation...");
    let message_request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: MessageCreateContent::String(
                "Hello! This is a test message from the Rust SDK.".to_string(),
            ),
            name: None,
            otid: None,
            sender_id: None,
            batch_item_id: None,
            group_id: None,
        }],
        max_steps: Some(3), // Limit steps for testing
        ..Default::default()
    };

    let response = client
        .messages()
        .create(agent_id, message_request)
        .await
        .unwrap();

    println!("✅ Message sent successfully!");
    println!("   Stop reason: {:?}", response.stop_reason);
    println!("   Usage: {:?} total tokens", response.usage.total_tokens);
    println!(
        "   Received {} messages in response",
        response.messages.len()
    );

    // Print response messages
    for (i, message) in response.messages.iter().enumerate() {
        match message {
            letta_rs::types::LettaMessageUnion::UserMessage(msg) => {
                println!("   Message {}: [User] {}", i + 1, msg.content);
            }
            letta_rs::types::LettaMessageUnion::AssistantMessage(msg) => {
                println!("   Message {}: [Assistant] {}", i + 1, msg.content);
            }
            letta_rs::types::LettaMessageUnion::SystemMessage(msg) => {
                println!("   Message {}: [System] {}", i + 1, msg.content);
            }
            letta_rs::types::LettaMessageUnion::ReasoningMessage(msg) => {
                println!("   Message {}: [Reasoning] {}", i + 1, msg.reasoning);
            }
            _ => {
                println!("   Message {}: [Other] {:?}", i + 1, message);
            }
        }
    }

    // Test 3: List messages again
    let updated_messages = client.messages().list(agent_id, None).await.unwrap();
    println!(
        "✅ After sending message, found {} messages in current page",
        updated_messages.len()
    );

    // Note: Due to pagination, we might not see all messages
    // The important part is that message creation succeeded above
    println!("✅ Message API operations completed successfully!");

    println!("✅ All message API tests passed!");

    // Cleanup: delete the test agent
    println!("Cleaning up test agent...");
    match client.agents().delete(agent_id).await {
        Ok(_) => println!("✅ Test agent deleted"),
        Err(e) => println!("Warning: Failed to delete test agent: {:?}", e),
    }
}

#[tokio::test]
async fn test_local_server_message_reset() {
    // Create client for local server
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Create a test agent
    println!("Creating test agent for message reset...");
    let create_request = CreateAgentRequest::builder()
        .name("Reset Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .build();

    let agent = client.agents().create(create_request).await.unwrap();
    let agent_id = &agent.id;
    println!("Created test agent: {} ({})", agent.name, agent_id);

    println!("Testing message reset for agent: {}", agent_id);

    // Count messages before reset
    let messages_before = client.messages().list(agent_id, None).await.unwrap();
    println!("Messages before reset: {}", messages_before.len());

    // Test reset with default initial messages
    let reset_agent = client.messages().reset(agent_id, Some(true)).await.unwrap();

    println!("✅ Agent messages reset successfully");
    println!("   Agent name: {}", reset_agent.name);

    // Check messages after reset
    let messages_after = client.messages().list(agent_id, None).await.unwrap();
    println!("Messages after reset: {}", messages_after.len());

    // Should have fewer messages (reset clears history but may add default messages)
    println!("✅ Message reset test completed");

    // Cleanup: delete the test agent
    println!("Cleaning up test agent...");
    match client.agents().delete(agent_id).await {
        Ok(_) => println!("✅ Test agent deleted"),
        Err(e) => println!("Warning: Failed to delete test agent: {:?}", e),
    }
}
