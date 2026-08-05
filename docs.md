# LiteRT-LM Rust Bindings Documentation

Comprehensive documentation for the `litert-lm-rust` crate, providing safe, idiomatic Rust bindings for Google's LiteRT-LM on-device LLM runtime.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [API Reference](#api-reference)
- [Advanced Usage](#advanced-usage)
- [Error Handling](#error-handling)
- [Platform Support](#platform-support)
- [Performance Optimization](#performance-optimization)
- [Troubleshooting](#troubleshooting)

## Overview

LiteRT-LM Rust bindings provide a safe, ergonomic interface to the LiteRT-LM C API. The crate manages native resources via RAII, converts errors to Rust `Result<T, Error>`, and supports streaming generation via callbacks.

### Key Features

- **Conversation API**: Chat-style `send_message` / streaming
- **Multimodal**: Vision & audio backends, image/audio content parts (path or blob)
- **MTP**: Multi-Token Prediction via speculative decoding
- **Session API**: Lower-level prefill / decode / generate_content
- **Tools**: Function calling, tool use
- **Thinking**: Reasoning/thinking tokens
- **Constrained Decoding**: Regex and JSON schema constraints
- **LoRA**: Low-Rank Adaptation support
- **Benchmarks**: Performance metrics

## Installation

### Basic Installation

```toml
[dependencies]
litert-lm-rust = "0.13"
```

The crate will automatically download the required native libraries from GitHub releases on first build (Windows only).

### Disable Auto-Download

If you prefer to provide your own native libraries:

```toml
[dependencies]
litert-lm-rust = { version = "0.13", default-features = false }
```

Then set the `LITERT_LM_LIB_DIR` environment variable to point to your library directory.

## Quick Start

### Basic Text Generation

```rust
use litert_lm_rust::{Backend, Engine, Message};

fn main() -> litert_lm_rust::Result<()> {
    let engine = Engine::builder("model.litertlm")
        .backend(Backend::Cpu)
        .build()?;
    
    let mut conversation = engine.create_conversation(Default::default())?;
    let reply = conversation.send_message(Message::user("Hello!"))?;
    
    println!("{}", reply);
    Ok(())
}
```

### Streaming Generation

```rust
use litert_lm_rust::{Backend, Engine, Message};

fn main() -> litert_lm_rust::Result<()> {
    let engine = Engine::builder("model.litertlm")
        .backend(Backend::Cpu)
        .build()?;
    
    let mut conversation = engine.create_conversation(Default::default())?;
    let stream = conversation.send_message_stream(Message::user("Tell me a story"))?;
    
    for event in stream.iter() {
        match event {
            litert_lm_rust::StreamEvent::Chunk(chunk) => {
                if let Some(text) = chunk.text {
                    print!("{}", text);
                }
            }
            litert_lm_rust::StreamEvent::StartFailed(code) => {
                eprintln!("Stream failed with code: {}", code);
            }
        }
    }
    
    Ok(())
}
```

## Core Concepts

### Engine

The `Engine` is the heavyweight model holder that loads model weights and creates lightweight sessions/conversations.

```rust
use litert_lm_rust::{Backend, Engine};

let engine = Engine::builder("model.litertlm")
    .backend(Backend::Cpu)
    .max_num_tokens(4096)
    .num_threads(4)
    .build()?;
```

### Conversation

The `Conversation` API provides a high-level chat interface that manages sessions, prompt templates, and multimodal preprocessing.

```rust
use litert_lm_rust::{ConversationConfig, Engine};

let mut conversation = engine.create_conversation(ConversationConfig {
    ..Default::default()
})?;
```

### Session

The `Session` API provides lower-level control for prefill/decode operations.

```rust
use litert_lm_rust::{SessionConfig, Engine};

let mut session = engine.create_session(SessionConfig {
    max_output_tokens: Some(1024),
    ..Default::default()
})?;
```

### Message

Messages represent chat interactions with roles and content.

```rust
use litert_lm_rust::{ContentPart, Message};

// Simple text message
let msg = Message::user("Hello");

// Multimodal message
let msg = Message::user_parts([
    ContentPart::text("Describe this image:"),
    ContentPart::image_path("photo.jpg"),
])?;
```

## API Reference

### EngineBuilder

Builder for configuring and creating an `Engine`.

#### Methods

- `new(model_path: impl AsRef<Path>) -> Self` - Create a new builder
- `backend(backend: Backend) -> Self` - Set the main execution backend
- `vision_backend(backend: Backend) -> Self` - Set vision backend for multimodal
- `audio_backend(backend: Backend) -> Self` - Set audio backend for multimodal
- `max_num_tokens(max: i32) -> Self` - Set maximum number of tokens
- `num_threads(n: i32) -> Self` - Set number of CPU threads
- `prefill_chunk_size(size: i32) -> Self` - Set prefill chunk size
- `cache_dir(dir: impl AsRef<Path>) -> Self` - Set cache directory
- `multi_token_prediction(enabled: bool) -> Self` - Enable MTP/speculative decoding
- `enable_benchmark() -> Self` - Enable benchmarking
- `build() -> Result<Engine>` - Build the engine

### Engine

Main model holder.

#### Methods

- `builder(model_path: impl AsRef<Path>) -> EngineBuilder` - Create a builder
- `create_session(config: SessionConfig) -> Result<Session>` - Create a session
- `create_conversation(config: ConversationConfig) -> Result<Conversation>` - Create a conversation
- `tokenize(text: &str) -> Result<Vec<i32>>` - Tokenize text
- `detokenize(tokens: &[i32]) -> Result<String>` - Detokenize tokens

### Conversation

High-level chat interface.

#### Methods

- `send_message(message: impl Into<Message>) -> Result<Message>` - Send a message (blocking)
- `send_message_with(message: impl Into<Message>, options: SendOptions) -> Result<Message>` - Send with options
- `send_message_stream(message: impl Into<Message>) -> Result<StreamEventReceiver>` - Send with streaming
- `send_message_stream_with(message: impl Into<Message>, options: SendOptions) -> Result<StreamEventReceiver>` - Send with options and streaming
- `clone_conversation() -> Result<Conversation>` - Clone the conversation
- `cancel_process() -> ()` - Cancel ongoing generation
- `token_count() -> Result<i32>` - Get current token count
- `render_message(message: &Message) -> Result<String>` - Render message to string
- `render_preface() -> Result<String>` - Render preface to string
- `benchmark_info() -> Result<BenchmarkInfo>` - Get benchmark information

### Session

Low-level session API.

#### Methods

- `generate_text(prompt: &str) -> Result<String>` - Generate text from prompt
- `generate_content(inputs: &[InputData]) -> Result<String>` - Generate content
- `generate_content_responses(inputs: &[InputData]) -> Result<Responses>` - Generate with responses
- `generate_content_stream(inputs: &[InputData]) -> Result<StreamEventReceiver>` - Generate with streaming
- `run_prefill(inputs: &[InputData]) -> Result<()>` - Run prefill
- `run_decode() -> Result<Responses>` - Run decode
- `run_decode_async() -> Result<StreamEventReceiver>` - Run decode async
- `cancel_process() -> ()` - Cancel ongoing process
- `benchmark_info() -> Result<BenchmarkInfo>` - Get benchmark information

### Message

Chat message representation.

#### Methods

- `new(role: Role, content: impl Into<Value>) -> Self` - Create new message
- `user(text: impl Into<String>) -> Self` - Create user message
- `system(text: impl Into<String>) -> Self` - Create system message
- `model(text: impl Into<String>) -> Self` - Create model message
- `tool(text: impl Into<String>) -> Self` - Create tool message
- `user_parts(parts: impl IntoIterator<Item = ContentPart>) -> Result<Self>` - Create multimodal user message
- `with_extra(key: impl Into<String>, value: impl Into<Value>) -> Self` - Add extra field
- `to_json_string() -> Result<String>` - Convert to JSON string
- `from_json_str(s: &str) -> Result<Self>` - Parse from JSON string
- `text() -> Option<String>` - Extract text content
- `tool_calls() -> Option<&Value>` - Get tool calls

### ContentPart

Multimodal content part.

#### Methods

- `text(text: impl Into<String>) -> Self` - Create text part
- `image_path(path: impl AsRef<Path>) -> Self` - Create image path part
- `image_bytes(bytes: &[u8]) -> Self` - Create image bytes part
- `audio_path(path: impl AsRef<Path>) -> Self` - Create audio path part
- `audio_bytes(bytes: &[u8]) -> Self` - Create audio bytes part

## Advanced Usage

### Multimodal Processing

```rust
use litert_lm_rust::{Backend, ContentPart, Engine, Message};

let engine = Engine::builder("gemma.litertlm")
    .backend(Backend::Cpu)
    .vision_backend(Backend::Gpu)
    .audio_backend(Backend::Cpu)
    .build()?;

let mut conversation = engine.create_conversation(Default::default())?;

let reply = conversation.send_message(Message::user_parts([
    ContentPart::text("Describe this image:"),
    ContentPart::image_path("photo.jpg"),
])?)?;
```

### Multi-Token Prediction (MTP)

```rust
use litert_lm_rust::{Backend, Engine};

let engine = Engine::builder("model.litertlm")
    .backend(Backend::Gpu)
    .multi_token_prediction(true) // Enable speculative decoding
    .build()?;
```

### Constrained Decoding

```rust
use litert_lm_rust::{
    ConversationOptionalArgs, ConstraintType, SendOptions,
};

let options = SendOptions {
    optional_args: Some(ConversationOptionalArgs {
        constraint_type: Some(ConstraintType::JsonSchema),
        constraint_string: Some(r#"{"type": "object"}"#.to_string()),
        ..Default::default()
    }),
    ..Default::default()
};

let reply = conversation.send_message_with(message, options)?;
```

### Tool Calling

```rust
use litert_lm_rust::{ConversationConfig, tool_declaration, Engine};
use serde_json::json;

let config = ConversationConfig {
    tools: Some(json!([tool_declaration(
        "get_weather",
        "Get current weather for a location",
        json!({
            "type": "object",
            "properties": {
                "location": {"type": "string"}
            },
            "required": ["location"]
        })
    )])),
    ..Default::default()
};

let mut conversation = engine.create_conversation(config)?;
let reply = conversation.send_message(Message::user("What's the weather in Tokyo?"))?;

if let Some(tool_calls) = reply.tool_calls() {
    println!("Tool calls: {}", tool_calls);
}
```

### Thinking/Reasoning

```rust
use litert_lm_rust::{ConversationConfig, ThinkingConfig, Engine};

let config = ConversationConfig {
    thinking: Some(ThinkingConfig {
        enable_thinking: Some(true),
        thinking_token_budget: Some(4096),
    }),
    ..Default::default()
};

let mut conversation = engine.create_conversation(config)?;
```

### Custom Sampling

```rust
use litert_lm_rust::{SamplerParams, SessionConfig, Engine};

let config = SessionConfig {
    sampler: Some(SamplerParams::top_k(40).with_temperature(0.8)),
    max_output_tokens: Some(1024),
    ..Default::default()
};

let mut session = engine.create_session(config)?;
```

### LoRA Adapters

```rust
use litert_lm_rust::{Engine, EngineBuilder};

let engine = Engine::builder("model.litertlm")
    .lora_rank(8)
    .supported_lora_ranks(vec![4, 8, 16])
    .build()?;
```

## Error Handling

All operations return `Result<T, Error>` where `Error` is an enum:

```rust
pub enum Error {
    NullPointer(&'static str),
    NativeStatus(&'static str, i32),
    ModelPath,
    Nul(std::ffi::NulError),
    Utf8(std::str::Utf8Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    IndexOutOfRange(usize, usize),
    Message(String),
}
```

### Common Errors

- `NullPointer`: Native function returned null pointer
- `NativeStatus`: Native function returned error code
- `ModelPath`: Invalid model path
- `Utf8`: Invalid UTF-8 from native library
- `Json`: JSON serialization/deserialization error
- `Io`: File I/O error

## Platform Support

### Windows

Fully supported with automatic native library download from GitHub releases.

### Linux

Supported but requires manual native library setup via environment variables.

### macOS

Supported but requires manual native library setup via environment variables.

## Performance Optimization

### Backend Selection

- **CPU**: Good for compatibility, slower performance
- **GPU**: Best performance for large models
- **Custom**: Vendor-specific backends (NPU, OpenVINO, etc.)

### MTP (Multi-Token Prediction)

Enable MTP for GPU backends to improve generation speed:

```rust
.multi_token_prediction(true)
```

### Thread Configuration

Optimize thread count based on your CPU:

```rust
.num_threads(num_cpus::get() as i32)
```

### Prefill Chunk Size

Adjust prefill chunk size for better memory/performance tradeoff:

```rust
.prefill_chunk_size(512)
```

## Troubleshooting

### Native Library Not Found

If you see "native library not found" warnings:

1. Enable auto-download (default): `cargo build --features download-native`
2. Or set `LITERT_LM_LIB_DIR` environment variable
3. Or place libraries in `prebuilt/`, `native/`, or `c/` directories

### Linking Errors

If you encounter linking errors:

1. Ensure native libraries are for your target platform
2. Check that import library (`.lib`/`.if.lib`) matches DLL
3. Verify `LITERT_LM_LIB_NAME` is correct

### Runtime DLL Errors

If the executable can't find DLLs at runtime:

1. Copy DLLs next to the executable
2. Add DLL directory to `PATH`
3. Use the auto-download feature (copies to correct location)

### Out of Memory

If you encounter OOM errors:

1. Reduce `max_num_tokens`
2. Use smaller models
3. Reduce `prefill_chunk_size`
4. Enable GPU backend if available

### Slow Performance

For better performance:

1. Enable MTP: `.multi_token_prediction(true)`
2. Use GPU backend: `.backend(Backend::Gpu)`
3. Optimize thread count: `.num_threads()`
4. Increase prefill chunk size: `.prefill_chunk_size()`

## Additional Resources

- [LiteRT-LM GitHub Repository](https://github.com/google-ai-edge/LiteRT-LM)
- [LiteRT-LM Documentation](https://ai.google.dev/edge/litert-lm)
- [Rust Crate Documentation](https://docs.rs/litert-lm-rust)
- [Examples](./examples/)
