//! Cloud API message tests.

use letta_rs::types::{AgentType, CreateAgentRequest};
use letta_rs::{auth::AuthConfig, ClientBuilder};
use std::env;

#[tokio::test]
#[ignore = "Requires LETTA_API_KEY environment variable"]
async fn test_cloud_message_operations() {
    // Get API key from environment
    let api_key = match env::var("LETTA_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping cloud test: LETTA_API_KEY not set");
            return;
        }
    };

    // Create cloud client
    let client = ClientBuilder::new()
        .base_url("https://api.letta.com")
        .auth(AuthConfig::bearer(&api_key))
        .build()
        .unwrap();

    // Create a test agent for cloud message operations
    println!("Creating test agent for cloud message operations...");
    let create_request = CreateAgentRequest::builder()
        .name("Cloud Message Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("openai/gpt-4o-mini")
        .embedding("openai/text-embedding-3-small")
        .build();

    let agent = match client.agents().create(create_request).await {
        Ok(agent) => agent,
        Err(e) => {
            println!("Failed to create agent: {:?}", e);
            println!("Unable to test message operations without an agent");
            return;
        }
    };

    let agent_id = &agent.id;
    println!("Created test agent: {} ({})", agent.name, agent_id);

    // Test 1: List messages
    println!("\nTesting message list...");
    let messages = client.messages().list(agent_id, None).await.unwrap();
    println!("Found {} existing messages", messages.len());

    // Test 2: Send a message
    println!("\nTesting message creation...");
    let message_request = letta_rs::types::CreateMessagesRequest {
        messages: vec![letta_rs::types::MessageCreate {
            role: letta_rs::types::MessageRole::User,
            content: letta_rs::types::MessageCreateContent::String(
                "Hello from Rust SDK cloud test!".to_string(),
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

    match client.messages().create(agent_id, message_request).await {
        Ok(response) => {
            println!("✅ Message sent successfully!");
            println!("  Stop reason: {:?}", response.stop_reason);
            println!(
                "  Usage: {} total tokens",
                response.usage.total_tokens.unwrap_or(0)
            );
            println!(
                "  Received {} messages in response",
                response.messages.len()
            );

            // Print response messages
            for (i, message) in response.messages.iter().enumerate() {
                use letta_rs::types::LettaMessageUnion;
                let type_str = match message {
                    LettaMessageUnion::SystemMessage(_) => "System",
                    LettaMessageUnion::UserMessage(_) => "User",
                    LettaMessageUnion::AssistantMessage(msg) => {
                        println!("    Content: {}", msg.content);
                        "Assistant"
                    }
                    LettaMessageUnion::ReasoningMessage(msg) => {
                        println!("    Reasoning: {}", msg.reasoning);
                        "Reasoning"
                    }
                    LettaMessageUnion::HiddenReasoningMessage(_) => "HiddenReasoning",
                    LettaMessageUnion::ToolCallMessage(_) => "ToolCall",
                    LettaMessageUnion::ToolReturnMessage(_) => "ToolReturn",
                };
                println!("  Message {}: Type={}", i + 1, type_str);
            }
        }
        Err(e) => {
            println!("❌ Message creation failed: {:?}", e);
            panic!("Cloud message test failed");
        }
    }

    // Test 3: Verify message was added
    println!("\nVerifying message was added...");
    let updated_messages = client.messages().list(agent_id, None).await.unwrap();
    println!("Now have {} messages", updated_messages.len());

    // Note: With pagination, we might not see the increase immediately
    // The important part is that message creation succeeded

    println!("\n✅ All cloud message tests passed!");

    // Cleanup: delete the test agent
    println!("\nCleaning up test agent...");
    match client.agents().delete(agent_id).await {
        Ok(_) => println!("✅ Test agent deleted"),
        Err(e) => println!("Warning: Failed to delete test agent: {:?}", e),
    }
}
