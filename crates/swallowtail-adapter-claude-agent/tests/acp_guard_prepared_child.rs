use super::{LocalCapturedGraph, capture, fixture};
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE, ClaudeAgentModelSelection,
    ClaudeAgentPreparationInput, ClaudeAgentPreparationProbe, ClaudeAgentSessionProfileInput,
    prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{
    LocalCapturedNodeRecipe, LocalExecutableLaunch, LocalProcessHost, LocalProcessLimits,
    LocalTrustedRuntime,
};
use swallowtail_runtime::{
    BoxFuture, CheckedProcessRef, CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef,
    ExecutableRef, InstalledExecutableTarget, MonotonicInstant, OperationContent,
    PreparedAccessEvidence, ProcessHandle, ProcessRequest, ProcessService, RequestId,
    RuntimeFailure, RuntimeTurnId, ScopeId, SessionOptions, TerminalStatus, TurnRequest,
    WorkingResourceRef,
};

const ACP_ENTRY: &str = r#"
import { IdleCompaction } from './idle.mjs';
const idle = new IdleCompaction(); idle.onIdle(); await idle.fire();
if (idle.armed || idle.fired) throw new Error('idle was not vetoed');
if (process.argv.includes('--version')) { process.stdout.write('0.100.0\n'); }
else {
  let buffer = ''; let model = 'default';
  const options = () => [{type:'select',id:'model',name:'Model',category:'model',currentValue:model,options:[{value:'default',name:'Default'},{value:'claude-sonnet-4-6',name:'Sonnet'}]}];
  const reply = (id, result) => process.stdout.write(JSON.stringify({jsonrpc:'2.0',id,result})+'\n');
  process.stdin.setEncoding('utf8');
  process.stdin.on('data', chunk => {
    buffer += chunk;
    for (;;) {
      const i = buffer.indexOf('\n'); if (i < 0) break;
      const m = JSON.parse(buffer.slice(0,i)); buffer = buffer.slice(i+1);
      if (m.method === 'initialize') reply(m.id, {protocolVersion:1,agentCapabilities:{_meta:{claudeCode:{promptQueueing:true}},promptCapabilities:{image:true,embeddedContext:true},loadSession:true,sessionCapabilities:{close:{},delete:{},resume:{}},providers:{}},agentInfo:{name:'@agentclientprotocol/claude-agent-acp',version:'0.100.0'},authMethods:[],_meta:{steering:{supported:true}}});
      else if (m.method === 'session/new') reply(m.id, {sessionId:'fake-captured-session',modes:{currentModeId:'default',availableModes:[{id:'default',name:'Default'}]},configOptions:options()});
      else if (m.method === 'session/set_config_option') { model = m.params.value; reply(m.id,{configOptions:options()}); }
      else if (m.method === 'session/prompt') {
        process.stdout.write(JSON.stringify({jsonrpc:'2.0',method:'session/update',params:{sessionId:'fake-captured-session',update:{sessionUpdate:'agent_message_chunk',content:{type:'text',text:'fake captured response'}}}})+'\n');
        reply(m.id,{stopReason:'end_turn',usage:{inputTokens:1,outputTokens:1,cachedReadTokens:0,cachedWriteTokens:0,totalTokens:2}});
      }
      else if (m.method === 'session/close') { reply(m.id,{}); }
      else throw new Error('unexpected fake method');
    }
  });
}
"#;

