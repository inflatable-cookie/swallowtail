use crate::live_support;

use futures_executor::block_on;
use live_support::{Call, LiveFixture, LiveScenario, TimeMode, complete, start_turn};
use serde_json::{Value, json};
use std::num::NonZeroU64;
use swallowtail_adapter_gemini::{
    GEMINI_LIVE_FACADE_REVISION, GeminiLiveContextWindowCompression, GeminiLiveSessionProfileInput,
    prepare_gemini_live,
};
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    CleanupOutcome, RequestId, RuntimeTurnId, TerminalStatus, WorkingStateRestorationOutcome,
};

include!("live_context_compression/support.rs");
include!("live_context_compression/rollover.rs");
include!("live_context_compression/restoration.rs");
include!("live_context_compression/composition.rs");
