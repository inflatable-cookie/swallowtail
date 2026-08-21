//! Contract 057 lifecycle proof for installed Claude Agent ACP: admission of
//! the local subscription profile, preparation reuse, readiness refresh,
//! subject observation, and update observation.
//!
//! Deterministic harness only: no live login or install probes, no browser
//! ports, no keychain extraction, no secret bytes in portable records.

#[path = "support/discovery.rs"]
mod support;

use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID, CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID,
    ClaudeAgentPreparationInput, ClaudeAgentPreparationProbe,
    claude_agent_acp_addable_route_descriptor, claude_agent_acp_claim, claude_agent_acp_descriptor,
    claude_agent_acp_subscription_access_profile, prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, AdmittedInstanceRecord,
    AuthenticatedSubjectObservation, ConfigFieldId, ConfigFieldRef, ConfiguredInstanceId,
    CredentialMechanism, CredentialState, EndpointAuthorization, EntitlementState, ExecutionHostId,
    InstanceEnablement, InstanceRevision, IntegrationFamilyId, ModelId, OverlayMarker, ProviderId,
    RuntimeReadiness, SubjectDisclosure, SupportAuthority,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceRecord,
    ConfiguredProviderInstanceSelectionReadiness, ConnectionLifecycleStore, Deadline,
    DiscoveryCancellation, HostServices, InstanceAdmissionRequest,
    ModelPresentationOverlayFailureKind, PreparedAccessEvidence, ReadinessRefreshRequest,
    RequestId, ScopeId, admit_instance, apply_stored_model_presentation_overlay,
    observe_authenticated_subject, observe_instance_update, refresh_readiness,
};

const INSTANCE: &str = "claude-agent.work";
const HOST: &str = "claude-agent.lifecycle.host";
const ACCESS: &str = "claude-agent.work.subscription";
const QUALIFIED_VERSION: &str = "0.70.0";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new(HOST).expect("host id is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("claude-agent").expect("family id is valid")
}

fn access_profile() -> AccessProfile {
    claude_agent_acp_subscription_access_profile(
        AccessProfileId::new(ACCESS).expect("access id is valid"),
    )
}

fn ready_access_status(profile: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        profile.id().clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn ready_evidence(profile: &AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(ready_access_status(profile))
}

fn services() -> (HostServices, FixtureHost) {
    let host = FixtureHost::new(Scenario::Version, QUALIFIED_VERSION);
    let services = host.services(host_id());
    (services, host)
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> AdmittedInstanceRecord {
    let descriptor = claude_agent_acp_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), route_id).with_config_refs([
            (
                ConfigFieldId::new(CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("claude-agent.acp").expect("config ref is valid"),
            ),
            (
                ConfigFieldId::new(CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("claude-agent.work.environment").expect("config ref is valid"),
            ),
        ]),
    )
    .expect("admission succeeds")
}

fn preparation_input(
    admitted: &AdmittedInstanceRecord,
    profile: &AccessProfile,
) -> Result<ClaudeAgentPreparationInput, swallowtail_runtime::PreparationFailure> {
    ClaudeAgentPreparationInput::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        ready_evidence(profile),
    )
}

fn probe() -> ClaudeAgentPreparationProbe {
    ClaudeAgentPreparationProbe::new(
        RequestId::new("claude-agent.lifecycle.prepare").expect("request id is valid"),
        ScopeId::new("claude-agent.lifecycle.prepare").expect("scope id is valid"),
        Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

#[test]
fn admission_writes_a_record_without_secret_bytes() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    assert_eq!(record.id(), &instance_id());
    assert_eq!(record.family().as_str(), "claude-agent");
    assert_eq!(record.route_id().as_str(), "claude-agent.acp");
    assert_eq!(
        record.topology(),
        swallowtail_core::RouteTopology::Installed
    );
    assert_eq!(record.credential_refs().len(), 0);
    assert_eq!(record.config_refs().len(), 2);
    let stored = store
        .get_instance(&instance_id())
        .expect("store read succeeds")
        .expect("instance is stored");
    let debug = format!("{stored:?}");
    assert!(debug.contains("ConfigFieldRef(\"<opaque>\")"));
    assert!(!debug.contains("claude-agent.work.binary-path"));
    assert!(!debug.contains("claude-agent.work.login"));
    assert!(!debug.contains("token"));
    assert!(!debug.contains("keychain"));
}

#[test]
fn missing_browser_ports_do_not_fail_the_subscription_path() {
    let (services, _) = services();
    assert!(services.url_open().is_none());
    assert!(services.loopback_callback().is_none());
    assert!(services.device_code_display().is_none());

    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    let profile = access_profile();
    assert_eq!(
        profile.credential_mechanism(),
        &CredentialMechanism::LocalUnauthenticated
    );
    assert!(profile.credential_reference().is_none());
    assert_eq!(record.credential_refs().len(), 0);
}

#[test]
fn prepare_still_accepts_the_admitted_identity_and_access_profile() {
    let (services, host) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile();

    let prepared = block_on(prepare_claude_agent(
        preparation_input(&admitted, &profile).expect("admitted fields produce preparation input"),
        probe(),
        services,
    ))
    .expect("admitted instance still prepares");

    assert_eq!(prepared.instance().id(), admitted.id());
    assert_eq!(prepared.access_profile(), &profile);
    assert!(prepared.observation().is_permitted());
    let process = host.observed_process();
    assert_eq!(process.executable, "claude-agent.acp");
    assert_eq!(process.arguments, ["--version"]);
    assert_eq!(process.environment_count, 0);
    assert!(process.working_resource.is_none());
    assert_eq!(host.credential_acquires(), 0);
    let debug = format!("{prepared:?}");
    assert!(!debug.contains("claude-agent.work.login"));
    assert!(!debug.contains("token"));
    assert!(!debug.contains("keychain"));
}

#[test]
fn refresh_writes_host_supplied_access_status_without_touching_enablement() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let descriptor = claude_agent_acp_addable_route_descriptor(&services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    let admitted = admit_instance(
        &catalog,
        &store,
        InstanceAdmissionRequest::new(instance_id(), family(), route_id)
            .with_enablement(InstanceEnablement::Disabled),
    )
    .expect("admission succeeds");
    assert_eq!(admitted.enablement(), InstanceEnablement::Disabled);
    assert!(admitted.access_status().is_none());

    let profile = access_profile();
    let refreshed = refresh_readiness(
        &store,
        ReadinessRefreshRequest::new(instance_id(), ready_access_status(&profile)),
    )
    .expect("refresh succeeds");

    let status = refreshed.access_status().expect("access status is stored");
    assert_eq!(status.profile_id(), profile.id());
    assert_eq!(status.credential(), CredentialState::NotRequired);
    assert_eq!(status.entitlement(), EntitlementState::Available);
    assert_eq!(
        status.endpoint_authorization(),
        EndpointAuthorization::Allowed
    );
    assert_eq!(status.runtime_readiness(), RuntimeReadiness::Ready);
    assert_eq!(
        status.support_authority(),
        SupportAuthority::IntegrationMaintainerSupported
    );
    assert_eq!(refreshed.enablement(), InstanceEnablement::Disabled);
}

#[test]
fn subject_stays_absent_for_claude_agent_acp() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    admitted_record(&services, &store);

    let observed = observe_authenticated_subject(
        &store,
        &instance_id(),
        AuthenticatedSubjectObservation::undisclosed(),
    )
    .expect("subject observation succeeds");

    assert_eq!(observed.email(), &SubjectDisclosure::Absent);
    assert_eq!(observed.login(), &SubjectDisclosure::Absent);
    assert_eq!(observed.plan(), &SubjectDisclosure::Absent);
}

