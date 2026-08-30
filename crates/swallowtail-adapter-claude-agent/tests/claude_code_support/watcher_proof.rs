use swallowtail_core::ProviderActivityRef;
use swallowtail_host_local::WatcherBridgeProofKind;
use swallowtail_runtime::{
    ActivityDisclosure, ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation,
    ActivityOperationId, ActivityStatus, CleanupOutcome, RuntimeEvent, RuntimeEventKind,
    RuntimeRunId, TerminalOutcome, TerminalStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WatcherProofFact {
    McpInitialized { turn: String },
    ToolsListed { turn: String },
    WatcherStarted { turn: String },
    StopHookStarted { turn: String, session: String },
    StopGateActive { turn: String, session: String },
    DirectGateActive { turn: String },
    StopHookResponded { turn: String, session: String },
    SameSessionContinuation { turn: String, session: String },
    WaitOrStop { turn: String },
    JoinedZero { turn: String },
    ProviderSucceeded { turn: String },
}

pub fn assert_stop_reentry_proof(facts: &[WatcherProofFact]) -> Result<(), &'static str> {
    let turn = facts
        .first()
        .and_then(fact_turn)
        .ok_or("stop re-entry sequence is empty")?;
    if facts.iter().any(|fact| fact_turn(fact) != Some(turn)) {
        return Err("stop re-entry facts bind mixed turns");
    }
    let session = facts.iter().find_map(fact_session);
    if facts
        .iter()
        .filter_map(fact_session)
        .any(|value| Some(value) != session)
    {
        return Err("stop re-entry facts bind mixed sessions");
    }
    if facts
        .iter()
        .any(|fact| matches!(fact, WatcherProofFact::DirectGateActive { .. }))
    {
        return Err("direct completion-gate use is not Stop-hook sourced");
    }
    let required = [
        "initialized",
        "tools",
        "started",
        "hook-started",
        "stop-gate",
        "hook-responded",
        "continuation",
        "wait-or-stop",
        "joined",
        "succeeded",
    ];
    let mut index = 0;
    for fact in facts {
        if fact_kind(fact) == required[index] {
            index += 1;
            if index == required.len() {
                return Ok(());
            }
        }
    }
    Err("stop re-entry sequence is incomplete or reordered")
}

fn fact_kind(fact: &WatcherProofFact) -> &'static str {
    match fact {
        WatcherProofFact::McpInitialized { .. } => "initialized",
        WatcherProofFact::ToolsListed { .. } => "tools",
        WatcherProofFact::WatcherStarted { .. } => "started",
        WatcherProofFact::StopHookStarted { .. } => "hook-started",
        WatcherProofFact::StopGateActive { .. } => "stop-gate",
        WatcherProofFact::DirectGateActive { .. } => "direct-gate",
        WatcherProofFact::StopHookResponded { .. } => "hook-responded",
        WatcherProofFact::SameSessionContinuation { .. } => "continuation",
        WatcherProofFact::WaitOrStop { .. } => "wait-or-stop",
        WatcherProofFact::JoinedZero { .. } => "joined",
        WatcherProofFact::ProviderSucceeded { .. } => "succeeded",
    }
}

fn fact_turn(fact: &WatcherProofFact) -> Option<&str> {
    match fact {
        WatcherProofFact::McpInitialized { turn }
        | WatcherProofFact::ToolsListed { turn }
        | WatcherProofFact::WatcherStarted { turn }
        | WatcherProofFact::StopHookStarted { turn, .. }
        | WatcherProofFact::StopGateActive { turn, .. }
        | WatcherProofFact::DirectGateActive { turn }
        | WatcherProofFact::StopHookResponded { turn, .. }
        | WatcherProofFact::SameSessionContinuation { turn, .. }
        | WatcherProofFact::WaitOrStop { turn }
        | WatcherProofFact::JoinedZero { turn }
        | WatcherProofFact::ProviderSucceeded { turn } => Some(turn.as_str()),
    }
}

fn fact_session(fact: &WatcherProofFact) -> Option<&str> {
    match fact {
        WatcherProofFact::StopHookStarted { session, .. }
        | WatcherProofFact::StopGateActive { session, .. }
        | WatcherProofFact::StopHookResponded { session, .. }
        | WatcherProofFact::SameSessionContinuation { session, .. } => Some(session.as_str()),
        _ => None,
    }
}

pub struct WatcherProofRecorder {
    turn: String,
    facts: Vec<WatcherProofFact>,
    seen_bridge: usize,
    hook_open_session: Option<String>,
}

