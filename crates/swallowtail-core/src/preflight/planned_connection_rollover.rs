use super::{PreflightDimension, PreflightFailure};
use crate::{
    Capability, CapabilityConstraint, DriverRole, OperationRequirements,
    PlannedConnectionRolloverPolicy,
};

pub(super) fn validate_planned_connection_rollover(
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let declarations: Vec<_> = requirements
        .capabilities()
        .filter(|requirement| requirement.capability() == Capability::PlannedConnectionRollover)
        .collect();

    match requirements.planned_connection_rollover() {
        PlannedConnectionRolloverPolicy::Disabled => {
            if declarations.is_empty() {
                Ok(())
            } else {
                Err(failure(
                    "Disabled planned rollover cannot require its capability",
                ))
            }
        }
        PlannedConnectionRolloverPolicy::Bounded(maximum) => {
            if requirements.driver_role() != DriverRole::RealtimeMediaSession {
                return Err(failure(
                    "Planned connection rollover is only valid for realtime-media sessions",
                ));
            }
            if declarations.len() != 1 {
                return Err(failure(
                    "Planned rollover requires exactly one capability declaration",
                ));
            }
            let bounds: Vec<_> = declarations[0]
                .constraints()
                .filter_map(|constraint| match constraint {
                    CapabilityConstraint::PlannedConnectionRolloverMaximumCount(value) => {
                        Some(*value)
                    }
                    _ => None,
                })
                .collect();
            if bounds.as_slice() != [maximum.get()] {
                return Err(failure(
                    "Planned rollover capability must declare the exact positive policy bound",
                ));
            }
            Ok(())
        }
    }
}

fn failure(message: &'static str) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::PlannedConnectionRollover, message)
}
