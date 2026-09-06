use crate::discovery_support;

use discovery_support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_CODE_HEADLESS_AXIS, CLAUDE_CODE_RESPONSE_ONLY_AXIS,
    ClaudeAgentAcpDriver, ClaudeCodeHeadlessDriver, ClaudeCodeResponseOnlyDriver,
    claude_agent_acp_claim,
};
use swallowtail_core::{
    DiscoveryStatus, ExecutionHostId, InstalledExecutableCompatibility, InterfaceVersionAxis,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, MonotonicInstant, RequestId,
    ScopeId,
};

#[test]
fn exact_wrapper_versions_probe_only_the_host_approved_target() {
    for (version, qualified) in [
        ("0.53.0", true),
        ("0.58.1", true),
        ("0.61.0", true),
        ("0.62.0", true),
        ("0.63.0", true),
        ("0.64.0", true),
        ("0.65.0", true),
        ("0.69.0", true),
        ("0.70.0", true),
        ("0.71.0", true),
        ("0.72.0", true),
        ("0.73.0", true),
        ("0.74.0", false),
    ] {
        let host_id = ExecutionHostId::new("fixture.host.discovery").expect("valid host");
        let host = FixtureHost::new(Scenario::Version, version);
        let outcome = block_on(driver().discover_installed_executable(
            request(host_id.clone()),
            host.services(host_id.clone()),
        ))
        .expect("discovery completes");
        assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
        let observation = outcome
            .installed_executable_observation()
            .expect("observation exists");
        assert_eq!(observation.execution_host_id(), &host_id);
        assert_eq!(observation.version().version().as_str(), version);
        assert_eq!(observation.claim_id(), claude_agent_acp_claim().id());
        assert_eq!(observation.is_qualified(), qualified);
        if !qualified {
            assert!(matches!(
                observation.compatibility(),
                InstalledExecutableCompatibility::UnverifiedNewer(_)
            ));
        }
        let process = host.observed_process();
        assert_eq!(process.executable, "claude-agent.fixture.executable");
        assert_eq!(process.arguments, ["--version"]);
        assert_eq!(process.environment_count, 0);
        assert!(process.working_resource.is_none());
        assert_eq!(host.credential_acquires(), 0);
        assert!(outcome.install_guidance().is_none());
    }
}

#[test]
fn claude_code_routes_probe_without_executing_install_guidance() {
    let host_id = ExecutionHostId::new("fixture.host.claude-code.discovery").expect("valid host");
    let headless_host = FixtureHost::new(Scenario::Version, "2.1.257 (Claude Code)");
    let headless = block_on(
        ClaudeCodeHeadlessDriver::new(
            EnvironmentRef::new("claude-code.fixture.environment").expect("environment"),
        )
        .discover_installed_executable(
            request_for_axis(host_id.clone(), CLAUDE_CODE_HEADLESS_AXIS),
            headless_host.services(host_id.clone()),
        ),
    )
    .expect("headless discovery completes");
    assert_eq!(headless.status(), DiscoveryStatus::Discovered);
    assert!(headless.install_guidance().is_none());
    let headless_process = headless_host.observed_process();
    assert_eq!(headless_process.arguments, ["--version"]);
    assert_eq!(headless_process.environment_count, 0);
    assert!(headless_process.working_resource.is_none());

    let response_host = FixtureHost::new(Scenario::Version, "2.1.257 (Claude Code)");
    let response = block_on(
        ClaudeCodeResponseOnlyDriver::new(
            EnvironmentRef::new("claude-code.fixture.environment").expect("environment"),
        )
        .discover_installed_executable(
            request_for_axis(host_id.clone(), CLAUDE_CODE_RESPONSE_ONLY_AXIS),
            response_host.services(host_id),
        ),
    )
    .expect("response-only discovery completes");
    assert_eq!(response.status(), DiscoveryStatus::Discovered);
    assert!(response.install_guidance().is_none());
    let response_process = response_host.observed_process();
    assert_eq!(response_process.arguments, ["--version"]);
    assert_eq!(response_process.environment_count, 0);
    assert!(response_process.working_resource.is_none());
}

