// Disposition (recorded, card 164): the full REST/WS corpus stays frozen as
// evidence from card 061. The interactive lifecycle subset is consumed
// (callbacks, frame cursor, handle records, turn-end reasons); the remaining
// decoder paths stay `#[allow(dead_code)]` for a future interactive
// activation card, which is a separate qualification, not a facade change.
#[allow(dead_code)]
mod common;
#[allow(dead_code)]
mod rest;
#[allow(dead_code)]
mod ws;

pub(crate) use rest::{
    decode_archive, decode_callback_resolution, decode_health, decode_interactive_session,
    decode_metadata, decode_pending_approvals, decode_pending_questions, decode_prompt_submission,
    decode_question_dismissal, decode_rest, decode_session, InteractiveSessionRecord,
    PendingProviderRequest, PromptStatus, RestFailureKind, RestReply, MAX_HTTP_BODY_BYTES,
};
pub(crate) use ws::{
    decode_ws_frame, encode_pong, TurnEndReason, WsEvent, WsEventEnvelope, WsFrame,
};

#[cfg(test)]
#[path = "protocol/tests.rs"]
mod tests;
