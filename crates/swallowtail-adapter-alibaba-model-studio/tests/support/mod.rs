#![allow(dead_code)]

use serde_json::Value;
use swallowtail_adapter_alibaba_model_studio::{
    AlibabaProtocolFailure, ConversationRef, SseDecoder, SseFrame, parse_conversation,
};

#[allow(dead_code, unused_imports)]
mod driver;

#[allow(unused_imports)]
pub use driver::{DriverCall, DriverFixture, FixtureRequest, ServerScenario};

pub fn conversation() -> ConversationRef {
    parse_conversation(bytes("conversation-created.json")).expect("conversation fixture is valid")
}

pub fn frames_from_chunks(
    input: &[u8],
    size: usize,
) -> Result<Vec<SseFrame>, AlibabaProtocolFailure> {
    let mut decoder = SseDecoder::default();
    let mut frames = Vec::new();
    for chunk in input.chunks(size) {
        frames.extend(decoder.push(chunk)?);
    }
    decoder.finish()?;
    Ok(frames)
}

pub fn bytes(name: &str) -> &'static [u8] {
    match name {
        "conversation-created.json" => {
            include_bytes!("../fixtures/model-studio-2026-07-22/conversation-created.json")
        }
        "items.json" => include_bytes!("../fixtures/model-studio-2026-07-22/items.json"),
        "items-incomplete.json" => {
            include_bytes!("../fixtures/model-studio-2026-07-22/items-incomplete.json")
        }
        "delete-item.json" => {
            include_bytes!("../fixtures/model-studio-2026-07-22/delete-item.json")
        }
        "delete-conversation.json" => {
            include_bytes!("../fixtures/model-studio-2026-07-22/delete-conversation.json")
        }
        "headers.json" => include_bytes!("../fixtures/model-studio-2026-07-22/headers.json"),
        "provider-error.json" => {
            include_bytes!("../fixtures/model-studio-2026-07-22/provider-error.json")
        }
        "success.sse" => include_bytes!("../fixtures/model-studio-2026-07-22/success.sse"),
        "unknown.sse" => include_bytes!("../fixtures/model-studio-2026-07-22/unknown.sse"),
        "reasoning.sse" => include_bytes!("../fixtures/model-studio-2026-07-22/reasoning.sse"),
        "sequence-drift.sse" => {
            include_bytes!("../fixtures/model-studio-2026-07-22/sequence-drift.sse")
        }
        "disconnect.sse" => include_bytes!("../fixtures/model-studio-2026-07-22/disconnect.sse"),
        "cleanup-race.json" => {
            include_bytes!("../fixtures/model-studio-2026-07-22/cleanup-race.json")
        }
        _ => panic!("unknown fixture {name}"),
    }
}

pub fn json_fixture(name: &str) -> Value {
    if name == "protocol.json" {
        serde_json::from_slice(include_bytes!(
            "../fixtures/model-studio-2026-07-22/protocol.json"
        ))
        .expect("protocol fixture is valid")
    } else {
        serde_json::from_slice(bytes(name)).expect("JSON fixture is valid")
    }
}
