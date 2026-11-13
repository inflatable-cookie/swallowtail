use super::{LocalCapturedGraph, capture, capture_native_graph, fixture};
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::Arc;
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
    LocalCapturedNodeRecipe, LocalCheckedNodeLaunch, LocalExecutableLaunch, LocalProcessHost,
    LocalProcessLimits, LocalTrustedRuntime,
};
use swallowtail_runtime::{
    CheckedProcessRef, CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef,
    ExecutableRef, InstalledExecutableTarget, MonotonicInstant, OperationContent,
    PreparedAccessEvidence, ProcessRequest, ProcessService, RequestId, RuntimeTurnId, ScopeId,
    SessionOptions, TerminalStatus, TurnRequest, WorkingResourceRef,
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

struct CheckedChild {
    local: LocalProcessHost,
    runtime: LocalTrustedRuntime,
    approval: LocalCheckedNodeLaunch,
}

fn node_path() -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").expect("PATH"))
        .map(|dir| dir.join("node"))
        .find(|candidate| candidate.is_file())
        .expect("fake Node interpreter")
}

fn fake_child(root: &std::path::Path, graph: LocalCapturedGraph) -> CheckedChild {
    fake_child_with_input_limit(root, graph, 8 * 1024 * 1024)
}

fn fake_child_with_input_limit(
    root: &std::path::Path,
    graph: LocalCapturedGraph,
    stdin_bytes: usize,
) -> CheckedChild {
    fake_child_with_environment(root, graph, stdin_bytes, vec![])
}

fn fake_child_with_environment(
    root: &std::path::Path,
    graph: LocalCapturedGraph,
    stdin_bytes: usize,
    environment_values: Vec<(std::ffi::OsString, std::ffi::OsString)>,
) -> CheckedChild {
    let original = node_path().canonicalize().expect("canonical Node path");
    let node = root.join("fixture-node");
    if !node.exists() {
        std::fs::copy(&original, &node).expect("copy fake interpreter into owned fixture");
    }
    // Homebrew's Node can use @loader_path for libnode.  Keep this fake
    // runnable after copying without changing the host installation.
    #[cfg(target_os = "macos")]
    {
        let lib = original
            .parent()
            .and_then(|p| p.parent())
            .expect("Node prefix")
            .join("lib");
        if let Ok(entries) = std::fs::read_dir(lib) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("libnode.") && name_str.ends_with(".dylib") {
                    let destination = root.join(name);
                    if !destination.exists() {
                        std::fs::copy(entry.path(), destination).expect("copy fake Node library");
                    }
                }
            }
        }
    }
    let node = node.canonicalize().expect("canonical fixture interpreter");
    let bytes = std::fs::read(&node).expect("read fake interpreter");
    let dependencies: Vec<_> = std::fs::read_dir(root)
        .expect("fixture directory")
        .map(|entry| entry.expect("dependency entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "dylib")
        })
        .map(|path| {
            let digest = Sha256::digest(std::fs::read(&path).expect("dependency source")).into();
            (path, digest)
        })
        .collect();
    let runtime = LocalTrustedRuntime::capture_with_dependencies(
        &node,
        Sha256::digest(&bytes).into(),
        &dependencies,
    )
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
    let launch = LocalExecutableLaunch::new(node)
        .with_prefix_arguments(recipe.prefix_arguments().map(std::ffi::OsStr::to_os_string))
        .with_node_startup_environment_filter();
    let executable = ExecutableRef::new("claude-agent.prepared.executable").expect("target");
    let environment =
        EnvironmentRef::new("claude-agent.prepared.environment").expect("environment");
    let resource = WorkingResourceRef::new("checked.workspace").expect("resource");
    assert!(
        LocalCheckedNodeLaunch::new(
            executable.clone(),
            runtime.clone(),
            recipe.clone(),
            [0; 32],
            vec![vec![]],
        )
        .is_err(),
        "a caller cannot substitute the host's approved identity"
    );
    let approval = LocalCheckedNodeLaunch::new(
        executable.clone(),
        runtime.clone(),
        recipe,
        expected,
        vec![vec![], vec!["--version".into()]],
    )
    .expect("host-owned fake graph policy");
    let local = LocalProcessHost::builder(LocalProcessLimits::new(
        128,
        256 * 1024,
        stdin_bytes,
        8 * 1024 * 1024,
        1024 * 1024,
    ))
    .approve_executable_launch(executable, launch.clone())
    .approve_environment(environment, environment_values)
    .approve_working_resource(resource, root)
    .approve_checked_node(
        CheckedProcessRef::new("fixture.checked.binding").expect("binding"),
        approval.clone(),
    )
    .expect("register fake graph in production host")
    .build();
    CheckedChild {
        local,
        runtime,
        approval,
    }
}

