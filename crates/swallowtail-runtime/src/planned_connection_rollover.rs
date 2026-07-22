use crate::RuntimeFailure;
use swallowtail_core::{PlannedConnectionRolloverPolicy, PreflightPlan, SafeDiagnostic};

pub fn validate_planned_connection_rollover_plan(
    plan: &PreflightPlan,
    requested: PlannedConnectionRolloverPolicy,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().planned_connection_rollover() == requested {
        Ok(())
    } else {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.planned_connection_rollover.plan_mismatch",
            "Planned connection-rollover policy does not match its immutable preflight plan",
        )))
    }
}
