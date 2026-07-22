use super::RealtimeMediaPreflightCase;
use std::num::NonZeroU32;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, PlannedConnectionRolloverPolicy,
};

pub(super) fn advertised_capability(
    case: RealtimeMediaPreflightCase,
    route: bool,
) -> Option<CapabilityRequirement> {
    if !is_case(case) || (!route && case == RealtimeMediaPreflightCase::RolloverInstanceMissing) {
        return None;
    }
    let bound = if route && case == RealtimeMediaPreflightCase::RolloverRouteWrongBound {
        2
    } else {
        1
    };
    Some(capability(bound))
}

pub(super) fn required_capability(
    case: RealtimeMediaPreflightCase,
) -> Option<CapabilityRequirement> {
    match case {
        RealtimeMediaPreflightCase::RolloverCapabilityWhileDisabled
        | RealtimeMediaPreflightCase::RolloverCanonical
        | RealtimeMediaPreflightCase::RolloverInstanceMissing
        | RealtimeMediaPreflightCase::RolloverRouteWrongBound => Some(capability(1)),
        RealtimeMediaPreflightCase::RolloverMismatchedBound => Some(capability(2)),
        RealtimeMediaPreflightCase::RolloverZeroBound => Some(capability(0)),
        _ => None,
    }
}

pub(super) fn policy(case: RealtimeMediaPreflightCase) -> PlannedConnectionRolloverPolicy {
    if matches!(
        case,
        RealtimeMediaPreflightCase::RolloverCanonical
            | RealtimeMediaPreflightCase::RolloverMissingCapability
            | RealtimeMediaPreflightCase::RolloverMismatchedBound
            | RealtimeMediaPreflightCase::RolloverZeroBound
            | RealtimeMediaPreflightCase::RolloverInstanceMissing
            | RealtimeMediaPreflightCase::RolloverRouteWrongBound
    ) {
        PlannedConnectionRolloverPolicy::Bounded(NonZeroU32::new(1).unwrap())
    } else {
        PlannedConnectionRolloverPolicy::Disabled
    }
}

fn capability(bound: u32) -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::PlannedConnectionRollover,
        [CapabilityConstraint::PlannedConnectionRolloverMaximumCount(
            bound,
        )],
    )
}

fn is_case(case: RealtimeMediaPreflightCase) -> bool {
    matches!(
        case,
        RealtimeMediaPreflightCase::RolloverCanonical
            | RealtimeMediaPreflightCase::RolloverCapabilityWhileDisabled
            | RealtimeMediaPreflightCase::RolloverMissingCapability
            | RealtimeMediaPreflightCase::RolloverMismatchedBound
            | RealtimeMediaPreflightCase::RolloverZeroBound
            | RealtimeMediaPreflightCase::RolloverInstanceMissing
            | RealtimeMediaPreflightCase::RolloverRouteWrongBound
    )
}
