# CLAUDE.md


This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This project is a Rust client library for the Letta REST API. Letta is a stateful AI agent platform that enables building agents with persistent memory and context across conversations. The library provides idiomatic Rust bindings for all Letta API endpoints, with a CLI tool included for testing and development purposes.

### Letta API Scope
The implementation will include all major API categories from the official specification:
- **Agents**: CRUD operations for AI agents with persistent state and messaging
- **Tools**: Tool management and execution framework
- **Sources**: Document upload and processing for agent knowledge
- **Groups**: Multi-agent group conversations
- **Identities**: Identity management for agents
- **Models**: Model configuration and management
- **Memory (Blocks)**: Memory block operations for persistent storage
- **Jobs**: Asynchronous job management
- **Authentication**: Bearer token-based API authentication
- **Additional**: Health checks, providers, runs, steps, tags, telemetry, batches, voice, templates, projects

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
  - `message.rs` - Message types (stub)
  - `tool.rs` - Tool types (stub)
  - `source.rs` - Source types (stub)
- `src/api/` - API endpoint implementations (stubs)
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

## API Reference

- **Official Documentation**: https://docs.letta.com/api-reference/
- **Base URLs**:
  - Local: `http://localhost:8283`
  - Cloud: `https://api.letta.com` (with API key)

## Important Notes

- When changing the package name in `Cargo.toml`, also update `flake.nix` and run `cargo generate-lockfile`
- The project includes pre-commit hooks for rustfmt and nixpkgs-fmt
- Darwin-specific dependencies (IOKit) are handled in the Nix configuration
- Template was designed for easy initialization via omnix: `nix run github:juspay/omnix -- init github:srid/letta-rs`
- Implementation should match the official Letta API specification at docs.letta.com
