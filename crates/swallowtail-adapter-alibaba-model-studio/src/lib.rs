//! Alibaba Model Studio Conversations and Responses direct-session driver.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod protocol;
mod selection;
mod transport;

pub use driver::AlibabaModelStudioDriver;
pub use failure::AlibabaProtocolFailure;
pub use protocol::{
    ConversationInventory, ConversationRef, DeletionConfirmation, DeletionKind, ItemRef, Method,
    ProviderEvent, ResponseRef, ResponseStream, SseDecoder, SseFrame, TurnOptions, WireRequest,
    parse_conversation, parse_deletion, parse_inventory, parse_provider_failure,
    parse_request_correlation,
};
pub use selection::{
    ACCESS_PROFILE_ID, ENDPOINT_AUDIENCE, EVIDENCE_DATE, EXACT_MODEL_ID, REGION,
    WORKSPACE_ENDPOINT_TEMPLATE, alibaba_model_studio_access_profile,
    alibaba_model_studio_descriptor, alibaba_model_studio_instance,
    alibaba_model_studio_requirements, alibaba_model_studio_route,
    validate_alibaba_model_studio_plan,
};
