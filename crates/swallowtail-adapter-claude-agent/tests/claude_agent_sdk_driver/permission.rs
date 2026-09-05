//! Read-write admission and permission-mode proofs for the SDK sidecar route.
//!
//! Every case is provider-free: a fake sidecar answers the private wire, and
//! no Node runtime, SDK package, native binary, credential, or provider turn
//! exists anywhere here.

use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, prepared_session, prepared_session_for,
};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile, ClaudeAgentSdkTool,
};
use swallowtail_core::{Capability, CapabilityConstraint, ResourceAccess};
use swallowtail_runtime::{
    Deadline, InteractiveSessionDriver, InteractiveSessionHandle, MonotonicInstant,
    SessionAccessPolicy,
};

/// Closes the route-local handle through the shared session surface.
fn close_session(
    session: swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle,
    request: swallowtail_runtime::SessionCleanupRequest,
    services: swallowtail_runtime::HostServices,
) -> swallowtail_runtime::CleanupOutcome {
    block_on((Box::new(session) as Box<dyn InteractiveSessionHandle>).close(request, services))
}

fn read_write() -> ClaudeAgentSdkSessionProfile {
    ClaudeAgentSdkSessionProfile::read_write(ClaudeAgentSdkPermissionMode::Default)
}

fn mode_deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(10_000))
}

fn plan_resource_access(plan: &swallowtail_core::PreflightPlan) -> Option<ResourceAccess> {
    plan.requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::WorkingResource)
        .and_then(|required| {
            required
                .constraints()
                .find_map(|constraint| match constraint {
                    CapabilityConstraint::ResourceAccess(access) => Some(*access),
                    _ => None,
                })
        })
}

#[test]
fn the_default_profile_prepares_the_unchanged_read_only_plan() {
    let prepared = prepared_session(host_id("claude-agent-sdk.fixture.default-profile"));
    assert_eq!(
        prepared.session_profile(),
        ClaudeAgentSdkSessionProfile::read_only()
    );
    assert_eq!(
        prepared.request().access_policy(),
        &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
    );
    assert_eq!(
        plan_resource_access(prepared.plan()),
        Some(ResourceAccess::Read)
    );
    assert_eq!(
        prepared.plan().instance_policy_id().as_str(),
        "claude-agent-sdk-ambient-read"
    );
}

#[test]
fn a_write_profile_prepares_a_read_write_plan_and_lease() {
    // Contract 013 keys the consumer-tool exclusion on a bounded profile's
    // claimed filesystem boundary, and this route claims none, so its ambient
    // read-write profile with consumer-mediated tool calls is admissible.
    let prepared = prepared_session_for(host_id("claude-agent-sdk.fixture.rw-plan"), read_write());
    assert_eq!(prepared.session_profile(), read_write());
    assert_eq!(
        prepared.request().access_policy(),
        &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
    );
    assert_eq!(
        plan_resource_access(prepared.plan()),
        Some(ResourceAccess::ReadWrite)
    );
    assert_eq!(
        prepared.plan().instance_policy_id().as_str(),
        "claude-agent-sdk-ambient-read-write"
    );
    // The capability the route needs is still declared: the write lease did
    // not cost it consumer tool exchange.
    assert!(
        prepared
            .plan()
            .requirements()
            .capabilities()
            .any(|required| required.capability() == Capability::ToolCalls)
    );
}

#[test]
fn a_write_session_opens_with_its_write_set_on_a_read_write_lease() {
    let host = host_id("claude-agent-sdk.fixture.rw-open");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session_for(host.clone(), read_write());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("a write session opens");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|value| value["command"] == "open")
        .expect("open command is sent");
    assert_eq!(
        open["params"]["tools"],
        serde_json::json!(["Read", "Glob", "Grep", "Edit", "Write", "MultiEdit"])
    );
    // Availability is restricted, never auto-allowed.
    assert!(open["params"].get("allowedTools").is_none());
    assert_eq!(open["params"]["permissionMode"], "default");
    assert!(session.session_profile().admits(ClaudeAgentSdkTool::Write));
    let _ = close_session(session, cleanup_request(), cleanup_services);
}

#[test]
fn a_host_that_grants_only_read_access_cannot_open_a_write_session() {
    // The plan asks for the lease the admitted set requires. A host granting
    // less fails the lease agreement, so no write tool reaches a read-only
    // working resource.
    let host = host_id("claude-agent-sdk.fixture.rw-lease");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete).granting_read_only_resource();
    let prepared = prepared_session_for(host.clone(), read_write());
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_route_session(services)) else {
        panic!("a read-only lease must refuse a write session");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.session_access.resource_access_mismatch"
    );
}

#[test]
fn an_explicit_bash_profile_cannot_open_with_a_read_only_lease() {
    let host = host_id("claude-agent-sdk.fixture.bash-lease");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete).granting_read_only_resource();
    let profile = ClaudeAgentSdkSessionProfile::new(
        [ClaudeAgentSdkTool::Bash],
        ClaudeAgentSdkPermissionMode::Default,
    )
    .expect("explicit Bash profile is admissible");
    let prepared = prepared_session_for(host.clone(), profile);
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_route_session(services)) else {
        panic!("a Bash profile must refuse a read-only lease");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.session_access.resource_access_mismatch"
    );
}

