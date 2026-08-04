use super::{
    PreparedProviderRecoveredResourceCleanupEvidence, ProviderRecoveredResourceCleanupAgreement,
    ProviderRecoveredResourceCleanupOutcome, ProviderRecoveredResourceCleanupPlan,
    ProviderRecoveredResourceCleanupRequest,
};
use crate::{
    AccessEvidenceSourceId, Deadline, ImmediateCancellation, InterruptedRunState, MonotonicInstant,
    PersistedProviderRecoveredResourceCleanupBinding, PreparedAccessEvidence,
    ProviderRecoveredResourceCleanupBinding, ProviderRecoveredResourceCleanupBindingFailureKind,
    RequestId, RuntimeRunId,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, CancellationScope, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer, HostServiceKind,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    OwnedRemoteResourceKind, PreflightContext, ProtocolFacadeId,
    ProviderRecoveredResourceCleanupEffect, RunRef, RuntimeReadiness, SupportAuthority,
    TransportFamilyId, preflight,
};

struct Fixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    capability: CapabilityRequirement,
}

fn fixture(route_revision: &str) -> Fixture {
    let capability = CapabilityRequirement::new(
        Capability::ProviderRecoveredResourceCleanup,
        [
            CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Environment),
            CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Session),
        ],
    );
    let profile = CapabilityProfile::new([capability.clone()]);
    let driver = DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("fixture.driver").expect("driver id is valid"),
            AdapterVersion::new("1.0.0").expect("driver version is valid"),
        ),
        IntegrationFamilyId::new("fixture").expect("family is valid"),
        TransportFamilyId::new("fixture-rpc").expect("transport is valid"),
    )
    .with_roles([DriverRole::ProviderRecoveredResourceCleanup])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::ProviderRecoveredResourceCleanup]);
    let access_id = AccessProfileId::new("fixture.access").expect("access id is valid");
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid"),
        InstanceRevision::new("instance-revision").expect("instance revision is valid"),
        AdapterId::new("fixture.driver").expect("driver id is valid"),
        ExecutionHostId::new("fixture.host").expect("host id is valid"),
        InstanceTargetRef::new("private/target").expect("target is valid"),
        InstanceOwnership::ExternalAttached,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("fixture.facade").expect("facade id is valid"),
        InstancePolicyId::new("fixture.policy").expect("policy id is valid"),
        profile.clone(),
    );
    let route = ModelRoute::new(
        ModelRouteId::new("fixture.route").expect("route id is valid"),
        ModelRouteRevision::new(route_revision).expect("route revision is valid"),
        instance.id().clone(),
        ModelId::new("fixture-model").expect("model id is valid"),
        profile,
    );
    let access_profile = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::Unauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new("fixture").expect("audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let access_status = AccessStatus::new(
        access_id,
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    Fixture {
        driver,
        instance,
        route,
        access_profile,
        access_status,
        capability,
    }
}

