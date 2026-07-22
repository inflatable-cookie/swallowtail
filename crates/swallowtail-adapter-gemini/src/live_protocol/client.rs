use super::handle::ProviderSessionHandle;
use crate::live::MODEL_RESOURCE;
use base64::Engine;
use serde_json::{Value, json};

pub(crate) enum ClientFrame<'a> {
    Setup {
        handle: Option<&'a ProviderSessionHandle>,
    },
    ActivityStart,
    Audio(&'a [u8]),
    ActivityEnd,
}

impl ClientFrame<'_> {
    pub(crate) fn to_json(&self) -> Value {
        match self {
            Self::Setup { handle } => {
                let session_resumption =
                    handle.map_or_else(|| json!({}), |handle| json!({"handle": handle.expose()}));
                json!({
                    "setup": {
                        "model": MODEL_RESOURCE,
                        "generationConfig": {
                            "responseModalities": ["AUDIO"],
                            "speechConfig": {
                                "voiceConfig": {
                                    "prebuiltVoiceConfig": {"voiceName": "Kore"}
                                }
                            },
                            "thinkingConfig": {"thinkingLevel": "MINIMAL"}
                        },
                        "realtimeInputConfig": {
                            "automaticActivityDetection": {"disabled": true},
                            "activityHandling": "NO_INTERRUPTION"
                        },
                        "sessionResumption": session_resumption,
                        "outputAudioTranscription": {}
                    }
                })
            }
            Self::ActivityStart => json!({"realtimeInput": {"activityStart": {}}}),
            Self::Audio(bytes) => json!({
                "realtimeInput": {
                    "audio": {
                        "data": base64::engine::general_purpose::STANDARD.encode(bytes),
                        "mimeType": "audio/pcm;rate=16000"
                    }
                }
            }),
            Self::ActivityEnd => json!({"realtimeInput": {"activityEnd": {}}}),
        }
    }
}
