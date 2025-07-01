//! Agent command implementations.

use crate::types::agent::{AgentType, CreateAgentRequest, EmbeddingConfig, ListAgentsParams};
use crate::types::common::LettaId;
use crate::types::models::LlmConfig;
use crate::LettaClient;
use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use std::io::Write;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub enum AgentCommand {
    /// List all agents
    List {
        /// Maximum number of agents to return
        #[arg(short = 'l', long, default_value = "10")]
        limit: u32,
        /// Filter by tags
        #[arg(short = 't', long)]
        tags: Vec<String>,
    },
    /// Create a new agent
    Create {
        /// Agent name
        #[arg(short = 'n', long)]
        name: String,
        /// System prompt
        #[arg(short = 's', long)]
        system: Option<String>,
        /// Agent type (memgpt, memgpt_v2, react, workflow, etc.)
        #[arg(short = 'a', long, default_value = "memgpt")]
        agent_type: String,
        /// Model to use (shorthand for llm_config)
        #[arg(short = 'm', long, default_value = "letta/letta-free")]
        model: String,
        /// Embedding model to use (shorthand for embedding_config)
        #[arg(short = 'e', long, default_value = "letta/letta-free")]
        embedding: String,
        /// Tags for the agent
        #[arg(short = 't', long)]
        tags: Vec<String>,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Get agent details
    Get {
        /// Agent ID
        id: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "pretty")]
        output: String,
    },
    /// Delete an agent
    Delete {
        /// Agent ID
        id: String,
        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

pub async fn handle(cmd: AgentCommand, client: &LettaClient) -> miette::Result<()> {
    match cmd {
        AgentCommand::List { limit, tags } => list_agents(client, limit, tags).await,
        AgentCommand::Create {
            name,
            system,
            agent_type,
            model,
            embedding,
            tags,
            output,
        } => {
            create_agent(
                client, name, system, agent_type, model, embedding, tags, &output,
            )
            .await
        }
        AgentCommand::Get { id, output } => get_agent(client, &id, &output).await,
        AgentCommand::Delete { id, yes } => delete_agent(client, &id, yes).await,
    }
}

async fn list_agents(client: &LettaClient, limit: u32, tags: Vec<String>) -> miette::Result<()> {
    println!("Listing agents...");
    let params = ListAgentsParams {
        limit: Some(limit),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    };

    match client.agents().list(Some(params)).await {
        Ok(agents) => {
            println!("Found {} agents:\n", agents.len());
            for agent in agents {
                println!("ID: {}", agent.id);
                println!("Name: {}", agent.name);
                if let Some(desc) = &agent.description {
                    println!("Description: {}", desc);
                }
                if let Some(tags) = &agent.tags {
                    if !tags.is_empty() {
                        println!("Tags: {:?}", tags);
                    }
                }
                println!(
                    "Created: {}",
                    agent
                        .created_at
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                );
                println!();
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error listing agents: {}", e);
            std::process::exit(1);
        }
    }
}

async fn create_agent(
    client: &LettaClient,
    name: String,
    system: Option<String>,
    agent_type: String,
    model: String,
    embedding: String,
    tags: Vec<String>,
    output: &str,
) -> miette::Result<()> {
    if output != "json" {
        println!("Creating agent '{}'...", name);
    }

    // Parse agent type
    let agent_type = AgentType::from_str(&agent_type)
        .map_err(|_| miette!("Invalid agent type: {}", agent_type))?;

    // Build the request
    let request = CreateAgentRequest::builder()
        .name(name)
        .description(system.clone())
        .agent_type(agent_type)
        .llm_config(LlmConfig::default().model(model))
        .embedding_config(EmbeddingConfig::default().embedding_model(embedding))
        .tags(if tags.is_empty() { None } else { Some(tags) })
        .build();

    match client.agents().create(request).await {
        Ok(agent) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&agent).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&agent).into_diagnostic()?
                );
            }
            _ => {
                println!("Agent created successfully!");
                println!("\nAgent Details:");
                println!("  ID: {}", agent.id);
                println!("  Name: {}", agent.name);
                if let Some(desc) = &agent.description {
                    println!("  Description: {}", desc);
                }
                println!("  Type: {:?}", agent.agent_type);
                println!("\nUse 'letta agent get {}' to see full details.", agent.id);
            }
        },
        Err(e) => Err(e).wrap_err("Failed to create agent"),
    }
}

async fn get_agent(client: &LettaClient, id: &str, output: &str) -> miette::Result<()> {
    let agent_id = LettaId::from_str(id).into_diagnostic()?;

    match client.agents().get(&agent_id).await {
        Ok(agent) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&agent).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&agent).into_diagnostic()?
                );
            }
            _ => {
                println!("Agent Details:");
                println!("  ID: {}", agent.id);
                println!("  Name: {}", agent.name);
                if let Some(desc) = &agent.description {
                    println!("  Description: {}", desc);
                }
                println!("  Type: {:?}", agent.agent_type);
                if let Some(tags) = &agent.tags {
                    if !tags.is_empty() {
                        println!("  Tags: {:?}", tags);
                    }
                }
                println!("  Model: {}", agent.llm_config.model);
                println!(
                    "  Embedding Model: {}",
                    agent.embedding_config.embedding_model
                );
                println!("  Messages: {}", agent.message_ids.len());
                println!(
                    "  Created: {}",
                    agent
                        .created_at
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                );
                println!(
                    "  Last Run: {}",
                    agent
                        .last_run_at
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "never".to_string())
                );
            }
        },
        Err(e) => Err(e).wrap_err("Failed to get agent"),
    }
}

async fn delete_agent(client: &LettaClient, id: &str, yes: bool) -> miette::Result<()> {
    if !yes {
        print!("Are you sure you want to delete agent {}? (y/N) ", id);
        std::io::stdout().flush().into_diagnostic()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).into_diagnostic()?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("Deleting agent {}...", id);
    let agent_id = LettaId::from_str(id).into_diagnostic()?;

    client.agents().delete(&agent_id).await?;
    println!("Agent deleted successfully.");

    Ok(())
}
