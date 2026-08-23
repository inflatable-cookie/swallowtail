use super::handle::ProviderSessionHandle;
use crate::GeminiLiveContextWindowCompression;
use crate::live::MODEL_RESOURCE;
use base64::Engine;
use serde_json::{Value, json};
use std::num::NonZeroU64;

pub(crate) enum ClientFrame<'a> {
    Setup {
        handle: Option<&'a ProviderSessionHandle>,
        thinking_level: &'a str,
        maximum_output_tokens: Option<NonZeroU64>,
        context_window_compression: Option<GeminiLiveContextWindowCompression>,
    },
    ActivityStart,
    Audio(&'a [u8]),
    ActivityEnd,
}

impl ClientFrame<'_> {
    pub(crate) fn to_json(&self) -> Value {
        match self {
            Self::Setup {
                handle,
                thinking_level,
                maximum_output_tokens,
                context_window_compression,
            } => {
                let session_resumption =
                    handle.map_or_else(|| json!({}), |handle| json!({"handle": handle.expose()}));
                let mut generation_config = json!({
                    "responseModalities": ["AUDIO"],
                    "speechConfig": {
                        "voiceConfig": {
                            "prebuiltVoiceConfig": {"voiceName": "Kore"}
                        }
                    },
                    "thinkingConfig": {"thinkingLevel": thinking_level}
                });
                if let Some(maximum) = maximum_output_tokens {
                    generation_config["maxOutputTokens"] = json!(maximum.get());
                }
                let mut setup = json!({
                    "setup": {
                        "model": MODEL_RESOURCE,
                        "generationConfig": generation_config,
                        "realtimeInputConfig": {
                            "automaticActivityDetection": {"disabled": true},
                            "activityHandling": "NO_INTERRUPTION"
                        },
                        "sessionResumption": session_resumption,
                        "outputAudioTranscription": {}
                    }
                });
                if context_window_compression.is_some() {
                    setup["setup"]["contextWindowCompression"] = json!({"slidingWindow": {}});
                }
                setup
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
