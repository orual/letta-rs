# CLAUDE.md


This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This project is a Rust client library for the Letta REST API. Letta is a stateful AI agent platform that enables building agents with persistent memory and context across conversations. The library provides idiomatic Rust bindings for all Letta API endpoints, with a CLI tool included for testing and development purposes.

### Letta API Scope
The implementation includes all major API categories from the official specification:

**Completed APIs:**
- ✅ **Agents**: CRUD operations for AI agents with persistent state and messaging
- ✅ **Messages**: Real-time messaging with streaming support (SSE)
- ✅ **Memory**: Core and archival memory operations with pagination
- ✅ **Blocks**: Memory block operations for persistent storage
- ✅ **Sources**: Document upload and processing for agent knowledge
- ✅ **Tools**: Tool management and execution framework (including MCP and Composio)
- ✅ **Groups**: Multi-agent group conversations
- ✅ **Jobs**: Asynchronous job management with source_id filtering
- ✅ **Projects**: Project management
- ✅ **Templates**: Agent templates
- ✅ **Runs**: Execution runs and steps
- ✅ **Health**: Health checks
- ✅ **Models**: Model configuration and management (LLM and embedding models)
- ✅ **Tags**: Tag management system with pagination support
- ✅ **Providers**: LLM provider management with full CRUD operations
- ✅ **Authentication**: Bearer token-based API authentication
- ✅ **Misc**: Provider listing endpoint

**Remaining APIs to Implement:**
- ❌ **Identities**: Identity management for agents
- ❌ **Telemetry**: Usage tracking and analytics
- ❌ **Batches**: Batch processing operations
- ❌ **Voice**: Voice conversation support

## Development Commands

### Building and Running
```bash
# Build the library
cargo build
nix build

# Run tests
cargo test

# Build and install the CLI testing tool
cargo install --path . --features cli

# Development with auto-recompilation
just watch
bacon --job run
```

### CLI Testing Tool
The project includes a CLI tool for testing the library functionality:
```bash
# Run the CLI directly
cargo run --features cli -- <command>

# Create an agent (generates JSON, actual API calls not yet implemented)
cargo run --features cli -- agent create -n "Test Agent" -m "letta/letta-free" -e "letta/letta-free" -o json

# Use with curl for actual API testing
cargo run --features cli -- agent create -n "Test" -m "letta/letta-free" -o json | curl -X POST http://localhost:8283/v1/agents -H "Content-Type: application/json" -d @-
```

### Development Environment
```bash
# Enter Nix development shell
nix develop

# Run all pre-commit hooks (formatting, linting)
just pre-commit-all
pre-commit run --all-files

# Update Nix flake inputs
nix flake update
```

### Development Instructions
Run `cargo check` frequently when producing code. This will help you catch errors early.
NEVER use `unsafe{}`. If you feel you need to, stop, think about other ways, and ask the user for help if needed.
NEVER ignore a failing test or change a test to make your code pass unless you can clearly demonstrate to the user that the test itself is a problem and needs to be changed.

### Testing
```bash
cargo test
cargo test --doc
```

### Local Development Server
A local Letta server is provided for development and testing:
```bash
# Start the local server (from local-server/ directory)
cd local-server
docker compose up -d

# Stop the server
docker compose down

# View server logs
docker logs letta-letta-server-1
```

The local server runs on `http://localhost:8283` and requires no authentication.

## Architecture

### Project Structure
- `src/lib.rs` - Library entry point
- `src/client.rs` - Main Letta client implementation
- `src/types/` - Data models for Letta API
  - `agent.rs` - Agent-related types and enums
  - `memory.rs` - Memory block types
  - `message.rs` - Message types
  - `tool.rs` - Tool types
  - `source.rs` - Source types
  - `groups.rs` - Group conversation types
  - `project.rs` - Project management types
  - `template.rs` - Agent template types
  - `runs.rs` - Execution run types
  - `health.rs` - Health check types
  - `models.rs` - Model configuration types
  - `tags.rs` - Tag-related types
  - `provider.rs` - Provider configuration types
  - `common.rs` - Shared types and utilities
- `src/api/` - API endpoint implementations
  - `agents.rs` - Agent CRUD operations
  - `messages.rs` - Message handling with agents
  - `memory.rs` - Core and archival memory operations
  - `sources.rs` - Document and data source management
  - `tools.rs` - Tool management
  - `blocks.rs` - Memory block operations
  - `groups.rs` - Multi-agent group conversations
  - `jobs.rs` - Asynchronous job management
  - `projects.rs` - Project management
  - `templates.rs` - Agent templates
  - `runs.rs` - Execution runs
  - `health.rs` - Health checks
  - `models.rs` - Model configuration endpoints
  - `tags.rs` - Tag management
  - `providers.rs` - Provider management
  - `misc.rs` - Miscellaneous endpoints
- `src/cli.rs` - CLI testing tool (binary: `letta`, requires `cli` feature)
- `tests/` - Integration tests
- `nix/modules/` - Modular Nix configuration
  - `devshell.nix` - Development shell configuration
  - `rust.nix` - Rust build configuration via rust-flake/crane
  - `pre-commit.nix` - Code formatting and linting hooks
- `justfile` - Command shortcuts for common development tasks
- `local-server/` - Local Letta server for development testing
  - `compose.yml` - Docker Compose configuration
  - `server.env` - Server environment variables
- `letta-node/` - Node.js/TypeScript SDK reference (git submodule)
- `letta-python/` - Python SDK reference (git submodule)

### Build System
This project uses a dual build approach:
1. **Cargo** - Standard Rust package manager
2. **Nix + Crane** - Reproducible builds via rust-flake

The Nix configuration uses flake-parts for modularity and imports rust-flake for Rust-specific build logic.

