//! Provider-free proofs for the Claude Agent SDK registered-only session.

use crate::claude_agent_sdk_driver::registered_tool_route::{courier_binary, preparation_for};
use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, preparation, prepared_session,
};
use futures_executor::block_on;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionPreparation;
use swallowtail_adapter_claude_agent::sdk::registered_tool::{
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER, ClaudeAgentSdkRegisteredOnlyBinding,
};
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkMcpServer, ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile,
    prepare_claude_agent_sdk_session,
};
use swallowtail_core::{
    AccessProfileId, Capability, CapabilityConstraint, ConfiguredInstanceId, CredentialRef,
    ExecutionHostId, InstanceRevision, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, ResourceAccess,
};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BoxFuture, Deadline, EnvironmentRef, ExecutableRef, InteractiveSessionHandle, MonotonicInstant,
    RegisteredToolCall, RegisteredToolDispatchContext, RegisteredToolDispatcher,
    RegisteredToolOutcome, RegisteredToolPayload, RegisteredToolResult, RegisteredToolSchemaDigest,
    RegisteredToolSchemaMediaType, RequestId, RuntimeFailure, SessionAccessPolicy,
    WorkingResourceRef,
};
use swallowtail_testkit::{ScriptedAdmissionPort, fixture_admission};

const CARRIER_TOOL: &str = "mcp__swallowtail-registered-tools__desktop_reconcile";
const OPEN_DEADLINE_TICKS: u64 = 10_000_000_000;

fn open_preparation(host: ExecutionHostId) -> ClaudeAgentSdkSessionPreparation {
    ClaudeAgentSdkSessionPreparation::new(
        ConfiguredInstanceId::new("claude-agent-sdk.fixture").expect("instance"),
        InstanceRevision::new("fixture-revision").expect("revision"),
        host,
        InstanceTargetRef::new("claude-agent-sdk.fixture.launch-recipe").expect("target"),
        EnvironmentRef::new("claude-agent-sdk.fixture.environment").expect("environment"),
        CredentialRef::new("claude-agent-sdk.fixture.delegated-subscription").expect("credential"),
        AccessProfileId::new("claude-agent-sdk.fixture.subscription").expect("access"),
        ModelRouteId::new("claude-agent-sdk.fixture.route").expect("route"),
        ModelRouteRevision::new("fixture-route-revision").expect("revision"),
        ModelId::new("claude-sonnet-5").expect("model"),
        WorkingResourceRef::new("claude-agent-sdk.fixture.workspace").expect("workspace"),
        RequestId::new("request-1").expect("request"),
        Deadline::at(MonotonicInstant::from_ticks(OPEN_DEADLINE_TICKS)),
    )
}

struct CountingDispatcher {
    calls: Arc<AtomicUsize>,
}

impl RegisteredToolDispatcher for CountingDispatcher {
    fn dispatch(
        &self,
        call: RegisteredToolCall,
        _context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let payload = RegisteredToolPayload::new(
            RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
            br#"{"ok":true}"#.to_vec(),
            call.binding().effective_bounds().max_result_bytes(),
        )
        .expect("bounded dispatcher result");
        let result = RegisteredToolResult::new(
            payload,
            RegisteredToolSchemaDigest::new("sha256:output").expect("output digest"),
        );
        Box::pin(std::future::ready(Ok(RegisteredToolOutcome::completed(
            &call, result,
        ))))
    }
}

fn executable() -> ExecutableRef {
    ExecutableRef::new("fixture.registered-tool.courier").expect("executable")
}

fn environment() -> EnvironmentRef {
    EnvironmentRef::new("fixture.registered-tool.environment").expect("env")
}

fn local_host(host: swallowtail_core::ExecutionHostId) -> LocalHostServices {
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected);
    LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable(), courier_binary())
        .approve_environment(environment(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }))
        .with_registered_tool_clock(Arc::new(fixture))
        .build_services(host)
}

fn registered_preparation(
    host: swallowtail_core::ExecutionHostId,
) -> swallowtail_runtime::RegisteredToolPreparation {
    preparation_for(
        host,
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        executable(),
        environment(),
    )
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

fn fixture_mcp_server() -> ClaudeAgentSdkMcpServer {
    ClaudeAgentSdkMcpServer::stdio(
        "fixture",
        "/usr/bin/node",
        ["server.mjs"],
        ["PATH", "HOME"],
        ["search"],
    )
    .expect("fixture MCP server is admissible")
}

#[test]
fn registered_only_read_and_read_write_bind_the_explicit_lease() {
    for access in [ResourceAccess::Read, ResourceAccess::ReadWrite] {
        let host = host_id("claude-agent-sdk.fixture.registered-only-plan");
        let local = local_host(host.clone());
        let binding = ClaudeAgentSdkRegisteredOnlyBinding::new(
            registered_preparation(host.clone()),
            access,
            ClaudeAgentSdkPermissionMode::Default,
        )
        .expect("mutating registered tools still qualify")
        .with_host(local);
        assert_eq!(binding.resource_access(), access);
        assert!(binding.session_profile().is_registered_only());
        assert!(!binding.session_profile().admits_writes());
        let prepared = prepare_claude_agent_sdk_session(
            preparation(host)
                .with_registered_only_binding(binding)
                .expect("registered-only preparation binds"),
            swallowtail_runtime::SessionOptions::default(),
        )
        .expect("registered-only preparation succeeds");
        assert!(prepared.session_profile().is_registered_only());
        assert_eq!(prepared.session_profile().resource_access(), access);
        assert_eq!(
            prepared.request().access_policy(),
            &SessionAccessPolicy::ambient_harness(access)
        );
        assert_eq!(plan_resource_access(prepared.plan()), Some(access));
        let policy = match access {
            ResourceAccess::ReadWrite => "claude-agent-sdk-ambient-read-write",
            ResourceAccess::Read => "claude-agent-sdk-ambient-read",
        };
        assert_eq!(prepared.plan().instance_policy_id().as_str(), policy);
    }
}

#[test]
fn an_extracted_registered_only_profile_cannot_prepare_without_its_binding() {
    let host = host_id("claude-agent-sdk.fixture.registered-only-unbound");
    let binding = ClaudeAgentSdkRegisteredOnlyBinding::new(
        registered_preparation(host.clone()),
        ResourceAccess::ReadWrite,
        ClaudeAgentSdkPermissionMode::Default,
    )
    .expect("registered-only construction qualifies");
    let Err(failure) = prepare_claude_agent_sdk_session(
        preparation(host).with_session_profile(binding.session_profile()),
        swallowtail_runtime::SessionOptions::default(),
    ) else {
        panic!("empty native without the registered binding must fail before a plan");
    };
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.profile.registered_only_unbound"
    );
}