#[test]
fn update_observation_reuses_the_acp_claim_and_032_evidence() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile();
    let prepared = block_on(prepare_claude_agent(
        preparation_input(&admitted, &profile).expect("admitted fields produce preparation input"),
        probe(),
        services,
    ))
    .expect("admitted instance prepares");
    let claim = claude_agent_acp_claim();

    let unobserved = observe_instance_update(&claim, None).expect("claim alone is valid");
    assert_eq!(unobserved.claim_id(), claim.id());
    assert!(unobserved.is_unobserved());
    assert_eq!(unobserved.compatibility(), None);

    let observed = observe_instance_update(&claim, Some(prepared.observation().clone()))
        .expect("prepared 032 observation matches the claim");
    assert_eq!(observed.claim_id(), claim.id());
    assert_eq!(observed.installed(), Some(prepared.observation()));
    assert!(matches!(
        observed.compatibility(),
        Some(swallowtail_core::InstalledExecutableCompatibility::Qualified(_))
    ));
}

fn snapshot_record(
    services: &HostServices,
    admitted: &AdmittedInstanceRecord,
) -> ConfiguredProviderInstanceRecord {
    let profile = access_profile();
    let evidence = ready_evidence(&profile);
    let prepared = block_on(prepare_claude_agent(
        preparation_input(admitted, &profile).expect("admitted fields produce preparation input"),
        probe(),
        services.clone(),
    ))
    .expect("admitted instance prepares");
    ConfiguredProviderInstanceRecord::admit(ConfiguredProviderInstanceAdmission::new(
        claude_agent_acp_descriptor(),
        prepared.instance().clone(),
        profile,
        evidence,
    ))
    .expect("047 snapshot assembles")
}

#[test]
fn overlay_does_not_invent_a_claude_agent_catalogue_provider_id() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let record = snapshot_record(&services, &admitted);
    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
    assert!(record.model_catalogue().is_none());
    let snapshot_debug = format!("{record:?}");
    assert!(!snapshot_debug.contains('@'));
    assert!(!snapshot_debug.contains("claude-agent.work.login"));
    assert!(!snapshot_debug.contains("claude-agent.work.binary-path"));

    let overlay = apply_stored_model_presentation_overlay(&store, &record)
        .expect("overlay projects onto the unmarked Claude Agent snapshot");

    assert_eq!(overlay.selection_readiness(), record.selection_readiness());
    assert_eq!(overlay.instance_id(), &instance_id());
    assert_eq!(overlay.entries().count(), 0);

    store
        .put_overlay_marker(
            OverlayMarker::without_provider(
                instance_id(),
                ModelId::new("claude-fixture").expect("model id is valid"),
            )
            .with_favourite(true),
        )
        .expect("unmarked overlay marker stores");
    let unmarked = apply_stored_model_presentation_overlay(&store, &record)
        .expect_err("instance-plus-model cannot invent a missing catalogue row");
    assert_eq!(
        unmarked.kind(),
        ModelPresentationOverlayFailureKind::UnknownModel
    );

    store
        .put_overlay_marker(
            OverlayMarker::new(
                instance_id(),
                ProviderId::new("claude-agent").expect("provider id is valid"),
                ModelId::new("claude-fixture").expect("model id is valid"),
            )
            .with_favourite(true),
        )
        .expect("overlay marker stores");
    let failure = apply_stored_model_presentation_overlay(&store, &record)
        .expect_err("an invented catalogue provider id fails closed");
    assert_eq!(
        failure.kind(),
        ModelPresentationOverlayFailureKind::UnknownModel
    );
}
