# letta

A Rust client library for the [Letta](https://letta.com) REST API, providing idiomatic Rust bindings for building stateful AI agents with persistent memory and context.

Unlike the Letta-provided TypeScript and Python libraries, this was not generated from the OpenAPI spec, but implemented by hand (with substantial LLM assistance). As such it exposes things in slightly different, mildly opinionated ways, and includes a number of Rust-oriented affordances.

[![Crates.io](https://img.shields.io/crates/v/letta.svg)](https://crates.io/crates/letta)
[![Documentation](https://docs.rs/letta/badge.svg)](https://docs.rs/letta)
[![License](https://img.shields.io/crates/l/letta.svg)](LICENSE)

## Features

- **Pagination**: Automatic cursor-based pagination with `PaginatedStream`
- **Type Safety**: Comprehensive type definitions for all API requests/responses
- **Flexible Configuration**: Support for cloud and local deployments
- **Rich Error Handling**: Detailed error types
- **Well Tested**: Extensive test coverage with integration tests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
letta = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use letta::{ClientConfig, LettaClient};
use letta::types::{CreateAgentRequest, AgentType, ModelEndpointType, EmbeddingEndpointType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client for local Letta server
    let config = ClientConfig::new("http://localhost:8283")?;
    let client = LettaClient::new(config)?;

    // Create an agent
    let agent_request = CreateAgentRequest {
        name: "My Assistant".to_string(),
        agent_type: Some(AgentType::MemGPT),
        llm_config: Some(json!({
            "model_endpoint_type": ModelEndpointType::Openai,
            "model_endpoint": "https://api.openai.com/v1",
            "model": "gpt-4",
        })),
        embedding_config: Some(json!({
            "embedding_endpoint_type": EmbeddingEndpointType::Openai,
            "embedding_endpoint": "https://api.openai.com/v1",
            "embedding_model": "text-embedding-ada-002",
        })),
        ..Default::default()
    };

    let agent = client.agents().create(agent_request).await?;
    println!("Created agent: {}", agent.id);

    // Send a message to the agent
    let response = client
        .messages()
        .send(&agent.id, "Hello! How are you today?", None)
        .await?;

    // Stream responses
    let mut stream = response.into_stream();
    while let Some(chunk) = stream.next().await {
        match chunk? {
            MessageChunk::AssistantMessage(msg) => {
                print!("{}", msg.message);
            }
            MessageChunk::FunctionCall(call) => {
                println!("Function: {} with args: {}", call.name, call.arguments);
            }
            _ => {}
        }
    }

    Ok(())
}
```

## API Coverage

### Core APIs
- ✅ **Agents** - Create, update, delete, and manage AI agents
- ✅ **Messages** - Send messages and stream responses with SSE
- ✅ **Memory** - Manage core and archival memory with semantic search
- ✅ **Tools** - Register and manage agent tools (functions)
- ✅ **Sources** - Upload documents and manage knowledge sources
- ✅ **Blocks** - Manage memory blocks and persistent storage

### Advanced APIs
- ✅ **Groups** - Multi-agent conversations
- ✅ **Runs** - Execution tracking and debugging
- ✅ **Jobs** - Asynchronous job management
- ✅ **Batch** - Batch message processing
- ✅ **Templates** - Agent templates for quick deployment
- ✅ **Projects** - Project organization
- ✅ **Models** - LLM and embedding model configuration
- ✅ **Providers** - LLM provider management
- ✅ **Identities** - Identity and permissions management
- ✅ **Tags** - Tag-based organization
- ✅ **Telemetry** - Usage tracking and monitoring
- 🚧 **Voice** - Voice conversation support (beta)

## Examples

### Creating an Agent with Custom Memory

```rust
use letta::types::{CreateBlock, BlockType};

// Create custom memory blocks
let human_block = CreateBlock {
    block_type: BlockType::Human,
    value: "Name: Alice\nRole: Software Engineer".to_string(),
    label: Some("human".to_string()),
    ..Default::default()
};

let persona_block = CreateBlock {
    block_type: BlockType::Persona,
    value: "You are a helpful coding assistant.".to_string(),
    label: Some("persona".to_string()),
    ..Default::default()
};

// Create agent with custom memory
let agent_request = CreateAgentRequest {
    name: "Code Helper".to_string(),
    memory_blocks: Some(vec![human_block, persona_block]),
    ..Default::default()
};
```

### Working with Archival Memory

```rust
// Add to archival memory
client
    .memory()
    .insert_archival_memory(&agent.id, "Important fact: Rust is memory safe")
    .await?;

// Search archival memory
let memories = client
    .memory()
    .search_archival_memory(&agent.id, "Rust safety", Some(10))
    .await?;

for memory in memories {
    println!("Found: {}", memory.text);
}
```

### Streaming with Pagination

```rust
// Get paginated list of agents
let mut agent_stream = client
    .agents()
    .paginated()
    .limit(10)
    .build();

while let Some(agent) = agent_stream.next().await {
    let agent = agent?;
    println!("Agent: {} ({})", agent.name, agent.id);
}
```

### Managing Tools

```rust
use letta::types::{CreateToolRequest, Tool};

// Create a custom tool
let tool = CreateToolRequest {
    name: "get_weather".to_string(),
    description: Some("Get current weather for a location".to_string()),
    source_code: r#"
def get_weather(location: str) -> str:
    """Get weather for a location."""
    return f"The weather in {location} is sunny and 72°F"
"#.to_string(),
    source_type: Some("python".to_string()),
    ..Default::default()
};

let created_tool = client.tools().create(tool).await?;

// Add tool to agent
client
    .agents()
    .add_tool(&agent.id, &created_tool.id)
    .await?;
```

## Configuration

### Local Development Server

```rust
// No authentication required for local server
let config = ClientConfig::new("http://localhost:8283")?;
let client = LettaClient::new(config)?;
```

### Letta Cloud

```rust
// Use API key for cloud deployment
let config = ClientConfig::new("https://api.letta.com")?
    .with_api_key("your-api-key");
let client = LettaClient::new(config)?;
```

### Custom Headers

```rust
// Add custom headers like X-Project
let config = ClientConfig::new("http://localhost:8283")?
    .with_header("X-Project", "my-project")?;
```

## Error Handling

The library provides comprehensive error handling with detailed context:

```rust
use letta::error::LettaError;

match client.agents().get(&agent_id).await {
    Ok(agent) => println!("Found agent: {}", agent.name),
    Err(LettaError::Api { status, message, .. }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/letta
cd letta

# Build the library
cargo build

# Run tests
cargo test

# Build documentation
cargo doc --open
```

### Running the Local Test Server

```bash
# Start local Letta server for testing
cd local-server
docker compose up -d

# Run integration tests
cargo test --features integration
```

### CLI Tool

A CLI tool is included for testing and development:

```bash
# Install the CLI
cargo install --path . --features cli

# List agents
letta agent list

# Create an agent (generates JSON for curl)
letta agent create -n "Test Agent" -m "letta/letta-free" -o json
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [reqwest](https://github.com/seanmonstar/reqwest) for HTTP operations
- Uses [tokio](https://github.com/tokio-rs/tokio) for async runtime
- Streaming support via [eventsource-stream](https://github.com/jpopesculian/eventsource-stream)
- Error handling with [miette](https://github.com/zkat/miette)

## Related Projects

- [Letta](https://github.com/letta-ai/letta) - The official Letta server
- [letta-node](https://github.com/letta-ai/letta-node) - TypeScript/JavaScript SDK
- [letta-python](https://github.com/letta-ai/letta-python) - Python SDK