#[test]
fn checked_production_host_rejects_startup_controls_and_runtime_dependency_drift() {
    let root = fixture();
    let graph = capture(&root, ACP_ENTRY);
    let unsafe_host = fake_child_with_environment(
        &root,
        graph.clone(),
        1024,
        vec![("NODE_OPTIONS".into(), "--require=unapproved.js".into())],
    );
    let scope = ScopeId::new("checked-env").expect("scope");
    let binding = CheckedProcessRef::new("fixture.checked.binding").expect("binding");
    let request = ProcessRequest::new(
        ExecutableRef::new("claude-agent.prepared.executable").expect("target"),
    )
    .with_environment([
        EnvironmentRef::new("claude-agent.prepared.environment").expect("environment")
    ]);
    assert!(
        block_on(unsafe_host.local.validate_checked(
            scope.clone(),
            binding.clone(),
            request.clone()
        ))
        .is_err()
    );
    assert!(
        block_on(
            unsafe_host
                .local
                .start_checked(scope.clone(), binding.clone(), request.clone())
        )
        .is_err()
    );
    let library = root.join("fixture-dependency.dylib");
    std::fs::write(&library, b"host-owned dependency").expect("dependency fixture");
    let host = fake_child(&root, graph);
    host.runtime
        .verify()
        .expect("complete pinned fixture closure");
    std::fs::write(library, b"substituted dependency").expect("dependency replacement");
    assert!(
        block_on(
            host.local
                .validate_checked(scope.clone(), binding.clone(), request.clone())
        )
        .is_err()
    );
    assert!(block_on(host.local.start_checked(scope, binding, request)).is_err());
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
    let graph = capture_native_graph(&root, ACP_ENTRY);
    std::fs::write(root.join("entry.mjs"), "throw new Error('replaced entry')")
        .expect("replace entry");
    std::fs::write(root.join("idle.mjs"), "throw new Error('replaced idle')")
        .expect("replace idle");
    let constrained = fake_child_with_input_limit(&root, graph.clone(), 4);
    let constrained_host = ExecutionHostId::new("fake.checked.constrained").expect("host");
    let constrained_resources = FixtureHost::new(Scenario::Success, "0.100.0");
    let constrained_services = constrained_resources
        .services(constrained_host.clone())
        .with_process(Arc::new(constrained.local.clone()));
    let constrained_probe = ClaudeAgentPreparationProbe::new(
        RequestId::new("checked-short").expect("request"),
        ScopeId::new("checked-short").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    );
    let constrained_binding = CheckedProcessRef::new("fixture.checked.binding").expect("binding");
    let constrained_result = block_on(prepare_claude_agent(
        preparation(constrained_host).with_checked_process(constrained_binding),
        constrained_probe,
        constrained_services,
    ));
    assert!(constrained_result.is_ok(), "{constrained_result:?}");
    let fake = fake_child(&root, graph);
    let request = ProcessRequest::new(
        ExecutableRef::new("claude-agent.prepared.executable").expect("target"),
    );
    let binding = CheckedProcessRef::new("fixture.checked.binding").expect("binding");
    let scope = ScopeId::new("checked-refusals").expect("scope");
    assert!(block_on(fake.local.start(scope.clone(), request.clone())).is_err());
    assert!(
        block_on(fake.local.start_checked(
            scope.clone(),
            CheckedProcessRef::new("unknown").expect("binding"),
            request.clone()
        ))
        .is_err()
    );
    assert!(
        block_on(fake.local.start_checked(
            scope,
            binding,
            request.with_arguments(["--eval".into(), "process.stdout.write('effect')".into()])
        ))
        .is_err()
    );
    let host_id = ExecutionHostId::new("fake.checked.local").expect("host");
    let resources = FixtureHost::new(Scenario::Success, "0.100.0");
    let services = resources
        .services(host_id.clone())
        .with_process(Arc::new(fake.local.clone()));
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
    fake.approval.revoke();
    assert!(block_on(profile.open_session(services.clone())).is_err());
    fake.runtime.revoke();
    assert!(block_on(profile.open_session(services)).is_err());
    assert_eq!(resources.resource_releases(), 3);
}

