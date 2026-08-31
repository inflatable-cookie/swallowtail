use super::super::*;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, CredentialMechanism, CredentialState,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
};

use super::fixtures::{contribution, minimal};
use super::harness::exec_run_with_access;
use super::naming::identities;

/// Builds the fixture Codex access profile under one support authority.
fn profile(authority: SupportAuthority) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("access.codex").unwrap(),
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(swallowtail_adapter_codex::CODEX_CHATGPT_SUBSCRIPTION_AUDIENCE)
            .unwrap(),
        authority,
    )
}

/// Builds one observed access snapshot with every dimension named explicitly.
fn status(
    credential: CredentialState,
    entitlement: EntitlementState,
    endpoint: EndpointAuthorization,
    readiness: RuntimeReadiness,
    authority: SupportAuthority,
) -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("access.codex").unwrap(),
        credential,
        entitlement,
        endpoint,
        readiness,
        authority,
    )
}

/// Prepares one minimal exec run under the supplied exact access evidence.
fn run_under(authority: SupportAuthority) -> swallowtail_adapter_codex::CodexPreparedExec {
    exec_run_with_access(
        "projection-exec-access",
        profile(authority),
        status(
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            authority,
        ),
    )
    .expect("an exec run prepares under a supported access posture")
}

/// Proves one changed access dimension rejects mixed exec assembly.
///
/// The two runs agree on instance, route, model, operation shape, and the four
/// readiness dimensions, and both publish the same rows at the same
/// availability and support posture. Only the exact support authority differs,
/// so the rejection cannot come from a model, revision, or aggregate summary.
#[test]
fn a_changed_access_dimension_rejects_mixed_exec_assembly() {
    let supported = contribution(
        &run_under(SupportAuthority::ProviderSupported),
        "codex.exec.access",
    );
    let maintained = contribution(
        &run_under(SupportAuthority::IntegrationMaintainerSupported),
        "codex.exec.access",
    );
    assert_eq!(
        identities(&supported),
        identities(&maintained),
        "both access postures publish the same exact census identities"
    );

    let ready = supported.applicability();
    let shifted = maintained.applicability();
    assert_ne!(
        ready.support_authority(),
        shifted.support_authority(),
        "the counterexample changes the support-authority dimension"
    );
    assert_eq!(ready.credential_state(), shifted.credential_state());
    assert_eq!(ready.entitlement_state(), shifted.entitlement_state());
    assert_eq!(
        ready.endpoint_authorization(),
        shifted.endpoint_authorization()
    );
    assert_eq!(ready.runtime_readiness(), shifted.runtime_readiness());
    assert_eq!(ready.credential_mechanism(), shifted.credential_mechanism());
    assert_eq!(ready.access_profile_id(), shifted.access_profile_id());
    assert_eq!(ready.instance_id(), shifted.instance_id());
    assert_eq!(ready.instance_revision(), shifted.instance_revision());
    assert_eq!(ready.instance_policy_id(), shifted.instance_policy_id());
    assert_eq!(ready.driver_identity(), shifted.driver_identity());
    assert_eq!(ready.protocol_facade_id(), shifted.protocol_facade_id());
    assert_eq!(ready.execution_host_id(), shifted.execution_host_id());
    assert_eq!(ready.driver_role(), shifted.driver_role());
    assert_eq!(ready.execution_layer(), shifted.execution_layer());
    assert_eq!(ready.operation_shape(), shifted.operation_shape());
    assert_eq!(ready.model(), shifted.model());
    assert_eq!(ready.resource_access(), shifted.resource_access());
    assert_eq!(ready.filesystem_boundary(), shifted.filesystem_boundary());
    assert_ne!(ready, shifted, "one exact access dimension separates them");

    for (admitted, borrowed) in supported.selection_rows().zip(maintained.selection_rows()) {
        assert_eq!(
            admitted.availability(),
            borrowed.availability(),
            "the aggregate availability summary does not move with the dimension"
        );
        assert_eq!(admitted.support(), borrowed.support());
    }

    let borrowed = maintained
        .selection_rows()
        .next()
        .expect("the shifted run publishes rows")
        .clone();
    let rejection = ConsumerRouteProjectionContribution::new(
        ready.clone(),
        supported.sources().cloned().collect::<Vec<_>>(),
        [borrowed],
        [],
        [],
    )
    .expect_err("a row proved under other access evidence cannot join this snapshot");
    assert_eq!(
        supported.sources().next().map(|source| source.id()),
        maintained.sources().next().map(|source| source.id()),
        "both snapshots name the same source, so only the access dimension differs"
    );
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

/// Proves a drifted access dimension yields no exec projection row at all.
///
/// Exec preparation admits exactly one readiness posture, so drifted access
/// evidence fails closed before any contribution exists to mix.
#[test]
fn a_drifted_access_dimension_prepares_no_exec_row() {
    let drifted = [
        status(
            CredentialState::Expired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ),
        status(
            CredentialState::Ready,
            EntitlementState::Exhausted,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ),
        status(
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Denied,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ),
        status(
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Degraded,
            SupportAuthority::ProviderSupported,
        ),
        status(
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ExperimentalObserved,
        ),
    ];
    for observed in drifted {
        let failure = exec_run_with_access(
            "projection-exec-drift",
            profile(SupportAuthority::ProviderSupported),
            observed,
        )
        .expect_err("drifted access evidence fails closed before a row exists");
        assert!(
            matches!(
                failure.stage(),
                PreparationStage::AccessEvidence | PreparationStage::Preflight
            ),
            "drifted access evidence is rejected at the access or preflight stage, not later"
        );
    }
}

/// Proves the admitted snapshot keeps all five dimensions separately visible.
#[test]
fn the_admitted_exec_snapshot_keeps_each_access_dimension_observable() {
    let admitted = contribution(&minimal(), "codex.exec.dimensions");
    let applicability = admitted.applicability();
    assert_eq!(applicability.credential_state(), CredentialState::Ready);
    assert_eq!(
        applicability.entitlement_state(),
        EntitlementState::Available
    );
    assert_eq!(
        applicability.endpoint_authorization(),
        EndpointAuthorization::Allowed
    );
    assert_eq!(applicability.runtime_readiness(), RuntimeReadiness::Ready);
    assert_eq!(
        applicability.support_authority(),
        SupportAuthority::ProviderSupported
    );
    assert_eq!(
        applicability.execution_host_id(),
        &ExecutionHostId::new("host.local").unwrap()
    );
}