#[test]
fn missing_claude_code_executable_is_absent_with_guidance() {
    let host_id = ExecutionHostId::new("fixture.host.claude-code.missing").expect("valid host");
    let headless_host = FixtureHost::new(Scenario::Version, "unused").with_missing_executable();
    let headless = block_on(
        ClaudeCodeHeadlessDriver::new(
            EnvironmentRef::new("claude-code.fixture.environment").expect("environment"),
        )
        .discover_installed_executable(
            request_for_axis_and_executable(
                host_id.clone(),
                CLAUDE_CODE_HEADLESS_AXIS,
                "claude-code.missing.fixture.executable",
            ),
            headless_host.services(host_id.clone()),
        ),
    )
    .expect("headless discovery completes");
    assert_eq!(headless.status(), DiscoveryStatus::Absent);
    let headless_guidance = headless.install_guidance().expect("headless guidance");
    assert_eq!(headless_guidance.display_name(), "Claude Code");
    assert_eq!(headless_guidance.frozen_on(), "2026-09-06");
    assert!(!headless_host.process_started());

    let response_host = FixtureHost::new(Scenario::Version, "unused").with_missing_executable();
    let response = block_on(
        ClaudeCodeResponseOnlyDriver::new(
            EnvironmentRef::new("claude-code.fixture.environment").expect("environment"),
        )
        .discover_installed_executable(
            request_for_axis_and_executable(
                host_id.clone(),
                CLAUDE_CODE_RESPONSE_ONLY_AXIS,
                "claude-code.missing.fixture.executable",
            ),
            response_host.services(host_id),
        ),
    )
    .expect("response-only discovery completes");
    assert_eq!(response.status(), DiscoveryStatus::Absent);
    let response_guidance = response.install_guidance().expect("response-only guidance");
    assert_eq!(response_guidance.display_name(), "Claude Code");
    assert_eq!(response_guidance.frozen_on(), "2026-09-06");
    assert!(!response_host.process_started());
}

#[test]
fn excluded_and_incompatible_versions_remain_distinct() {
    for version in ["0.52.0", "0.58.0"] {
        let host_id = ExecutionHostId::new("fixture.host.incompatible").expect("valid host");
        let host = FixtureHost::new(Scenario::Version, version);
        let outcome = block_on(
            driver()
                .discover_installed_executable(request(host_id.clone()), host.services(host_id)),
        )
        .expect("discovery completes");
        assert_eq!(outcome.status(), DiscoveryStatus::Incompatible);
        assert!(
            outcome
                .installed_executable_observation()
                .is_some_and(|observation| !observation.is_permitted())
        );
    }
}

fn request(host: ExecutionHostId) -> InstalledExecutableDiscoveryRequest {
    request_for_axis(host, CLAUDE_AGENT_ACP_AXIS)
}

fn request_for_axis(host: ExecutionHostId, axis: &str) -> InstalledExecutableDiscoveryRequest {
    request_for_axis_and_executable(host, axis, "claude-agent.fixture.executable")
}

fn request_for_axis_and_executable(
    host: ExecutionHostId,
    axis: &str,
    executable: &str,
) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("claude-agent-version-probe").expect("valid request"),
        ScopeId::new("claude-agent-version-probe").expect("valid scope"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new(executable).expect("valid executable"),
            InterfaceVersionAxis::new(axis).expect("valid axis"),
        ),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn driver() -> ClaudeAgentAcpDriver {
    ClaudeAgentAcpDriver::new(
        EnvironmentRef::new("claude-agent.fixture.environment").expect("valid environment"),
        swallowtail_core::CredentialRef::new("claude-agent.fixture.api-key")
            .expect("valid credential"),
    )
}