### Key Dependencies
- `clap` (4.5+) with derive and env features for CLI argument parsing
- Development tools: just, bacon, nixd, cargo-doc-live

### Agent Type System

The crate provides comprehensive agent data models with full enum support:

- **AgentType**: MemGPT, MemGPTv2, React, Workflow, SplitThread, Sleeptime, VoiceConvo, VoiceSleeptime
- **ModelEndpointType**: Openai, Anthropic, Cohere, GoogleAi, Azure, Groq, Ollama, vLLM, Mistral, Together, etc.
- **EmbeddingEndpointType**: Openai, Azure, Cohere, HuggingFace, Ollama
- **CreateAgentRequest**: Includes all fields from API spec including tool_rules, initial_message_sequence, tool_exec_environment_variables, response_format, enable_reasoner, message_buffer_autoclear

### Error Handling Strategy

The crate uses a sophisticated error handling system matching the TypeScript/Python SDK patterns:

- **LettaError::Api**: Handles HTTP API errors with full response context
  - `status`: HTTP status code
  - `message`: Extracted or default error message
  - `code`: Optional error code from API
  - `body`: Raw response body (text/HTML/JSON)
  - `json_body`: Parsed JSON if response was valid JSON
- **Smart parsing**: Automatically extracts messages/codes from common JSON error fields
- **Fallback handling**: Graceful handling of non-JSON error responses
- **Rich diagnostics**: miette integration for excellent error reporting
- **Retry detection**: Built-in classification of retryable vs non-retryable errors

## Letta API Implementation Details

### Core API Endpoints Structure
- `/v1/agents` - Agent lifecycle management and configuration
- `/v1/agents/{agent_id}/messages` - Message exchange with streaming support
- `/v1/agents/{agent_id}/core-memory` - In-context memory management
- `/v1/agents/{agent_id}/archival-memory` - Vector-based long-term memory
- `/v1/tools` - Tool management and execution
- `/v1/sources` - Document and data source management
- `/v1/blocks` - Memory block operations
- `/v1/groups` - Multi-agent group conversations

### Key Technical Requirements
- **Authentication**: Bearer token validation
- **Streaming**: Server-sent events (SSE) for real-time responses
- **Memory Management**: Core, archival, and recall memory systems
- **Vector Storage**: Semantic search capabilities for archival memory
- **Tool Execution**: Dynamic tool loading and execution framework
- **File Handling**: Document upload and processing pipeline

### Pagination Pattern
All list endpoints use cursor-based pagination with `before`, `after`, `limit` parameters.

### Pagination Implementation Status

The library provides a generic `PaginatedStream` that supports automatic cursor-based pagination, filtering, mapping, and collection of results. It supports both ID-based cursors (`LettaId`) and string-based cursors.

**Implemented with Pagination Support:**
- ✅ `agents.paginated()` - Full pagination support with ID-based cursors
- ✅ `messages.paginated()` - Pagination for message lists with ID-based cursors
- ✅ `memory.archival_paginated()` - Pagination for archival memory with ID-based cursors
- ✅ `tags.paginated()` - Pagination with string-based cursors
- ✅ `providers.paginated()` - Pagination with ID-based cursors

**APIs that could support pagination (have cursor params):**
- ⬜ Tools API - `list()` has before/after/limit params
- ⬜ Sources API - `list_files()` and `list_passages()` have pagination params
- ⬜ Groups API - Group messages may support pagination

**APIs without cursor-based pagination support:**
- Blocks API - Only has `limit` parameter, no cursor support
- Memory: `list_core_memory_blocks()`, `list_agent_tools()` - No pagination params
- Sources: main `list()` method - No pagination params
- Various tool/MCP/Composio list methods - No pagination params
- Runs API: Uses array-based filtering, not cursor pagination
- Jobs API: Has limit but no cursor pagination

## API Reference

- **Official Documentation**: https://docs.letta.com/api-reference/
- **Base URLs**:
  - Local: `http://localhost:8283`
  - Cloud: `https://api.letta.com` (with API key)
- **Reference Implementations**:
  - TypeScript SDK: `letta-node/` submodule
  - Python SDK: `letta-python/` submodule

## Recent Implementation Notes

### Models API
- Supports filtering by provider category using repeated query parameters for arrays
- `ProviderCategory` enum with values: Base, Byok
- Query parameters properly handle Vec<ProviderCategory> serialization

### Tags API
- Simple string-based API returning Vec<String>
- Supports pagination with after/limit and query text filtering
- Uses string-based cursor pagination

### Providers API
- Full CRUD operations with LettaId-based IDs (format: `provider-<uuid>`)
- `api_key` is required for creation
- Updates limited to: `api_key`, `access_key`, `region` only
- Provider types include: OpenAI, Anthropic, Azure, Google AI, Groq, Cohere, etc.
- Delete endpoint returns JSON response with success message

## Implementation Priorities

1. **High Priority**:
   - Identities API (for agent identity management)
   - Pagination for Tools API
   - Pagination for Sources files/passages

2. **Medium Priority**:
   - Telemetry API (usage tracking)
   - Batches API (batch operations)

3. **Low Priority**:
   - Voice API (voice conversation support)
   - Examples directory with common use cases

## Development Principles

- NEVER ignore a test
- **ALWAYS ENSURE that tests will fail (via assert or panic with descriptive message) on any error condition**
- Use proper error handling with detailed context (LettaError types)
- Follow existing patterns for consistency
- Verify API behavior with curl when implementing new endpoints

## Development Guidance

- Use the web or context7 to help find docs, in addition to any other reference material
- Check TypeScript/Python SDKs for API patterns and expected behavior
- Test with local Letta server before assuming implementation is correct