#[derive(Clone)]
struct FakeCheckedChild {
    local: LocalProcessHost,
    graph: LocalCapturedGraph,
    runtime: LocalTrustedRuntime,
    launch: LocalExecutableLaunch,
    expected: [u8; 32],
    revoked: Arc<AtomicBool>,
    checked: Arc<AtomicUsize>,
    ordinary: Arc<AtomicUsize>,
    bootstrap: Arc<[u8]>,
}
impl FakeCheckedChild {
    fn allows(&self, binding: &CheckedProcessRef, request: &ProcessRequest) -> bool {
        let candidate = LocalCapturedNodeRecipe::for_trusted_runtime(
            &self.graph,
            CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
            [1; 32],
            &self.runtime,
            [5; 32],
        );
        !self.revoked.load(Ordering::SeqCst)
            && binding.as_host_value() == "fixture.checked.binding"
            && request.executable().as_host_value() == "claude-agent.prepared.executable"
            && (request.arguments().len() == 0
                || request.arguments().collect::<Vec<_>>() == ["--version"])
            && candidate.as_ref().is_ok_and(|value| {
                value.fingerprint().digest() == &self.expected
                    && value.bootstrap() == self.bootstrap.as_ref()
                    && value
                        .verify_trusted_launch(&self.runtime, &self.launch)
                        .is_ok()
            })
    }
    fn denied() -> RuntimeFailure {
        RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
            "fixture.checked.denied",
            "fake checked host denied",
        ))
    }
}
impl ProcessService for FakeCheckedChild {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        self.ordinary.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Err(Self::denied()) })
    }
    fn validate_checked(
        &self,
        _scope: ScopeId,
        binding: CheckedProcessRef,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let accepted = self.allows(&binding, &request);
        Box::pin(async move {
            if accepted {
                Ok(())
            } else {
                Err(Self::denied())
            }
        })
    }
    fn start_checked(
        &self,
        scope: ScopeId,
        binding: CheckedProcessRef,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        if !self.allows(&binding, &request) {
            return Box::pin(async { Err(Self::denied()) });
        }
        self.checked.fetch_add(1, Ordering::SeqCst);
        let local = self.local.clone();
        let runtime = self.runtime.clone();
        let graph = self.graph.clone();
        Box::pin(async move {
            let recipe = LocalCapturedNodeRecipe::for_trusted_runtime(
                &graph,
                CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
                [1; 32],
                &runtime,
                [5; 32],
            )
            .map_err(|_| Self::denied())?;
            local
                .start_trusted_candidate(
                    scope,
                    request,
                    &runtime,
                    &recipe,
                    std::time::Duration::from_secs(5),
                )
                .await
        })
    }
}

fn node_path() -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").expect("PATH"))
        .map(|dir| dir.join("node"))
        .find(|candidate| candidate.is_file())
        .expect("fake Node interpreter")
}

fn fake_child(root: &std::path::Path, graph: LocalCapturedGraph) -> FakeCheckedChild {
    fake_child_with_input_limit(root, graph, 8 * 1024 * 1024)
}

fn fake_child_with_input_limit(
    root: &std::path::Path,
    graph: LocalCapturedGraph,
    stdin_bytes: usize,
) -> FakeCheckedChild {
    let node = node_path().canonicalize().expect("canonical Node path");
    let bytes = std::fs::read(&node).expect("read fake interpreter");
    let runtime = LocalTrustedRuntime::capture(&node, Sha256::digest(&bytes).into())
        .expect("trusted fake-host runtime");
    let recipe = LocalCapturedNodeRecipe::for_trusted_runtime(
        &graph,
        CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
        [1; 32],
        &runtime,
        [5; 32],
    )
    .expect("fake recipe");
    let expected = *recipe.fingerprint().digest();
    let bootstrap: Arc<[u8]> = Arc::from(recipe.bootstrap());
    let launch = LocalExecutableLaunch::new(node)
        .with_prefix_arguments(recipe.prefix_arguments().map(std::ffi::OsStr::to_os_string))
        .with_node_startup_environment_filter();
    let executable = ExecutableRef::new("claude-agent.prepared.executable").expect("target");
    let environment =
        EnvironmentRef::new("claude-agent.prepared.environment").expect("environment");
    let resource = WorkingResourceRef::new("checked.workspace").expect("resource");
    let local = LocalProcessHost::builder(LocalProcessLimits::new(
        128,
        256 * 1024,
        stdin_bytes,
        8 * 1024 * 1024,
        1024 * 1024,
    ))
    .approve_executable_launch(executable, launch.clone())
    .approve_environment(environment, [])
    .approve_working_resource(resource, root)
    .build();
    FakeCheckedChild {
        local,
        graph,
        runtime,
        launch,
        expected,
        revoked: Arc::new(AtomicBool::new(false)),
        checked: Arc::new(AtomicUsize::new(0)),
        ordinary: Arc::new(AtomicUsize::new(0)),
        bootstrap,
    }
}

