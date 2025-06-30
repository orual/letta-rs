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

## API Reference

- **Official Documentation**: https://docs.letta.com/api-reference/
- **Base URLs**:
  - Local: `http://localhost:8283`
  - Cloud: `https://api.letta.com` (with API key)
- **Reference Implementations**:
  - TypeScript SDK: `letta-node/` submodule
  - Python SDK: `letta-python/` submodule

## Implementation Roadmap

Based on analysis of Python and TypeScript SDKs, our current Rust implementation covers ~5% of the full API surface. Here's our comprehensive plan:

### Current Status (Completed)
- ✅ Basic agent CRUD operations (list, create, get, delete)
- ✅ Core project structure and error handling
- ✅ Integration tests against local server
- ✅ Basic type system for agents and memory blocks

### Phase 1: Core Infrastructure
**Priority: CRITICAL - needed for basic functionality**

1. **Complete Agent API** - finish all agent endpoints
   - `count()`, `export_file()`, `import_file()`
   - `summarize_agent_conversation()`, `search()`
   - Agent sub-APIs: core_memory, archival_memory, tools, sources

2. **Message API with Streaming** - implement SSE streaming for messages
   - `/v1/agents/{id}/messages` with full CRUD
   - Server-sent events streaming via reqwest + tokio
   - Token-level streaming with `stream_tokens` parameter

3. **Enhanced Error Handling** - status-specific error types
   - HTTP status code to specific error type mapping
   - Better error messages matching Python/TypeScript patterns

4. **Environment Management** - proper cloud vs local handling
   - Environment enum (LettaCloud, Local)
   - Authentication per environment

### Phase 2: Memory & Tools
**Priority: HIGH - core Letta functionality**

5. **Memory API Completeness**
   - Core memory operations (get/set persona, human)
   - Archival memory with semantic search
   - Memory blocks and variables management
   - Passage retrieval and management

6. **Tool Management** - basic CRUD operations
   - Tool creation, listing, updating, deletion
   - `create_from_function()` for auto tool creation
   - MCP server integration (later phase)

7. **Source Management** - file upload/processing
   - Document upload and processing status
   - Source CRUD operations
   - File handling pipeline


### Phase 3: Advanced Features
**Priority: MEDIUM - advanced use cases**

8. **Groups & Multi-Agent**
   - Multi-agent group conversations
   - Group management and coordination

9. **Runs & Execution Management**
    - Execution runs tracking
    - Run state management

10. **Jobs & Steps** - async processing with feedback
    - Asynchronous job management
    - Step feedback and management
    - Background processing patterns

11. **Advanced Streaming**
    - Token-level streaming refinements
    - Multiple concurrent streams
    - Stream error handling

12. **Templates & Projects**
    - Agent template system
    - Project management functionality

### Phase 4: Ecosystem Features
**Priority: LOW - nice-to-have**

13. **Batches & Telemetry**
    - Batch job processing
    - Usage tracking and telemetry

14. **Voice Integration**
    - Voice conversation support
    - Audio processing pipeline

15. **Client scoped auth tokens for cloud API**

16. **Comprehensive Type System**
    - Complete all missing types (~230 types)
    - Auto-generated type definitions
    - Complex union types for message content

### Missing API Categories (22 total)
Currently missing these major API endpoints:
- Batches, Blocks, ClientSideAccessTokens, EmbeddingModels
- Groups, Identities, Jobs, Models, Projects
- Providers, Runs, Steps, Tags, Telemetry, Templates, Voice
- Plus nested sub-APIs under agents (context, tools, sources, etc.)

### Implementation Strategy
1. **Focus on high-value first** - agents, messages, memory cover 80% of use cases
2. **Streaming early** - core to Letta's value proposition
3. **Auto-generate types** - both Python/TS use codegen, we should too
4. **API compatibility** - match Python/TS patterns where possible
5. **Comprehensive testing** - maintain test coverage as we expand

