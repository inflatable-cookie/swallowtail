//! Contract 057 addable-route, admission, and prepared-handoff proof for the
//! Pi SDK sidecar route. Deterministic fixtures only: no Node runtime, no
//! SDK package, and no provider access.

mod support;

use std::sync::Arc;
use support::{FIXTURE_SESSION_REF, SidecarFixtureHost, SidecarScenario};
use swallowtail_adapter_pi::{
    PI_SDK_SIDECAR_ADDABLE_ROUTE_ID, PI_SDK_SIDECAR_CREDENTIAL_FIELD_ID,
    PI_SDK_SIDECAR_ENVIRONMENT_FIELD_ID, PI_SDK_SIDECAR_LAUNCH_RECIPE_FIELD_ID,
    PiSdkSidecarSessionPreparation, pi_sdk_sidecar_addable_route_descriptor,
    pi_sdk_sidecar_descriptor, prepare_pi_sdk_sidecar_session,
};
use swallowtail_core::{
    AddableRouteAvailability, AddableRouteMissingRequirement, AdmittedInstanceRecord,
    ConfigFieldId, ConfigFieldRef, ConfiguredInstanceId, CredentialFieldId, CredentialRef,
    ExecutionHostId, InstanceRevision, IntegrationFamilyId, ModelId, ModelRouteId,
    ModelRouteRevision, ProviderId, RouteTopology,
};
use swallowtail_host_local::{
    LocalProcessHost, LocalProcessLimits, MemoryConnectionLifecycleStore,
};
use swallowtail_runtime::{
    AddableRouteCatalog, ConnectionLifecycleStore, CredentialService, HostServices,
    InstanceAdmissionRequest, ProcessService, RequestId, SessionOptions, WorkingResourceRef,
    admit_instance,
};

const INSTANCE: &str = "pi.fixture.sdk-sidecar.admitted";
const LAUNCH_RECIPE_REF: &str = "pi.fixture.host-private.launch-recipe";
const ENVIRONMENT_REF: &str = "pi.fixture.host-private.environment";
const CREDENTIAL_REF: &str = "pi.fixture.delegated-auth";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("pi.fixture.sdk-sidecar.addable").expect("valid host")
}

fn local_services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    HostServices::new(host_id())
        .with_process(Arc::new(host.clone()) as Arc<dyn ProcessService>)
        .with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

fn admit(store: &MemoryConnectionLifecycleStore) -> AdmittedInstanceRecord {
    let descriptor = pi_sdk_sidecar_addable_route_descriptor(&local_services());
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(
            ConfiguredInstanceId::new(INSTANCE).expect("valid instance"),
            IntegrationFamilyId::new("pi").expect("valid family"),
            swallowtail_core::AddableRouteId::new(PI_SDK_SIDECAR_ADDABLE_ROUTE_ID)
                .expect("valid route id"),
        )
        .with_config_refs([
            (
                ConfigFieldId::new(PI_SDK_SIDECAR_LAUNCH_RECIPE_FIELD_ID).expect("valid config id"),
                ConfigFieldRef::new(LAUNCH_RECIPE_REF).expect("valid config ref"),
            ),
            (
                ConfigFieldId::new(PI_SDK_SIDECAR_ENVIRONMENT_FIELD_ID).expect("valid config id"),
                ConfigFieldRef::new(ENVIRONMENT_REF).expect("valid config ref"),
            ),
        ])
        .with_credential_refs([(
            CredentialFieldId::new(PI_SDK_SIDECAR_CREDENTIAL_FIELD_ID)
                .expect("valid credential id"),
            CredentialRef::new(CREDENTIAL_REF).expect("valid credential ref"),
        )]),
    )
    .expect("admission succeeds")
}

fn preparation_from(
    admitted: &AdmittedInstanceRecord,
    request_id: &str,
) -> PiSdkSidecarSessionPreparation {
    PiSdkSidecarSessionPreparation::from_admitted(
        admitted,
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        host_id(),
        swallowtail_core::AccessProfileId::new("pi.fixture.harness-auth").expect("valid access id"),
        ModelRouteId::new("pi.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        ProviderId::new("fixture-provider").expect("valid provider"),
        ModelId::new("fixture-model").expect("valid model"),
        WorkingResourceRef::new("pi.fixture.workspace").expect("valid resource"),
        RequestId::new(request_id).expect("valid request"),
    )
    .expect("admitted record produces preparation input")
}

#[test]
fn descriptor_is_installed_and_matches_the_sidecar_driver() {
    let descriptor = pi_sdk_sidecar_addable_route_descriptor(&local_services());

    assert_eq!(descriptor.id().as_str(), "pi.sdk-sidecar");
    assert_eq!(descriptor.topology(), RouteTopology::Installed);
    assert_eq!(descriptor.driver(), pi_sdk_sidecar_descriptor().identity());
    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Available
    );
    assert_eq!(descriptor.sign_in_actions().len(), 0);
    assert_eq!(descriptor.credential_fields().len(), 1);
    assert_eq!(descriptor.config_fields().len(), 2);
}

#[test]
fn missing_host_services_mark_the_route_unavailable() {
    let descriptor = pi_sdk_sidecar_addable_route_descriptor(&HostServices::new(host_id()));
    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    );

    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    let process_only =
        HostServices::new(host_id()).with_process(Arc::new(host) as Arc<dyn ProcessService>);
    assert_eq!(
        pi_sdk_sidecar_addable_route_descriptor(&process_only).availability(),
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    );
}