#[test]
fn a_session_sends_its_admitted_set_and_selected_mode_on_open() {
    let host = host_id("claude-agent-sdk.fixture.mode-open");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session_for(
        host.clone(),
        ClaudeAgentSdkSessionProfile::read_only()
            .with_permission_mode(ClaudeAgentSdkPermissionMode::Plan),
    );
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("a plan session opens");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|value| value["command"] == "open")
        .expect("open command is sent");
    assert_eq!(
        open["params"]["tools"],
        serde_json::json!(["Read", "Glob", "Grep"])
    );
    assert_eq!(open["params"]["permissionMode"], "plan");
    // The mode the handle reports is the one the sidecar confirmed at open.
    assert_eq!(
        session.permission_mode(),
        ClaudeAgentSdkPermissionMode::Plan
    );
    let _ = close_session(session, cleanup_request(), cleanup_services);
}

#[test]
fn a_write_driver_cannot_open_a_read_only_plan() {
    // The profile and the plan are one decision. A driver admitting writes
    // against a read-only prepared plan is refused before any acquisition.
    let host = host_id("claude-agent-sdk.fixture.profile-drift");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let driver = prepared
        .low_level_driver()
        .with_session_profile(read_write());
    let services = fixture.services(host);
    let Err(error) = block_on(driver.open_session(
        prepared.plan().clone(),
        prepared.request().clone(),
        services,
    )) else {
        panic!("a widened driver profile must fail closed");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.request_plan_mismatch"
    );
    // Nothing was acquired: the refusal precedes every host effect.
    assert_eq!(fixture.credential_acquisitions(), 0);
    assert!(fixture.cleanup_events().is_empty());
}

#[test]
fn a_permission_mode_other_than_the_requested_one_fails_open_closed() {
    let host = host_id("claude-agent-sdk.fixture.mode-drift");
    let fixture = SdkFixtureHost::new(SdkScenario::PermissionModeDrift);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("a drifted permission mode must fail closed");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_mismatch"
    );
}

#[test]
fn a_mid_session_mode_change_round_trips_the_confirmed_mode() {
    let host = host_id("claude-agent-sdk.fixture.mode-change");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session =
        block_on(prepared.open_route_session(services.clone())).expect("session opens");
    assert_eq!(
        session.permission_mode(),
        ClaudeAgentSdkPermissionMode::Default
    );

    let planned = block_on(session.set_permission_mode(
        ClaudeAgentSdkPermissionMode::Plan,
        services.clone(),
        mode_deadline(),
    ))
    .expect("plan is confirmed");
    assert_eq!(planned, ClaudeAgentSdkPermissionMode::Plan);
    assert_eq!(
        session.permission_mode(),
        ClaudeAgentSdkPermissionMode::Plan
    );

    let restored = block_on(session.set_permission_mode(
        ClaudeAgentSdkPermissionMode::Default,
        services.clone(),
        mode_deadline(),
    ))
    .expect("default is confirmed");
    assert_eq!(restored, ClaudeAgentSdkPermissionMode::Default);
    assert_eq!(
        session.permission_mode(),
        ClaudeAgentSdkPermissionMode::Default
    );

    // The admitted tool set never moved with the mode.
    assert!(!session.session_profile().admits(ClaudeAgentSdkTool::Write));
    assert!(session.session_profile().admits(ClaudeAgentSdkTool::Read));
    let changes: Vec<_> = fixture
        .inputs()
        .into_iter()
        .filter(|value| value["command"] == "set_permission_mode")
        .map(|value| {
            value["params"]["mode"]
                .as_str()
                .unwrap_or_default()
                .to_owned()
        })
        .collect();
    assert_eq!(changes, ["plan".to_owned(), "default".to_owned()]);
    let _ = close_session(session, cleanup_request(), cleanup_services);
}

#[test]
fn a_rejected_or_unconfirmed_mode_change_is_never_a_silent_success() {
    for (scenario, code) in [
        (
            SdkScenario::PermissionModeRejected,
            "swallowtail.claude-agent.sdk.permission_mode_rejected",
        ),
        (
            SdkScenario::PermissionModeUnconfirmed,
            "swallowtail.claude-agent.sdk.permission_mode_unconfirmed",
        ),
    ] {
        let host = host_id("claude-agent-sdk.fixture.mode-refused");
        let fixture = SdkFixtureHost::new(scenario);
        let prepared = prepared_session(host.clone());
        let services = fixture.services(host);
        let cleanup_services = services.clone();
        let mut session =
            block_on(prepared.open_route_session(services.clone())).expect("session opens");
        let Err(error) = block_on(session.set_permission_mode(
            ClaudeAgentSdkPermissionMode::Plan,
            services.clone(),
            mode_deadline(),
        )) else {
            panic!("{scenario:?} must fail rather than report success");
        };
        assert_eq!(error.diagnostic().code(), code);
        // The session keeps the last mode it actually had.
        assert_eq!(
            session.permission_mode(),
            ClaudeAgentSdkPermissionMode::Default
        );
        let _ = close_session(session, cleanup_request(), cleanup_services);
    }
}
