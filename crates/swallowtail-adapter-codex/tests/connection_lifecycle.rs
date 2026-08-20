//! Contract 057 lifecycle proof for installed Codex app-server: admission of
//! the cached ChatGPT subscription profile, preparation reuse, readiness
//! refresh, subject observation, and update observation.
//!
//! Deterministic harness only: no live login or install probes, no browser
//! ports, no ChatGPT token extraction, no secret bytes in portable records.

use crate::support::{FakeProcessService, host_services_for};
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_codex::{
    CODEX_APP_SERVER_BINARY_PATH_FIELD_ID, CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID, CODEX_CLI_AXIS,
    CodexPreparationInput, CodexPreparationProbe, CodexPreparedDriver,
    codex_app_server_addable_route_descriptor, codex_chatgpt_subscription_access_profile,
    prepare_codex,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfigFieldId, ConfigFieldRef,
    ConfiguredInstanceId, CredentialState, EndpointAuthorization, EntitlementState,
    ExecutionHostId, InstanceRevision, IntegrationFamilyId, InterfaceVersionAxis, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, BoxFuture, ConnectionLifecycleStore, Deadline, DeadlineObservation,
    DiscoveryCancellation, EnvironmentRef, ExecutableRef, HostServices, InstanceAdmissionRequest,
    InstalledExecutableTarget, MonotonicInstant, PreparedAccessEvidence, RequestId, ScopeId,
    TimeService, admit_instance,
};

const INSTANCE: &str = "codex.work";
const HOST: &str = "codex.lifecycle.host";
const ACCESS: &str = "codex.work.chatgpt";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new(HOST).expect("host id is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("codex").expect("family id is valid")
}

fn access_profile() -> AccessProfile {
    codex_chatgpt_subscription_access_profile(
        AccessProfileId::new(ACCESS).expect("access id is valid"),
    )
}

fn ready_evidence(profile: &AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(ready_access_status(profile))
}

fn ready_access_status(profile: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        profile.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn services() -> (HostServices, Arc<dyn swallowtail_runtime::ProcessService>) {
    let (process, _) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let services = host_services_for(host_id(), process.clone()).with_time(Arc::new(PendingTime));
    (services, process)
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> swallowtail_core::AdmittedInstanceRecord {
    let descriptor = codex_app_server_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), route_id)
        .with_config_refs([
            (
                ConfigFieldId::new(CODEX_APP_SERVER_BINARY_PATH_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("codex.work.binary-path").expect("config ref is valid"),
            ),
            (
                ConfigFieldId::new(CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("codex.work.environment").expect("config ref is valid"),
            ),
        ]),
    )
    .expect("admission succeeds")
}

fn preparation_input(
    admitted: &swallowtail_core::AdmittedInstanceRecord,
    profile: &AccessProfile,
) -> CodexPreparationInput {
    CodexPreparationInput::new(
        CodexPreparedDriver::AppServer,
        admitted.id().clone(),
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("codex-app-server").expect("executable ref is valid"),
            InterfaceVersionAxis::new(CODEX_CLI_AXIS).expect("version axis is valid"),
        ),
        EnvironmentRef::new("codex.work.login").expect("environment ref is valid"),
        profile.clone(),
        ready_evidence(profile),
    )
}

fn probe() -> CodexPreparationProbe {
    CodexPreparationProbe::new(
        RequestId::new("codex.lifecycle.prepare").expect("request id is valid"),
        ScopeId::new("codex.lifecycle.prepare").expect("scope id is valid"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

#[test]
fn admission_writes_a_record_without_secret_bytes() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    assert_eq!(record.id(), &instance_id());
    assert_eq!(record.family().as_str(), "codex");
    assert_eq!(record.route_id().as_str(), "codex.app-server");
    assert_eq!(record.topology(), swallowtail_core::RouteTopology::Installed);
    assert_eq!(record.credential_refs().len(), 0);
    assert_eq!(record.config_refs().len(), 2);
    let stored = store
        .get_instance(&instance_id())
        .expect("store read succeeds")
        .expect("instance is stored");
    let debug = format!("{stored:?}");
    assert!(debug.contains("ConfigFieldRef(\"<opaque>\")"));
    assert!(!debug.contains("codex.work.binary-path"));
    assert!(!debug.contains("codex.work.login"));
    assert!(!debug.contains("token"));
}

#[test]
fn missing_browser_ports_do_not_fail_the_chatgpt_path() {
    let (services, _) = services();
    assert!(services.url_open().is_none());
    assert!(services.loopback_callback().is_none());
    assert!(services.device_code_display().is_none());

    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    let profile = access_profile();
    assert_eq!(
        profile.credential_mechanism(),
        &swallowtail_core::CredentialMechanism::InteractiveOauth
    );
    assert!(profile.credential_reference().is_none());
    assert_eq!(record.credential_refs().len(), 0);
}

#[test]
fn prepare_still_accepts_the_admitted_identity_and_access_profile() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile();

    let prepared = block_on(prepare_codex(
        preparation_input(&admitted, &profile),
        probe(),
        services,
    ))
    .expect("admitted instance still prepares");

    assert_eq!(prepared.instance().id(), admitted.id());
    assert_eq!(prepared.access_profile(), &profile);
    assert!(prepared.observation().is_permitted());
    let debug = format!("{prepared:?}");
    assert!(!debug.contains("codex.work.login"));
    assert!(!debug.contains("token"));
}

struct PendingTime;

impl TimeService for PendingTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}