impl WatcherProofRecorder {
    pub fn new(turn: impl Into<String>) -> Self {
        Self {
            turn: turn.into(),
            facts: Vec::new(),
            seen_bridge: 0,
            hook_open_session: None,
        }
    }

    pub fn facts(&self) -> &[WatcherProofFact] {
        &self.facts
    }

    pub fn ingest_bridge(&mut self, proof: &[WatcherBridgeProofKind]) {
        for kind in proof.iter().skip(self.seen_bridge) {
            self.ingest_kind(*kind);
        }
        self.seen_bridge = proof.len().max(self.seen_bridge);
    }

    pub fn ingest_kind(&mut self, kind: WatcherBridgeProofKind) {
        if let Some(fact) = self.map_bridge(kind) {
            self.push(fact);
        }
    }

    pub fn ingest_event(&mut self, event: &RuntimeEvent) {
        let RuntimeEventKind::Activity(observation) = event.kind() else {
            return;
        };
        match observation.kind() {
            ActivityKind::Hook => {
                let raw = observation
                    .provider_activity_ref()
                    .map(|value| value.as_provider_value())
                    .unwrap_or_default();
                let (session, phase) = raw
                    .rsplit_once('|')
                    .map(|(session, phase)| (session.to_owned(), phase))
                    .unwrap_or_else(|| (raw.to_owned(), ""));
                match phase {
                    "Stop.started" => {
                        self.hook_open_session = Some(session.clone());
                        self.push(WatcherProofFact::StopHookStarted {
                            turn: self.turn.clone(),
                            session,
                        });
                    }
                    "Stop.responded" => {
                        self.hook_open_session = None;
                        self.push(WatcherProofFact::StopHookResponded {
                            turn: self.turn.clone(),
                            session,
                        });
                    }
                    _ => {}
                }
            }
            ActivityKind::AssistantMessage
                if self
                    .facts
                    .iter()
                    .any(|fact| matches!(fact, WatcherProofFact::StopHookResponded { .. })) =>
            {
                if let Some(session) = self.facts.iter().find_map(fact_session) {
                    self.push(WatcherProofFact::SameSessionContinuation {
                        turn: self.turn.clone(),
                        session: session.to_owned(),
                    });
                }
            }
            ActivityKind::HostWatcher if observation.phase() == ActivityLifecyclePhase::Started => {
                self.push(WatcherProofFact::WatcherStarted {
                    turn: self.turn.clone(),
                });
            }
            _ => {}
        }
    }

    pub fn ingest_terminal(&mut self, outcome: &TerminalOutcome) {
        if outcome.cleanup() == &CleanupOutcome::Clean {
            self.push(WatcherProofFact::JoinedZero {
                turn: self.turn.clone(),
            });
        }
        if outcome.status() == &TerminalStatus::Completed {
            self.push(WatcherProofFact::ProviderSucceeded {
                turn: self.turn.clone(),
            });
        }
    }

    fn map_bridge(&self, kind: WatcherBridgeProofKind) -> Option<WatcherProofFact> {
        match kind {
            WatcherBridgeProofKind::Initialize => Some(WatcherProofFact::McpInitialized {
                turn: self.turn.clone(),
            }),
            WatcherBridgeProofKind::ToolsList => Some(WatcherProofFact::ToolsListed {
                turn: self.turn.clone(),
            }),
            WatcherBridgeProofKind::Start => Some(WatcherProofFact::WatcherStarted {
                turn: self.turn.clone(),
            }),
            WatcherBridgeProofKind::CompletionGateActive => {
                Some(if let Some(session) = self.hook_open_session.clone() {
                    WatcherProofFact::StopGateActive {
                        turn: self.turn.clone(),
                        session,
                    }
                } else {
                    WatcherProofFact::DirectGateActive {
                        turn: self.turn.clone(),
                    }
                })
            }
            WatcherBridgeProofKind::Wait | WatcherBridgeProofKind::Stop => {
                Some(WatcherProofFact::WaitOrStop {
                    turn: self.turn.clone(),
                })
            }
            WatcherBridgeProofKind::CompletionGateIdle => None,
        }
    }

    fn push(&mut self, fact: WatcherProofFact) {
        if self.facts.last() != Some(&fact) {
            self.facts.push(fact);
        }
    }
}

pub fn hook_event(session: &str, phase: &str) -> RuntimeEvent {
    let observation = ActivityObservation::new(
        ActivityId::new(format!("hook-{session}-{phase}")).expect("activity id"),
        ActivityOperationId::Run(RuntimeRunId::new("watcher-proof").expect("run")),
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
