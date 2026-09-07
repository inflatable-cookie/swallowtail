//! Provider-free conformance for the registered-tool vocabulary itself.

use super::*;
use crate::{Deadline, HostServices, MonotonicInstant, RuntimeTurnId, ScopeId};
use std::future::ready;
use std::sync::Arc;
use std::time::Duration;
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId, HostServiceKind};

struct AlwaysCurrent;

impl ConsumerAdmissionHostService for AlwaysCurrent {
    fn validate(
        &self,
        _binding: &ConsumerAdmissionBinding,
        _phase: AdmissionPhase,
    ) -> crate::BoxFuture<'_, Result<AdmissionVerdict, crate::RuntimeFailure>> {
        Box::pin(ready(Ok(AdmissionVerdict::Current)))
    }
}

fn host() -> ExecutionHostId {
    ExecutionHostId::new("fixture.host.local").expect("host id")
}

fn tool(local: &str) -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new("swallowtail.conformance").expect("namespace"),
        RegisteredToolLocalName::new(local).expect("local name"),
    )
}

fn schema() -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("swallowtail.conformance.schema").expect("namespace"),
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect"),
        RegisteredServerRevision::new("1").expect("revision"),
        RegisteredToolSchemaDigest::new("sha256:body").expect("digest"),
        RegisteredToolSchemaDocument::new("{\"secret\":\"body\"}").expect("document"),
    )
}

fn snapshot() -> RegisteredToolSnapshot {
    RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("server").expect("server id"),
        revision: RegisteredServerRevision::new("1").expect("revision"),
        execution_host_id: host(),
        declarations: vec![
            RegisteredToolDeclaration::new(
                tool("echo"),
                RegisteredToolExecutionKind::NativeClient,
                schema(),
                schema(),
                RegisteredToolEffectPosture::ReadOnly,
                RegisteredToolRetryPosture::NeverRetry,
                RegisteredToolBounds::ceiling(),
            )
            .expect("declaration"),
        ],
        transports: vec![
            RegisteredToolTransportSupport::new(
                RegisteredToolTransport::HostMediatedCallback,
                [protocol_version()],
            )
            .expect("transport support"),
        ],
        required_services: [HostServiceKind::Time].into_iter().collect(),
        credential_references: Vec::new(),
        executable_recipes: Vec::new(),
        environment_recipes: Vec::new(),
        bounds: RegisteredToolBounds::ceiling(),
        source: RegisteredToolSource::new(
            RegisteredToolSourceId::new("source").expect("source id"),
            MonotonicInstant::from_ticks(1),
        ),
    })
    .expect("snapshot")
}

fn protocol_version() -> RegisteredToolProtocolVersion {
    RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
        .expect("protocol version")
}

fn admission() -> ConsumerAdmissionBinding {
    ConsumerAdmissionBinding::new(
        ConsumerProcessIncarnation::new("incarnation").expect("incarnation"),
        ConsumerWorkspaceGeneration::initial(),
        ConsumerTaskGeneration::initial(),
        AdmittedTaskId::new("task").expect("task"),
        AdmittedSessionId::new("session").expect("session"),
        AdmittedAttemptId::new("attempt").expect("attempt"),
        Arc::new(AlwaysCurrent),
    )
}

fn selection() -> RegisteredToolSelection {
    RegisteredToolSelection::new(
        Arc::new(snapshot()),
        [tool("echo")],
        RegisteredToolTransport::HostMediatedCallback,
        protocol_version(),
    )
    .expect("selection")
}

#[test]
fn host_service_kind_stays_exhaustive_for_the_registered_profile() {
    let hosts = HostServices::new(host());

    assert!(hosts.registered_tool_bridge().is_none());
    assert!(!hosts.available_kinds().iter().any(|kind| {
        format!("{kind:?}")
            .to_ascii_lowercase()
            .contains("registeredtool")
    }));
}

#[test]
fn missing_required_services_are_reported_before_provider_work() {
    let hosts = HostServices::new(host());
    let readiness = RegisteredToolReadiness::evaluate(&hosts, &selection());

    assert!(
        readiness
            .missing_services()
            .contains(&HostServiceKind::Time)
    );
    assert_eq!(readiness.port(), RegisteredToolPortAvailability::Absent);
    assert_eq!(
        readiness
            .require_ready()
            .expect_err("an absent port fails first")
            .kind(),
        RegisteredToolFailureKind::MissingHostService
    );
}

