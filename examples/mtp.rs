//! Multi-Token Prediction (MTP) example.
//!
//! MTP accelerates decode via speculative decoding — recommended on GPU.
//!
//! ```bash
//! cargo run --example mtp -- model.litertlm
//! ```

use litert_lm_rust::{Backend, Engine, Message};
use std::env;

fn main() -> litert_lm_rust::Result<()> {
    let model = env::args()
        .nth(1)
        .expect("usage: mtp <model.litertlm> [prompt]");
    let prompt = env::args()
        .nth(2)
        .unwrap_or_else(|| "Explain multi-token prediction briefly.".into());

    let engine = Engine::builder(&model)
        .backend(Backend::Gpu)
        .multi_token_prediction(true) // maps to enable_speculative_decoding
        .build()?;

    let mut conversation = engine.create_conversation(Default::default())?;
    let reply = conversation.send_message(Message::user(prompt))?;
    println!("{reply}");
    Ok(())
}
