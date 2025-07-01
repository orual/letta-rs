//! Command-line interface for the Letta client.

use clap::Parser;
use letta::types::agent::{AgentType, CreateAgentRequest, ListAgentsParams};
use letta::types::common::LettaId;
use letta::types::memory::{
    ArchivalMemoryQueryParams, Block, CreateArchivalMemoryRequest, UpdateMemoryBlockRequest,
};
use letta::types::message::{CreateMessagesRequest, MessageCreate, MessageRole};
use letta::types::tool::{CreateToolRequest, ListToolsParams, SourceType};
use letta::{auth::AuthConfig, ClientConfig, LettaClient};
use miette::{miette, Context, IntoDiagnostic};
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
    /// Memory operations
    #[command(subcommand)]
    Memory(MemoryCommand),
    /// Tool operations
    #[command(subcommand)]
    Tools(ToolsCommand),
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

#[derive(Parser, Debug)]
enum MemoryCommand {
    /// View agent's core memory
    View {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Edit a core memory block
    Edit {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Block label (human, persona, etc.)
        #[arg(short = 'b', long)]
        block: String,
        /// New value for the block
        value: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// List archival memory
    Archival {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Search query
        #[arg(short = 'q', long)]
        query: Option<String>,
        /// Maximum number of results
        #[arg(short = 'l', long, default_value = "20")]
        limit: u32,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Add to archival memory
    Add {
        /// Agent ID
        #[arg(short = 'a', long)]
        agent_id: String,
        /// Text to add to archival memory
        text: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
}

#[derive(Parser, Debug)]
enum ToolsCommand {
    /// List all tools
    List {
        /// Maximum number of tools to return
        #[arg(short = 'l', long, default_value = "20")]
        limit: u32,
        /// Filter by tags
        #[arg(short = 't', long)]
        tags: Vec<String>,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Create a new tool from Python file
    Create {
        /// Path to Python file containing the tool function
        #[arg(short = 'p', long)]
        python: String,
        /// Path to JSON file containing the function schema
        #[arg(short = 's', long)]
        schema: String,
        /// Path to JSON file containing args schema (optional, will extract from schema if not provided)
        #[arg(short = 'a', long)]
        args_schema: Option<String>,
        /// Tool description (optional, will use from schema if not provided)
        #[arg(short = 'd', long)]
        description: Option<String>,
        /// Tags for the tool
        #[arg(short = 't', long)]
        tags: Vec<String>,
        /// Return character limit
        #[arg(short = 'r', long)]
        return_limit: Option<u32>,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "summary")]
        output: String,
    },
    /// Get tool details
    Get {
        /// Tool ID
        id: String,
        /// Output format (json, pretty, summary)
        #[arg(short = 'o', long, default_value = "pretty")]
        output: String,
    },
    /// Delete a tool
    Delete {
        /// Tool ID
        id: String,
        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    // Install miette's fancy error handler for better diagnostics
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(3)
                .tab_width(4)
                .build(),
        )
    }))?;

    // Install panic hook to get nice error reports on panics
    miette::set_panic_hook();

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
        Command::Memory(mem_cmd) => match mem_cmd {
            MemoryCommand::View { agent_id, output } => {
                view_memory(&client, &agent_id, &output).await?;
            }
            MemoryCommand::Edit {
                agent_id,
                block,
                value,
                output,
            } => {
                edit_memory_block(&client, &agent_id, &block, &value, &output).await?;
            }
            MemoryCommand::Archival {
                agent_id,
                query,
                limit,
                output,
            } => {
                list_archival_memory(&client, &agent_id, query, limit, &output).await?;
            }
            MemoryCommand::Add {
                agent_id,
                text,
                output,
            } => {
                add_archival_memory(&client, &agent_id, &text, &output).await?;
            }
        },
        Command::Tools(tools_cmd) => match tools_cmd {
            ToolsCommand::List {
                limit,
                tags,
                output,
            } => {
                list_tools(&client, limit, tags, &output).await?;
            }
            ToolsCommand::Create {
                python,
                schema,
                args_schema,
                description,
                tags,
                return_limit,
                output,
            } => {
                create_tool(
                    &client,
                    &python,
                    &schema,
                    args_schema.as_deref(),
                    description,
                    tags,
                    return_limit,
                    &output,
                )
                .await?;
            }
            ToolsCommand::Get { id, output } => {
                get_tool(&client, &id, &output).await?;
            }
            ToolsCommand::Delete { id, yes } => {
                delete_tool(&client, &id, yes).await?;
            }
        },
        Command::Health => {
            println!("Checking health...");
            check_health(&client).await?;
        }
    }

    Ok(())
}

