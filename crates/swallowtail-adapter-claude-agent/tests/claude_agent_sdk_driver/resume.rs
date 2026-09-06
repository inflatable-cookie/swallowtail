//! Provider-free proofs for SDK persistence, resume, and route-local listing.

use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, prepared_session, prepared_session_for,
    turn_request,
};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkSessionListing, ClaudeAgentSdkSessionProfile,
};
use swallowtail_core::{
    ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, ResourceAccess,
    SessionAccessPolicy, SessionRef,
};
use swallowtail_runtime::{InteractiveSessionHandle, RequestId, SessionResumeBinding};

fn persistent_profile() -> ClaudeAgentSdkSessionProfile {
    ClaudeAgentSdkSessionProfile::read_only().with_persist_session(true)
}

fn binding(host: &ExecutionHostId) -> SessionResumeBinding {
    SessionResumeBinding::new(
        SessionRef::new("session-1").expect("fixture provider session id is valid"),
        ConfiguredInstanceId::new("claude-agent-sdk.fixture").expect("fixture instance is valid"),
        host.clone(),
        ModelRouteId::new("claude-agent-sdk.fixture.route").expect("fixture route is valid"),
        ModelId::new("claude-sonnet-5").expect("fixture model is valid"),
        swallowtail_runtime::WorkingResourceRef::new("claude-agent-sdk.fixture.workspace")
            .expect("fixture resource is valid"),
        SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
    )
}

fn request_id(value: &str) -> RequestId {
    RequestId::new(value).expect("fixture request id is valid")
}

fn finish_turn(
    session: &mut swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle,
    services: swallowtail_runtime::HostServices,
) {
    let mut turn = block_on(session.start_turn(turn_request("resume-turn", "continue"), services))
        .expect("resumed first turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("resumed turn has a terminal outcome"),
    );
    assert_eq!(
        outcome.status(),
        &swallowtail_runtime::TerminalStatus::Completed
    );
    let _ = block_on(turn.close());
}

fn close_session(
    session: swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle,
    services: swallowtail_runtime::HostServices,
) {
    let _ = block_on(Box::new(session).close(cleanup_request(), services));
}

#[test]
fn persistence_is_opt_in_and_resume_attaches_without_replay() {
    let host = host_id("claude-agent-sdk.fixture.resume-complete");
    let fixture = SdkFixtureHost::new(SdkScenario::ResumeComplete);
    let prepared = prepared_session_for(host.clone(), persistent_profile());
    assert!(prepared.session_profile().persist_session());
    assert!(
        prepared
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == swallowtail_core::Capability::Resume)
    );
    let services = fixture.services(host.clone());
    let cleanup_services = services.clone();
    let mut session = block_on(
        prepared
            .resume_session(request_id("resume-1"), binding(&host), services.clone())
            .expect("resume request prepares"),
    )
    .expect("resume attaches");
    assert_eq!(
        session
            .provider_session_ref()
            .expect("resumed handle retains provider identity")
            .as_provider_value(),
        "session-1"
    );
    finish_turn(&mut session, services.clone());
    assert!(
        session.resume_binding().is_some(),
        "the resumed handle remains resumable after its first turn"
    );
    let inputs = fixture.inputs();
    let open = inputs
        .iter()
        .find(|input| input["command"] == "open")
        .expect("resume sends open command");
    assert_eq!(open["params"]["resume"], "session-1");
    assert_eq!(open["params"]["persistSession"], true);
    assert!(inputs.iter().all(|input| input["command"] != "load"));
    close_session(session, cleanup_services);
}

#[test]
fn persistence_off_refuses_resume_before_any_host_or_sidecar_effect() {
    let host = host_id("claude-agent-sdk.fixture.resume-disabled");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host.clone());
    let future = prepared
        .resume_session(request_id("resume-disabled"), binding(&host), services)
        .expect("resume request shape is constructible");
    let error = match block_on(future) {
        Ok(_) => panic!("default profile cannot resume"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.resume_persistence_disabled"
    );
    assert_eq!(fixture.credential_acquisitions(), 0);
    assert!(fixture.inputs().is_empty());
}