fn preparation(host: ExecutionHostId) -> ClaudeAgentPreparationInput {
    let id = AccessProfileId::new("checked.local-access").expect("access");
    ClaudeAgentPreparationInput::new(
        ConfiguredInstanceId::new("checked.instance").expect("instance"),
        InstanceRevision::new("1").expect("revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("claude-agent.prepared.executable").expect("target"),
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("claude-agent.prepared.environment").expect("environment"),
        AccessProfile::new(
            id.clone(),
            CredentialMechanism::LocalUnauthenticated,
            EntitlementMetering::SubscriptionAllowance,
            EndpointAudience::new("api.anthropic.com").expect("audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            id,
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        )),
    )
}

#[test]
fn prepared_checked_driver_uses_captured_fake_child_and_joins() {
    let root = fixture();
    let graph = capture(&root, ACP_ENTRY);
    std::fs::write(root.join("entry.mjs"), "throw new Error('replaced entry')")
        .expect("replace entry");
    std::fs::write(root.join("idle.mjs"), "throw new Error('replaced idle')")
        .expect("replace idle");
    let constrained = fake_child_with_input_limit(&root, graph.clone(), 4);
    let constrained_host = ExecutionHostId::new("fake.checked.constrained").expect("host");
    let constrained_resources = FixtureHost::new(Scenario::Success, "0.100.0");
    let constrained_services = constrained_resources
        .services(constrained_host.clone())
        .with_process(Arc::new(constrained.clone()));
    let constrained_probe = ClaudeAgentPreparationProbe::new(
        RequestId::new("checked-short").expect("request"),
        ScopeId::new("checked-short").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    );
    let constrained_binding = CheckedProcessRef::new("fixture.checked.binding").expect("binding");
    assert!(
        block_on(prepare_claude_agent(
            preparation(constrained_host).with_checked_process(constrained_binding),
            constrained_probe,
            constrained_services
        ))
        .is_ok()
    );
    assert_eq!(constrained.checked.load(Ordering::SeqCst), 1);
    assert_eq!(constrained.ordinary.load(Ordering::SeqCst), 0);
    let fake = fake_child(&root, graph);
    let wrong_runtime = LocalCapturedNodeRecipe::new(
        &fake.graph,
        CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
        [1; 32],
        [3; 32],
        [5; 32],
    )
    .expect("candidate");
    assert!(
        wrong_runtime
            .verify_trusted_launch(&fake.runtime, &fake.launch)
            .is_err()
    );
    let mut substituted = fake.clone();
    substituted.bootstrap = Arc::from(b"substituted bootstrap".as_slice());
    let substituted_request = ProcessRequest::new(
        ExecutableRef::new("claude-agent.prepared.executable").expect("target"),
    );
    assert!(
        block_on(substituted.validate_checked(
            ScopeId::new("checked-substitution").expect("scope"),
            CheckedProcessRef::new("fixture.checked.binding").expect("binding"),
            substituted_request
        ))
        .is_err()
    );
    assert_eq!(substituted.checked.load(Ordering::SeqCst), 0);
    let mut unsafe_prefix = fake.clone();
    unsafe_prefix.launch = LocalExecutableLaunch::new(node_path().canonicalize().expect("Node"))
        .with_prefix_arguments(["--require".into(), "unapproved.js".into()])
        .with_node_startup_environment_filter();
    assert!(
        block_on(unsafe_prefix.validate_checked(
            ScopeId::new("checked-prefix").expect("scope"),
            CheckedProcessRef::new("fixture.checked.binding").expect("binding"),
            ProcessRequest::new(
                ExecutableRef::new("claude-agent.prepared.executable").expect("target")
            ),
        ))
        .is_err()
    );
    assert_eq!(unsafe_prefix.checked.load(Ordering::SeqCst), 0);
    let host_id = ExecutionHostId::new("fake.checked.local").expect("host");
    let resources = FixtureHost::new(Scenario::Success, "0.100.0");
    let services = resources
        .services(host_id.clone())
        .with_process(Arc::new(fake.clone()));
    let binding = CheckedProcessRef::new("fixture.checked.binding").expect("binding");
    let probe = ClaudeAgentPreparationProbe::new(
        RequestId::new("checked-probe").expect("request"),
        ScopeId::new("checked-probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    );
    let prepared = block_on(prepare_claude_agent(
        preparation(host_id).with_checked_process(binding),
        probe,
        services.clone(),
    ))
    .expect("fake prepared discovery");
    let profile = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            RequestId::new("checked-open").expect("request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("checked.route").expect("route"),
                ModelRouteRevision::new("1").expect("revision"),
                ModelId::new("claude-sonnet-4-6").expect("model"),
            ),
            WorkingResourceRef::new("checked.workspace").expect("workspace"),
            SessionOptions::default(),
        ))
        .expect("profile");
    let mut session =
        block_on(profile.open_session(services.clone())).expect("captured fake session");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("captured-turn").expect("turn"),
            OperationContent::new("fake prompt").expect("prompt"),
        ),
        services.clone(),
    ))
    .expect("fake turn");
    let terminal = turn.take_terminal_outcome().expect("terminal");
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(resources.cleanup_request(), services.clone())),
        CleanupOutcome::Clean
    );
    fake.runtime.revoke();
    assert!(block_on(profile.open_session(services)).is_err());
    assert_eq!(fake.checked.load(Ordering::SeqCst), 2);
    assert_eq!(fake.ordinary.load(Ordering::SeqCst), 0);
    assert_eq!(resources.resource_releases(), 2);
}