async fn list_agents(client: &LettaClient, limit: u32, tags: Vec<String>) -> miette::Result<()> {
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
) -> miette::Result<()> {
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
                let json = serde_json::to_string(&agent).into_diagnostic()?;
                println!("{}", json);
            }
            "pretty" => {
                let json = serde_json::to_string_pretty(&agent).into_diagnostic()?;
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

async fn get_agent(client: &LettaClient, id: &str, output: &str) -> miette::Result<()> {
    let agent_id = LettaId::from_str(id).into_diagnostic()?;
    match client.agents().get(&agent_id).await {
        Ok(agent) => match output {
            "json" => {
                let json = serde_json::to_string(&agent).into_diagnostic()?;
                println!("{}", json);
            }
            "pretty" => {
                let json = serde_json::to_string_pretty(&agent).into_diagnostic()?;
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

async fn check_health(client: &LettaClient) -> miette::Result<()> {
    let health = client.health().check().await?;

    println!("Server is healthy!");
    println!("\nServer Details:");
    println!("  Status: {}", health.status);
    println!("  Version: {}", health.version);

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
) -> miette::Result<()> {
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

    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

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
                            println!("{}", serde_json::to_string(&msg).into_diagnostic()?);
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
                                    println!(
                                        "{}",
                                        serde_json::to_string_pretty(&msg).into_diagnostic()?
                                    );
                                }
                            }
                        }
                    }
                }
                Ok(letta::StreamingEvent::StopReason(reason)) => {
                    if output == "json" {
                        println!("{}", serde_json::to_string(&reason).into_diagnostic()?);
                    }
                }
                Ok(letta::StreamingEvent::Usage(usage)) => {
                    if output == "json" {
                        println!("{}", serde_json::to_string(&usage).into_diagnostic()?);
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
                    println!("{}", serde_json::to_string(&response).into_diagnostic()?);
                }
                "pretty" => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&response).into_diagnostic()?
                    );
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
) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    let params = letta::types::message::ListMessagesRequest {
        limit: Some(limit),
        ..Default::default()
    };

    match client.messages().list(&agent_id, Some(params)).await {
        Ok(messages) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&messages).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&messages).into_diagnostic()?
                );
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

