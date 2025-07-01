//! Command-line interface for the Letta client.

use clap::Parser;
use letta::types::agent::{AgentType, CreateAgentRequest, ListAgentsParams};
use letta::types::common::LettaId;
use letta::types::memory::Block;
use letta::types::message::{CreateMessagesRequest, MessageCreate, MessageRole};
use letta::{auth::AuthConfig, ClientConfig, LettaClient};
use std::io::Write;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author = "Orual", version, about = "Letta REST API client")]
/// Letta command-line interface
struct Args {
    /// Base URL for the Letta API (defaults to http://localhost:8283)
    #[arg(
        short = 'u',
        long,
        env = "LETTA_BASE_URL",
        default_value = "http://localhost:8283"
    )]
    base_url: String,

    /// API key for authentication (optional, can also use LETTA_API_KEY env var)
    #[arg(short = 'k', long, env = "LETTA_API_KEY")]
    api_key: Option<String>,

    /// Enable verbose output
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Command to execute
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Agent operations
    #[command(subcommand)]
    Agent(AgentCommand),
    /// Message operations
    #[command(subcommand)]
    Message(MessageCommand),
    /// Health check
    Health,
}

#[derive(Parser, Debug)]
enum AgentCommand {
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

#[derive(Parser, Debug)]
enum MessageCommand {
    /// Send a message to an agent
    Send {
        /// Agent ID to send message to
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Message text to send
        message: String,
        /// Message role (user, system, assistant)
        #[arg(short = 'r', long, default_value = "user")]
        role: String,
        /// Maximum steps for processing
        #[arg(short = 's', long)]
        max_steps: Option<i32>,
        /// Disable streaming (by default, messages are streamed)
        #[arg(long)]
        no_stream: bool,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// List messages for an agent
    List {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Maximum number of messages to return
        #[arg(short = 'l', long, default_value = "20")]
        limit: i32,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Set up logging if verbose
    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    // Create client configuration
    let mut config = ClientConfig::new(&args.base_url)?;

    // Set up authentication if API key is provided
    if let Some(api_key) = args.api_key.or_else(|| std::env::var("LETTA_API_KEY").ok()) {
        config = config.auth(AuthConfig::bearer(api_key));
    }

    // Create client
    let client = LettaClient::new(config)?;

    // Execute command
    match args.command {
        Command::Agent(agent_cmd) => match agent_cmd {
            AgentCommand::List { limit, tags } => {
                list_agents(&client, limit, tags).await?;
            }
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
                    &client, name, system, agent_type, model, embedding, tags, &output,
                )
                .await?;
            }
            AgentCommand::Get { id, output } => {
                get_agent(&client, &id, &output).await?;
            }
            AgentCommand::Delete { id, yes } => {
                delete_agent(&client, &id, yes).await?;
            }
        },
        Command::Message(msg_cmd) => match msg_cmd {
            MessageCommand::Send {
                agent_id,
                message,
                role,
                max_steps,
                no_stream,
                output,
            } => {
                send_message(
                    &client, &agent_id, &message, &role, max_steps, !no_stream, &output,
                )
                .await?;
            }
            MessageCommand::List {
                agent_id,
                limit,
                output,
            } => {
                list_messages(&client, &agent_id, limit, &output).await?;
            }
        },
        Command::Health => {
            println!("Checking health...");
            check_health(&client).await?;
        }
    }

