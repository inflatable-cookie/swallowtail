use swallowtail_core::{PreflightPlan, RunRef};
use swallowtail_runtime::{ProviderRunCheckpoint, RuntimeFailure, RuntimeRunId};

const CURSOR_VERSION: u8 = 1;

pub(crate) fn checkpoint(
    plan: &PreflightPlan,
    runtime_run_id: RuntimeRunId,
    provider_run_ref: RunRef,
    sequence: u64,
) -> Result<ProviderRunCheckpoint, RuntimeFailure> {
    let mut cursor = Vec::with_capacity(9);
    cursor.push(CURSOR_VERSION);
    cursor.extend_from_slice(&sequence.to_be_bytes());
    ProviderRunCheckpoint::new(plan, runtime_run_id, provider_run_ref, cursor).map_err(|_| {
        crate::failure::failure(
            "swallowtail.openai.run_checkpoint_invalid",
            "OpenAI response checkpoint could not be represented",
        )
    })
}

pub(crate) fn decode_cursor(checkpoint: &ProviderRunCheckpoint) -> Result<u64, RuntimeFailure> {
    let cursor = checkpoint.cursor();
    if cursor.len() != 9 || cursor[0] != CURSOR_VERSION {
        return Err(crate::failure::failure(
            "swallowtail.openai.run_checkpoint_invalid",
            "OpenAI response checkpoint cursor is invalid",
        ));
    }
    let mut sequence = [0_u8; 8];
    sequence.copy_from_slice(&cursor[1..]);
    Ok(u64::from_be_bytes(sequence))
}
