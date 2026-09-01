#[path = "support/discovery.rs"]
mod support;

use futures_executor::block_on;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, ClaudeAgentAcpDriver, claude_agent_acp_claim,
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
    }
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
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("claude-agent-version-probe").expect("valid request"),
        ScopeId::new("claude-agent-version-probe").expect("valid scope"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("claude-agent.fixture.executable").expect("valid executable"),
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("valid axis"),
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
