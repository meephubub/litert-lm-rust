//! Multimodal Conversation example (image + audio parts).
//!
//! Requires a multimodal `.litertlm` model and vision/audio backends.
//!
//! ```bash
//! cargo run --example multimodal -- model.litertlm photo.jpg [audio.wav]
//! ```

use litert_lm_rust::{Backend, ContentPart, Engine, Message};
use std::env;

fn main() -> litert_lm_rust::Result<()> {
    let mut args = env::args().skip(1);
    let model = args.next().expect("usage: multimodal <model> <image> [audio]");
    let image = args.next().expect("image path required");
    let audio = args.next();

    let engine = Engine::builder(&model)
        .backend(Backend::Cpu)
        .vision_backend(Backend::Gpu)
        .audio_backend(Backend::Cpu)
        .build()?;

    let mut parts = vec![
        ContentPart::text("Describe the following image:"),
        ContentPart::image_path(&image),
    ];
    if let Some(audio_path) = audio {
        parts.push(ContentPart::text("Also transcribe this audio:"));
        parts.push(ContentPart::audio_path(audio_path));
    }

    let mut conversation = engine.create_conversation(Default::default())?;
    let reply = conversation.send_message(Message::user_parts(parts)?)?;
    println!("{reply}");
    Ok(())
}
