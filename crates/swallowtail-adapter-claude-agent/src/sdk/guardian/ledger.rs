//! The open guard's state machine: one mutex decides claim versus cleanup.
//!
//! Three separate pieces of state cannot express this safely. A flag read
//! before a ledger take permits a recording to land after the take, and permits
//! open to report success after cleanup already terminated what it acquired.
//! So the phase and the acquisitions live under one lock, and the transition
//! `Armed -> Claimed` or `Armed -> Cleaning` is a single atomic choice.
//!
//! Cleanup additionally waits for recording to end before it takes anything.
//! The open future holds a [`RecordingLease`] for exactly as long as it can
//! record, and that lease is dropped whether the future completes or the
//! caller's bound drops it mid-flight, so no acquisition can be stranded on
//! the far side of the take.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{CredentialLease, JoinedTask, ProcessHandle, ResourceLease};

use super::Signal;

/// Everything the open path has acquired so far.
///
/// Recorded as each acquisition succeeds, so cleanup covers a partial open, not
/// only a complete one.
#[derive(Default)]
pub(crate) struct Acquisitions {
    pub(crate) credential: Option<CredentialLease>,
    pub(crate) resource: Option<ResourceLease>,
    pub(crate) process: Option<Arc<dyn ProcessHandle>>,
    pub(crate) pump: Option<Box<dyn JoinedTask>>,
}

impl Acquisitions {
    fn take(&mut self) -> Self {
        Self {
            credential: self.credential.take(),
            resource: self.resource.take(),
            process: self.process.take(),
            pump: self.pump.take(),
        }
    }
}

/// Which side of the guard owns what the open path acquired.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Phase {
    /// Open is still running; neither side owns the acquisitions yet.
    Armed,
    /// Open completed inside its bound and took ownership.
    Claimed,
    /// The deadline or a failure won; the guard owns cleanup.
    Cleaning,
}

struct State {
    phase: Phase,
    acquisitions: Acquisitions,
    /// True while the open future can still record an acquisition.
    recording: bool,
}

/// The guard's shared state. Every transition and every recording goes through
/// the one lock here.
pub(crate) struct GuardLedger {
    state: Mutex<State>,
    /// Triggered when the open future can no longer record, so cleanup knows
    /// the ledger it takes is the final one.
    recording_ended: Arc<Signal>,
}

impl GuardLedger {
    pub(crate) fn new() -> (Arc<Self>, RecordingLease) {
        let ledger = Arc::new(Self {
            state: Mutex::new(State {
                phase: Phase::Armed,
                acquisitions: Acquisitions::default(),
                recording: true,
            }),
            recording_ended: Arc::new(Signal::default()),
        });
        let lease = RecordingLease {
            ledger: Arc::clone(&ledger),
        };
        (ledger, lease)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, State> {
        self.state
            .lock()
            .expect("SDK open-guard ledger lock poisoned")
    }

    pub(crate) fn record_credential(&self, lease: CredentialLease) {
        self.lock().acquisitions.credential = Some(lease);
    }

    pub(crate) fn record_resource(&self, lease: ResourceLease) {
        self.lock().acquisitions.resource = Some(lease);
    }

    pub(crate) fn record_process(&self, process: Arc<dyn ProcessHandle>) {
        self.lock().acquisitions.process = Some(process);
    }

    pub(crate) fn record_pump(&self, pump: Box<dyn JoinedTask>) {
        self.lock().acquisitions.pump = Some(pump);
    }

    /// Validates the recorded credential lease in place, so a rejected lease is
    /// still released by the guard rather than by the caller.
    pub(crate) fn credential_matches(
        &self,
        scope: &swallowtail_runtime::ScopeId,
        reference: &swallowtail_core::CredentialRef,
        audience: &swallowtail_core::EndpointAudience,
    ) -> bool {
        let state = self.lock();
        matches!(
            state.acquisitions.credential,
            Some(CredentialLease::Delegated(_))
        ) && state.acquisitions.credential.as_ref().is_some_and(|lease| {
            lease.scope() == scope && lease.reference() == reference && lease.audience() == audience
        })
    }

    /// Takes ownership back on the success path. `None` means cleanup already
    /// won the transition, so open must not report success: what it acquired is
    /// being terminated.
    pub(crate) fn claim(&self) -> Option<Acquisitions> {
        let mut state = self.lock();
        if state.phase == Phase::Cleaning {
            return None;
        }
        state.phase = Phase::Claimed;
        Some(state.acquisitions.take())
    }

    /// Moves the guard to cleanup unless open already claimed. Returns whether
    /// cleanup owns the acquisitions.
    pub(crate) fn begin_cleanup(&self) -> bool {
        let mut state = self.lock();
        if state.phase == Phase::Claimed {
            return false;
        }
        state.phase = Phase::Cleaning;
        true
    }

    /// Waits until the open future can no longer record, then takes the final
    /// ledger. Recording ends when that future is dropped or completes, so this
    /// never waits on provider work.
    pub(crate) async fn take_for_cleanup(&self) -> Acquisitions {
        if !self.lock().recording {
            return self.lock().acquisitions.take();
        }
        self.recording_ended.future().await;
        self.lock().acquisitions.take()
    }
}

/// Held by the open future for exactly as long as it can record an acquisition.
pub(crate) struct RecordingLease {
    ledger: Arc<GuardLedger>,
}

impl Drop for RecordingLease {
    fn drop(&mut self) {
        self.ledger.lock().recording = false;
        self.ledger.recording_ended.trigger();
    }
}

/// Set when the guard woke on the caller's deadline rather than on the failure
/// signal, so the open path can report the deadline as the cause instead of
/// whatever the collapsing connection said next.
#[derive(Default)]
pub(crate) struct DeadlineFlag(AtomicBool);

impl DeadlineFlag {
    pub(crate) fn set(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    pub(crate) fn fired(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod ledger_tests;