#[test]
fn resume_cwd_and_account_mismatches_fail_before_turn_acceptance() {
    for (scenario, expected) in [
        (
            SdkScenario::ResumeCwdMismatch,
            "swallowtail.claude-agent.sdk.resume_cwd_mismatch",
        ),
        (
            SdkScenario::ResumeAccountMismatch,
            "swallowtail.claude-agent.sdk.resume_account_mismatch",
        ),
    ] {
        let host = host_id(match scenario {
            SdkScenario::ResumeCwdMismatch => "claude-agent-sdk.fixture.resume-cwd",
            _ => "claude-agent-sdk.fixture.resume-account",
        });
        let fixture = SdkFixtureHost::new(scenario);
        let prepared = prepared_session_for(host.clone(), persistent_profile());
        let services = fixture.services(host.clone());
        let cleanup_services = services.clone();
        let result = block_on(
            prepared
                .resume_session(
                    request_id("resume-mismatch"),
                    binding(&host),
                    services.clone(),
                )
                .expect("resume request prepares"),
        );
        if scenario == SdkScenario::ResumeAccountMismatch {
            let error = match result {
                Ok(_) => panic!("wrong account fails at initialize"),
                Err(error) => error,
            };
            assert_eq!(error.diagnostic().code(), expected);
            assert!(
                fixture
                    .inputs()
                    .iter()
                    .all(|input| input["command"] != "query")
            );
        } else {
            let mut session = result.expect("resume reaches first-turn validation");
            let error = match block_on(
                session.start_turn(turn_request("resume-cwd", "continue"), services),
            ) {
                Ok(_) => panic!("wrong cwd fails before turn acceptance"),
                Err(error) => error,
            };
            assert_eq!(error.diagnostic().code(), expected);
            close_session(session, cleanup_services);
        }
    }
}

#[test]
fn unknown_session_and_resume_session_at_are_typed() {
    let host = host_id("claude-agent-sdk.fixture.resume-unknown");
    let fixture = SdkFixtureHost::new(SdkScenario::ResumeSessionUnknown);
    let prepared = prepared_session_for(host.clone(), persistent_profile());
    let services = fixture.services(host.clone());
    let cleanup_services = services.clone();
    let mut session = block_on(
        prepared
            .resume_session(
                request_id("resume-unknown"),
                binding(&host),
                services.clone(),
            )
            .expect("resume request prepares"),
    )
    .expect("resume open reaches first-turn validation");
    let error = match block_on(
        session.start_turn(turn_request("resume-unknown-turn", "continue"), services),
    ) {
        Ok(_) => panic!("unknown session fails closed"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.resume_session_unknown"
    );
    close_session(session, cleanup_services);

    let host = host_id("claude-agent-sdk.fixture.resume-boundary");
    let fixture = SdkFixtureHost::new(SdkScenario::ResumeBoundaryRejected);
    let prepared = prepared_session_for(host.clone(), persistent_profile());
    let services = fixture.services(host.clone());
    let cleanup_services = services.clone();
    let mut session = block_on(
        prepared
            .resume_session_at(
                request_id("resume-boundary"),
                binding(&host),
                "message-boundary-1",
                services.clone(),
            )
            .expect("resume-at request prepares"),
    )
    .expect("resume-at open attaches");
    let error = match block_on(
        session.start_turn(turn_request("resume-boundary-turn", "continue"), services),
    ) {
        Ok(_) => panic!("provider boundary rejection is typed"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.resume_boundary_invalid"
    );
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("resume-at sends open command");
    assert_eq!(open["params"]["resumeSessionAt"], "message-boundary-1");
    close_session(session, cleanup_services);
}

#[test]
fn listing_returns_only_bounded_metadata_in_the_leased_scope() {
    let host = host_id("claude-agent-sdk.fixture.session-listing");
    let fixture = SdkFixtureHost::new(SdkScenario::SessionListing);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let entries: Vec<ClaudeAgentSdkSessionListing> =
        block_on(prepared.list_sessions(services)).expect("listing succeeds");
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    assert_eq!(
        entry.provider_session_ref().as_provider_value(),
        "session-1"
    );
    assert_eq!(entry.cwd(), "/fixture/claude-agent-sdk-workspace");
    assert_eq!(entry.created_at_unix_milliseconds(), Some(100));
    assert_eq!(entry.last_modified_unix_milliseconds(), 200);
    assert_eq!(entry.title(), Some("Fixture session"));
    assert!(
        fixture
            .inputs()
            .iter()
            .any(|input| input["command"] == "list_sessions")
    );
    assert!(
        fixture
            .inputs()
            .iter()
            .all(|input| input["command"] != "open")
    );
    assert!(
        fixture
            .inputs()
            .iter()
            .all(|input| input["command"] != "query")
    );
}