### Current API Coverage
- **Agents**: 100% complete ✅ (all endpoints implemented and tested)
- **Messages**: 95% complete ✅ (all endpoints including SSE streaming, update, async)
  - Missing: Individual message GET/DELETE (API doesn't appear to have these)
- **Memory**:
  - Core Memory: 100% complete ✅ (all endpoints implemented and tested)
  - Archival Memory: 100% complete ✅ (all endpoints implemented and tested)
    - Note: PATCH endpoint has server-side bug (returns tuples instead of Passage objects)
  - Memory Blocks: 5% complete (types only)
- **Tools**: 100% complete ✅ (all endpoints including agent sub-API implemented and tested)
- **Sources**: 100% complete ✅ (all endpoints including file uploads and agent sub-API implemented and tested)
- **Type System**: 100% complete ✅ (LettaId type for all ID fields throughout the API)
- **Error Handling**: 100% complete ✅ (status-specific error mapping with smart extraction)
- **Environment Management**: 100% complete ✅ (Cloud vs Local with convenience constructors)
- **All other categories**: 0% complete

### Recent Improvements
- ✅ Custom `LettaId` type for handling both bare UUIDs and prefixed UUIDs
  - Automatically handles formats like "agent-550e8400-e29b-41d4-a716-446655440000"
  - Seamless conversion to/from String for API calls
  - Full serde support for JSON serialization
- ✅ Message update functionality (PATCH)
- ✅ Async message creation endpoint returning Run objects
- ✅ Comprehensive error handling with detailed API error responses
- ✅ Complete refactoring of all ID fields to use LettaId type throughout codebase
- ✅ Status-specific error mapping (HTTP status codes to specific error types)
  - 401 → Auth, 404 → NotFound, 422 → Validation, 429 → RateLimit, 408/504 → RequestTimeout
  - Smart extraction of resource information from error messages
  - Validation field extraction from 422 errors
  - Retry-after header parsing for rate limits
- ✅ Environment management system
  - `LettaEnvironment` enum with Cloud and SelfHosted variants
  - Convenience constructors: `LettaClient::cloud(token)` and `LettaClient::local()`
  - Builder pattern for advanced configuration
  - Automatic authentication handling based on environment

Target: 90%+ API coverage following this roadmap.

## Phase 1: Core Infrastructure ✅ COMPLETED

Phase 1 has been completed! All core infrastructure is now in place:
- ✅ All agent sub-APIs (core memory, archival memory, tools, sources)
- ✅ Message API with SSE streaming support
- ✅ Enhanced error handling with status-specific mapping
- ✅ Environment management (Cloud vs Local)
- ✅ Health endpoint implementation
- ✅ Type-safe LettaId throughout the codebase

For detailed Phase 1 implementation notes, see [PHASE1_ARCHIVE.md](./PHASE1_ARCHIVE.md).

## Phase 2: Memory & Tool Systems ✅ COMPLETED

Phase 2 has been completed! All memory and tool systems are now fully implemented:
- ✅ Memory Blocks API with full CRUD operations
- ✅ All Tool endpoints including MCP and Composio integration
- ✅ Infrastructure improvements (retry logic, enhanced errors, timeout)

For detailed Phase 2 implementation notes, see [PHASE2_ARCHIVE.md](./PHASE2_ARCHIVE.md).

### 1. Memory Blocks API (Priority: HIGH) ✅ COMPLETED
- [x] GET `/v1/blocks/` - List all memory blocks
- [x] POST `/v1/blocks/` - Create a new memory block
- [x] GET `/v1/blocks/{block_id}` - Get a specific block
- [x] PATCH `/v1/blocks/{block_id}` - Update a block
- [x] DELETE `/v1/blocks/{block_id}` - Delete a block
- [x] GET `/v1/blocks/count` - Get blocks count

All endpoints implemented with:
- Full CRUD operations
- Filtering support (label, templates_only, name, identity_id, identifier_keys)
- Metadata support
- Comprehensive integration tests

### 2. Tool Features (Priority: MEDIUM)
#### Core Tool CRUD ✅ COMPLETE
- [x] GET `/v1/tools/` - List all tools
- [x] POST `/v1/tools/` - Create a new tool  
- [x] GET `/v1/tools/{tool_id}` - Get a tool by ID
- [x] PATCH `/v1/tools/{tool_id}` - Update a tool
- [x] DELETE `/v1/tools/{tool_id}` - Delete a tool
- [x] GET `/v1/tools/count` - Get tools count
- [x] PUT `/v1/tools/` - Upsert a tool
- [x] POST `/v1/tools/run` - Run tool from source code ✅ COMPLETED

#### MCP Integration ✅ COMPLETE
- [x] GET `/v1/tools/mcp/servers` - List all configured MCP servers
- [x] PUT `/v1/tools/mcp/servers` - Add a new MCP server
- [x] GET `/v1/tools/mcp/servers/{mcp_server_name}/tools` - List tools for a specific MCP server
- [x] POST `/v1/tools/mcp/servers/{mcp_server_name}/{mcp_tool_name}` - Add an MCP tool to Letta
- [x] DELETE `/v1/tools/mcp/servers/{mcp_server_name}` - Delete an MCP server
- [x] PATCH `/v1/tools/mcp/servers/{mcp_server_name}` - Update an MCP server configuration
- [x] POST `/v1/tools/mcp/servers/test` - Test an MCP server connection

All endpoints implemented with:
- Full union type support for different server types (SSE, STDIO, Streamable HTTP)
- Proper request/response types matching TypeScript SDK
- Integration with retry logic
- Comprehensive integration tests using STDIO servers

#### Composio Integration ✅ COMPLETE
- [x] GET `/v1/tools/composio/apps` - List all Composio apps
- [x] GET `/v1/tools/composio/apps/{composio_app_name}/actions` - List actions for a specific Composio app
- [x] POST `/v1/tools/composio/{composio_action_name}` - Add a Composio tool to Letta

#### Other ✅ COMPLETE
- [x] POST `/v1/tools/add-base-tools` - Upsert base tools (adds/updates default tool set)
- [ ] ~~POST `/v1/tools/from-function`~~ - Client-side feature (not an API endpoint)
  - Could implement with PyO3/WASM in future for Rust function extraction

### 3. Infrastructure Improvements (Priority: MEDIUM)
- [x] Add retry logic for transient failures (429, 503, etc.) ✅ COMPLETED
  - Exponential backoff with jitter
  - Configurable retry attempts and delays
  - Smart error classification (retryable vs non-retryable)
  - Respects Retry-After headers for rate limits
  - Comprehensive integration tests
- [x] Improve error types to provide more context ✅ COMPLETED
  - Added URL and HTTP method to API errors
  - Enhanced error help messages with request details
  - Added `tracing` instrumentation to all HTTP methods
  - All errors now provide rich diagnostic information
- [x] Add request timeout configuration ✅ COMPLETED
  - Configurable via `ClientBuilder::timeout()`
  - Default 30 second timeout
  - Applied to all HTTP operations

### Endpoint Availability: Local vs Cloud

#### Agent API Endpoints
| Endpoint | Local Server | Cloud API | Notes |
|----------|--------------|-----------|-------|
| `list()` | ✅ | ✅ | Full CRUD operations |
| `create()` | ✅ | ✅ | |
| `get()` | ✅ | ✅ | |
| `delete()` | ✅ | ✅ | |
| `summarize_agent_conversation()` | ✅ | ✅ | |
| `count()` | ✅ | ✅ | |
| `export_file()` | ✅ | ✅ | Returns JSON as string |
| `import_file()` | ✅ | ✅ | Multipart file upload |
| `search()` | ❌ | ✅ | Cloud only, requires `project_id` |

**Local Server**: `http://localhost:8283` (no auth required)
**Cloud API**: `https://api.letta.com` (requires API key)

All agent endpoints work on both local and cloud except search, which is cloud-only and requires a project_id parameter.

## Important Notes

- When changing the package name in `Cargo.toml`, also update `flake.nix` and run `cargo generate-lockfile`
- The project includes pre-commit hooks for rustfmt and nixpkgs-fmt
- Darwin-specific dependencies (IOKit) are handled in the Nix configuration
- Template was designed for easy initialization via omnix: `nix run github:juspay/omnix -- init github:srid/letta-rs`
- Implementation should match the official Letta API specification at docs.letta.com
- **NEVER ignore a test without asking first and getting user approval**

## TODO Management

### Current TODO Status

#### Completed ✅
1. Create custom ID type for string-prefixed UUIDs
2. Refactor all ID fields to use LettaId type
3. Create status-specific error type mapping
4. Add environment management (Cloud vs Local)

#### In Progress 🚧
- Phase 3 planning (see Phase 3 section below)

#### Notes
- All integration tests are now passing (cloud tests remain ignored as they require API keys)
- Streaming tests pass locally when streaming is supported
- Archival memory tests pass despite server-side bugs

## Key Implementation Insights

### ID Format
Letta uses prefixed UUIDs throughout the API. Common prefixes include:
- `agent-` for agent IDs
- `message-` for message IDs
- `tool-` for tool IDs
- `source-` for source IDs
- `block-` for memory block IDs
- `passage-` for archival memory passage IDs
- `run-` for async job run IDs
- `file-` for file IDs

The API accepts both bare UUIDs and prefixed UUIDs, but typically returns prefixed versions.

### Known Server Issues
1. **Archival Memory Update Bug**: The PATCH `/v1/agents/{id}/archival-memory/{memory_id}` endpoint has a server-side bug where it returns response data as tuples instead of proper Passage objects. This causes deserialization failures.
2. **Embedding Requirements**: When updating archival memory text, the server requires embedding and embedding_config fields to be provided (even though they should be optional) because the embeddings need to match the updated text.
3. **Tool Creation Requirements**: The server has strict validation for tool creation:
   - Python functions MUST have a docstring with an `Args:` section documenting each parameter
   - Even if you provide `json_schema` and `args_json_schema`, the docstring is still mandatory
   - Without proper docstrings, you'll get errors like: `"Parameter 'param_name' in function 'function_name' lacks a description in the docstring"`
   - See `CreateToolRequest` documentation in `src/types/tool.rs` for details

### API Patterns
1. **Redundant IDs**: Many update endpoints include the resource ID both in the URL path and the request body
2. **Tool Names**: Tools can be referenced by both name (string) and ID when attaching to agents
3. **Pagination**: All list endpoints support cursor-based pagination with `before`, `after`, and `limit` parameters
4. **SSE Streaming**: Message streaming uses Server-Sent Events with automatic retry and proper error handling

### Test Resource Management
To prevent server resource exhaustion, tests should clean up all created resources. A cleanup script is provided:

```bash
# Clean up all test resources (agents, tools, blocks, sources, MCP servers)
./cleanup_test_resources.sh
```

The script identifies test resources by name patterns (containing "test", "Test", "echo", etc.) and deletes them. This should be run if the server shows high CPU usage after running tests.

**Note**: Some tests may not clean up properly, especially if they fail. The MCP integration tests in particular may leave resources behind.

## Development Principles

- NEVER ignore a test
