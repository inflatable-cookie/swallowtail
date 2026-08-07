use crate::failure::failure;
use swallowtail_core::Diagnostic;
use swallowtail_runtime::{CleanupOutcome, PreparationFailure, PreparationStage, RuntimeFailure};

pub(super) fn preparation_failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}

pub(super) fn checkpoint_required() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_checkpoint_required",
        "Kimi local-server reconciliation requires an exact operation checkpoint",
    )
}

pub(super) fn stale_checkpoint() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_checkpoint_stale",
        "Kimi local-server reconciliation checkpoint is stale or discontinuous",
    )
}

pub(super) fn binding_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_binding_mismatch",
        "Kimi local-server reconciliation observed a different provider operation",
    )
}

pub(super) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_protocol_failed",
        "Kimi local-server reconciliation protocol failed",
    )
}

pub(super) fn cancelled() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_cancelled",
        "Kimi local-server reconciliation was cancelled",
    )
}

pub(super) fn timed_out() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_timed_out",
        "Kimi local-server reconciliation timed out",
    )
}

pub(super) fn cleanup_failure(cleanup: CleanupOutcome) -> RuntimeFailure {
    match cleanup {
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            RuntimeFailure::new(diagnostic)
        }
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => protocol_failure(),
    }
}
