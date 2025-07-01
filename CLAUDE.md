# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Git Workflow - Feature Branches

**IMPORTANT**: Now that the project is stable, we use feature branches for all development:

1. **Before starting any work**, create a feature branch:
   ```bash
   git checkout -b feature/descriptive-name
   # Examples: feature/add-default-impls, fix/batch-api-errors, docs/improve-examples
   ```

2. **Commit regularly** as you work:
   - After each logical change or set of related edits
   - Use clear, descriptive commit messages
   - Example: `git commit -m "Add Default impl for UpdateMemoryBlockRequest"`

3. **When feature is complete**, create a pull request to main
   - This keeps main stable and CI runs only on complete changes
   - Allows for code review and discussion

4. **Branch naming conventions**:
   - `feature/` - New features or enhancements
   - `fix/` - Bug fixes
   - `docs/` - Documentation improvements
   - `refactor/` - Code refactoring
   - `test/` - Test additions or improvements

## Development Principles

- Run `cargo check` frequently when producing code. This will help you catch errors early.
- NEVER use `unsafe{}`. If you feel you need to, stop, think about other ways, and ask the user for help if needed.
- NEVER ignore a failing test or change a test to make your code pass
- NEVER ignore a test
- ALWAYS fix compile errors before moving on.
- **ALWAYS ENSURE that tests will fail (via assert or panic with descriptive message) on any error condition**
- Use proper error handling with detailed context (LettaError types)
- Follow existing patterns for consistency
- Verify API behavior with curl when implementing new endpoints
- Use the web or context7 to help find docs, in addition to any other reference material
- Check TypeScript/Python SDKs for API patterns and expected behavior
- Test with local Letta server before assuming implementation is correct


## Project Overview

This is the development guide for contributing to letta, a Rust client library for the Letta REST API. This document contains internal implementation details, coding standards, and development workflows. For user documentation, see README.md.

### Implementation Status

**All 20 planned APIs are implemented.** Voice API is in beta with generic JSON support due to undocumented structure.

#### Pagination Implementation Matrix
- **Full pagination**: agents, messages, memory.archival, tags, providers, tools, sources.files, sources.passages, identities
- **No pagination**: blocks, groups, jobs, runs, batch (API limitations)
- **Generic types used**: Voice (serde_json::Value), Identities properties (serde_json::Value)

## Development Commands

```bash
# Quick start
nix develop           # Enter dev shell with all tools
just watch           # Auto-recompile on changes
just test           # Run all tests
just pre-commit-all # Format and lint

# Local Letta server for testing
cd local-server && docker compose up -d

# CLI tool (generates JSON, doesn't make API calls)
cargo run --features cli -- agent create -n "Test" -o json | \
  curl -X POST http://localhost:8283/v1/agents -H "Content-Type: application/json" -d @-
```

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
  - `identities.rs` - Identity types (User, Org, Other)
  - `batch.rs` - Batch processing types
  - `telemetry.rs` - Telemetry trace types
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
  - `batch.rs` - Batch processing operations
  - `telemetry.rs` - Telemetry trace retrieval
  - `voice.rs` - Voice conversation support (beta)
- `src/cli.rs` - CLI testing tool (binary: `letta`, requires `cli` feature)
- `tests/` - Integration tests
- `nix/modules/` - Modular Nix configuration
  - `devshell.nix` - Development shell configuration
  - `rust.nix` - Rust build configuration via rust-flake/crane
  - `pre-commit.nix` - Code formatting and linting hooks
- `justfile` - Command shortcuts for common development tasks
- `compose.yml` - Docker Compose configuration for local test server
- `server.env` - Server environment variables (need to provide an example, since this is not committed)

### Build System
This project uses a dual build approach:
1. **Cargo** - Standard Rust package manager
2. **Nix + Crane** - Reproducible builds via rust-flake

The Nix configuration uses flake-parts for modularity and imports rust-flake for Rust-specific build logic.

### Key Dependencies
- `clap` (4.5+) with derive and env features for CLI argument parsing
- Development tools: just, bacon, nixd, cargo-doc-live, docker compose

### Critical Implementation Details

#### Error Response Parsing
The API returns errors in various formats. Our error handler checks these fields in order:
1. `detail` (string or array)
2. `error.message`
3. `error`
4. `message`
5. `msg`

