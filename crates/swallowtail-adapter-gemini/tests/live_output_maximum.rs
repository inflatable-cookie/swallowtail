use crate::live_support;

use futures_executor::block_on;
use live_support::{Call, LiveFixture, LiveScenario, TimeMode, config, rollover_policy};
use std::num::NonZeroU64;
use swallowtail_adapter_gemini::{
    GEMINI_LIVE_FACADE_REVISION, GEMINI_LIVE_MAX_OUTPUT_TOKENS,
    GEMINI_LIVE_OUTPUT_MAXIMUM_SUPERSEDED_FACADE_REVISION, GEMINI_LIVE_SUPERSEDED_FACADE_REVISION,
    GEMINI_LIVE_THINKING_SUPERSEDED_FACADE_REVISION, GeminiLiveDriver,
    GeminiLiveSessionProfileInput, prepare_gemini_live,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, PreflightPlan, ReasoningMode,
};
use swallowtail_runtime::{OpenRealtimeMediaSessionRequest, RealtimeMediaSessionDriver, RequestId};

include!("live_output_maximum/support.rs");
include!("live_output_maximum/preparation.rs");
include!("live_output_maximum/rejections.rs");
include!("live_output_maximum/facades.rs");
