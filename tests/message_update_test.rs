//! Integration tests for message update functionality.

use letta_rs::client::ClientBuilder;
use letta_rs::error::LettaResult;
use letta_rs::types::{
    Block, CreateAgentRequest, CreateMessagesRequest, MessageCreate, MessageRole,
    UpdateMessageRequest, UpdateUserMessage, UpdateUserMessageContent,
};
use letta_rs::{LettaClient, LettaId};
use serial_test::serial;

/// Create a test client for the local server.
fn create_test_client() -> LettaResult<LettaClient> {
    ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()
}

/// Create a test agent.
async fn create_test_agent(client: &LettaClient) -> LettaResult<LettaId> {
    let request = CreateAgentRequest::builder()
        .name("Test Message Update Agent")
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .memory_block(Block {
            id: None,
            label: "human".to_string(),
            value: "The human's name is Test User.".to_string(),
            limit: Some(1000),
            is_template: false,
            preserve_on_migration: true,
            read_only: false,
            description: Some("Human information".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        })
        .memory_block(Block {
            id: None,
            label: "persona".to_string(),
            value: "I am a helpful test assistant.".to_string(),
            limit: Some(500),
            is_template: false,
            preserve_on_migration: true,
            read_only: false,
            description: Some("Agent persona".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        })
        .build();

    let agent = client.agents().create(request).await?;
    Ok(agent.id)
}

#[tokio::test]
#[serial]
async fn test_update_user_message() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // First, create a message
    let create_request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: "Original message content".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let _response = client.messages().create(&agent_id, create_request).await?;

    // List messages to find the user message we just created
    let messages = client.messages().list(&agent_id, None).await?;

    // Find the user message with our content
    let user_message_id = messages
        .iter()
        .find_map(|msg| match msg {
            letta_rs::types::LettaMessageUnion::UserMessage(user_msg)
                if user_msg.content == "Original message content" =>
            {
                Some(user_msg.id.clone())
            }
            _ => None,
        })
        .expect("Should find the user message we created");

    // Update the message
    let update_request = UpdateMessageRequest::UserMessage(UpdateUserMessage {
        content: UpdateUserMessageContent::String("Updated message content".to_string()),
    });

    let updated_message = client
        .messages()
        .update(&agent_id, &user_message_id, update_request)
        .await?;

    // Verify the update
    match updated_message {
        letta_rs::types::LettaMessageUnion::UserMessage(user_msg) => {
            assert_eq!(user_msg.content, "Updated message content");
        }
        _ => panic!("Expected user message in response"),
    }

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_list_messages() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Create some messages
    for i in 0..3 {
        let create_request = CreateMessagesRequest {
            messages: vec![MessageCreate {
                role: MessageRole::User,
                content: format!("Test message {}", i).into(),
                ..Default::default()
            }],
            ..Default::default()
        };

        client.messages().create(&agent_id, create_request).await?;
    }

    // List messages
    let messages = client.messages().list(&agent_id, None).await?;

    // Should have at least the messages we created plus agent responses
    assert!(messages.len() >= 3);

    // Check that we can find our user messages
    let user_messages: Vec<_> = messages
        .iter()
        .filter_map(|msg| match msg {
            letta_rs::types::LettaMessageUnion::UserMessage(user_msg) => Some(user_msg),
            _ => None,
        })
        .collect();

    assert!(user_messages.len() >= 3);

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_create_async_message() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Create a message asynchronously
    let create_request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: "Tell me a joke".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let run = client
        .messages()
        .create_async(&agent_id, create_request)
        .await?;

    // Verify we got a run object
    assert_eq!(run.id.prefix(), Some("run")); // Runs have 'run' prefix

    // Check initial status
    if let Some(status) = run.status {
        match status {
            letta_rs::types::JobStatus::Created
            | letta_rs::types::JobStatus::Running
            | letta_rs::types::JobStatus::Pending => {
                println!("Run created with status: {:?}", status);
            }
            _ => panic!("Unexpected initial status: {:?}", status),
        }
    }

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}