async fn view_memory(client: &LettaClient, agent_id: &str, output: &str) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    match client.memory().get_core_memory(&agent_id).await {
        Ok(memory) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&memory).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&memory).into_diagnostic()?
                );
            }
            _ => {
                println!("Core Memory for Agent:\n");
                for block in &memory.blocks {
                    println!("Block: {} ({})", block.label, block.limit.unwrap_or(0));
                    println!("{}", "-".repeat(50));
                    println!("{}\n", block.value);
                }
            }
        },
        Err(e) => {
            eprintln!("Error retrieving memory: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn edit_memory_block(
    client: &LettaClient,
    agent_id: &str,
    block_label: &str,
    new_value: &str,
    output: &str,
) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    let request = UpdateMemoryBlockRequest {
        value: Some(new_value.to_string()),
        ..Default::default()
    };

    match client
        .memory()
        .update_core_memory_block(&agent_id, block_label, request)
        .await
    {
        Ok(block) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&block).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&block).into_diagnostic()?
                );
            }
            _ => {
                println!("Memory block updated successfully!");
                println!("\nUpdated Block:");
                println!("  Label: {}", block.label);
                println!("  Value: {}", block.value);
                if let Some(limit) = block.limit {
                    println!("  Limit: {} characters", limit);
                }
            }
        },
        Err(e) => {
            eprintln!("Error updating memory block: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn list_archival_memory(
    client: &LettaClient,
    agent_id: &str,
    query: Option<String>,
    limit: u32,
    output: &str,
) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    let params = ArchivalMemoryQueryParams {
        search: query,
        limit: Some(limit),
        ..Default::default()
    };

    match client
        .memory()
        .list_archival_memory(&agent_id, Some(params))
        .await
    {
        Ok(passages) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&passages).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&passages).into_diagnostic()?
                );
            }
            _ => {
                if passages.is_empty() {
                    println!("No archival memory found.");
                } else {
                    println!("Found {} archival memory passages:\n", passages.len());
                    for (i, passage) in passages.iter().enumerate() {
                        println!(
                            "{}. [{}]",
                            i + 1,
                            passage
                                .created_at
                                .as_ref()
                                .map(|t| t.to_string())
                                .unwrap_or_else(|| "unknown".to_string())
                        );
                        println!("   {}", passage.text);
                        if let Some(embed_vec) = &passage.embedding {
                            println!("   Embedding: {} dimensions", embed_vec.len());
                        }
                        println!();
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error listing archival memory: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn add_archival_memory(
    client: &LettaClient,
    agent_id: &str,
    text: &str,
    output: &str,
) -> miette::Result<()> {
    let agent_id = LettaId::from_str(agent_id).into_diagnostic()?;

    let request = CreateArchivalMemoryRequest {
        text: text.to_string(),
    };

    match client
        .memory()
        .create_archival_memory(&agent_id, request)
        .await
    {
        Ok(passages) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&passages).into_diagnostic()?);
            }
            "pretty" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&passages).into_diagnostic()?
                );
            }
            _ => {
                println!("Archival memory added successfully!");
                if let Some(passage) = passages.first() {
                    println!("\nAdded Passage:");
                    println!("  ID: {}", passage.id);
                    println!("  Text: {}", passage.text);
                    println!(
                        "  Created: {}",
                        passage
                            .created_at
                            .as_ref()
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| "unknown".to_string())
                    );
                }
            }
        },
        Err(e) => {
            eprintln!("Error adding archival memory: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn list_tools(
    client: &LettaClient,
    limit: u32,
    _tags: Vec<String>,
    output: &str,
) -> miette::Result<()> {
    println!("Listing tools...");

    let mut params = ListToolsParams::default();
    params.limit = Some(limit);
    // Note: tags filtering would need to be done client-side as API doesn't support it

    let tools = client.tools().list(Some(params)).await?;

    match output {
        "json" => {
            println!("{}", serde_json::to_string(&tools).into_diagnostic()?);
        }
        "pretty" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&tools).into_diagnostic()?
            );
        }
        _ => {
            if tools.is_empty() {
                println!("No tools found.");
            } else {
                println!("Found {} tools:\n", tools.len());
                for tool in tools {
                    println!(
                        "ID: {}",
                        tool.id
                            .as_ref()
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "N/A".to_string())
                    );
                    println!("Name: {}", tool.name);
                    if let Some(desc) = &tool.description {
                        println!("Description: {}", desc);
                    }
                    if let Some(tool_type) = &tool.tool_type {
                        println!("Type: {:?}", tool_type);
                    }
                    if let Some(tags) = &tool.tags {
                        if !tags.is_empty() {
                            println!("Tags: {:?}", tags);
                        }
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}

/// Validate that a JSON schema matches Letta's expected tool schema format
fn validate_tool_schema(schema: &serde_json::Value) -> miette::Result<()> {
    // Check required top-level fields
    if !schema.is_object() {
        return Err(miette!("Schema must be a JSON object"));
    }

    let obj = schema.as_object().unwrap();

    // Required field: name
    if !obj.contains_key("name") {
        return Err(miette!(
            "Schema missing required field 'name'. Example:\n{{
  \"name\": \"my_tool\",
  \"description\": \"Tool description\",
  \"parameters\": {{ ... }}
}}"
        ));
    }

    if !obj["name"].is_string() {
        return Err(miette!("Field 'name' must be a string"));
    }

    // Required field: parameters
    if !obj.contains_key("parameters") {
        return Err(miette!(
            "Schema missing required field 'parameters'. Example:\n{{
  \"name\": \"my_tool\",
  \"description\": \"...\",
  \"parameters\": {{
    \"type\": \"object\",
    \"properties\": {{ ... }},
    \"required\": [ ... ]
  }}
}}"
        ));
    }

    let params = &obj["parameters"];
    if !params.is_object() {
        return Err(miette!("Field 'parameters' must be a JSON object"));
    }

    // Validate parameters structure
    let params_obj = params.as_object().unwrap();

    // Check for 'type' field in parameters
    if !params_obj.contains_key("type") {
        return Err(miette!(
            "Parameters missing 'type' field. Should be:\n\"parameters\": {{
  \"type\": \"object\",
  \"properties\": {{ ... }}
}}"
        ));
    }

    if params_obj["type"] != "object" {
        return Err(miette!(
            "Parameters 'type' should be \"object\", got: {}",
            params_obj["type"]
        ));
    }

    // Check for 'properties' field
    if !params_obj.contains_key("properties") {
        return Err(miette!(
            "Parameters missing 'properties' field. Example:\n\"parameters\": {{
  \"type\": \"object\",
  \"properties\": {{
    \"arg1\": {{ \"type\": \"string\", \"description\": \"...\" }},
    \"arg2\": {{ \"type\": \"integer\", \"description\": \"...\" }}
  }}
}}"
        ));
    }

    let properties = &params_obj["properties"];
    if !properties.is_object() {
        return Err(miette!("Parameters 'properties' must be a JSON object"));
    }

    // Validate each property
    let props_obj = properties.as_object().unwrap();
    for (prop_name, prop_value) in props_obj {
        if !prop_value.is_object() {
            return Err(miette!(
                "Property '{}' must be an object with 'type' and 'description' fields",
                prop_name
            ));
        }

        let prop_obj = prop_value.as_object().unwrap();
        if !prop_obj.contains_key("type") {
            return Err(miette!(
                "Property '{}' missing 'type' field. Example:\n\"{}\" : {{
  \"type\": \"string\",
  \"description\": \"Description of {}\"
}}",
                prop_name,
                prop_name,
                prop_name
            ));
        }

        if !prop_obj.contains_key("description") {
            return Err(miette!(
                "Property '{}' missing 'description' field. Properties should have:\n{{
  \"type\": \"...\",
  \"description\": \"What this parameter does\"
}}",
                prop_name
            ));
        }
    }

    // Warn about optional but recommended fields
    if !obj.contains_key("description") {
        eprintln!("Warning: Schema missing 'description' field. It's recommended to include a tool description.");
    }

    Ok(())
}

