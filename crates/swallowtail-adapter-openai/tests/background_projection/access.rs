use crate::{fixture, server, services};

use fixture::Fixture;
use server::ServerMode;
use services::TimeMode;
use swallowtail_adapter_openai::{
    OPENAI_BACKGROUND_ACCESS_PROFILE_ID, OPENAI_BACKGROUND_ENDPOINT,
    OpenAiBackgroundPreparationInput, openai_background_access_profile,
    openai_background_descriptor, openai_background_facade_binding, openai_background_instance,
    openai_background_model_route, prepare_openai_background,
};
use swallowtail_core::{
    AccessProfileId, AccessRequirement, AccessStatus, CredentialState, DriverRole,
    EndpointAuthorization, EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership,
    InstanceRevision, InstanceTargetRef, ModelRouteRevision, OperationRequirements, OperationShape,
    PreflightContext, PreflightPlan, RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, CredentialRef, PreparationStage, PreparedAccessEvidence,
};

use super::fixtures::{contribution, minimal, profile_input};

/// Builds one observed access snapshot with every dimension named explicitly.
fn status(
    credential: CredentialState,
    entitlement: EntitlementState,
    endpoint: EndpointAuthorization,
    readiness: RuntimeReadiness,
    authority: SupportAuthority,
) -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new(OPENAI_BACKGROUND_ACCESS_PROFILE_ID).expect("profile id is valid"),
        credential,
        entitlement,
        endpoint,
        readiness,
        authority,
    )
}

/// Returns the four snapshots a drifted access observation can still form.
///
/// Support authority is fixed by the configured instance and access profile
/// together, so a drifted support authority cannot form a snapshot at all. It
/// is covered by the preparation counterexample instead.
fn drifted_snapshots() -> [AccessStatus; 4] {
    [
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
    ]
}

/// Returns every drifted observation, including the drifted support authority.
fn drifted_observations() -> Vec<AccessStatus> {
    let mut observations = drifted_snapshots().to_vec();
    observations.push(status(
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ExperimentalObserved,
    ));
    observations
}

/// Builds the exact background snapshot under one drifted access observation.
///
/// Every identity, route, model, and operation dimension matches the prepared
/// run, so only the named access dimension separates the two snapshots.
fn drifted_applicability(observed: &AccessStatus) -> ConsumerRouteApplicability {
    ConsumerRouteApplicability::from_plan(&drifted_plan(observed))
}

fn drifted_plan(observed: &AccessStatus) -> PreflightPlan {
    let descriptor = openai_background_descriptor();
    let access = openai_background_access_profile(projection_credential());
    let instance = openai_background_instance(
        InstanceRevision::new("prepared-1").expect("revision is valid"),
        ExecutionHostId::new("host.local").expect("host id is valid"),
        InstanceTargetRef::new(OPENAI_BACKGROUND_ENDPOINT).expect("target is valid"),
        access.id().clone(),
    );
    let route = openai_background_model_route(
        instance.id().clone(),
        ModelRouteRevision::new("projection-1").expect("route revision is valid"),
    );
    let host_services = descriptor
        .required_host_services(DriverRole::StructuredRun)
        .collect::<Vec<_>>();
    let requirements = OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        instance.execution_host_id().clone(),
        AccessRequirement::new(access.id().clone())
            .with_credential_states([observed.credential()])
            .with_entitlement_states([observed.entitlement()])
            .with_endpoint_authorizations([observed.endpoint_authorization()])
            .with_runtime_readiness([observed.runtime_readiness()])
            .with_support_authorities([observed.support_authority()]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(host_services.clone())
    .with_interface_versions([openai_background_facade_binding()])
    .require_model_route();
    let context = PreflightContext::new(&descriptor, &instance, &access, observed, host_services)
        .with_model_route(&route);
    preflight(&context, &requirements).expect("the drifted access snapshot is well formed")
}

/// Proves one changed access dimension rejects mixed background assembly.
///
/// The drifted snapshot agrees on instance, route, model, and operation shape,
/// so the rejection comes from the exact access dimension rather than a model,
/// revision, or aggregate availability summary.
#[test]
fn a_changed_access_dimension_rejects_mixed_background_assembly() {
    let admitted = contribution(&minimal(), "openai.background.access");
    let ready = admitted.applicability();
    let borrowed = admitted
        .selection_rows()
        .next()
        .expect("the prepared run publishes rows")
        .clone();
    for observed in drifted_snapshots() {
        let shifted = drifted_applicability(&observed);
        assert_eq!(shifted.instance_id(), ready.instance_id());
        assert_eq!(shifted.instance_revision(), ready.instance_revision());
        assert_eq!(shifted.instance_policy_id(), ready.instance_policy_id());
        assert_eq!(shifted.driver_identity(), ready.driver_identity());
        assert_eq!(shifted.protocol_facade_id(), ready.protocol_facade_id());
        assert_eq!(shifted.execution_host_id(), ready.execution_host_id());
        assert_eq!(shifted.driver_role(), ready.driver_role());
        assert_eq!(shifted.execution_layer(), ready.execution_layer());
        assert_eq!(shifted.operation_shape(), ready.operation_shape());
        assert_eq!(shifted.model(), ready.model());
        assert_eq!(shifted.access_profile_id(), ready.access_profile_id());
        assert_eq!(shifted.credential_mechanism(), ready.credential_mechanism());
        assert_eq!(shifted.resource_access(), ready.resource_access());
        assert_eq!(shifted.filesystem_boundary(), ready.filesystem_boundary());
        assert_ne!(&shifted, ready, "one exact access dimension separates them");

        let rejection = ConsumerRouteProjectionContribution::new(
            shifted,
            admitted.sources().cloned().collect::<Vec<_>>(),
            [borrowed.clone()],
            [],
            [],
        )
        .expect_err("a row proved under other access evidence cannot join this snapshot");
        assert_eq!(
            rejection.kind(),
            ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
        );
    }
}

/// Proves a drifted access dimension yields no background projection row.
///
/// The background route admits exactly one readiness posture, so drifted
/// access evidence fails closed before any contribution exists to mix.
#[test]
fn a_drifted_access_dimension_prepares_no_background_row() {
    for observed in drifted_observations() {
        let harness = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
        let failure = prepare_openai_background(drifted_input(observed), &harness.services())
            .and_then(|prepared| {
                prepared.prepare_background_run(profile_input("projection-background-drift"))
            })
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

/// Builds background preparation input under one drifted access observation.
fn drifted_input(observed: AccessStatus) -> OpenAiBackgroundPreparationInput {
    OpenAiBackgroundPreparationInput::new(
        InstanceRevision::new("prepared-1").expect("revision is valid"),
        ExecutionHostId::new("host.local").expect("host id is valid"),
        InstanceTargetRef::new(OPENAI_BACKGROUND_ENDPOINT).expect("target is valid"),
        openai_background_access_profile(projection_credential()),
        PreparedAccessEvidence::caller_asserted(observed),
    )
}

fn projection_credential() -> CredentialRef {
    CredentialRef::new("openai-projection-credential").expect("credential ref is valid")
}

/// Proves the admitted snapshot keeps all five dimensions separately visible.
#[test]
fn the_admitted_background_snapshot_keeps_each_access_dimension_observable() {
    let admitted = contribution(&minimal(), "openai.background.dimensions");
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
        applicability.operation_shape(),
        OperationShape::StructuredRun
    );
}
