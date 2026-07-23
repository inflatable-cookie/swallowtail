use super::{IncompatibleOperationPolicy, OperationPolicy};
use swallowtail_core::PreflightPlan;

/// Rejects requests that omit or change the preflight-bound harness configuration posture.
pub fn validate_harness_configuration_policy(
    plan: &PreflightPlan,
    policy: &OperationPolicy,
) -> Result<(), IncompatibleOperationPolicy> {
    if plan.requirements().harness_configuration_posture() == policy.harness_configuration_posture()
    {
        Ok(())
    } else {
        Err(IncompatibleOperationPolicy::harness_configuration_mismatch())
    }
}

impl IncompatibleOperationPolicy {
    fn harness_configuration_mismatch() -> Self {
        Self {
            diagnostic: swallowtail_core::SafeDiagnostic::new(
                "swallowtail.operation_policy_rejected",
                "Harness configuration does not match the preflight-bound posture",
            ),
        }
    }
}
