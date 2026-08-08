use std::sync::{Arc, Mutex};
use swallowtail_core::{Diagnostic, ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    DebugObservation, DebugObservationKind, DiagnosticObserver, HostServices,
};

/// Host-owned sink that records restricted debug observations for local inspection.
#[derive(Default)]
pub struct RecordingDebugObserver {
    observations: Mutex<Vec<DebugObservation>>,
}

impl RecordingDebugObserver {
    pub fn observations(&self) -> Vec<DebugObservation> {
        self.observations.lock().expect("observation lock").clone()
    }
}

impl DiagnosticObserver for RecordingDebugObserver {
    fn observe(&self, _diagnostic: &Diagnostic) {}

    fn observe_debug(&self, observation: &DebugObservation) {
        self.observations
            .lock()
            .expect("observation lock")
            .push(observation.clone());
    }
}

/// Ordinary hosts omit the observer; debug hosts register one explicitly.
pub fn services_with_debug_observer(observer: Arc<dyn DiagnosticObserver>) -> HostServices {
    HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"))
        .with_diagnostic_observer(observer)
}

/// Emit helpers no-op when no observer is registered.
pub fn emit_sample(services: &HostServices) {
    services.emit_diagnostic(&Diagnostic::new(SafeDiagnostic::new(
        "fixture.diagnostic",
        "Fixture diagnostic",
    )));
    services.emit_debug_observation(
        &DebugObservation::new(
            DebugObservationKind::ProtocolParse,
            "method=item/plan/delta; excerpt=<path>",
        )
        .with_route("codex.app_server")
        .with_stage("rpc.pump.inbound")
        .with_correlated_code("swallowtail.codex.app_server.malformed_notification"),
    );
}

fn main() {
    let observer = Arc::new(RecordingDebugObserver::default());
    let services = services_with_debug_observer(observer.clone());
    emit_sample(&services);
    assert_eq!(observer.observations().len(), 1);
    assert!(!format!("{}", observer.observations()[0]).contains("item/plan/delta"));
}
