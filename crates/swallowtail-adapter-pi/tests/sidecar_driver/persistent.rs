use super::make_host_id;
use crate::support::{FIXTURE_SESSION_REF, SidecarFixtureHost, SidecarScenario, close_session};
use futures_executor::block_on;
use swallowtail_adapter_pi::{PiSdkSidecarSessionPreparation, prepare_pi_sdk_sidecar_session};
use swallowtail_core::{
    AccessProfileId, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ResourceAccess, SessionAccessPolicy,
};
use swallowtail_runtime::{
    CleanupOutcome, EnvironmentRef, RequestId, SessionOptions, SessionResumeBinding,
    WorkingResourceRef,
};

fn preparation(host: ExecutionHostId, request_id: &str) -> PiSdkSidecarSessionPreparation {
    PiSdkSidecarSessionPreparation::new(
        ConfiguredInstanceId::new("pi.fixture.sdk-sidecar.instance").expect("valid instance"),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        host,
        InstanceTargetRef::new("pi.fixture.pinned-launch-recipe").expect("valid target"),
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        swallowtail_core::CredentialRef::new("pi.fixture.delegated-auth")
            .expect("valid credential"),
        AccessProfileId::new("pi.fixture.harness-auth").expect("valid access id"),
        ModelRouteId::new("pi.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        ProviderId::new("fixture-provider").expect("valid provider"),
        ModelId::new("fixture-model").expect("valid model"),
        WorkingResourceRef::new("pi.fixture.workspace").expect("valid resource"),
        RequestId::new(request_id).expect("valid request"),
    )
}

fn ambient_read() -> SessionAccessPolicy {
    SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
}

#[test]
fn new_session_returns_the_opaque_identity_and_exact_restart_binding() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.new-binding");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let prepared = prepare_pi_sdk_sidecar_session(
        preparation(host_id.clone(), "sidecar-new"),
        SessionOptions::default(),
    )
    .expect("sidecar session prepares");
    let services = fixture.services(host_id);
    let session = block_on(prepared.open_session(services.clone())).expect("sidecar session opens");

    assert_eq!(
        session
            .provider_session_ref()
            .map(|reference| reference.as_provider_value()),
        Some(FIXTURE_SESSION_REF)
    );
    let binding = session
        .resume_binding()
        .expect("new session returns its restart binding");
    assert_eq!(
        binding.provider_session_ref().as_provider_value(),
        FIXTURE_SESSION_REF
    );
    assert!(binding.matches_plan(prepared.plan()));
    assert_eq!(
        binding.origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Created
    );
    let debug = format!("{binding:?}");
    assert!(!debug.contains(FIXTURE_SESSION_REF));
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn persisted_bindings_round_trip_only_under_exact_dimensions() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.persist");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let prepared = prepare_pi_sdk_sidecar_session(
        preparation(host_id.clone(), "sidecar-persist"),
        SessionOptions::default(),
    )
    .expect("sidecar session prepares");
    let services = fixture.services(host_id);
    let session = block_on(prepared.open_session(services.clone())).expect("sidecar session opens");
    let binding = session.resume_binding().expect("binding exists").clone();
    let resource = prepared
        .request()
        .working_resource()
        .expect("resource")
        .clone();
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );

    let record = binding
        .export_persisted(prepared.plan())
        .expect("binding exports");
    assert!(!format!("{record:?}").contains(FIXTURE_SESSION_REF));
    let restored = SessionResumeBinding::restore_persisted(
        &record,
        prepared.plan(),
        &resource,
        &ambient_read(),
    )
    .expect("binding restores under exact dimensions");
    assert_eq!(restored, binding);

    // Drifted working resource never reconstructs a binding.
    let drifted = SessionResumeBinding::restore_persisted(
        &record,
        prepared.plan(),
        &WorkingResourceRef::new("pi.fixture.other-workspace").expect("valid resource"),
        &ambient_read(),
    )
    .expect_err("drifted resource fails");
    assert_eq!(
        drifted.kind(),
        swallowtail_runtime::SessionResumeBindingPersistenceFailureKind::AttachmentMismatch
    );

    // Malformed, oversized, unsupported-version, and corrupted records fail.
    let bytes = record.as_bytes().to_vec();
    for (mutated, kind) in [
        (bytes[..bytes.len() - 4].to_vec(), "InvalidEncoding"),
        (vec![0u8; 9 * 1024], "Oversized"),
        (
            {
                let mut copy = bytes.clone();
                copy[16] = 0x7f;
                copy
            },
            "UnsupportedVersion",
        ),
        (
            {
                let mut copy = bytes.clone();
                let index = bytes.len() - 33;
                copy[index] ^= 0x01;
                copy
            },
            "IntegrityMismatch",
        ),
    ] {
        let outcome = swallowtail_runtime::PersistedSessionResumeBinding::from_bytes(&mutated)
            .map_err(|failure| failure.kind());
        match (outcome, kind) {
            (Err(actual), expected) => assert_eq!(format!("{actual:?}"), expected),
            (Ok(record), "UnsupportedVersion") => {
                let failure = SessionResumeBinding::restore_persisted(
                    &record,
                    prepared.plan(),
                    &resource,
                    &ambient_read(),
                )
                .expect_err("unsupported version fails on restore");
                assert_eq!(format!("{:?}", failure.kind()), kind);
            }
            (Ok(_), expected) => panic!("mutated record accepted as {expected}"),
        }
    }
}

#[test]
fn close_preserves_durable_provider_state_for_later_attachment() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.preserve");
    let prepared = prepare_pi_sdk_sidecar_session(
        preparation(host_id.clone(), "sidecar-preserve"),
        SessionOptions::default(),
    )
    .expect("sidecar session prepares");
    let first = SidecarFixtureHost::new(SidecarScenario::Complete);
    let first_services = first.services(host_id.clone());
    let session =
        block_on(prepared.open_session(first_services.clone())).expect("first session opens");
    let binding = session.resume_binding().expect("binding exists").clone();
    assert_eq!(
        block_on(close_session(session, first_services)),
        CleanupOutcome::Clean
    );

    let second = SidecarFixtureHost::new(SidecarScenario::Complete);
    let second_services = second.services(host_id);
    let loaded = block_on(
        prepared
            .load_session(
                RequestId::new("sidecar-preserve-load").expect("valid request"),
                binding,
                second_services.clone(),
            )
            .expect("load request builds"),
    )
    .expect("closed provider session loads again");
    let (_, session) = loaded.into_parts();
    assert_eq!(
        block_on(close_session(session, second_services)),
        CleanupOutcome::Clean
    );
}
