use swallowtail_core::{
    AccessStatus, CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow,
};

pub(super) fn status(
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

pub(super) fn assert_one_access_difference(ready: &AccessStatus, shifted: &AccessStatus) {
    let differences = [
        ready.credential() != shifted.credential(),
        ready.entitlement() != shifted.entitlement(),
        ready.endpoint_authorization() != shifted.endpoint_authorization(),
        ready.runtime_readiness() != shifted.runtime_readiness(),
        ready.support_authority() != shifted.support_authority(),
    ];
    assert_eq!(differences.into_iter().filter(|value| *value).count(), 1);
}

pub(super) fn assert_shared_dimensions(
    left: &ConsumerRouteProjectionContribution,
    right: &ConsumerRouteProjectionContribution,
) {
    assert_shared_applicability(left.applicability(), right.applicability());
}

pub(super) fn assert_shared_applicability(
    left: &ConsumerRouteApplicability,
    right: &ConsumerRouteApplicability,
) {
    assert_eq!(left.operation_shape(), right.operation_shape());
    assert_eq!(left.access_profile_id(), right.access_profile_id());
    assert_eq!(left.credential_state(), right.credential_state());
    assert_eq!(left.entitlement_state(), right.entitlement_state());
    assert_eq!(
        left.endpoint_authorization(),
        right.endpoint_authorization()
    );
    assert_eq!(left.runtime_readiness(), right.runtime_readiness());
    assert_eq!(left.support_authority(), right.support_authority());
}

pub(super) fn assert_rejects(
    target: &ConsumerRouteProjectionContribution,
    row: ConsumerRouteProjectionRow,
) {
    assert_rejects_applicability(target.applicability().clone(), target, row);
}

pub(super) fn assert_rejects_applicability(
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

pub(super) fn first_row(
    contribution: &ConsumerRouteProjectionContribution,
) -> &ConsumerRouteProjectionRow {
    super::rows(contribution)
        .next()
        .expect("contribution has a row")
}
