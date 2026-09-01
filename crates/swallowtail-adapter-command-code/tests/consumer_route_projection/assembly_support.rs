use swallowtail_core::{
    AccessStatus, CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow,
};

pub(super) fn access(
    base: &AccessStatus,
    credential: Option<CredentialState>,
    entitlement: Option<EntitlementState>,
    endpoint: Option<EndpointAuthorization>,
    readiness: Option<RuntimeReadiness>,
    support: Option<SupportAuthority>,
) -> AccessStatus {
    AccessStatus::new(
        base.profile_id().clone(),
        credential.unwrap_or(base.credential()),
        entitlement.unwrap_or(base.entitlement()),
        endpoint.unwrap_or(base.endpoint_authorization()),
        readiness.unwrap_or(base.runtime_readiness()),
        support.unwrap_or(base.support_authority()),
    )
}

pub(super) fn assert_one_difference(ready: &AccessStatus, shifted: &AccessStatus) {
    let differences = [
        ready.credential() != shifted.credential(),
        ready.entitlement() != shifted.entitlement(),
        ready.endpoint_authorization() != shifted.endpoint_authorization(),
        ready.runtime_readiness() != shifted.runtime_readiness(),
        ready.support_authority() != shifted.support_authority(),
    ];
    assert_eq!(differences.into_iter().filter(|value| *value).count(), 1);
}

pub(super) fn assert_same_route_access(
    left: &ConsumerRouteProjectionContribution,
    right: &ConsumerRouteProjectionContribution,
) {
    assert_eq!(
        left.applicability().driver_identity(),
        right.applicability().driver_identity()
    );
    assert_eq!(
        left.applicability().operation_shape(),
        right.applicability().operation_shape()
    );
    assert_eq!(left.applicability().model(), right.applicability().model());
    assert_eq!(
        left.applicability().credential_state(),
        right.applicability().credential_state()
    );
    assert_eq!(
        left.applicability().support_authority(),
        right.applicability().support_authority()
    );
}

pub(super) fn reject(
    applicability: ConsumerRouteApplicability,
    source: &ConsumerRouteProjectionContribution,
    row: ConsumerRouteProjectionRow,
) {
    let failure = ConsumerRouteProjectionContribution::new(
        applicability,
        source.sources().cloned().collect::<Vec<_>>(),
        [row],
        [],
        [],
    )
    .expect_err("mixed applicability fails closed");
    assert_eq!(
        failure.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

pub(super) fn rebind(
    row: &ConsumerRouteProjectionRow,
    applicability: ConsumerRouteApplicability,
) -> ConsumerRouteProjectionRow {
    ConsumerRouteProjectionRow::new(
        row.identity().clone(),
        applicability,
        row.source().clone(),
        row.source_class(),
        row.evidence_strength(),
        row.lifecycle(),
    )
    .with_support(row.support())
    .with_availability(row.availability())
    .with_actor_posture(row.actor_posture())
    .with_state_support(row.state_support())
}

pub(super) fn first(
    contribution: &ConsumerRouteProjectionContribution,
) -> &ConsumerRouteProjectionRow {
    super::rows(contribution)
        .next()
        .expect("contribution has a row")
}
