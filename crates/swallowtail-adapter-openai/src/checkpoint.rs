use swallowtail_core::{PreflightPlan, RunRef};
use swallowtail_runtime::{ProviderRunCheckpoint, RuntimeFailure, RuntimeRunId};

const CURSOR_VERSION: u8 = 1;
const SELECTED_SERVICE_TIER_CURSOR_VERSION: u8 = 2;

pub(crate) fn checkpoint(
    plan: &PreflightPlan,
    runtime_run_id: RuntimeRunId,
    provider_run_ref: RunRef,
    sequence: u64,
    service_tier: Option<crate::OpenAiBackgroundServiceTier>,
) -> Result<ProviderRunCheckpoint, RuntimeFailure> {
    let mut cursor = Vec::with_capacity(9);
    cursor.push(if service_tier.is_some() {
        SELECTED_SERVICE_TIER_CURSOR_VERSION
    } else {
        CURSOR_VERSION
    });
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
    if cursor.len() != 9 {
        return Err(crate::failure::failure(
            "swallowtail.openai.run_checkpoint_invalid",
            "OpenAI response checkpoint cursor is invalid",
        ));
    }
    if cursor[0] == SELECTED_SERVICE_TIER_CURSOR_VERSION {
        return Err(crate::failure::failure(
            "swallowtail.openai.run_checkpoint_service_tier_unsupported",
            "OpenAI background service-tier selection cannot be reconciled from a checkpoint",
        ));
    }
    if cursor[0] != CURSOR_VERSION {
        return Err(crate::failure::failure(
            "swallowtail.openai.run_checkpoint_invalid",
            "OpenAI response checkpoint cursor is invalid",
        ));
    }
    let mut sequence = [0_u8; 8];
    sequence.copy_from_slice(&cursor[1..]);
    Ok(u64::from_be_bytes(sequence))
}
