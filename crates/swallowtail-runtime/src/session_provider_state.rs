use crate::{RuntimeFailure, SessionProviderStatePolicy};
use swallowtail_core::{PreflightPlan, SafeDiagnostic};

pub fn validate_session_provider_state_plan(
    plan: &PreflightPlan,
    requested: SessionProviderStatePolicy,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().session_provider_state_policy() == Some(requested) {
        Ok(())
    } else {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.session_provider_state.plan_mismatch",
            "Session provider-state policy does not match its immutable preflight plan",
        )))
    }
}
