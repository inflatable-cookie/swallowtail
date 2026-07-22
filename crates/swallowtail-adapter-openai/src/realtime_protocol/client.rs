use base64::Engine;
use serde_json::{Value, json};
use swallowtail_runtime::MediaChunk;

pub(crate) enum ClientEvent<'a> {
    SessionUpdate,
    InputAudioAppend {
        event_id: &'a str,
        chunk: &'a MediaChunk,
    },
    InputAudioCommit {
        event_id: &'a str,
    },
    ResponseCreate {
        event_id: &'a str,
    },
    ResponseCancel {
        event_id: &'a str,
        response_id: Option<&'a str>,
    },
}

impl ClientEvent<'_> {
    pub(crate) fn to_json(&self) -> Value {
        match self {
            Self::SessionUpdate => json!({
                "event_id": "session-config-1",
                "type": "session.update",
                "session": {
                    "type": "realtime",
                    "model": "gpt-realtime-2.1",
                    "output_modalities": ["audio"],
                    "audio": {
                        "input": {
                            "format": {"type": "audio/pcm", "rate": 24000},
                            "turn_detection": null
                        },
                        "output": {
                            "format": {"type": "audio/pcm", "rate": 24000},
                            "voice": "marin"
                        }
                    },
                    "tools": []
                }
            }),
            Self::InputAudioAppend { event_id, chunk } => json!({
                "event_id": event_id,
                "type": "input_audio_buffer.append",
                "audio": base64::engine::general_purpose::STANDARD.encode(chunk.bytes())
            }),
            Self::InputAudioCommit { event_id } => json!({
                "event_id": event_id,
                "type": "input_audio_buffer.commit"
            }),
            Self::ResponseCreate { event_id } => json!({
                "event_id": event_id,
                "type": "response.create"
            }),
            Self::ResponseCancel {
                event_id,
                response_id,
            } => {
                let mut event = json!({
                    "event_id": event_id,
                    "type": "response.cancel"
                });
                if let Some(response_id) = response_id {
                    event["response_id"] = json!(response_id);
                }
                event
            }
        }
    }
}
