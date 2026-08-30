//! Bounded reserved-operation names observed on the watcher HTTP bridge.

/// Closed names of reserved watcher-bridge MCP operations.
///
/// This is a proof observation, not a consumer event bus. It retains only
/// operation kind, never endpoint, bearer, path, arguments, or response text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatcherBridgeProofKind {
    /// MCP `initialize` succeeded.
    Initialize,
    /// MCP `tools/list` succeeded.
    ToolsList,
    /// Reserved start tool succeeded.
    Start,
    /// Completion gate observed active or unjoined work.
    CompletionGateActive,
    /// Completion gate observed idle work.
    CompletionGateIdle,
    /// Reserved wait tool succeeded.
    Wait,
    /// Reserved stop tool succeeded.
    Stop,
}

const PROOF_BOUND: usize = 32;

#[derive(Clone)]
pub(super) struct ProofLog {
    kinds: std::sync::Arc<std::sync::Mutex<Vec<WatcherBridgeProofKind>>>,
}

impl ProofLog {
    pub(super) fn new() -> Self {
        Self {
            kinds: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub(super) fn record(&self, kind: WatcherBridgeProofKind) {
        let mut kinds = self
            .kinds
            .lock()
            .expect("watcher bridge proof lock poisoned");
        if kinds.len() < PROOF_BOUND {
            kinds.push(kind);
        }
    }

    pub(super) fn snapshot(&self) -> Vec<WatcherBridgeProofKind> {
        self.kinds
            .lock()
            .expect("watcher bridge proof lock poisoned")
            .clone()
    }
}

impl std::fmt::Debug for ProofLog {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ProofLog").finish_non_exhaustive()
    }
}