/// Validate that Python source code has proper docstring formatting for Letta
fn validate_python_docstring(source_code: &str, function_name: &str) -> miette::Result<()> {
    // Quick and dirty docstring validation

    // Check if there's a docstring at all (look for triple quotes after function def)
    let has_docstring = source_code.contains("\"\"\"") || source_code.contains("'''");
    if !has_docstring {
        return Err(miette!(
            "Function '{}' missing docstring. Letta requires a docstring with Args and Returns sections:\n\ndef {}(...):\n    \"\"\"\n    Brief description.\n    \n    Args:\n        param1: Description\n        param2: Description\n    \n    Returns:\n        Description of return value\n    \"\"\"\n    ...",
            function_name, function_name
        ));
    }

    // Extract docstring content (quick and dirty - find content between first set of triple quotes)
    let docstring_content = if let Some(start) = source_code.find("\"\"\"") {
        if let Some(end) = source_code[start + 3..].find("\"\"\"") {
            &source_code[start + 3..start + 3 + end]
        } else {
            return Err(miette!(
                "Malformed docstring - missing closing triple quotes"
            ));
        }
    } else if let Some(start) = source_code.find("'''") {
        if let Some(end) = source_code[start + 3..].find("'''") {
            &source_code[start + 3..start + 3 + end]
        } else {
            return Err(miette!(
                "Malformed docstring - missing closing triple quotes"
            ));
        }
    } else {
        return Err(miette!("Could not find docstring"));
    };

    // Check for Args section
    let has_args_section = docstring_content.contains("Args:")
        || docstring_content.contains("Arguments:")
        || docstring_content.contains("Parameters:");

    if !has_args_section {
        return Err(miette!(
            "Docstring missing 'Args:' section. Letta requires parameter documentation:\n\n    Args:\n        param_name: Description of parameter\n        another_param: Description\n\nWithout this, you'll get: \"Parameter 'X' in function '{}' lacks a description in the docstring\"",
            function_name
        ));
    }

    // Check for Returns section (warning only)
    let has_returns_section =
        docstring_content.contains("Returns:") || docstring_content.contains("Return:");

    if !has_returns_section {
        eprintln!("Warning: Docstring missing 'Returns:' section. It's recommended to document return values.");
    }

    Ok(())
}