fn preflight_for(fixture: &Fixture, include_time: bool) -> swallowtail_core::PreflightPlan {
    let mut requirements = OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::ProviderRecoveredResourceCleanup,
        DriverRole::ProviderRecoveredResourceCleanup,
        fixture.instance.execution_host_id().clone(),
        AccessRequirement::new(fixture.access_profile.id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_capabilities([fixture.capability.clone()])
    .require_model_route();
    if include_time {
        requirements = requirements.with_host_services([HostServiceKind::Time]);
    }
    preflight(
        &PreflightContext::new(
            &fixture.driver,
            &fixture.instance,
            &fixture.access_profile,
            &fixture.access_status,
            include_time.then_some(HostServiceKind::Time),
        )
        .with_model_route(&fixture.route),
        &requirements,
    )
    .expect("preflight is valid")
}

fn cleanup_binding(
    plan: &swallowtail_core::PreflightPlan,
    runtime_run_id: &str,
) -> ProviderRecoveredResourceCleanupBinding {
    ProviderRecoveredResourceCleanupBinding::new(
        plan,
        RuntimeRunId::new(runtime_run_id).expect("runtime run id is valid"),
        RunRef::new("provider/private/run").expect("provider run ref is valid"),
        [
            OwnedRemoteResourceKind::Environment,
            OwnedRemoteResourceKind::Session,
        ],
        b"private adapter resource binding",
    )
    .expect("cleanup binding is valid")
}

fn cleanup_plan(runtime_run_id: &str) -> ProviderRecoveredResourceCleanupPlan {
    let fixture = fixture("route-revision");
    let preflight = preflight_for(&fixture, true);
    let binding = cleanup_binding(&preflight, runtime_run_id);
    ProviderRecoveredResourceCleanupPlan::new(
        preflight,
        ProviderRecoveredResourceCleanupAgreement::new(
            binding,
            Some(Deadline::at(MonotonicInstant::from_ticks(100))),
        ),
    )
    .expect("cleanup plan is valid")
}

#[test]
fn persisted_binding_is_versioned_bounded_integrity_checked_and_route_bound() {
    let first = fixture("route-revision");
    let first_plan = preflight_for(&first, true);
    let binding = cleanup_binding(&first_plan, "runtime-run");
    let record = binding
        .export_persisted(&first_plan)
        .expect("binding exports");
    let restored = ProviderRecoveredResourceCleanupBinding::restore_persisted(&record, &first_plan)
        .expect("binding restores on the same route");
    assert_eq!(restored.runtime_run_id().as_str(), "runtime-run");
    assert_eq!(restored.resource_kinds().len(), 2);
    assert_eq!(
        format!("{record:?}"),
        "PersistedProviderRecoveredResourceCleanupBinding(<opaque>)"
    );

    let second = fixture("changed-route-revision");
    let second_plan = preflight_for(&second, true);
    assert_eq!(
        ProviderRecoveredResourceCleanupBinding::restore_persisted(&record, &second_plan)
            .expect_err("route drift rejects")
            .kind(),
        ProviderRecoveredResourceCleanupBindingFailureKind::AttachmentMismatch
    );

    let mut corrupted = record.as_bytes().to_vec();
    corrupted[20] ^= 1;
    assert_eq!(
        PersistedProviderRecoveredResourceCleanupBinding::from_bytes(corrupted)
            .expect_err("corruption rejects")
            .kind(),
        ProviderRecoveredResourceCleanupBindingFailureKind::IntegrityMismatch
    );
    assert_eq!(
        PersistedProviderRecoveredResourceCleanupBinding::from_bytes(vec![0; 24 * 1024 + 1])
            .expect_err("oversized record rejects")
            .kind(),
        ProviderRecoveredResourceCleanupBindingFailureKind::Oversized
    );
}

#[test]
fn persisted_binding_rejects_other_record_types_and_unknown_versions() {
    assert_eq!(
        PersistedProviderRecoveredResourceCleanupBinding::from_bytes(b"SWST-RUN-CHECKPT")
            .expect_err("run checkpoint is not cleanup authority")
            .kind(),
        ProviderRecoveredResourceCleanupBindingFailureKind::InvalidEncoding
    );

    let fixture = fixture("route-revision");
    let plan = preflight_for(&fixture, true);
    let mut bytes = cleanup_binding(&plan, "runtime-run")
        .export_persisted(&plan)
        .expect("binding exports")
        .as_bytes()
        .to_vec();
    let version_offset = b"SWST-RSRC-CLEAN".len();
    bytes[version_offset..version_offset + 2].copy_from_slice(&2_u16.to_be_bytes());
    resign(&mut bytes);
    assert_eq!(
        PersistedProviderRecoveredResourceCleanupBinding::from_bytes(bytes)
            .expect_err("unknown version rejects")
            .kind(),
        ProviderRecoveredResourceCleanupBindingFailureKind::UnsupportedVersion
    );
}

#[test]
fn plan_request_and_prepared_attachment_preserve_exact_operation() {
    let fixture = fixture("route-revision");
    let preflight = preflight_for(&fixture, true);
    let binding = cleanup_binding(&preflight, "runtime-run");
    let plan = ProviderRecoveredResourceCleanupPlan::new(
        preflight,
        ProviderRecoveredResourceCleanupAgreement::new(binding, None),
    )
    .expect("plan is valid");
    let request = ProviderRecoveredResourceCleanupRequest::from_plan(
        RequestId::new("cleanup-request").expect("request id is valid"),
        &plan,
    )
    .expect("request is valid");
    let prepared = PreparedProviderRecoveredResourceCleanupEvidence::from_plan(
        plan.clone(),
        PreparedAccessEvidence::observed(
            fixture.access_status,
            AccessEvidenceSourceId::new("private/access/source").expect("source id is valid"),
        ),
    )
    .expect("prepared attachment is valid");

    assert_eq!(
        prepared.operation().binding().driver_role(),
        DriverRole::ProviderRecoveredResourceCleanup
    );
    assert_eq!(request.agreement(), plan.agreement());
}

#[test]
fn cross_operation_requests_and_wrong_cancellation_scope_fail_closed() {
    let first = cleanup_plan("runtime-run-one");
    let second = cleanup_plan("runtime-run-two");
    let first_request = ProviderRecoveredResourceCleanupRequest::from_plan(
        RequestId::new("cleanup-request").expect("request id is valid"),
        &first,
    )
    .expect("request is valid");
    assert_eq!(
        ProviderRecoveredResourceCleanupOutcome::new(
            &second,
            &first_request,
            ProviderRecoveredResourceCleanupEffect::Applied,
        )
        .expect_err("cross-operation request rejects")
        .diagnostic()
        .code(),
        "swallowtail.provider_recovered_resource_cleanup.plan_mismatch"
    );

    let wrong_scope = ProviderRecoveredResourceCleanupRequest::new(
        RequestId::new("wrong-scope").expect("request id is valid"),
        &first,
        Arc::new(ImmediateCancellation::new(
            CancellationScope::ProviderRunReconciliation,
        )),
    )
    .expect_err("wrong cancellation scope rejects");
    assert_eq!(
        wrong_scope.diagnostic().code(),
        "swallowtail.provider_recovered_resource_cleanup.cancellation_scope_mismatch"
    );
}

#[test]
fn deadline_requires_time_and_outcomes_preserve_active_and_partial_truth() {
    let fixture = fixture("route-revision");
    let preflight = preflight_for(&fixture, false);
    let binding = cleanup_binding(&preflight, "runtime-run");
    let error = ProviderRecoveredResourceCleanupPlan::new(
        preflight,
        ProviderRecoveredResourceCleanupAgreement::new(
            binding,
            Some(Deadline::at(MonotonicInstant::from_ticks(100))),
        ),
    )
    .expect_err("deadline without time rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.provider_recovered_resource_cleanup.time_service_required"
    );

    let plan = cleanup_plan("runtime-run");
    let request = ProviderRecoveredResourceCleanupRequest::from_plan(
        RequestId::new("cleanup-request").expect("request id is valid"),
        &plan,
    )
    .expect("request is valid");
    for effect in [
        ProviderRecoveredResourceCleanupEffect::RejectedActiveOrUnknown,
        ProviderRecoveredResourceCleanupEffect::FailedBeforeEffect,
        ProviderRecoveredResourceCleanupEffect::PartiallyApplied,
        ProviderRecoveredResourceCleanupEffect::UnconfirmedAfterEffect,
    ] {
        let outcome = ProviderRecoveredResourceCleanupOutcome::new(&plan, &request, effect)
            .expect("effect truth remains observable");
        assert_eq!(outcome.effect(), effect);
        assert!(!outcome.effect().is_complete());
    }
}

#[test]
fn provider_input_wait_is_observable_and_nonterminal() {
    assert!(!InterruptedRunState::WaitingForProviderInput.is_terminal());
}

fn resign(record: &mut [u8]) {
    let payload_end = record.len() - 32;
    let digest = Sha256::digest(&record[..payload_end]);
    record[payload_end..].copy_from_slice(&digest);
}