#[test]
fn descriptor_and_records_never_expose_paths_or_secret_bytes() {
    let descriptor = pi_sdk_sidecar_addable_route_descriptor(&local_services());
    let debug = format!("{descriptor:?}");
    for private in [
        LAUNCH_RECIPE_REF,
        ENVIRONMENT_REF,
        CREDENTIAL_REF,
        "PI_SDK_SIDECAR_SDK_MODULE",
        "PI_SDK_SIDECAR_AGENT_DIR",
        "PI_SDK_SIDECAR_SESSION_DIR",
        "node",
    ] {
        assert!(!debug.contains(private), "descriptor leaks {private}");
    }

    let store = MemoryConnectionLifecycleStore::new();
    let record = admit(&store);
    let debug = format!("{record:?}");
    for private in [LAUNCH_RECIPE_REF, ENVIRONMENT_REF, CREDENTIAL_REF] {
        assert!(!debug.contains(private), "record leaks {private}");
    }
}

#[test]
fn admission_prepares_and_opens_through_the_fixture_sidecar() {
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admit(&store);
    assert_eq!(record_route(&admitted), PI_SDK_SIDECAR_ADDABLE_ROUTE_ID);
    assert!(
        store
            .get_instance(&ConfiguredInstanceId::new(INSTANCE).expect("valid instance"))
            .expect("store read succeeds")
            .is_some()
    );

    let prepared = prepare_pi_sdk_sidecar_session(
        preparation_from(&admitted, "sidecar-admit"),
        SessionOptions::default(),
    )
    .expect("admitted instance prepares");
    assert_eq!(prepared.plan().instance_id().as_str(), INSTANCE);
    assert_eq!(
        prepared.plan().instance_target_ref().as_host_value(),
        LAUNCH_RECIPE_REF
    );

    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let session = futures_executor::block_on(prepared.open_session(fixture.services(host_id())))
        .expect("sidecar session opens through the admitted handoff");
    assert_eq!(
        session
            .provider_session_ref()
            .map(|reference| reference.as_provider_value()),
        Some(FIXTURE_SESSION_REF)
    );
    assert!(session.resume_binding().is_some());
    assert_eq!(
        futures_executor::block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
}

#[test]
fn preparation_fails_closed_on_admission_drift() {
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admit(&store);

    let wrong_route = AdmittedInstanceRecord::new(
        ConfiguredInstanceId::new(INSTANCE).expect("valid instance"),
        IntegrationFamilyId::new("pi").expect("valid family"),
        swallowtail_core::AddableRouteId::new("pi.rpc").expect("valid route id"),
        pi_sdk_sidecar_descriptor().identity().clone(),
        RouteTopology::Installed,
    );
    let error = preparation_from_result(&wrong_route)
        .err()
        .expect("another route fails closed");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.pi.sdk-sidecar.preparation.route_mismatch"
    );

    let bare = AdmittedInstanceRecord::new(
        ConfiguredInstanceId::new(INSTANCE).expect("valid instance"),
        IntegrationFamilyId::new("pi").expect("valid family"),
        swallowtail_core::AddableRouteId::new(PI_SDK_SIDECAR_ADDABLE_ROUTE_ID)
            .expect("valid route id"),
        pi_sdk_sidecar_descriptor().identity().clone(),
        RouteTopology::Installed,
    );
    let error = preparation_from_result(&bare)
        .err()
        .expect("missing refs fail closed");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.pi.sdk-sidecar.preparation.launch_recipe_missing"
    );

    let no_credential = bare.clone().with_config_refs(
        admitted
            .config_refs()
            .map(|(id, r)| (id.clone(), r.clone())),
    );
    let error = preparation_from_result(&no_credential)
        .err()
        .expect("missing credential fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.pi.sdk-sidecar.preparation.credential_ref_missing"
    );

    let wrong_driver = AdmittedInstanceRecord::new(
        ConfiguredInstanceId::new(INSTANCE).expect("valid instance"),
        IntegrationFamilyId::new("pi").expect("valid family"),
        swallowtail_core::AddableRouteId::new(PI_SDK_SIDECAR_ADDABLE_ROUTE_ID)
            .expect("valid route id"),
        swallowtail_adapter_pi::pi_rpc_descriptor()
            .identity()
            .clone(),
        RouteTopology::Installed,
    )
    .with_config_refs(
        admitted
            .config_refs()
            .map(|(id, r)| (id.clone(), r.clone())),
    )
    .with_credential_refs(
        admitted
            .credential_refs()
            .map(|(id, r)| (id.clone(), r.clone())),
    );
    let error = preparation_from_result(&wrong_driver)
        .err()
        .expect("driver drift fails closed");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.pi.sdk-sidecar.preparation.driver_mismatch"
    );
}

fn record_route(record: &AdmittedInstanceRecord) -> &str {
    record.route_id().as_str()
}

fn preparation_from_result(
    admitted: &AdmittedInstanceRecord,
) -> Result<PiSdkSidecarSessionPreparation, swallowtail_runtime::PreparationFailure> {
    PiSdkSidecarSessionPreparation::from_admitted(
        admitted,
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        host_id(),
        swallowtail_core::AccessProfileId::new("pi.fixture.harness-auth").expect("valid access id"),
        ModelRouteId::new("pi.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        ProviderId::new("fixture-provider").expect("valid provider"),
        ModelId::new("fixture-model").expect("valid model"),
        WorkingResourceRef::new("pi.fixture.workspace").expect("valid resource"),
        RequestId::new("sidecar-admit-drift").expect("valid request"),
    )
}