#[test]
fn consumer_mcp_cannot_share_a_registered_only_session() {
    let host = host_id("claude-agent-sdk.fixture.registered-only-mcp");
    let local = local_host(host.clone());
    let mcp = ClaudeAgentSdkSessionProfile::read_only()
        .with_mcp_servers([fixture_mcp_server()])
        .expect("consumer MCP binding is admissible");
    let Err(failure) = preparation(host.clone())
        .with_mcp_binding(mcp)
        .with_registered_only(
            registered_preparation(host),
            local,
            ResourceAccess::ReadWrite,
        )
    else {
        panic!("registered-only rejects consumer MCP");
    };
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.profile.registered_only_mcp_conflict"
    );
}

#[test]
fn additive_registered_tools_do_not_infer_write_access_from_mutating_effect() {
    let host = host_id("claude-agent-sdk.fixture.registered-additive-access");
    let local = local_host(host.clone());
    let prepared = prepare_claude_agent_sdk_session(
        preparation(host.clone())
            .with_registered_tools(registered_preparation(host), local)
            .expect("additive registration still qualifies"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("additive registration still prepares");
    assert!(!prepared.session_profile().is_registered_only());
    assert_eq!(
        prepared.session_profile(),
        ClaudeAgentSdkSessionProfile::read_only()
    );
    assert_eq!(
        prepared.session_profile().resource_access(),
        ResourceAccess::Read
    );
    assert_eq!(
        plan_resource_access(prepared.plan()),
        Some(ResourceAccess::Read)
    );
}

#[test]
fn registered_only_open_admits_only_carrier_spellings() {
    let host = host_id("claude-agent-sdk.fixture.registered-only-open");
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected);
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable(), courier_binary())
        .approve_environment(environment(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }))
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let prepared = prepare_claude_agent_sdk_session(
        open_preparation(host.clone())
            .with_registered_only(
                preparation_for(
                    host,
                    fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
                    executable(),
                    environment(),
                ),
                local,
                ResourceAccess::ReadWrite,
            )
            .expect("registered-only open binds the host"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("registered-only open prepares");
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("registered-only opens");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("registered-only open is on the wire");
    assert_eq!(open["params"]["tools"], serde_json::json!([CARRIER_TOOL]));
    assert!(open["params"].get("allowedTools").is_none());
    let servers = open["params"]["mcpServers"]
        .as_array()
        .expect("courier is declared");
    assert_eq!(servers.len(), 1);
    assert_eq!(servers[0]["name"], CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER);
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn a_registered_only_write_session_cannot_open_on_a_read_only_lease() {
    let host = host_id("claude-agent-sdk.fixture.registered-only-lease");
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected).granting_read_only_resource();
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable(), courier_binary())
        .approve_environment(environment(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }))
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let prepared = prepare_claude_agent_sdk_session(
        preparation(host.clone())
            .with_registered_only(
                preparation_for(
                    host,
                    fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
                    executable(),
                    environment(),
                ),
                local,
                ResourceAccess::ReadWrite,
            )
            .expect("registered-only write still qualifies"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("registered-only write prepares");
    let Err(error) = block_on(prepared.open_route_session(services)) else {
        panic!("a read-only lease must refuse a registered-only write session");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.session_access.resource_access_mismatch"
    );
}

#[test]
fn additive_open_still_prefixes_native_tools() {
    let host = host_id("claude-agent-sdk.fixture.registered-additive-open");
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected);
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable(), courier_binary())
        .approve_environment(environment(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }))
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let prepared = prepare_claude_agent_sdk_session(
        open_preparation(host.clone())
            .with_registered_tools(
                preparation_for(
                    host,
                    fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
                    executable(),
                    environment(),
                ),
                local,
            )
            .expect("additive registration still binds"),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("additive registration still prepares");
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("additive open succeeds");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("additive open is on the wire");
    assert_eq!(
        open["params"]["tools"],
        serde_json::json!(["Read", "Glob", "Grep", CARRIER_TOOL])
    );
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn the_default_profile_is_unchanged_without_registered_tools() {
    let prepared = prepared_session(host_id(
        "claude-agent-sdk.fixture.registered-only-default-stable",
    ));
    assert!(!prepared.session_profile().is_registered_only());
    assert_eq!(
        prepared.session_profile(),
        ClaudeAgentSdkSessionProfile::read_only()
    );
}
