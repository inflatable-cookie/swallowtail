use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};
use swallowtail_runtime::{BoxFuture, CheckedProcessRef, ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure};
use swallowtail_host_local::{LocalCapturedGraph, LocalCapturedImport, LocalCapturedLaunchFingerprint, LocalCapturedSources, LocalExpectedSource};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct CheckedFixture {
    inner: FixtureHost,
    revoked: Arc<AtomicBool>,
    checked_starts: Arc<AtomicUsize>,
    ordinary_starts: Arc<AtomicUsize>,
    graph: LocalCapturedGraph,
    expected: [u8; 32],
    axes: [[u8; 32]; 5],
}
impl CheckedFixture {
    fn new(inner: FixtureHost, graph: LocalCapturedGraph, expected: [u8; 32]) -> Self {
        Self { inner, revoked: Arc::new(AtomicBool::new(false)), checked_starts: Arc::new(AtomicUsize::new(0)), ordinary_starts: Arc::new(AtomicUsize::new(0)), graph, expected, axes: [[1; 32], [2; 32], [3; 32], [4; 32], [5; 32]] }
    }
    fn accepts(&self, binding: &CheckedProcessRef, request: &ProcessRequest) -> bool {
        let candidate = LocalCapturedLaunchFingerprint::new(&self.graph, self.axes[0], self.axes[1], self.axes[2], self.axes[3], self.axes[4]);
        !self.revoked.load(Ordering::SeqCst)
            && candidate.as_ref().is_ok_and(|value| value.digest() == &self.expected)
            && binding.as_host_value() == "fixture.checked.binding"
            && request.executable().as_host_value() == "claude-agent.prepared.executable"
            && (request.arguments().len() == 0 || request.arguments().collect::<Vec<_>>() == ["--version"])
    }
    fn denied() -> RuntimeFailure {
        RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new("fixture.checked.denied", "checked fixture denied"))
    }
    fn services(&self, host: ExecutionHostId) -> swallowtail_runtime::HostServices {
        self.inner.services(host).with_process(Arc::new(self.clone()))
    }
}
impl ProcessService for CheckedFixture {
    fn start(&self, scope: ScopeId, request: ProcessRequest) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        self.ordinary_starts.fetch_add(1, Ordering::SeqCst);
        self.inner.start(scope, request)
    }
    fn validate_checked(&self, _scope: ScopeId, binding: CheckedProcessRef, request: ProcessRequest) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let accepted = self.accepts(&binding, &request);
        Box::pin(async move { if accepted { Ok(()) } else { Err(Self::denied()) } })
    }
    fn start_checked(&self, scope: ScopeId, binding: CheckedProcessRef, request: ProcessRequest) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        if !self.accepts(&binding, &request) { return Box::pin(async { Err(Self::denied()) }); }
        self.checked_starts.fetch_add(1, Ordering::SeqCst);
        self.inner.start(scope, request)
    }
}

fn checked_fixture_graph(entry: &[u8]) -> (PathBuf, LocalCapturedGraph) {
    let template = std::env::temp_dir().join("prepared-capture.XXXXXXXX");
    let output = std::process::Command::new("/usr/bin/mktemp").arg("-d").arg(template).env_clear().output().expect("mktemp");
    assert!(output.status.success());
    let root = PathBuf::from(String::from_utf8(output.stdout).expect("fixture path").trim());
    std::fs::write(root.join("entry.mjs"), entry).expect("entry");
    std::fs::write(root.join("idle.mjs"), b"fake idle").expect("idle");
    let sources = LocalCapturedSources::capture(&root, &[
        LocalExpectedSource::new("entry.mjs", Sha256::digest(entry).into()),
        LocalExpectedSource::new("idle.mjs", Sha256::digest(b"fake idle").into()),
    ]).expect("captured fixture");
    let graph = LocalCapturedGraph::new(&sources, Path::new("entry.mjs"), Path::new("idle.mjs"), &[LocalCapturedImport::new("entry.mjs", "./idle.mjs", "idle.mjs")]).expect("fixture graph");
    (root, graph)
}