#[test]
fn consumer_limits_only_narrow_effective_bounds() {
    let narrow = RegisteredToolBounds::new(1, 1_024, 2_048, 512, 4, Duration::from_secs(5))
        .expect("narrow bounds");
    let effective = selection().effective_bounds().narrowed(narrow);

    assert_eq!(effective.max_argument_bytes(), 1_024);
    assert_eq!(effective.max_queued_progress_items(), 4);
    assert_eq!(effective.max_call_duration(), Duration::from_secs(5));
    assert_eq!(
        effective.max_outstanding_calls(),
        MAX_REGISTERED_TOOL_OUTSTANDING_CALLS
    );
}

#[test]
fn bounds_never_widen_the_first_tranche_ceiling() {
    let error = RegisteredToolBounds::new(
        1,
        MAX_REGISTERED_TOOL_ARGUMENT_BYTES + 1,
        1,
        1,
        1,
        Duration::from_secs(1),
    )
    .expect_err("widening must fail");
    let zero = RegisteredToolBounds::new(0, 1, 1, 1, 1, Duration::from_secs(1))
        .expect_err("zero bounds must fail");

    assert_eq!(error.kind(), RegisteredToolFailureKind::LimitExceeded);
    assert_eq!(zero.kind(), RegisteredToolFailureKind::IdentityRejected);
}

#[test]
fn schema_and_payload_bodies_never_reach_formatting() {
    let schema = schema();
    let payload = RegisteredToolPayload::new(
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        b"private-argument".to_vec(),
        64,
    )
    .expect("payload");

    assert!(!format!("{schema:?}").contains("secret"));
    assert!(!format!("{payload:?}").contains("private-argument"));
    assert!(!format!("{payload}").contains("private-argument"));
    assert_eq!(payload.expose_for_execution(), b"private-argument");
    assert_eq!(
        schema.document().expose_for_execution(),
        "{\"secret\":\"body\"}"
    );
}

#[test]
fn admission_binding_compares_fixed_identity_and_redacts_its_port() {
    let binding = admission();
    let rendered = format!("{binding:?}");

    assert!(binding.identity_matches(&admission()));
    assert!(rendered.contains("<opaque>"));
    assert!(rendered.contains("<private consumer admission port>"));
    assert!(!rendered.contains("incarnation\": \"incarnation"));
}

#[test]
fn a_non_live_lease_mints_no_binding_and_issues_no_call() {
    let request = RegisteredToolOpenRequest::new(
        host(),
        ConfiguredInstanceId::new("instance").expect("instance"),
        ScopeId::new("scope").expect("scope"),
        RuntimeTurnId::new("turn").expect("turn"),
        selection(),
        admission(),
        Deadline::at(MonotonicInstant::from_ticks(10)),
    );
    let lease = RegisteredToolBridgeLease::new(
        request,
        RegisteredToolLeaseGeneration::initial(),
        RegisteredToolTransportGeneration::initial(),
    );

    assert!(!lease.is_live());
    assert!(lease.validated_binding().is_none());
    assert!(lease.endpoint().is_none());
    assert!(lease.bearer().is_none());
}

#[test]
fn swallowtail_enforced_posture_never_widens() {
    assert!(!RegisteredToolEnforcedPosture::automatic_replay_allowed());
    assert!(!RegisteredToolEnforcedPosture::reconnect_replays_calls());
    assert!(RegisteredToolEnforcedPosture::teardown_joins_issued_work());
}

#[test]
fn only_the_host_mediated_callback_transport_is_qualified() {
    assert_eq!(
        REGISTERED_TOOL_QUALIFIED_TRANSPORTS,
        &[RegisteredToolTransport::HostMediatedCallback]
    );
    assert!(!RegisteredToolTransport::HostMediatedCallback.binds_listener());
    assert!(RegisteredToolTransport::PrivateLoopbackHttp.binds_listener());
    assert!(RegisteredToolTransport::PrivateLoopbackSse.binds_listener());
}

#[test]
fn every_failure_kind_keeps_a_stable_namespaced_code() {
    for kind in [
        RegisteredToolFailureKind::UnsupportedRegistration,
        RegisteredToolFailureKind::UnknownExecutionOutcome,
        RegisteredToolFailureKind::TeardownFailed,
        RegisteredToolFailureKind::Revoked,
    ] {
        let failure = RegisteredToolFailure::new(kind);
        assert!(
            failure
                .diagnostic()
                .code()
                .starts_with("swallowtail.registered_tool.")
        );
        assert_eq!(
            failure.into_runtime_failure().diagnostic().code(),
            kind.code()
        );
    }
}