async fn create_tool(
    client: &LettaClient,
    python_path: &str,
    schema_path: &str,
    args_schema_path: Option<&str>,
    description: Option<String>,
    tags: Vec<String>,
    return_limit: Option<u32>,
    output: &str,
) -> miette::Result<()> {
    // Read Python source code
    let source_code = std::fs::read_to_string(python_path)
        .into_diagnostic()
        .wrap_err(format!("Failed to read Python file: {}", python_path))?;

    // Read JSON schema
    let schema_content = std::fs::read_to_string(schema_path)
        .into_diagnostic()
        .wrap_err(format!("Failed to read schema file: {}", schema_path))?;

    let json_schema: serde_json::Value = serde_json::from_str(&schema_content)
        .into_diagnostic()
        .wrap_err("Failed to parse JSON schema")?;

    // Validate the schema format
    validate_tool_schema(&json_schema)?;

    // Extract function name for validation
    let function_name = json_schema
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("unknown");

    // Validate Python docstring
    validate_python_docstring(&source_code, function_name)?;

    // Extract name from schema if present
    let tool_name = json_schema
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or_else(|| miette!("Schema must contain a 'name' field"))?
        .to_string();

    // Extract description from schema if not provided
    let tool_description = description.or_else(|| {
        json_schema
            .get("description")
            .and_then(|d| d.as_str())
            .map(String::from)
    });

    // Read args schema if provided, otherwise try to extract from main schema
    let args_json_schema = if let Some(args_path) = args_schema_path {
        let args_content = std::fs::read_to_string(args_path)
            .into_diagnostic()
            .wrap_err(format!("Failed to read args schema file: {}", args_path))?;

        Some(
            serde_json::from_str(&args_content)
                .into_diagnostic()
                .wrap_err("Failed to parse args JSON schema")?,
        )
    } else {
        // Try to extract from main schema
        json_schema.get("parameters").cloned()
    };

    // Build the request
    let request = CreateToolRequest {
        source_code,
        source_type: Some(SourceType::Python),
        json_schema: Some(json_schema),
        args_json_schema,
        description: tool_description,
        tags: if tags.is_empty() { None } else { Some(tags) },
        return_char_limit: return_limit,
        ..Default::default()
    };

    if output != "json" {
        println!("Creating tool '{}'...", tool_name);
    }

    match client.tools().create(request).await {
        Ok(tool) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&tool).into_diagnostic()?);
            }
            "pretty" => {
                println!("{}", serde_json::to_string_pretty(&tool).into_diagnostic()?);
            }
            _ => {
                println!("Tool created successfully!");
                println!("\nTool Details:");
                println!(
                    "  ID: {}",
                    tool.id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
                println!("  Name: {}", tool.name);
                if let Some(desc) = &tool.description {
                    println!("  Description: {}", desc);
                }
                if let Some(tool_type) = &tool.tool_type {
                    println!("  Type: {:?}", tool_type);
                }
                println!(
                    "\nUse 'letta tools get {}' to see full details.",
                    tool.id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| tool.name.clone())
                );
            }
        },
        Err(e) => {
            return Err(e).wrap_err("Failed to create tool");
        }
    }

    Ok(())
}

async fn get_tool(client: &LettaClient, id: &str, output: &str) -> miette::Result<()> {
    let tool_id = LettaId::from_str(id).into_diagnostic()?;

    match client.tools().get(&tool_id).await {
        Ok(tool) => match output {
            "json" => {
                println!("{}", serde_json::to_string(&tool).into_diagnostic()?);
            }
            "pretty" => {
                println!("{}", serde_json::to_string_pretty(&tool).into_diagnostic()?);
            }
            _ => {
                println!("Tool Details:");
                println!(
                    "  ID: {}",
                    tool.id
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
                println!("  Name: {}", tool.name);
                if let Some(desc) = &tool.description {
                    println!("  Description: {}", desc);
                }
                if let Some(tool_type) = &tool.tool_type {
                    println!("  Type: {:?}", tool_type);
                }
                if let Some(tags) = &tool.tags {
                    if !tags.is_empty() {
                        println!("  Tags: {:?}", tags);
                    }
                }
                if let Some(return_limit) = tool.return_char_limit {
                    println!("  Return Limit: {} characters", return_limit);
                }

                // Show source code
                if let Some(source) = &tool.source_code {
                    println!("\nSource Code:");
                    println!("{}", "-".repeat(60));
                    println!("{}", source);
                    println!("{}", "-".repeat(60));
                }

                // Show schemas
                if let Some(schema) = &tool.json_schema {
                    println!("\nJSON Schema:");
                    println!(
                        "{}",
                        serde_json::to_string_pretty(schema).into_diagnostic()?
                    );
                }

                if let Some(args_schema) = &tool.args_json_schema {
                    println!("\nArgs Schema:");
                    println!(
                        "{}",
                        serde_json::to_string_pretty(args_schema).into_diagnostic()?
                    );
                }

                println!(
                    "\nCreated: {}",
                    tool.created_at
                        .as_ref()
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                );
                println!(
                    "Updated: {}",
                    tool.updated_at
                        .as_ref()
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                );
            }
        },
        Err(e) => {
            return Err(e).wrap_err("Failed to get tool");
        }
    }

    Ok(())
}

async fn delete_tool(client: &LettaClient, id: &str, yes: bool) -> miette::Result<()> {
    if !yes {
        print!("Are you sure you want to delete tool {}? (y/N) ", id);
        std::io::stdout().flush().into_diagnostic()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).into_diagnostic()?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("Deleting tool {}...", id);
    let tool_id = LettaId::from_str(id).into_diagnostic()?;

    client.tools().delete(&tool_id).await?;
    println!("Tool deleted successfully.");

    Ok(())
}