#[cfg(unix)]
#[test]
fn prepared_guarded_binding_refuses_missing_mismatch_and_revocation_without_fallback() {
    let host_id = ExecutionHostId::new("fixture.checked.host").expect("host");
    let (root, graph) = checked_fixture_graph(b"fake entry");
    let expected = *LocalCapturedLaunchFingerprint::new(&graph, [1;32], [2;32], [3;32], [4;32], [5;32]).expect("candidate").digest();
    std::fs::write(root.join("entry.mjs"), b"replaced after capture").expect("replace fixture");
    let captured_config: serde_json::Value = serde_json::from_slice(graph.configuration()).expect("configuration");
    assert_eq!(captured_config["files"][0]["source"], "fake entry");
    let binding = CheckedProcessRef::new("fixture.checked.binding").expect("binding");
    let missing = FixtureHost::new(Scenario::Version, "0.100.0");
    let failure = block_on(prepare_claude_agent(preparation_input(host_id.clone()).with_checked_process(binding.clone()), probe(), missing.services(host_id.clone()))).expect_err("legacy host must refuse before discovery");
    assert_eq!(failure.diagnostic().safe().code(), "swallowtail.process.checked_binding_unavailable");

    let authority = CheckedFixture::new(FixtureHost::new(Scenario::Version, "0.100.0"), graph.clone(), expected);
    let (_, other_graph) = checked_fixture_graph(b"substituted source");
    let other = CheckedFixture::new(FixtureHost::new(Scenario::Version, "0.100.0"), other_graph, expected);
    assert!(block_on(prepare_claude_agent(preparation_input(host_id.clone()).with_checked_process(binding.clone()), probe(), other.services(host_id.clone()))).is_err());
    assert_eq!(other.checked_starts.load(Ordering::SeqCst), 0);
    let mut changed_recipe = authority.clone();
    changed_recipe.axes[3][0] ^= 1;
    assert!(block_on(prepare_claude_agent(preparation_input(host_id.clone()).with_checked_process(binding.clone()), probe(), changed_recipe.services(host_id.clone()))).is_err());
    let mismatch = CheckedProcessRef::new("fixture.other.binding").expect("binding");
    assert!(block_on(prepare_claude_agent(preparation_input(host_id.clone()).with_checked_process(mismatch), probe(), authority.services(host_id.clone()))).is_err());
    assert_eq!(authority.checked_starts.load(Ordering::SeqCst), 0);
    assert_eq!(authority.ordinary_starts.load(Ordering::SeqCst), 0);
    let wrong_target = ProcessRequest::new(ExecutableRef::new("substituted.interpreter").expect("target"));
    assert!(block_on(authority.validate_checked(ScopeId::new("checked").expect("scope"), binding.clone(), wrong_target)).is_err());
    let startup = ProcessRequest::new(ExecutableRef::new("claude-agent.prepared.executable").expect("target"))
        .with_arguments(["--require=unapproved".to_owned()]);
    assert!(block_on(authority.validate_checked(ScopeId::new("checked").expect("scope"), binding.clone(), startup)).is_err());

    let prepared = block_on(prepare_claude_agent(preparation_input(host_id.clone()).with_checked_process(binding), probe(), authority.services(host_id.clone()))).expect("fake checked discovery");
    assert_eq!(authority.checked_starts.load(Ordering::SeqCst), 1);
    let profile = prepared.prepare_session(ClaudeAgentSessionProfileInput::new(
        RequestId::new("checked-open").expect("request"),
        ClaudeAgentModelSelection::new(ModelRouteId::new("checked.route").expect("route"), ModelRouteRevision::new("1").expect("revision"), ModelId::new("claude-sonnet-4-6").expect("model")),
        WorkingResourceRef::new("checked.workspace").expect("resource"),
        SessionOptions::default(),
    )).expect("prepared profile");
    let operation = CheckedFixture::new(FixtureHost::new(Scenario::Success, "0.100.0"), graph.clone(), expected);
    let session = block_on(profile.open_session(operation.services(host_id.clone()))).expect("fake checked session");
    assert_eq!(block_on(session.close(operation.inner.cleanup_request(), operation.services(host_id.clone()))), CleanupOutcome::Clean);
    assert_eq!(operation.checked_starts.load(Ordering::SeqCst), 1);
    assert_eq!(operation.ordinary_starts.load(Ordering::SeqCst), 0);
    let cancel = CheckedFixture::new(FixtureHost::new(Scenario::Cancellation, "0.100.0"), graph.clone(), expected);
    let services = cancel.services(host_id.clone());
    let mut session = block_on(profile.open_session(services.clone())).expect("checked cancellation session");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(RuntimeTurnId::new("checked-turn").expect("turn"), OperationContent::new("pending turn").expect("prompt")),
        services.clone(),
    )).expect("turn starts");
    cancel.inner.wait_for_write("session/prompt");
    block_on(turn.cancellation().request()).expect("cancel requested");
    let terminal = turn.take_terminal_outcome().expect("terminal");
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close(cancel.inner.cleanup_request(), services)), CleanupOutcome::Clean);
    assert_eq!(cancel.ordinary_starts.load(Ordering::SeqCst), 0);
    assert_eq!(cancel.inner.resource_releases(), 1);
    assert_eq!(cancel.inner.credential_releases(), 1);
    let revoked = CheckedFixture::new(FixtureHost::new(Scenario::Success, "0.100.0"), graph.clone(), expected);
    revoked.revoked.store(true, Ordering::SeqCst);
    assert!(block_on(profile.open_session(revoked.services(host_id))).is_err());
    assert_eq!(revoked.checked_starts.load(Ordering::SeqCst), 0);
    assert_eq!(revoked.ordinary_starts.load(Ordering::SeqCst), 0);
    assert_eq!(revoked.inner.resource_releases(), 1);
    assert_eq!(revoked.inner.credential_releases(), 1);
}
