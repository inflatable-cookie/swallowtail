#![allow(dead_code)]

use crate::{discovery_support, support};

use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_gemini::{
    GEMINI_CLI_ACP_AXIS, GeminiCliPreparationInput, GeminiCliPreparationProbe,
    GeminiCliPreparedDriver, GeminiCliPreparedIntegration, GeminiPreparationInput,
    GeminiPreparationProbe, GeminiSessionProfileInput, prepare_gemini_acp, prepare_gemini_cli,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation,
    InstanceRevision, InterfaceVersionAxis, ResourceAccess, RuntimeReadiness, SessionAccessPolicy,
    SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, RuntimeTurnId, ScopeId, SessionOptions, TerminalStatus, TurnRequest,
    WorkingResourceRef,
};
use swallowtail_testkit::{
    assert_observable_activity_trace, assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn solution_facade_keeps_acp_selection_typed() {
    let host_id = ExecutionHostId::new("fixture.prepared.cli-facade").expect("valid host");
    let discovery_host = DiscoveryHost::new("0.51.0");
    let selected = block_on(prepare_gemini_cli(
        cli_preparation_input(host_id.clone()),
        cli_probe(),
        discovery_host.services(host_id),
    ))
    .expect("Gemini CLI ACP prepares through the solution facade");

    assert_eq!(selected.driver(), GeminiCliPreparedDriver::Acp);
    let GeminiCliPreparedIntegration::Acp(prepared) = selected else {
        panic!("ACP selection remains typed");
    };
    assert_eq!(
        prepared.observation().version().version().as_str(),
        "0.51.0"
    );
}

#[test]
fn prepared_sessions_bind_version_access_and_observation_only_model_policy() {
    for (host_value, version, qualified) in [
        ("fixture.prepared.local", "0.51.0", true),
        ("fixture.prepared.remote", "0.52.0", false),
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let operation_host = FixtureHost::with_version(Scenario::Success, version);
        let discovery_host = DiscoveryHost::new(version);
        let operation_services = operation_host.services(host_id.clone());
        let preparation_services = discovery_host
            .services(host_id.clone())
            .with_working_resource(
                operation_services
                    .working_resource()
                    .expect("resource service")
                    .clone(),
            )
            .with_working_resource_io(
                operation_services
                    .working_resource_io()
                    .expect("resource I/O service")
                    .clone(),
            );
        let prepared = block_on(prepare_gemini_acp(
            preparation_input(host_id.clone()),
            probe(),
            preparation_services,
        ))
        .expect("Gemini ACP prepares");
        let profile = prepared
            .prepare_session(GeminiSessionProfileInput::new(
                RequestId::new("gemini-prepared-open").expect("valid request"),
                WorkingResourceRef::new("gemini.prepared.workspace").expect("valid resource"),
                SessionOptions::default(),
            ))
            .expect("session profile prepares");

        assert_eq!(
            profile
                .evidence()
                .observation()
                .version()
                .version()
                .as_str(),
            version
        );
        assert_eq!(profile.evidence().observation().is_qualified(), qualified);
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.request().access_policy(),
            &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        );
        assert!(profile.plan().model_route_id().is_none());
        assert!(profile.plan().model_id().is_none());
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        let activity_profile = profile.evidence().operation().observable_activity().clone();

        let mut session =
            block_on(profile.open_session(operation_services)).expect("prepared session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("gemini-prepared-turn").expect("valid turn"),
                OperationContent::new("private prepared prompt").expect("valid prompt"),
            ),
            operation_host.services(host_id),
        ))
        .expect("prepared turn starts");
        let outcome = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        let mut events = turn.take_events().expect("events");
        let events = block_on(async move {
            let mut observed = Vec::new();
            while let Some(event) = events.next().await {
                observed.push(event.expect("event succeeds"));
            }
            observed
        });
        assert_observable_activity_trace(&activity_profile, &events);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(operation_host.releases(), 1);
    }
}

