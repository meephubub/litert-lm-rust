//! Basic integration test for LiteRT-LM Rust bindings.
//!
//! This test loads the model file and runs a simple generation to verify
//! the bindings work correctly. It is ignored by default because it requires
//! the native DLLs to be available.
//!
//! Run with: `cargo test -- --ignored`

use litert_lm_rust::{Backend, Engine, LogSeverity, Message};

#[test]
#[ignore = "requires native DLLs and model file"]
fn test_basic_generation() {
    // Set log level to reduce noise
    litert_lm_rust::set_min_log_level(LogSeverity::Warning);

    // Use the model file from the repository root
    let model_path = "gemma-4-E2B-it.litertlm";

    // Build the engine with CPU backend
    let engine = Engine::builder(model_path)
        .backend(Backend::Cpu)
        .build()
        .expect("Failed to create engine - ensure model file exists and DLLs are on PATH");

    // Create a conversation with default config
    let mut conversation = engine
        .create_conversation(Default::default())
        .expect("Failed to create conversation");

    // Send a simple prompt
    let reply = conversation
        .send_message(Message::user("Say hello"))
        .expect("Failed to send message");

    // Assert that we got a non-empty response
    let text = reply.text().expect("Failed to extract text from response");
    assert!(!text.is_empty(), "Response should not be empty");
    assert!(
        text.len() > 5,
        "Response should be more than 5 characters"
    );

    println!("Test passed. Response: {}", text);
}