#### Headers and Authentication
- Bearer token in Authorization header
- Optional headers: `X-Project`, `user-id`
- All headers handled via ClientConfig

#### Streaming (SSE) Implementation
- Uses `eventsource-stream` crate
- Handles reconnection and error recovery
- Parses JSON chunks from `data:` lines
- Supports assistant messages, function calls, and usage events

## Testing Strategy

### Running Tests

```bash
# Unit tests only (no server required)
cargo test --lib --bins
cargo test --doc

# Local server integration tests (requires Docker)
nix run .#test-local

# Cloud API tests (requires LETTA_API_KEY)
LETTA_API_KEY=your-key nix run .#test-cloud

# Run all tests
LETTA_API_KEY=your-key nix run .#test-all
```

### Test Environment Setup

1. **Local Server Tests**:
   - Copy `server.env.example` to `server.env`
   - The example file contains minimal config for embedded PostgreSQL
   - Optional: Add your own API keys for Azure, Composio, etc.

2. **Cloud API Tests**:
   - Set `LETTA_API_KEY` environment variable
   - Tests marked with `#[ignore]` require this

### Test Categories

- **Unit Tests**: Type serialization, error parsing, pagination logic
- **Integration Tests**: Full API workflows against local server
- **Cloud Tests**: Tests against production Letta API (marked `#[ignore]`)

### CI Integration

The Nix build runs unit tests by default during `nix build`. Integration tests require Docker and must be run separately:

```bash
# In CI with Docker available:
nix build  # Runs unit tests only
nix run .#test-local  # Runs integration tests with Docker

# Or use standard cargo in CI:
cargo test --lib --bins  # Unit tests
cargo test --doc         # Doc tests
./nix/test-local-server.sh  # Integration tests
```

### Known Test Issues
- Archival memory test has server response handling bug
- CLI doesn't make actual API calls (generates JSON only)
- Integration tests require Docker daemon access

## Adding New APIs Checklist

1. Create types in `src/types/{api_name}.rs`
2. Add API module in `src/api/{api_name}.rs`
3. Export from `src/types/mod.rs` and `src/api/mod.rs`
4. Add convenience method to `LettaClient`
5. Write integration tests
6. Update README.md API coverage section
7. Check if pagination is supported (look for cursor params)
8. Handle any special cases (generic JSON, file uploads, streaming)

## Development Resources

- **API Spec**: https://docs.letta.com/api-reference/
- **Reference SDKs**: Check letta-node/ and letta-python/ submodules for patterns
- **Local Testing**: Use local-server/ for development
- **Type Discovery**: Use `curl -v` against local server to inspect responses

## Remaining Tasks

1. ~~**Rename crate to `letta`**~~ - ✅ Completed
2. **Documentation pass** - Update examples to use new ergonomic features
3. ~~**Finish CLI implementation**~~ - ✅ Completed - CLI now makes actual API calls
4. **Implement upsert-from-function** - Port Python SDK's function-based agent creation feature

## CLI Implementation Status

The CLI (`letta` binary) is now fully functional with complete API integration. It supports:

### Current Features
- **Agent Management**: list, create, get, delete operations
- **Health Check**: Server status verification
- **Authentication**: API key via CLI arg or environment variable
- **Output Formats**: JSON, pretty-printed JSON, and human-readable summaries
- **Error Handling**: Proper exit codes and user-friendly error messages

### Future CLI Improvements

1. **Additional Commands**:
   - `message` subcommand for sending messages to agents
   - `memory` subcommand for viewing/editing agent memory
   - `tools` subcommand for managing tools
   - `sources` subcommand for document management
   - `batch` subcommand for batch operations

2. **Interactive Features**:
   - Interactive agent chat mode (`letta chat <agent-id>`)
   - Streaming message responses with progress indicators
   - Auto-completion for agent IDs and tool names
   - Configuration file support (`~/.letta/config.toml`)

3. **Quality of Life**:
   - Colored output with `--color` flag
   - Table formatting for list commands
   - Progress bars for long operations
   - Retry logic for transient failures
   - Cache frequently used data (agent lists, tool names)

4. **Advanced Features**:
   - Export/import agent configurations
   - Bulk operations (delete multiple agents, batch create)
   - Agent templates for quick creation
   - Performance profiling with `--profile` flag
   - Dry-run mode for testing commands

