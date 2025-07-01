//! Command-line interface for the Letta client.

use clap::Parser;
use letta::types::agent::{AgentType, CreateAgentRequest};
use letta::types::memory::Block;
use letta::{auth::AuthConfig, ClientConfig, LettaClient};

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
        #[arg(short = 'm', long)]
        model: Option<String>,
        /// Embedding model to use (shorthand for embedding_config)
        #[arg(short = 'e', long)]
        embedding: Option<String>,
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
        Command::Health => {
            println!("Checking health...");
            // TODO: Implement actual health check
            println!("Health check not yet implemented");
        }
    }

    Ok(())
}

async fn list_agents(
    _client: &LettaClient,
    limit: u32,
    tags: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Listing agents (limit: {}, tags: {:?})...", limit, tags);
    // TODO: Implement when agent API is ready
    println!("Agent listing not yet implemented");
    Ok(())
}

async fn create_agent(
    _client: &LettaClient,
    name: String,
    system: Option<String>,
    agent_type: String,
    model: Option<String>,
    embedding: Option<String>,
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

    // Add default memory blocks
    request = request
        .memory_block(Block {
            id: None,
            label: "human".to_string(),
            value: "The human's name is not yet known.".to_string(),
            limit: Some(2000),
            is_template: false,
            preserve_on_migration: false,
            read_only: false,
            description: None,
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
            value: format!("I am {}, a helpful AI assistant.", name),
            limit: Some(2000),
            is_template: false,
            preserve_on_migration: false,
            read_only: false,
            description: None,
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        });

    let mut request = request.build();

    // Add model if specified (shorthand field)
    if let Some(model) = model {
        request.model = Some(model);
    }

    // Add embedding if specified (shorthand field)
    if let Some(embedding) = embedding {
        request.embedding = Some(embedding);
    }

    match output {
        "json" => {
            let json = serde_json::to_string(&request)?;
            println!("{}", json);
            // Don't print the note in JSON mode
            return Ok(());
        }
        "pretty" => {
            let json = serde_json::to_string_pretty(&request)?;
            println!("Request JSON:\n{}", json);
        }
        _ => {
            println!("Creating agent '{}'...", name);
            println!("  Type: {:?}", agent_type);
            if let Some(ref model) = request.model {
                println!("  Model: {}", model);
            }
            if let Some(ref embedding) = request.embedding {
                println!("  Embedding: {}", embedding);
            }
            if let Some(ref tags) = request.tags {
                if !tags.is_empty() {
                    println!("  Tags: {:?}", tags);
                }
            }
            println!("\nUse --output json to see the full request.");
        }
    }

    // TODO: Actually send the request when API implementation is ready
    println!("\nNote: API implementation not yet complete. Use the JSON output with curl for now.");

    Ok(())
}

async fn get_agent(
    _client: &LettaClient,
    id: &str,
    _output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting agent {}...", id);
    // TODO: Implement when agent API is ready
    println!("Agent get not yet implemented");
    Ok(())
}

async fn delete_agent(
    _client: &LettaClient,
    id: &str,
    yes: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !yes {
        println!("Are you sure you want to delete agent {}? (y/N)", id);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("Deleting agent {}...", id);
    // TODO: Implement when agent API is ready
    println!("Agent deletion not yet implemented");
    Ok(())
}
