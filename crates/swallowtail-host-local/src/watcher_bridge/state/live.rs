use super::{LiveLease, SessionPhase, drive, owning_turn, reap_finished};
use crate::watcher_bridge::failure::{
    busy_failure, closed_failure, duplicate_failure, foreign_failure, frozen_failure,
    handshake_failure,
};
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    RuntimeFailure, RuntimeTurnId, ScopeId, WATCHER_BRIDGE_MAX_CONCURRENT_CONNECTIONS,
    WATCHER_BRIDGE_MAX_CORRELATION_IDS, WATCHER_BRIDGE_MAX_IN_FLIGHT_REQUESTS,
    WatcherBridgeAdmission, WatcherBridgeCompletionState, WatcherBridgeGeneration, WatcherSnapshot,
};

impl LiveLease {
    pub(in crate::watcher_bridge) fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub(in crate::watcher_bridge) fn matches(
        &self,
        host: &ExecutionHostId,
        scope: &ScopeId,
        turn: &RuntimeTurnId,
        generation: WatcherBridgeGeneration,
    ) -> Result<(), RuntimeFailure> {
        if self.is_closed() {
            return Err(closed_failure());
        }
        if &self.execution_host_id != host
            || &self.scope != scope
            || &self.turn != turn
            || self.generation != generation
        {
            return Err(foreign_failure());
        }
        Ok(())
    }

    pub(in crate::watcher_bridge) fn admit_connection(&self) -> Result<(), RuntimeFailure> {
        if self.is_closed() {
            return Err(closed_failure());
        }
        loop {
            let current = self.connection_count.load(Ordering::SeqCst);
            if current >= WATCHER_BRIDGE_MAX_CONCURRENT_CONNECTIONS {
                return Err(busy_failure());
            }
            if self
                .connection_count
                .compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return Ok(());
            }
        }
    }

    pub(in crate::watcher_bridge) fn release_connection(&self) {
        self.connection_count.fetch_sub(1, Ordering::SeqCst);
    }

    pub(in crate::watcher_bridge) fn retain_connection(&self, handle: JoinHandle<()>) {
        let mut connections = self
            .connections
            .lock()
            .expect("watcher bridge connection lock poisoned");
        reap_finished(&mut connections);
        connections.push(handle);
    }

    pub(in crate::watcher_bridge) fn admit_request(
        &self,
        correlation: &str,
    ) -> Result<(), RuntimeFailure> {
        if self.is_closed() {
            return Err(closed_failure());
        }
        let mut requests = self
            .requests
            .lock()
            .expect("watcher bridge request lock poisoned");
        if requests.in_flight >= WATCHER_BRIDGE_MAX_IN_FLIGHT_REQUESTS
            || requests.seen.len() >= WATCHER_BRIDGE_MAX_CORRELATION_IDS
        {
            return Err(busy_failure());
        }
        if !requests.seen.insert(correlation.to_owned()) {
            return Err(duplicate_failure());
        }
        requests.in_flight += 1;
        Ok(())
    }

    pub(in crate::watcher_bridge) fn release_request(&self) {
        let mut requests = self
            .requests
            .lock()
            .expect("watcher bridge request lock poisoned");
        requests.in_flight = requests.in_flight.saturating_sub(1);
    }

    pub(in crate::watcher_bridge) fn begin_create(&self) -> Result<(), RuntimeFailure> {
        let mut gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
        if gate.admission != WatcherBridgeAdmission::Open {
            return Err(if gate.admission == WatcherBridgeAdmission::Frozen {
                frozen_failure()
            } else {
                closed_failure()
            });
        }
        gate.creating += 1;
        Ok(())
    }

    pub(in crate::watcher_bridge) fn end_create(&self) {
        let mut gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
        gate.creating = gate.creating.saturating_sub(1);
        self.creating_changed.notify_all();
    }

    pub(in crate::watcher_bridge) fn require_not_closed(
        &self,
    ) -> Result<WatcherBridgeAdmission, RuntimeFailure> {
        let gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
        if gate.admission == WatcherBridgeAdmission::Closed {
            Err(closed_failure())
        } else {
            Ok(gate.admission)
        }
    }

    pub(in crate::watcher_bridge) fn admit_initialize(&self) -> Result<(), RuntimeFailure> {
        let mut session = self
            .session
            .lock()
            .expect("watcher bridge session lock poisoned");
        if *session != SessionPhase::New {
            return Err(handshake_failure());
        }
        *session = SessionPhase::Handshake;
        Ok(())
    }

    pub(in crate::watcher_bridge) fn admit_initialized(&self) -> Result<(), RuntimeFailure> {
        let mut session = self
            .session
            .lock()
            .expect("watcher bridge session lock poisoned");
        if *session != SessionPhase::Handshake {
            return Err(handshake_failure());
        }
        *session = SessionPhase::Ready;
        Ok(())
    }

    pub(in crate::watcher_bridge) fn require_ready(&self) -> Result<(), RuntimeFailure> {
        let session = self
            .session
            .lock()
            .expect("watcher bridge session lock poisoned");
        if *session == SessionPhase::Ready {
            Ok(())
        } else {
            Err(handshake_failure())
        }
    }

    pub(in crate::watcher_bridge) fn completion_gate(
        &self,
    ) -> Result<WatcherBridgeCompletionState, RuntimeFailure> {
        loop {
            {
                let gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
                match gate.admission {
                    WatcherBridgeAdmission::Closed => return Err(closed_failure()),
                    WatcherBridgeAdmission::Frozen => {
                        drop(gate);
                        return Ok(WatcherBridgeCompletionState::new(
                            WatcherBridgeAdmission::Frozen,
                            self.remaining()?,
                        ));
                    }
                    WatcherBridgeAdmission::Open if gate.creating > 0 => {
                        drop(
                            self.creating_changed
                                .wait(gate)
                                .expect("watcher bridge gate lock poisoned"),
                        );
                        continue;
                    }
                    WatcherBridgeAdmission::Open => {}
                }
            }
            let remaining = self.remaining()?;
            let mut gate = self.gate.lock().expect("watcher bridge gate lock poisoned");
            match gate.admission {
                WatcherBridgeAdmission::Closed => return Err(closed_failure()),
                WatcherBridgeAdmission::Frozen => {
                    return Ok(WatcherBridgeCompletionState::new(
                        WatcherBridgeAdmission::Frozen,
                        remaining,
                    ));
                }
                WatcherBridgeAdmission::Open if gate.creating > 0 => {}
                WatcherBridgeAdmission::Open if remaining.is_empty() => {
                    gate.admission = WatcherBridgeAdmission::Frozen;
                    return Ok(WatcherBridgeCompletionState::new(
                        WatcherBridgeAdmission::Frozen,
                        Vec::new(),
                    ));
                }
                WatcherBridgeAdmission::Open => {
                    return Ok(WatcherBridgeCompletionState::new(
                        WatcherBridgeAdmission::Open,
                        remaining,
                    ));
                }
            }
        }
    }

    fn remaining(&self) -> Result<Vec<WatcherSnapshot>, RuntimeFailure> {
        match drive(self.watcher.list(owning_turn(&self.turn)?)) {
            Ok(snapshots) => Ok(snapshots
                .into_iter()
                .filter(|snapshot| !snapshot.phase().is_joined())
                .collect()),
            Err(error)
                if matches!(
                    error.diagnostic().code(),
                    "swallowtail.local_watcher.turn_not_found"
                        | "swallowtail.local_watcher.turn_retired"
                ) =>
            {
                Ok(Vec::new())
            }
            Err(error) => Err(error),
        }
    }
}