#[test]
fn unsupported_options_fail_before_session_process_effects() {
    let host_id = ExecutionHostId::new("fixture.prepared.options").expect("valid host");
    let operation_host = FixtureHost::new(Scenario::Success);
    let operation_services = operation_host.services(host_id.clone());
    let discovery_host = DiscoveryHost::new("0.51.0");
    let preparation_services = discovery_host
        .services(host_id.clone())
        .with_working_resource(
            operation_services
                .working_resource()
                .expect("resource service")
                .clone(),
        )
        .with_working_resource_io(
            operation_services
                .working_resource_io()
                .expect("resource I/O service")
                .clone(),
        );
    let prepared = block_on(prepare_gemini_acp(
        preparation_input(host_id),
        probe(),
        preparation_services,
    ))
    .expect("Gemini ACP prepares");
    let result = prepared.prepare_session(GeminiSessionProfileInput::new(
        RequestId::new("gemini-options").expect("valid request"),
        WorkingResourceRef::new("gemini.prepared.workspace").expect("valid resource"),
        SessionOptions::default().with_reasoning_mode(
            swallowtail_core::ReasoningMode::new("high").expect("valid reasoning mode"),
        ),
    ));
    assert!(result.is_err());
    assert!(operation_host.writes().is_empty());
}

#[test]
fn bounded_write_profile_derives_exact_capability_policy_and_invocation() {
    let host_id = ExecutionHostId::new("fixture.prepared.write").expect("valid host");
    let operation_host = FixtureHost::new(Scenario::Success);
    let operation_services = operation_host.services(host_id.clone());
    let discovery_host = DiscoveryHost::new("0.51.0");
    let preparation_services = discovery_host
        .services(host_id.clone())
        .with_working_resource(
            operation_services
                .working_resource()
                .expect("resource service")
                .clone(),
        )
        .with_working_resource_io(
            operation_services
                .working_resource_io()
                .expect("resource I/O service")
                .clone(),
        );
    let prepared = block_on(prepare_gemini_acp(
        preparation_input(host_id),
        probe(),
        preparation_services,
    ))
    .expect("Gemini ACP prepares");
    let profile = prepared
        .prepare_session(GeminiSessionProfileInput::bounded_write(
            RequestId::new("gemini-prepared-write").expect("valid request"),
            WorkingResourceRef::new("gemini.prepared.workspace").expect("valid resource"),
            SessionOptions::default(),
        ))
        .expect("bounded write profile prepares");

    assert_eq!(
        profile.request().access_policy(),
        &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
    );
    assert_eq!(
        profile
            .plan()
            .requirements()
            .session_access_policy()
            .and_then(SessionAccessPolicy::resource_access),
        Some(ResourceAccess::ReadWrite)
    );
    assert_prepared_operation_evidence_matches_plan(profile.evidence().operation(), profile.plan());

    let session = block_on(profile.open_session(operation_services)).expect("write profile opens");
    assert_eq!(
        operation_host.observed_process().arguments,
        ["--acp", "--approval-mode", "auto_edit"]
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

fn preparation_input(host: ExecutionHostId) -> GeminiPreparationInput {
    GeminiPreparationInput::new(
        ConfiguredInstanceId::new("gemini.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("gemini.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(GEMINI_CLI_ACP_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("gemini.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("gemini.prepared.access").expect("valid access"),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("gemini-developer-api").expect("valid audience"),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(
            CredentialRef::new("gemini.prepared.credential").expect("valid credential"),
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn cli_preparation_input(host: ExecutionHostId) -> GeminiCliPreparationInput {
    GeminiCliPreparationInput::new(
        GeminiCliPreparedDriver::Acp,
        ConfiguredInstanceId::new("gemini.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("gemini.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(GEMINI_CLI_ACP_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("gemini.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("gemini.prepared.access").expect("valid access"),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("gemini-developer-api").expect("valid audience"),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(
            CredentialRef::new("gemini.prepared.credential").expect("valid credential"),
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("gemini.prepared.access").expect("valid access"),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn probe() -> GeminiPreparationProbe {
    GeminiPreparationProbe::new(
        RequestId::new("gemini-prepared-probe").expect("valid request"),
        ScopeId::new("gemini-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn cli_probe() -> GeminiCliPreparationProbe {
    GeminiCliPreparationProbe::new(
        RequestId::new("gemini-cli-prepared-probe").expect("valid request"),
        ScopeId::new("gemini-cli-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}