#[test]
fn checked_prepared_child_preserves_denial_without_permission_callback() {
    let root = fixture();
    let entry = ACP_ENTRY.replace("0.100.0", "0.106.0").replace(
        "text:'fake captured response'", "text:'The read was denied; continuing.'"
    ).replace(
        "else if (m.method === 'session/prompt') {",
        r#"else if (m.method === 'session/prompt') {
          for (const update of [
            {sessionUpdate:'tool_call',toolCallId:'denied',title:'Read fixture',kind:'read',status:'in_progress'},
            {sessionUpdate:'tool_call_update',toolCallId:'denied',status:'failed'},
          ]) process.stdout.write(JSON.stringify({jsonrpc:'2.0',method:'session/update',params:{sessionId:'fake-captured-session',update}})+'\n');"#
    );
    let graph = capture_native_graph(&root, &entry);
    let fake = fake_child(&root, graph);
    let host_id = ExecutionHostId::new("fake.checked.denied").expect("host");
    let resources = FixtureHost::new(Scenario::Success, "0.106.0");
    let services = resources
        .services(host_id.clone())
        .with_process(Arc::new(fake.local));
    let prepared = block_on(prepare_claude_agent(
        preparation(host_id).with_checked_process(
            CheckedProcessRef::new("fixture.checked.binding").expect("binding"),
        ),
        ClaudeAgentPreparationProbe::new(
            RequestId::new("denied-probe").expect("request"),
            ScopeId::new("denied-probe").expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        ),
        services.clone(),
    ))
    .expect("checked fake discovery");
    assert!(!prepared.observation().is_qualified());
    let profile = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            RequestId::new("denied-open").expect("request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("checked.route").expect("route"),
                ModelRouteRevision::new("1").expect("revision"),
                ModelId::new("claude-sonnet-4-6").expect("model"),
            ),
            WorkingResourceRef::new("checked.workspace").expect("workspace"),
            SessionOptions::default(),
        ))
        .expect("profile");
    let mut session = block_on(profile.open_session(services.clone())).expect("checked session");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("denied-turn").expect("turn"),
            OperationContent::new("attempt a read").expect("prompt"),
        ),
        services.clone(),
    ))
    .expect("turn");
    let mut events = turn.take_events().expect("events");
    let terminal = turn.take_terminal_outcome().expect("terminal");
    let (observed, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.expect("event"));
        }
        (observed, terminal.await)
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("continued output").as_str(),
        "The read was denied; continuing."
    );
    assert!(!observed.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::CallbackRequested(_)
    )));
    let activity = observed
        .iter()
        .position(|event| {
            matches!(
                event.kind(),
                swallowtail_runtime::RuntimeEventKind::Activity(_)
            )
        })
        .expect("denied activity");
    let output = observed
        .iter()
        .position(|event| {
            matches!(
                event.kind(),
                swallowtail_runtime::RuntimeEventKind::OutputDelta
            )
        })
        .expect("continued output");
    assert!(activity < output);
    assert_eq!(resources.reads(), 0);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(resources.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
    assert_eq!(resources.resource_releases(), 1);
}
