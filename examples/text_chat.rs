//! Text-only Conversation example.
//!
//! ```bash
//! export LITERT_LM_LIB_DIR=/path/to/native
//! cargo run --example text_chat -- /path/to/model.litertlm "Hello!"
//! ```

use litert_lm_rust::{Backend, Engine, Message};
use std::env;

fn main() -> litert_lm_rust::Result<()> {
    let mut args = env::args().skip(1);
    let model = args
        .next()
        .expect("usage: text_chat <model.litertlm> [prompt]");
    let prompt = args
        .next()
        .unwrap_or_else(|| "What is the tallest building in the world?".into());

    let engine = Engine::builder(&model).backend(Backend::Cpu).build()?;
    let mut conversation = engine.create_conversation(Default::default())?;
    let reply = conversation.send_message(Message::user(prompt))?;
    println!("{reply}");
    Ok(())
}
