use claude_code_support::watcher_proof::{
    WatcherProofFact, WatcherProofRecorder, assert_stop_reentry_proof,
};
use swallowtail_core::ProviderActivityRef;
use swallowtail_host_local::WatcherBridgeProofKind;
use swallowtail_runtime::{
    ActivityDisclosure, ActivityId, ActivityObservation, ActivityOperationId, ActivityStatus,
    RuntimeRunId, RuntimeTurnId, WATCHER_BRIDGE_TOOLS_LIST_METHOD,
};

static NEXT_TEMP_WORKSPACE: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

fn hook_event(session: &str, phase: &str) -> RuntimeEvent {
    hook_event_for("turn-a", session, phase)
}

fn hook_event_for(turn: &str, session: &str, phase: &str) -> RuntimeEvent {
    let observation = ActivityObservation::new(
        ActivityId::new(format!("hook-{session}-{phase}")).expect("activity id"),
        ActivityOperationId::Run(RuntimeRunId::new(turn).expect("run")),
        ActivityKind::Hook,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Completed,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .expect("hook observation")
    .with_provider_activity_ref(
        ProviderActivityRef::new(format!("{session}|{phase}")).expect("provider ref"),
    );
    RuntimeEvent::new(1, RuntimeEventKind::Activity(observation))
}

fn bound_ok() -> Vec<WatcherProofFact> {
    let turn = "turn-a".to_owned();
    let session = "session-a".to_owned();
    vec![
        WatcherProofFact::McpInitialized { turn: turn.clone() },
        WatcherProofFact::ToolsListed { turn: turn.clone() },
        WatcherProofFact::WatcherStarted { turn: turn.clone() },
        WatcherProofFact::StopHookStarted {
            turn: turn.clone(),
            session: session.clone(),
        },
        WatcherProofFact::StopGateActive {
            turn: turn.clone(),
            session: session.clone(),
        },
        WatcherProofFact::StopHookResponded {
            turn: turn.clone(),
            session: session.clone(),
        },
        WatcherProofFact::SameSessionContinuation {
            turn: turn.clone(),
            session,
        },
        WatcherProofFact::WaitOrStop { turn: turn.clone() },
        WatcherProofFact::JoinedZero { turn: turn.clone() },
        WatcherProofFact::ProviderSucceeded { turn },
    ]
}

#[test]
fn stop_reentry_oracle_accepts_only_the_ordered_conjunction() {
    assert!(assert_stop_reentry_proof(&bound_ok()).is_ok());
}

#[test]
fn stop_reentry_oracle_rejects_proactive_wait() {
    let facts = bound_ok()
        .into_iter()
        .filter(|fact| {
            !matches!(
                fact,
                WatcherProofFact::StopHookStarted { .. }
                    | WatcherProofFact::StopGateActive { .. }
                    | WatcherProofFact::StopHookResponded { .. }
                    | WatcherProofFact::SameSessionContinuation { .. }
            )
        })
        .collect::<Vec<_>>();
    assert!(assert_stop_reentry_proof(&facts).is_err());
}

#[test]
fn recorder_rejects_direct_gate_without_stop_hook() {
    let mut recorder = WatcherProofRecorder::new("turn-a");
    recorder.ingest_bridge(&[
        WatcherBridgeProofKind::Initialize,
        WatcherBridgeProofKind::ToolsList,
        WatcherBridgeProofKind::Start,
        WatcherBridgeProofKind::CompletionGateActive,
        WatcherBridgeProofKind::Wait,
    ]);
    assert!(recorder
        .facts()
        .iter()
        .any(|fact| matches!(fact, WatcherProofFact::DirectGateActive { .. })));
    assert!(assert_stop_reentry_proof(recorder.facts()).is_err());
}

#[test]
fn recorder_rejects_cross_session_hook_phases() {
    let mut recorder = WatcherProofRecorder::new("turn-a");
    recorder.ingest_bridge(&[
        WatcherBridgeProofKind::Initialize,
        WatcherBridgeProofKind::ToolsList,
        WatcherBridgeProofKind::Start,
    ]);
    recorder.ingest_event(&hook_event("session-a", "Stop.started"));
    recorder.ingest_kind(WatcherBridgeProofKind::CompletionGateActive);
    recorder.ingest_event(&hook_event("session-b", "Stop.responded"));
    recorder.ingest_kind(WatcherBridgeProofKind::Wait);
    assert!(recorder
        .facts()
        .iter()
        .any(|fact| matches!(fact, WatcherProofFact::StopHookStarted { session, .. } if session == "session-a")));
    assert!(recorder
        .facts()
        .iter()
        .any(|fact| matches!(fact, WatcherProofFact::StopHookResponded { session, .. } if session == "session-b")));
    assert!(assert_stop_reentry_proof(recorder.facts()).is_err());
}

#[test]
fn stop_reentry_oracle_rejects_terminal_only_adapter_rejection() {
    let facts = bound_ok().into_iter().take(3).collect::<Vec<_>>();
    assert!(assert_stop_reentry_proof(&facts).is_err());
}

struct TempWorkspace {
    path: std::path::PathBuf,
}

impl TempWorkspace {
    fn create() -> Self {
        let sequence = NEXT_TEMP_WORKSPACE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "swallowtail-watcher-proof-{}-{sequence}",
            std::process::id(),
        ));
        std::fs::create_dir(&path).expect("workspace is created without collision");
        Self { path }
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[test]
fn temporary_workspace_cleanup_is_established_before_assertions() {
    let workspace = TempWorkspace::create();
    let path = workspace.path.clone();
    std::panic::catch_unwind(move || {
        assert!(workspace.path.is_dir());
        panic!("assertion failure after workspace owner is live");
    })
    .expect_err("panic is required");
    assert!(!path.exists(), "workspace survived an assertion panic");
}

#[test]
fn recorder_ignores_foreign_operation_hook_events() {
    let mut recorder = WatcherProofRecorder::new("turn-a");
    recorder.ingest_event(&hook_event_for("turn-b", "session-a", "Stop.started"));
    assert!(recorder.facts().is_empty());
}
