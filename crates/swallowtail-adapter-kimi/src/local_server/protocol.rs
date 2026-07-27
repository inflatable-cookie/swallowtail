// Card 061 froze the full REST/WS corpus. Card 062 consumes only the lifecycle
// subset; card 064 will activate the remaining interactive decoder paths.
#[allow(dead_code)]
mod common;
#[allow(dead_code)]
mod rest;
#[allow(dead_code)]
mod ws;

pub(crate) use rest::{
    InteractiveSessionRecord, MAX_HTTP_BODY_BYTES, PendingProviderRequest, PromptStatus,
    RestFailureKind, RestReply, decode_archive, decode_callback_resolution, decode_health,
    decode_interactive_session, decode_metadata, decode_pending_approvals,
    decode_pending_questions, decode_prompt_submission, decode_question_dismissal, decode_rest,
    decode_session,
};
pub(crate) use ws::{TurnEndReason, WsEvent, WsEventEnvelope, WsFrame, decode_ws_frame};

#[cfg(test)]
#[path = "protocol/tests.rs"]
mod tests;
