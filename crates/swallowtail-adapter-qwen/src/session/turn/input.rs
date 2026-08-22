use super::super::QwenSessionHandle;
use crate::control::establish_reasoning_and_write_user;
use crate::driver::write_prompt;
use swallowtail_runtime::{Deadline, ProcessHandle, RuntimeFailure, TurnRequest};

pub(super) async fn write_turn_input(
    session: &QwenSessionHandle,
    process: &dyn ProcessHandle,
    request: &TurnRequest,
    deadline: Deadline,
) -> Result<Vec<serde_json::Value>, RuntimeFailure> {
    if let Some(reasoning) = session.reasoning.as_ref() {
        establish_reasoning_and_write_user(
            process,
            reasoning,
            session
                .services
                .time()
                .expect("validated Qwen time")
                .wait_until(deadline),
            request.content(),
        )
        .await
    } else {
        write_prompt(process, request.content()).await?;
        Ok(Vec::new())
    }
}
