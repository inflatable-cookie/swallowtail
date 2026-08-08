use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    PreparationFailure, PreparationStage, RuntimeFailure,
    preparation_failure as shared_preparation_failure, probe_outcome_failure,
    probe_runtime_failure,
};

pub(super) fn discovery_runtime_failure(error: RuntimeFailure) -> PreparationFailure {
    probe_runtime_failure(&error, "swallowtail.oh_my_pi.discovery_axis_mismatch")
}

pub(super) fn discovery_outcome_failure(outcome: &DiscoveryOutcome) -> PreparationFailure {
    probe_outcome_failure(
        outcome,
        "swallowtail.oh_my_pi.preparation.discovery_rejected",
        "OhMyPi installed executable discovery was not promotable",
    )
}

pub(super) fn preparation_failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    shared_preparation_failure(stage, code, message)
}
