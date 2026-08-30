use swallowtail_host_local::WatcherBridgeProofKind;
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, CleanupOutcome, RuntimeEvent, RuntimeEventKind,
    TerminalOutcome, TerminalStatus,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatcherProofFact {
    McpInitialized,
    ToolsListed,
    WatcherStarted,
    ActiveCompletionBlocked,
    StopHookObserved,
    SameSessionContinuation,
    WaitOrStop,
    JoinedZero,
    ProviderSucceeded,
}

pub fn assert_stop_reentry_proof(facts: &[WatcherProofFact]) -> Result<(), &'static str> {
    let required = [
        WatcherProofFact::McpInitialized,
        WatcherProofFact::ToolsListed,
        WatcherProofFact::WatcherStarted,
        WatcherProofFact::ActiveCompletionBlocked,
        WatcherProofFact::StopHookObserved,
        WatcherProofFact::SameSessionContinuation,
        WatcherProofFact::WaitOrStop,
        WatcherProofFact::JoinedZero,
        WatcherProofFact::ProviderSucceeded,
    ];
    let mut index = 0;
    for fact in facts {
        if *fact == required[index] {
            index += 1;
            if index == required.len() {
                return Ok(());
            }
        }
    }
    Err("stop re-entry sequence is incomplete or reordered")
}

#[allow(dead_code)]
pub struct WatcherProofRecorder {
    facts: Vec<WatcherProofFact>,
    seen_bridge: usize,
}

#[allow(dead_code)]
impl WatcherProofRecorder {
    pub fn new() -> Self {
        Self {
            facts: Vec::new(),
            seen_bridge: 0,
        }
    }

    pub fn facts(&self) -> &[WatcherProofFact] {
        &self.facts
    }

    pub fn ingest_bridge(&mut self, proof: &[WatcherBridgeProofKind]) {
        for kind in proof.iter().skip(self.seen_bridge) {
            if let Some(fact) = map_bridge(*kind) {
                self.push(fact);
            }
        }
        self.seen_bridge = proof.len();
    }

    pub fn ingest_event(&mut self, event: &RuntimeEvent) {
        let RuntimeEventKind::Activity(observation) = event.kind() else {
            return;
        };
        match observation.kind() {
            ActivityKind::Hook => self.push(WatcherProofFact::StopHookObserved),
            ActivityKind::AssistantMessage
                if self.facts.contains(&WatcherProofFact::StopHookObserved) =>
            {
                self.push(WatcherProofFact::SameSessionContinuation);
            }
            ActivityKind::HostWatcher if observation.phase() == ActivityLifecyclePhase::Started => {
                self.push(WatcherProofFact::WatcherStarted);
            }
            _ => {}
        }
    }

    pub fn ingest_terminal(&mut self, outcome: &TerminalOutcome) {
        if outcome.cleanup() == &CleanupOutcome::Clean {
            self.push(WatcherProofFact::JoinedZero);
        }
        if outcome.status() == &TerminalStatus::Completed {
            self.push(WatcherProofFact::ProviderSucceeded);
        }
    }

    fn push(&mut self, fact: WatcherProofFact) {
        if self.facts.last() != Some(&fact) {
            self.facts.push(fact);
        }
    }
}

fn map_bridge(kind: WatcherBridgeProofKind) -> Option<WatcherProofFact> {
    match kind {
        WatcherBridgeProofKind::Initialize => Some(WatcherProofFact::McpInitialized),
        WatcherBridgeProofKind::ToolsList => Some(WatcherProofFact::ToolsListed),
        WatcherBridgeProofKind::Start => Some(WatcherProofFact::WatcherStarted),
        WatcherBridgeProofKind::CompletionGateActive => {
            Some(WatcherProofFact::ActiveCompletionBlocked)
        }
        WatcherBridgeProofKind::Wait | WatcherBridgeProofKind::Stop => {
            Some(WatcherProofFact::WaitOrStop)
        }
        WatcherBridgeProofKind::CompletionGateIdle => None,
    }
}
