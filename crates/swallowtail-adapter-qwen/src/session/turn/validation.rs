use super::super::QwenSessionHandle;
use crate::validation::{failure, unsupported};
use swallowtail_runtime::{RuntimeFailure, TurnRequest};

pub(super) fn validate_turn(
    session: &QwenSessionHandle,
    request: &TurnRequest,
) -> Result<(), RuntimeFailure> {
    let state = session.state.lock().expect("Qwen session lock poisoned");
    if !state.usable {
        return Err(failure(
            "swallowtail.qwen.headless.session_unusable",
            "Qwen interactive session can no longer accept turns",
        ));
    }
    if state.completed_turns >= 24 {
        return Err(failure(
            "swallowtail.qwen.headless.turn_limit",
            "Qwen interactive session reached its bounded turn limit",
        ));
    }
    drop(state);
    if request.attachments().len() != 0 || request.structured_output().is_some() {
        return Err(unsupported("turn attachments or structured output"));
    }
    let deadline = request
        .deadline()
        .ok_or_else(|| unsupported("a turn without an explicit host deadline"))?;
    if session.services.time().expect("validated Qwen time").now() >= deadline.instant() {
        return Err(failure(
            "swallowtail.qwen.headless.deadline_elapsed",
            "Qwen turn deadline elapsed before provider work",
        ));
    }
    Ok(())
}
