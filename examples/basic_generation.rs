//! Basic text generation example using the `litert-lm-rust` crate.
//!
//! # Usage
//!
//! ```powershell
//! # From the litert-lm-rust workspace root:
//! cargo run --example basic_generation -- gemma-4-E2B-it.litertlm "Explain gravity in one sentence."
//! ```
//!
//! The DLLs in `prebuilt/` must be on `PATH` or copied next to the executable:
//!   litert-lm.dll  libLiteRt.dll  libGemmaModelConstraintProvider.dll
//!   libLiteRtTopKWebGpuSampler.dll  libLiteRtWebGpuAccelerator.dll  libwebgpu_dawn.dll
//!
//! # Steps demonstrated
//!   1. Create engine settings from a model path + backend
//!   2. Build the Engine (loads model weights)
//!   3. Create a Conversation (manages session + prompt template)
//!   4. Send a prompt (blocking)
//!   5. Print the generated response

use litert_lm_rust::{Backend, Engine, LogSeverity, Message};
use std::env;

fn main() -> litert_lm_rust::Result<()> {
    // ── 1. Parse CLI arguments ────────────────────────────────────────────────
    let mut args = env::args().skip(1);
    let model_path = args
        .next()
        .unwrap_or_else(|| "gemma-4-E2B-it.litertlm".to_string());
    let prompt = args
        .next()
        .unwrap_or_else(|| "What is the capital of France?".to_string());

    // Silence verbose native logs; show warnings and above.
    litert_lm_rust::set_min_log_level(LogSeverity::Warning);

    println!("Model  : {model_path}");
    println!("Prompt : {prompt}");
    println!("─────────────────────────────────────────────");

    // ── 2. Build the Engine (loads model weights) ─────────────────────────────
    // Uses the CPU backend. Swap to Backend::Gpu for GPU acceleration.
    let engine = Engine::builder(&model_path)
        .backend(Backend::Cpu)
        .build()
        .map_err(|e| {
            eprintln!(
                "ERROR: failed to load model '{model_path}': {e}\n\
                 Make sure the .litertlm file exists and the DLLs are on PATH."
            );
            e
        })?;

    // ── 3. Create a Conversation ──────────────────────────────────────────────
    // Default config applies the model's built-in prompt template.
    let mut conversation = engine.create_conversation(Default::default())?;

    // ── 4. Send the prompt (blocking) ─────────────────────────────────────────
    let reply = conversation.send_message(Message::user(&prompt))?;

    // ── 5. Print response ─────────────────────────────────────────────────────
    println!("{reply}");

    Ok(())
}
