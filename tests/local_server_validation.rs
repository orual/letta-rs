//! Integration tests for validating data structures against local Letta server.

use letta_rs::types::agent::{AgentType, CreateAgentRequest, ToolRule};
use letta_rs::types::memory::MemoryBlock;
use letta_rs::{ClientConfig, LettaClient};

#[tokio::test]
async fn test_create_agent_json_format() {
    // Create client for local server
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Build a request with all our updated types including advanced features
    let request = CreateAgentRequest {
        name: Some("Validation Test Agent".to_string()),
        system: Some("You are a test agent for validating Rust data structures".to_string()),
        agent_type: Some(AgentType::MemGPT),
        memory_blocks: Some(vec![
            MemoryBlock {
                id: None,
                label: "human".to_string(),
                value: "The human's name is unknown.".to_string(),
                limit: Some(2000),
                is_template: false,
                preserve_on_migration: false,
                read_only: false,
                description: Some("Information about the human".to_string()),
                metadata: None,
                name: None,
                organization_id: None,
                created_by_id: None,
                last_updated_by_id: None,
                created_at: None,
                updated_at: None,
            },
            MemoryBlock {
                id: None,
                label: "persona".to_string(),
                value: "I am a validation test agent.".to_string(),
                limit: Some(2000),
                is_template: false,
                preserve_on_migration: false,
                read_only: false,
                description: Some("Agent persona".to_string()),
                metadata: None,
                name: None,
                organization_id: None,
                created_by_id: None,
                last_updated_by_id: None,
                created_at: None,
                updated_at: None,
            },
        ]),
        tool_rules: Some(vec![
            ToolRule::ContinueLoop {
                tool_name: "core_memory_append".to_string(),
                prompt_template: Some("Continue after memory append".to_string()),
            },
            ToolRule::ExitLoop {
                tool_name: "send_message".to_string(),
                prompt_template: Some("Exit after sending message".to_string()),
            },
        ]),
        tags: Some(vec!["test".to_string(), "validation".to_string()]),
        model: Some("letta/letta-free".to_string()),
        embedding: Some("letta/letta-free".to_string()),
        ..Default::default()
    };

    // Show the JSON that will be sent
    let json = serde_json::to_string_pretty(&request).unwrap();
    println!("Request JSON:\n{}", json);

    // Create agent using our SDK
    let created_agent = client
        .agents()
        .create(request)
        .await
        .expect("Failed to create agent");

    println!(
        "✅ Successfully created agent with ID: {}",
        created_agent.id
    );
    println!("   Name: {}", created_agent.name);
    println!("   Type: {:?}", created_agent.agent_type);

    // Verify the agent has the expected properties
    assert_eq!(created_agent.name, "Validation Test Agent");
    assert_eq!(created_agent.agent_type, AgentType::MemGPT);

    // Clean up - delete the agent
    client
        .agents()
        .delete(&created_agent.id.to_string())
        .await
        .expect("Failed to delete agent");

    println!("✅ Cleanup: Agent deleted successfully");
}

#[tokio::test]
async fn test_tool_rule_serialization() {
    // Test that our tool rule enum serializes correctly
    let rules = vec![
        ToolRule::ContinueLoop {
            tool_name: "test_tool".to_string(),
            prompt_template: None,
        },
        ToolRule::MaxCountPerStep {
            tool_name: "limited_tool".to_string(),
            prompt_template: Some("Limited to {{max_count_limit}} calls".to_string()),
            max_count_limit: 3,
        },
        ToolRule::Conditional {
            tool_name: "conditional_tool".to_string(),
            prompt_template: None,
            default_child: Some("default_action".to_string()),
            child_output_mapping: std::collections::HashMap::from([
                ("success".to_string(), "success_handler".to_string()),
                ("error".to_string(), "error_handler".to_string()),
            ]),
            require_output_mapping: true,
        },
    ];

    for rule in rules {
        let json = serde_json::to_string_pretty(&rule).unwrap();
        println!("Tool rule JSON:\n{}\n", json);

        // Verify we can deserialize it back
        let _deserialized: ToolRule = serde_json::from_str(&json).unwrap();
    }
}

#[tokio::test]
async fn test_list_agents() {
    // Create client for local server
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // List agents using our SDK
    let agents = client
        .agents()
        .list(None)
        .await
        .expect("Failed to list agents");

    println!("✅ Found {} agents", agents.len());

    // Display agent details
    for (i, agent) in agents.iter().enumerate() {
        println!("\nAgent {}:", i + 1);
        println!("  ID: {}", agent.id);
        println!("  Name: {}", agent.name);
        println!("  Type: {:?}", agent.agent_type);

        // Check tool rules
        if let Some(rules) = &agent.tool_rules {
            println!("  Tool rules: {} rules", rules.len());
            for rule in rules.iter().take(2) {
                match rule {
                    ToolRule::ContinueLoop { tool_name, .. } => {
                        println!("    - {} (ContinueLoop)", tool_name);
                    }
                    ToolRule::ExitLoop { tool_name, .. } => {
                        println!("    - {} (ExitLoop)", tool_name);
                    }
                    ToolRule::MaxCountPerStep {
                        tool_name,
                        max_count_limit,
                        ..
                    } => {
                        println!("    - {} (MaxCountPerStep: {})", tool_name, max_count_limit);
                    }
                    ToolRule::Conditional { tool_name, .. } => {
                        println!("    - {} (Conditional)", tool_name);
                    }
                    ToolRule::Terminal { tool_name, .. } => {
                        println!("    - {} (Terminal)", tool_name);
                    }
                    ToolRule::Child { tool_name, .. } => {
                        println!("    - {} (Child)", tool_name);
                    }
                    ToolRule::Parent { tool_name, .. } => {
                        println!("    - {} (Parent)", tool_name);
                    }
                    ToolRule::RequiredBeforeExit { tool_name, .. } => {
                        println!("    - {} (RequiredBeforeExit)", tool_name);
                    }
                    ToolRule::Init { tool_name, .. } => {
                        println!("    - {} (Init)", tool_name);
                    }
                }
            }
        }

        // Check memory blocks
        if let Some(memory) = &agent.memory {
            println!("  Memory blocks: {} blocks", memory.blocks.len());
            for block in memory.blocks.iter().take(2) {
                println!(
                    "    - {}: {}",
                    block.label,
                    block.value.chars().take(50).collect::<String>()
                );
            }
        }
    }

    // Validate that we successfully deserialized proper Agent structs
    assert!(!agents.is_empty(), "Should have at least one agent");

    // Verify each agent has required fields
    for agent in &agents {
        assert!(!agent.id.is_empty(), "Agent ID should not be empty");
        assert!(!agent.name.is_empty(), "Agent name should not be empty");
        // agent_type should be a valid enum variant (already validated by serde)
    }
}