    Ok(())
}

async fn list_agents(
    client: &LettaClient,
    limit: u32,
    tags: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Listing agents...");

    let mut params = ListAgentsParams::default();
    params.limit = Some(limit);
    if !tags.is_empty() {
        params.tags = Some(tags);
    }

    let agents = client.agents().list(Some(params)).await?;

    if agents.is_empty() {
        println!("No agents found.");
    } else {
        println!("Found {} agents:\n", agents.len());
        for agent in agents {
            println!("ID: {}", agent.id);
            println!("Name: {}", agent.name);
            println!("Type: {:?}", agent.agent_type);
            if !agent.tags.is_empty() {
                println!("Tags: {:?}", agent.tags);
            }
            if let Some(desc) = &agent.description {
                println!("Description: {}", desc);
            }
            println!("Created: {}", agent.created_at);
            println!();
        }
    }

    Ok(())
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
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse agent type
    let agent_type = match agent_type.as_str() {
        "memgpt" => AgentType::MemGPT,
        "memgpt_v2" => AgentType::MemGPTv2,
        "react" => AgentType::React,
        "workflow" => AgentType::Workflow,
        "split_thread" => AgentType::SplitThread,
        "sleeptime" => AgentType::Sleeptime,
        "voice_convo" => AgentType::VoiceConvo,
        "voice_sleeptime" => AgentType::VoiceSleeptime,
        _ => {
            eprintln!("Unknown agent type: {}. Using default (memgpt)", agent_type);
            AgentType::MemGPT
        }
    };

    // Build the request
    let mut request = CreateAgentRequest::builder()
        .name(name.clone())
        .agent_type(agent_type)
        .tags(tags);

    if let Some(system) = system {
        request = request.system(system);
    }

    // Add default memory blocks using the new ergonomic constructors
    request = request
        .memory_block(Block::human("The human's name is not yet known."))
        .memory_block(Block::persona(format!(
            "I am {}, a helpful AI assistant.",
            name
        )));

    let mut request = request.build();

    // Add model (shorthand field)
    request.model = Some(model);

    // Add embedding (shorthand field)
    request.embedding = Some(embedding);

    // Show what we're doing if not in JSON mode
    if output != "json" {
        println!("Creating agent '{}'...", name);
    }

    // Send the actual request
    match client.agents().create(request).await {
        Ok(agent) => match output {
            "json" => {
                let json = serde_json::to_string(&agent)?;
                println!("{}", json);
            }
            "pretty" => {
                let json = serde_json::to_string_pretty(&agent)?;
                println!("{}", json);
            }
            _ => {
                println!("Agent created successfully!");
                println!("\nAgent Details:");
                println!("  ID: {}", agent.id);
                println!("  Name: {}", agent.name);
                println!("  Type: {:?}", agent.agent_type);
                if let Some(ref llm_config) = agent.llm_config {
                    println!("  Model: {}", llm_config.model);
                }
                if let Some(ref embedding_config) = agent.embedding_config {
                    if let Some(ref model) = embedding_config.embedding_model {
                        println!("  Embedding: {}", model);
                    }
                }
                if !agent.tags.is_empty() {
                    println!("  Tags: {:?}", agent.tags);
                }
                println!("\nUse 'letta agent get {}' to see full details.", agent.id);
            }
        },
        Err(e) => {
            eprintln!("Error creating agent: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn get_agent(
    client: &LettaClient,
    id: &str,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let agent_id = LettaId::from_str(id)?;
    match client.agents().get(&agent_id).await {
        Ok(agent) => match output {
            "json" => {
                let json = serde_json::to_string(&agent)?;
                println!("{}", json);
            }
            "pretty" => {
                let json = serde_json::to_string_pretty(&agent)?;
                println!("{}", json);
            }
            _ => {
                println!("Agent Details:");
                println!("  ID: {}", agent.id);
                println!("  Name: {}", agent.name);
                println!("  Type: {:?}", agent.agent_type);
                if let Some(ref system) = agent.system {
                    println!("\nSystem Prompt:");
                    println!("  {}", system);
                }
                if let Some(ref llm_config) = agent.llm_config {
                    println!("\nLLM Configuration:");
                    println!("  Model: {}", llm_config.model);
                    if let Some(context) = llm_config.context_window {
                        println!("  Context Window: {}", context);
                    }
                }
                if let Some(ref embedding_config) = agent.embedding_config {
                    println!("\nEmbedding Configuration:");
                    if let Some(ref model) = embedding_config.embedding_model {
                        println!("  Model: {}", model);
                    }
                    if let Some(dim) = embedding_config.embedding_dim {
                        println!("  Dimensions: {}", dim);
                    }
                }
                if !agent.tools.is_empty() {
                    println!("\nTools: {} attached", agent.tools.len());
                }
                if !agent.tags.is_empty() {
                    println!("\nTags: {:?}", agent.tags);
                }
                if let Some(ref desc) = agent.description {
                    println!("\nDescription: {}", desc);
                }
                println!("\nCreated: {}", agent.created_at);
                println!("Updated: {}", agent.updated_at);
            }
        },
        Err(e) => {
            eprintln!("Error getting agent: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn delete_agent(
    client: &LettaClient,
    id: &str,
    yes: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !yes {
        print!("Are you sure you want to delete agent {}? (y/N) ", id);
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("Deleting agent {}...", id);
    let agent_id = LettaId::from_str(id)?;
    match client.agents().delete(&agent_id).await {
        Ok(_) => {
            println!("Agent deleted successfully.");
        }
        Err(e) => {
            eprintln!("Error deleting agent: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn check_health(client: &LettaClient) -> Result<(), Box<dyn std::error::Error>> {
    match client.health().check().await {
        Ok(health) => {
            println!("Server is healthy!");
            println!("\nServer Details:");
            println!("  Status: {}", health.status);
            println!("  Version: {}", health.version);
        }
        Err(e) => {
            eprintln!("Error checking health: {}", e);
            eprintln!("\nThe server may be down or unreachable.");
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn send_message(
    client: &LettaClient,
    agent_id: &str,
    message: &str,
    role: &str,
    max_steps: Option<i32>,
    stream: bool,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse message role
    let role = match role.to_lowercase().as_str() {
        "user" => MessageRole::User,
        "system" => MessageRole::System,
        "assistant" => MessageRole::Assistant,
        _ => {
            eprintln!("Invalid role: {}. Using 'user'.", role);
            MessageRole::User
        }
    };

    // Create the message
    let message = MessageCreate {
        role,
        content: message.into(),
        ..Default::default()
    };

    // Create the request
    let mut request = CreateMessagesRequest {
        messages: vec![message],
        ..Default::default()
    };

    if let Some(steps) = max_steps {
        request.max_steps = Some(steps);
    }

    let agent_id = LettaId::from_str(agent_id)?;

    if stream {
        // Handle streaming response
        use futures::StreamExt;

        if output != "json" {
            println!("Streaming response from agent...\n");
        }

        // Use token streaming for better UX when not in JSON mode
        let stream_tokens = output != "json";
        let mut stream = client
            .messages()
            .create_stream(&agent_id, request, stream_tokens)
            .await?;

        while let Some(event) = stream.next().await {
            match event {
                Ok(letta::StreamingEvent::Message(msg)) => {
                    match output {
                        "json" => {
                            println!("{}", serde_json::to_string(&msg)?);
                        }
                        _ => {
                            // Pretty print the message based on type
                            use letta::types::message::LettaMessageUnion;
                            match msg {
                                LettaMessageUnion::UserMessage(m) => {
                                    println!("User: {}", m.content);
                                }
                                LettaMessageUnion::AssistantMessage(m) => {
                                    println!("Assistant: {}", m.content);
                                }
                                LettaMessageUnion::SystemMessage(m) => {
                                    println!("System: {}", m.content);
                                }
                                LettaMessageUnion::ToolCallMessage(m) => {
                                    println!(
                                        "Tool Call: {} - {}",
                                        m.tool_call.name, m.tool_call.arguments
                                    );
                                }
                                LettaMessageUnion::ToolReturnMessage(m) => {
                                    println!("Tool Result: {}", m.tool_return);
                                }
                                _ => {
                                    // For other message types, show the JSON
                                    println!("{}", serde_json::to_string_pretty(&msg)?);
                                }
                            }
                        }
                    }
                }
                Ok(letta::StreamingEvent::StopReason(reason)) => {
                    if output == "json" {
                        println!("{}", serde_json::to_string(&reason)?);
                    }
                }
                Ok(letta::StreamingEvent::Usage(usage)) => {
                    if output == "json" {
                        println!("{}", serde_json::to_string(&usage)?);
                    } else if output != "summary" {
                        println!("\nUsage Statistics:");
                        if let Some(steps) = usage.step_count {
                            println!("  Steps: {}", steps);
                        }
                        if let Some(tokens) = usage.total_tokens {
                            println!("  Total Tokens: {}", tokens);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Streaming error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        // Handle non-streaming response
        if output != "json" {
            println!("Sending message to agent...");
        }

        match client.messages().create(&agent_id, request).await {
            Ok(response) => match output {
                "json" => {
                    println!("{}", serde_json::to_string(&response)?);
                }
                "pretty" => {
                    println!("{}", serde_json::to_string_pretty(&response)?);
                }
                _ => {
                    println!("Message sent successfully!\n");

                    // Display the conversation
                    for msg in &response.messages {
                        use letta::types::message::LettaMessageUnion;
                        match msg {
                            LettaMessageUnion::UserMessage(m) => {
                                println!("👤 User: {}", m.content);
                            }
                            LettaMessageUnion::AssistantMessage(m) => {
                                println!("🤖 Assistant: {}", m.content);
                            }
                            LettaMessageUnion::SystemMessage(m) => {
                                println!("💻 System: {}", m.content);
                            }
                            LettaMessageUnion::ToolCallMessage(m) => {
                                println!(
                                    "🔧 Tool Call: {} - {}",
                                    m.tool_call.name, m.tool_call.arguments
                                );
                            }
                            LettaMessageUnion::ToolReturnMessage(m) => {
                                println!("📊 Tool Result: {}", m.tool_return);
                            }
                            _ => {}
                        }
                    }

                    // Show stop reason
                    println!("\nStop Reason: {:?}", response.stop_reason.stop_reason);

                    // Show usage if available
                    println!("\nUsage:");
                    if let Some(steps) = response.usage.step_count {
                        println!("  Steps: {}", steps);
                    }
                    if let Some(tokens) = response.usage.total_tokens {
                        println!("  Total Tokens: {}", tokens);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error sending message: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

async fn list_messages(
    client: &LettaClient,
    agent_id: &str,
    limit: i32,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let agent_id = LettaId::from_str(agent_id)?;

    let params = letta::types::message::ListMessagesRequest {
        limit: Some(limit),
        ..Default::default()
    };

    match client.messages().list(&agent_id, Some(params)).await {
        Ok(messages) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&messages)?);
            }
            "pretty" => {
                println!("{}", serde_json::to_string_pretty(&messages)?);
            }
            _ => {
                if messages.is_empty() {
                    println!("No messages found.");
                } else {
                    println!("Found {} messages:\n", messages.len());

                    for msg in messages {
                        use letta::types::message::LettaMessageUnion;
                        match msg {
                            LettaMessageUnion::UserMessage(m) => {
                                println!("User [{}]", m.date);
                                println!("   {}\n", m.content);
                            }
                            LettaMessageUnion::AssistantMessage(m) => {
                                println!("Assistant [{}]", m.date);
                                println!("   {}\n", m.content);
                            }
                            LettaMessageUnion::SystemMessage(m) => {
                                println!("System [{}]", m.date);
                                println!("   {}\n", m.content);
                            }
                            LettaMessageUnion::ToolCallMessage(m) => {
                                println!("Tool Call [{}]", m.date);
                                println!("   Tool: {}", m.tool_call.name);
                                println!("   Args: {}\n", m.tool_call.arguments);
                            }
                            LettaMessageUnion::ToolReturnMessage(m) => {
                                println!("Tool Result [{}]", m.date);
                                println!("   {}\n", m.tool_return);
                            }
                            LettaMessageUnion::ReasoningMessage(m) => {
                                println!("Reasoning [{}]", m.date);
                                println!("   {}\n", m.reasoning);
                            }
                            LettaMessageUnion::HiddenReasoningMessage(m) => {
                                println!("Hidden Reasoning [{}]", m.date);
                                println!("   State: {:?}\n", m.state);
                            }
                        }
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error listing messages: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