5. **Developer Tools**:
   - Debug output with request/response details
   - API endpoint override for testing
   - Mock mode for offline development
   - OpenAPI spec generation from CLI

## Recent Implementation Notes

### Ergonomic Improvements (Completed)

All planned ergonomic improvements have been implemented:

1. **Default Implementations**: Added for all request types with optional fields
2. **Builder Patterns**: Implemented for `CreateAgentRequest`, `CreateBlockRequest`, `CreateToolRequest`, `ConditionalToolRule`
3. **Smart Constructors**: 
   - `Block::human()`, `Block::persona()`, `Block::new()` with builder methods
   - `LLMConfig::openai()`, `LLMConfig::anthropic()`, `LLMConfig::local()`
   - `ResponseFormat::text()`, `ResponseFormat::json()`
4. **From/Into Implementations**: Used throughout for string conversions
5. **Tool Rule Builders**: Complete set of constructors for all `ToolRule` variants
6. **SmartDefault Trait**: Applied to enums like `AgentType` and `ResponseFormatType`

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
- Delete endpoint returns null response (204 No Content)

### Identities API
- Full CRUD operations with LettaId-based IDs (format: `identity-<uuid>`)
- Supports identity types: User, Org, Other (serialized as lowercase)
- Upsert endpoint only supports updating existing identities (returns 404 for non-existent)
- Properties system for flexible metadata storage
- Delete endpoint returns null response (204 No Content)
- Pagination support with string-based cursors

### Tools API
- Pagination support added with string-based cursors
- Full tool lifecycle management

### Sources API
- Added pagination for files (`paginated_files()`) and passages (`paginated_passages()`)
- Both use string-based cursor pagination

### Batch API
- Full batch message processing implementation
- List batch messages with optional filtering by run/step IDs
- Uses `ListBatchMessagesParams` with limit parameter (no cursor pagination)
- **Note**: Server support varies - requires `LETTA_ENABLE_BATCH_JOB_POLLING=true` environment variable
- Some server versions may return `NotImplementedError` when creating batches
- Renamed `BatchMessageCreate` to `BatchMessage` with additional optional fields (name, tool_calls, tool_call_id)

### Telemetry API
- Provider trace retrieval by step ID
- Returns `ProviderTrace` objects with telemetry data
- Currently supports trace retrieval only (not full usage tracking)

### Voice API (Beta)
- Basic voice chat completions endpoint at `/v1/voice-beta/{agent_id}/chat/completions`
- Uses generic JSON values (`serde_json::Value`) due to undocumented API structure
- Expects OpenAI-compatible chat completion requests based on docs
- `create_voice_chat_completions` endpoint with optional `user-id` header support
- Designed for streaming voice agent interactions with `voice_convo_agent` type
- **Note**: Requires OPENAI_API_KEY environment variable configured on server
- API structure is undocumented and subject to change

## Implementation Gotchas

### API-Specific Quirks

1. **Providers API**: Updates only accept `api_key`, `access_key`, `region` (not all fields)
2. **Identities API**: Upsert only updates existing (404 for new), use create instead
3. **Delete endpoints**: Return 204 No Content (handled as Option<T>)
4. **Voice API**: Completely undocumented, uses OpenAI-like format
5. **Batch API**: Has limit but no cursor pagination
6. **File uploads**: Use multipart form with proper content-type detection

### Serialization Edge Cases

1. **Enum lowercase**: Many enums serialize as lowercase (e.g., identity_type)
2. **Skip None fields**: Use `#[serde(skip_serializing_if = "Option::is_none")]`
3. **Flatten for generic**: Use `#[serde(flatten)]` for pass-through JSON
4. **Array query params**: Repeated params for Vec<T> (e.g., ?category=Base&category=Byok)

## Contribution Guidelines

### Code Style
- Use `cargo fmt` and `cargo clippy` (enforced by pre-commit)
- Follow existing patterns for consistency
- Add doc comments for all public APIs
- Include usage examples in doc comments

### Testing Requirements
- Add unit tests for new types
- Add integration tests for new APIs
- Test error cases and edge conditions
- Verify against local server before PR

### Documentation
- Update README.md for user-facing changes
- Keep CLAUDE.md updated for implementation details
- Add inline documentation for complex logic
- Include examples in rustdoc comments
