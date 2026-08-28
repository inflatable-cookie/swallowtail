#![deny(missing_docs)]

use crate::debug_observation::{DebugObservation, DebugObservationKind, failure_debug_observation};
use crate::{
    AttachmentService, BlockingWorkService, CredentialService, DeviceCodeDisplayService,
    DiagnosticObserver, LoopbackCallbackService, ModelArtifactService, NetworkPolicyService,
    ProcessService, RuntimeFailure, SchemaService, ScopedTaskService, ServingEndpointService,
    TimeService, UrlOpenService, WatcherHostService, WorkingResourceIoService,
    WorkingResourceService,
};
use std::collections::BTreeSet;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use swallowtail_core::{Diagnostic, ExecutionHostId, HostServiceKind, SafeDiagnostic};
use swallowtail_idioms::{IdiomSignal, IdiomSink, IdiomSource};

/// Explicit host-service registry supplied to runtime roles.
///
/// Services are optional and exact: registering one service does not imply a
/// fallback for another, and the execution-host identity is always checked
/// separately from service presence.
#[derive(Clone)]
pub struct HostServices {
    execution_host_id: ExecutionHostId,
    task: Option<Arc<dyn ScopedTaskService>>,
    blocking_work: Option<Arc<dyn BlockingWorkService>>,
    time: Option<Arc<dyn TimeService>>,
    process: Option<Arc<dyn ProcessService>>,
    network: Option<Arc<dyn NetworkPolicyService>>,
    credential: Option<Arc<dyn CredentialService>>,
    working_resource: Option<Arc<dyn WorkingResourceService>>,
    working_resource_io: Option<Arc<dyn WorkingResourceIoService>>,
    attachment: Option<Arc<dyn AttachmentService>>,
    model_artifact: Option<Arc<dyn ModelArtifactService>>,
    serving_endpoint: Option<Arc<dyn ServingEndpointService>>,
    schema: Option<Arc<dyn SchemaService>>,
    diagnostic_observer: Option<Arc<dyn DiagnosticObserver>>,
    idiom_source: Option<Arc<dyn IdiomSource>>,
    idiom_recorder: Option<Arc<dyn IdiomSink>>,
    url_open: Option<Arc<dyn UrlOpenService>>,
    loopback_callback: Option<Arc<dyn LoopbackCallbackService>>,
    device_code_display: Option<Arc<dyn DeviceCodeDisplayService>>,
    watcher: Option<Arc<dyn WatcherHostService>>,
}

impl HostServices {
    /// Creates an empty registry bound to one execution host.
    #[must_use]
    pub fn new(execution_host_id: ExecutionHostId) -> Self {
        Self {
            execution_host_id,
            task: None,
            blocking_work: None,
            time: None,
            process: None,
            network: None,
            credential: None,
            working_resource: None,
            working_resource_io: None,
            attachment: None,
            model_artifact: None,
            serving_endpoint: None,
            schema: None,
            diagnostic_observer: None,
            idiom_source: None,
            idiom_recorder: None,
            url_open: None,
            loopback_callback: None,
            device_code_display: None,
            watcher: None,
        }
    }

    /// Returns the execution host that owns every registered service.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    /// Verifies that the registry belongs to the expected execution host.
    pub fn require_execution_host(&self, expected: &ExecutionHostId) -> Result<(), RuntimeFailure> {
        if &self.execution_host_id == expected {
            Ok(())
        } else {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "swallowtail.execution_host_mismatch",
                "Runtime services belong to a different execution host",
            )))
        }
    }

    /// Registers the scoped asynchronous task service.
    #[must_use]
    pub fn with_task(mut self, service: Arc<dyn ScopedTaskService>) -> Self {
        self.task = Some(service);
        self
    }

    /// Registers the blocking-work service.
    #[must_use]
    pub fn with_blocking_work(mut self, service: Arc<dyn BlockingWorkService>) -> Self {
        self.blocking_work = Some(service);
        self
    }

    /// Registers the monotonic and observation clock service.
    #[must_use]
    pub fn with_time(mut self, service: Arc<dyn TimeService>) -> Self {
        self.time = Some(service);
        self
    }

    /// Registers the local process service.
    #[must_use]
    pub fn with_process(mut self, service: Arc<dyn ProcessService>) -> Self {
        self.process = Some(service);
        self
    }

    /// Registers the network-policy service.
    #[must_use]
    pub fn with_network(mut self, service: Arc<dyn NetworkPolicyService>) -> Self {
        self.network = Some(service);
        self
    }

    /// Registers the credential-lease service.
    #[must_use]
    pub fn with_credential(mut self, service: Arc<dyn CredentialService>) -> Self {
        self.credential = Some(service);
        self
    }

    /// Registers the working-resource resolution service.
    #[must_use]
    pub fn with_working_resource(mut self, service: Arc<dyn WorkingResourceService>) -> Self {
        self.working_resource = Some(service);
        self
    }

    /// Registers the working-resource I/O service.
    #[must_use]
    pub fn with_working_resource_io(mut self, service: Arc<dyn WorkingResourceIoService>) -> Self {
        self.working_resource_io = Some(service);
        self
    }

    /// Registers the attachment materialization service.
    #[must_use]
    pub fn with_attachment(mut self, service: Arc<dyn AttachmentService>) -> Self {
        self.attachment = Some(service);
        self
    }

    /// Registers the model-artifact materialization service.
    #[must_use]
    pub fn with_model_artifact(mut self, service: Arc<dyn ModelArtifactService>) -> Self {
        self.model_artifact = Some(service);
        self
    }

    /// Registers the serving-endpoint service.
    #[must_use]
    pub fn with_serving_endpoint(mut self, service: Arc<dyn ServingEndpointService>) -> Self {
        self.serving_endpoint = Some(service);
        self
    }

    /// Registers the structured-output schema service.
    #[must_use]
    pub fn with_schema(mut self, service: Arc<dyn SchemaService>) -> Self {
        self.schema = Some(service);
        self
    }

    /// Registers the redacted diagnostic observer.
    #[must_use]
    pub fn with_diagnostic_observer(mut self, service: Arc<dyn DiagnosticObserver>) -> Self {
        self.diagnostic_observer = Some(service);
        self
    }

    /// Registers the opt-in idiom selection source.
    #[must_use]
    pub fn with_idiom_source(mut self, service: Arc<dyn IdiomSource>) -> Self {
        self.idiom_source = Some(service);
        self
    }

    /// Registers the opt-in fail-soft idiom signal recorder.
    #[must_use]
    pub fn with_idiom_recorder(mut self, service: Arc<dyn IdiomSink>) -> Self {
        self.idiom_recorder = Some(service);
        self
    }

    /// Registers the interactive URL-open port. Registration does not start sign-in.
    #[must_use]
    pub fn with_url_open(mut self, service: Arc<dyn UrlOpenService>) -> Self {
        self.url_open = Some(service);
        self
    }

    /// Registers the sign-in loopback-callback port. Registration does not start sign-in.
    #[must_use]
    pub fn with_loopback_callback(mut self, service: Arc<dyn LoopbackCallbackService>) -> Self {
        self.loopback_callback = Some(service);
        self
    }

    /// Registers the device-code display port. Registration does not start sign-in.
    #[must_use]
    pub fn with_device_code_display(mut self, service: Arc<dyn DeviceCodeDisplayService>) -> Self {
        self.device_code_display = Some(service);
        self
    }

    /// Registers the optional watcher host port. Registration does not start work.
    #[must_use]
    pub fn with_watcher(mut self, service: Arc<dyn WatcherHostService>) -> Self {
        self.watcher = Some(service);
        self
    }

    /// Returns the scoped task service when registered.
    #[must_use]
    pub fn task(&self) -> Option<&Arc<dyn ScopedTaskService>> {
        self.task.as_ref()
    }

    /// Returns the blocking-work service when registered.
    #[must_use]
    pub fn blocking_work(&self) -> Option<&Arc<dyn BlockingWorkService>> {
        self.blocking_work.as_ref()
    }

    /// Returns the time service when registered.
    #[must_use]
    pub fn time(&self) -> Option<&Arc<dyn TimeService>> {
        self.time.as_ref()
    }

    /// Returns the process service when registered.
    #[must_use]
    pub fn process(&self) -> Option<&Arc<dyn ProcessService>> {
        self.process.as_ref()
    }

    /// Returns the network-policy service when registered.
    #[must_use]
    pub fn network(&self) -> Option<&Arc<dyn NetworkPolicyService>> {
        self.network.as_ref()
    }

    /// Returns the credential service when registered.
    #[must_use]
    pub fn credential(&self) -> Option<&Arc<dyn CredentialService>> {
        self.credential.as_ref()
    }

    /// Returns the working-resource service when registered.
    #[must_use]
    pub fn working_resource(&self) -> Option<&Arc<dyn WorkingResourceService>> {
        self.working_resource.as_ref()
    }

    /// Returns the working-resource I/O service when registered.
    #[must_use]
    pub fn working_resource_io(&self) -> Option<&Arc<dyn WorkingResourceIoService>> {
        self.working_resource_io.as_ref()
    }

    /// Returns the attachment service when registered.
    #[must_use]
    pub fn attachment(&self) -> Option<&Arc<dyn AttachmentService>> {
        self.attachment.as_ref()
    }

    /// Returns the model-artifact service when registered.
    #[must_use]
    pub fn model_artifact(&self) -> Option<&Arc<dyn ModelArtifactService>> {
        self.model_artifact.as_ref()
    }

    /// Returns the serving-endpoint service when registered.
    #[must_use]
    pub fn serving_endpoint(&self) -> Option<&Arc<dyn ServingEndpointService>> {
        self.serving_endpoint.as_ref()
    }

    /// Returns the schema service when registered.
    #[must_use]
    pub fn schema(&self) -> Option<&Arc<dyn SchemaService>> {
        self.schema.as_ref()
    }

    /// Returns the diagnostic observer when registered.
    #[must_use]
    pub fn diagnostic_observer(&self) -> Option<&Arc<dyn DiagnosticObserver>> {
        self.diagnostic_observer.as_ref()
    }

    /// Returns the idiom selection source when registered.
    #[must_use]
    pub fn idiom_source(&self) -> Option<&Arc<dyn IdiomSource>> {
        self.idiom_source.as_ref()
    }

    /// Returns the fail-soft idiom recorder when registered.
    #[must_use]
    pub fn idiom_recorder(&self) -> Option<&Arc<dyn IdiomSink>> {
        self.idiom_recorder.as_ref()
    }

    /// Returns the URL-open port when registered.
    #[must_use]
    pub fn url_open(&self) -> Option<&Arc<dyn UrlOpenService>> {
        self.url_open.as_ref()
    }

    /// Returns the loopback-callback port when registered.
    #[must_use]
    pub fn loopback_callback(&self) -> Option<&Arc<dyn LoopbackCallbackService>> {
        self.loopback_callback.as_ref()
    }

    /// Returns the device-code display port when registered.
    #[must_use]
    pub fn device_code_display(&self) -> Option<&Arc<dyn DeviceCodeDisplayService>> {
        self.device_code_display.as_ref()
    }

    /// Returns the watcher host port when registered.
    #[must_use]
    pub fn watcher(&self) -> Option<&Arc<dyn WatcherHostService>> {
        self.watcher.as_ref()
    }

    /// Records one idiom signal to the registered recorder, or no-ops when
    /// absent or when the recorder panics.
    pub fn record_idiom_signal(&self, signal: IdiomSignal) {
        let Some(recorder) = self.idiom_recorder.as_ref() else {
            return;
        };
        let _ = catch_unwind(AssertUnwindSafe(|| recorder.record(&signal)));
    }

    /// Emits one diagnostic to the registered observer, or no-ops when absent.
    ///
    /// Observer panics are swallowed so debug sinks cannot alter lifecycle truth.
    pub fn emit_diagnostic(&self, diagnostic: &Diagnostic) {
        let Some(observer) = self.diagnostic_observer.as_ref() else {
            return;
        };
        let _ = catch_unwind(AssertUnwindSafe(|| observer.observe(diagnostic)));
    }

    /// Emits one debug observation to the registered observer, or no-ops when absent.
    ///
    /// Observer panics are swallowed so debug sinks cannot alter lifecycle truth.
    pub fn emit_debug_observation(&self, observation: &DebugObservation) {
        let Some(observer) = self.diagnostic_observer.as_ref() else {
            return;
        };
        let _ = catch_unwind(AssertUnwindSafe(|| observer.observe_debug(observation)));
    }

    /// Emits one failure-path debug observation, or no-ops when no observer is registered.
    pub fn emit_failure_debug(
        &self,
        kind: DebugObservationKind,
        route: &'static str,
        stage: &'static str,
        code: &'static str,
        detail: impl Into<String>,
    ) {
        self.emit_debug_observation(&failure_debug_observation(kind, route, stage, code, detail));
    }

    /// Returns the exact set of service kinds present in this registry.
    #[must_use]
    pub fn available_kinds(&self) -> BTreeSet<HostServiceKind> {
        let mut kinds = BTreeSet::new();
        if self.task.is_some() {
            kinds.insert(HostServiceKind::Task);
        }
        if self.blocking_work.is_some() {
            kinds.insert(HostServiceKind::BlockingWork);
        }
        if self.time.is_some() {
            kinds.insert(HostServiceKind::Time);
        }
        if self.process.is_some() {
            kinds.insert(HostServiceKind::Process);
        }
        if self.network.is_some() {
            kinds.insert(HostServiceKind::Network);
        }
        if self.credential.is_some() {
            kinds.insert(HostServiceKind::Credential);
        }
        if self.working_resource.is_some() {
            kinds.insert(HostServiceKind::WorkingResource);
        }
        if self.working_resource_io.is_some() {
            kinds.insert(HostServiceKind::WorkingResourceIo);
        }
        if self.attachment.is_some() {
            kinds.insert(HostServiceKind::Attachment);
        }
        if self.model_artifact.is_some() {
            kinds.insert(HostServiceKind::ModelArtifact);
        }
        if self.serving_endpoint.is_some() {
            kinds.insert(HostServiceKind::ServingEndpoint);
        }
        if self.schema.is_some() {
            kinds.insert(HostServiceKind::Schema);
        }
        if self.diagnostic_observer.is_some() {
            kinds.insert(HostServiceKind::DiagnosticObserver);
        }
        if self.idiom_source.is_some() {
            kinds.insert(HostServiceKind::IdiomSource);
        }
        if self.idiom_recorder.is_some() {
            kinds.insert(HostServiceKind::IdiomRecorder);
        }
        if self.url_open.is_some() {
            kinds.insert(HostServiceKind::UrlOpen);
        }
        if self.loopback_callback.is_some() {
            kinds.insert(HostServiceKind::LoopbackCallback);
        }
        if self.device_code_display.is_some() {
            kinds.insert(HostServiceKind::DeviceCodeDisplay);
        }
        if self.watcher.is_some() {
            kinds.insert(HostServiceKind::Watcher);
        }
        kinds
    }
}

#[cfg(test)]
mod tests {
    use super::HostServices;
    use crate::debug_observation::{DebugObservation, DebugObservationKind};
    use crate::{CleanupOutcome, DiagnosticObserver, TerminalStatus};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use swallowtail_core::{Diagnostic, ExecutionHostId, SafeDiagnostic};

    #[test]
    fn service_registry_rejects_a_different_execution_host() {
        let local = ExecutionHostId::new("host.local").expect("host id is valid");
        let remote = ExecutionHostId::new("host.remote").expect("host id is valid");
        let services = HostServices::new(local.clone());

        services
            .require_execution_host(&local)
            .expect("matching host is accepted");
        let failure = services
            .require_execution_host(&remote)
            .expect_err("different host is rejected");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.execution_host_mismatch"
        );
        assert!(!format!("{failure}").contains(remote.as_str()));
    }

    #[test]
    fn emit_helpers_noop_without_observer() {
        let services =
            HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"));
        services.emit_diagnostic(&Diagnostic::new(SafeDiagnostic::new(
            "fixture.diagnostic",
            "Fixture diagnostic",
        )));
        services.emit_debug_observation(&DebugObservation::new(
            DebugObservationKind::Lifecycle,
            "prep started",
        ));
    }

    #[test]
    fn emit_debug_observation_reaches_registered_observer() {
        let observer = Arc::new(CountingObserver::default());
        let services =
            HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"))
                .with_diagnostic_observer(observer.clone());

        services.emit_debug_observation(
            &DebugObservation::new(DebugObservationKind::WireInbound, "method=x")
                .with_correlated_code("fixture.code"),
        );

        assert_eq!(observer.debug_count.load(Ordering::SeqCst), 1);
        assert_eq!(
            observer
                .last_debug
                .lock()
                .expect("lock")
                .as_ref()
                .map(DebugObservation::correlated_code),
            Some(Some("fixture.code"))
        );
    }

    #[test]
    fn emit_failure_debug_sets_route_stage_and_code() {
        let observer = Arc::new(CountingObserver::default());
        let services =
            HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"))
                .with_diagnostic_observer(observer.clone());

        services.emit_failure_debug(
            DebugObservationKind::HostProcess,
            "pi",
            "installed_discovery.probe",
            "swallowtail.pi.discovery_failed",
            "pi installed discovery did not produce a compatible observation",
        );

        let observation = observer
            .last_debug
            .lock()
            .expect("lock")
            .clone()
            .expect("observation");
        assert_eq!(observation.kind(), DebugObservationKind::HostProcess);
        assert_eq!(observation.route(), Some("pi"));
        assert_eq!(observation.stage(), Some("installed_discovery.probe"));
        assert_eq!(
            observation.correlated_code(),
            Some("swallowtail.pi.discovery_failed")
        );
    }

    #[test]
    fn sign_in_ports_are_optional_and_registration_does_not_start_sign_in() {
        use swallowtail_core::HostServiceKind;

        let idle = Arc::new(IdleSignInPorts);
        let empty =
            HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"));
        assert!(!empty.available_kinds().contains(&HostServiceKind::UrlOpen));
        assert!(
            !empty
                .available_kinds()
                .contains(&HostServiceKind::LoopbackCallback)
        );
        assert!(
            !empty
                .available_kinds()
                .contains(&HostServiceKind::DeviceCodeDisplay)
        );

        let services =
            HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"))
                .with_url_open(idle.clone())
                .with_loopback_callback(idle.clone())
                .with_device_code_display(idle);
        assert!(
            services
                .available_kinds()
                .contains(&HostServiceKind::UrlOpen)
        );
        assert!(
            services
                .available_kinds()
                .contains(&HostServiceKind::LoopbackCallback)
        );
        assert!(
            services
                .available_kinds()
                .contains(&HostServiceKind::DeviceCodeDisplay)
        );
        assert!(
            !services
                .available_kinds()
                .contains(&HostServiceKind::Credential)
        );
        assert!(
            !services
                .available_kinds()
                .contains(&HostServiceKind::Process)
        );
        assert!(
            !services
                .available_kinds()
                .contains(&HostServiceKind::Network)
        );
    }

    #[test]
    fn watcher_port_is_optional_and_registration_does_not_start_work() {
        use swallowtail_core::HostServiceKind;

        let empty =
            HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"));
        assert!(!empty.available_kinds().contains(&HostServiceKind::Watcher));
        assert!(empty.watcher().is_none());

        let services =
            HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"))
                .with_watcher(Arc::new(IdleWatcherPort));
        assert!(
            services
                .available_kinds()
                .contains(&HostServiceKind::Watcher)
        );
        assert!(services.watcher().is_some());
    }

    struct IdleSignInPorts;

    impl crate::UrlOpenService for IdleSignInPorts {
        fn open(
            &self,
            _scope: crate::ScopeId,
            _url: crate::ApprovedUrlRef,
        ) -> crate::BoxFuture<'static, Result<(), crate::RuntimeFailure>> {
            panic!("registering a URL-open port must not start sign-in");
        }
    }

    impl crate::LoopbackCallbackService for IdleSignInPorts {
        fn bind(
            &self,
            _scope: crate::ScopeId,
        ) -> crate::BoxFuture<'static, Result<crate::LoopbackCallbackLease, crate::RuntimeFailure>>
        {
            panic!("registering a loopback port must not start sign-in");
        }

        fn poll(
            &self,
            _lease: &crate::LoopbackCallbackLease,
        ) -> crate::BoxFuture<
            'static,
            Result<Option<crate::LoopbackCallbackReceipt>, crate::RuntimeFailure>,
        > {
            panic!("registering a loopback port must not start sign-in");
        }

        fn materialize_credential(
            &self,
            _receipt: &crate::LoopbackCallbackReceipt,
            _audience: &swallowtail_core::EndpointAudience,
        ) -> Result<crate::CredentialRef, crate::RuntimeFailure> {
            panic!("registering a loopback port must not start sign-in");
        }

        fn release(
            &self,
            _lease: crate::LoopbackCallbackLease,
        ) -> crate::BoxFuture<'static, crate::CleanupOutcome> {
            panic!("registering a loopback port must not start sign-in");
        }
    }

    impl crate::DeviceCodeDisplayService for IdleSignInPorts {
        fn display(
            &self,
            _scope: crate::ScopeId,
            _prompt: crate::DeviceCodePrompt,
        ) -> crate::BoxFuture<'static, Result<(), crate::RuntimeFailure>> {
            panic!("registering a device-code port must not start sign-in");
        }

        fn poll_authorization(
            &self,
            _scope: &crate::ScopeId,
        ) -> crate::BoxFuture<
            'static,
            Result<Option<crate::DeviceAuthorizationReceipt>, crate::RuntimeFailure>,
        > {
            panic!("registering a device-code port must not start sign-in");
        }

        fn materialize_credential(
            &self,
            _receipt: &crate::DeviceAuthorizationReceipt,
            _audience: &swallowtail_core::EndpointAudience,
        ) -> Result<crate::CredentialRef, crate::RuntimeFailure> {
            panic!("registering a device-code port must not start sign-in");
        }
    }

    struct IdleWatcherPort;

    impl crate::WatcherHostService for IdleWatcherPort {
        fn accept_start(
            &self,
            _turn: crate::RuntimeTurnId,
            _summary: Option<swallowtail_core::WatcherSummary>,
        ) -> crate::BoxFuture<'static, Result<crate::WatcherSnapshot, crate::RuntimeFailure>>
        {
            panic!("registering a watcher port must not start work");
        }

        fn inspect(
            &self,
            _owning_turn: swallowtail_core::WatcherOwningTurn,
            _watcher_id: swallowtail_core::WatcherId,
        ) -> crate::BoxFuture<'static, Result<crate::WatcherSnapshot, crate::RuntimeFailure>>
        {
            panic!("registering a watcher port must not start work");
        }

        fn list(
            &self,
            _owning_turn: swallowtail_core::WatcherOwningTurn,
        ) -> crate::BoxFuture<'static, Result<Vec<crate::WatcherSnapshot>, crate::RuntimeFailure>>
        {
            panic!("registering a watcher port must not start work");
        }

        fn wait(
            &self,
            _owning_turn: swallowtail_core::WatcherOwningTurn,
            _watcher_id: swallowtail_core::WatcherId,
        ) -> crate::BoxFuture<
            'static,
            Result<crate::WatcherWaitRepresentation, crate::RuntimeFailure>,
        > {
            panic!("registering a watcher port must not start work");
        }

        fn request_stop(
            &self,
            _owning_turn: swallowtail_core::WatcherOwningTurn,
            _watcher_id: swallowtail_core::WatcherId,
        ) -> crate::BoxFuture<
            'static,
            Result<
                (crate::WatcherStopAcknowledgement, crate::WatcherSnapshot),
                crate::RuntimeFailure,
            >,
        > {
            panic!("registering a watcher port must not start work");
        }

        fn stop_and_join_all(
            &self,
            _turn: crate::RuntimeTurnId,
            _cause: swallowtail_core::WatcherCleanupCause,
        ) -> crate::BoxFuture<
            'static,
            Result<(Vec<crate::WatcherSnapshot>, CleanupOutcome), crate::RuntimeFailure>,
        > {
            panic!("registering a watcher port must not start work");
        }
    }

    #[test]
    fn observer_panic_does_not_alter_terminal_or_cleanup_truth() {
        let services =
            HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"))
                .with_diagnostic_observer(Arc::new(PanickingObserver));

        services.emit_diagnostic(&Diagnostic::new(SafeDiagnostic::new(
            "fixture.diagnostic",
            "Fixture diagnostic",
        )));
        services.emit_debug_observation(&DebugObservation::new(
            DebugObservationKind::Cleanup,
            "cleanup context",
        ));

        let status =
            TerminalStatus::RuntimeFailed(SafeDiagnostic::new("fixture.failure", "Fixture failed"));
        let cleanup = CleanupOutcome::Failed(SafeDiagnostic::new(
            "fixture.cleanup_failed",
            "Cleanup failed",
        ));
        assert!(matches!(status, TerminalStatus::RuntimeFailed(_)));
        assert!(matches!(cleanup, CleanupOutcome::Failed(_)));
    }

    #[derive(Default)]
    struct CountingObserver {
        debug_count: AtomicUsize,
        last_debug: Mutex<Option<DebugObservation>>,
    }

    impl DiagnosticObserver for CountingObserver {
        fn observe(&self, _diagnostic: &Diagnostic) {}

        fn observe_debug(&self, observation: &DebugObservation) {
            self.debug_count.fetch_add(1, Ordering::SeqCst);
            *self.last_debug.lock().expect("lock") = Some(observation.clone());
        }
    }

    struct PanickingObserver;

    impl DiagnosticObserver for PanickingObserver {
        fn observe(&self, _diagnostic: &Diagnostic) {
            panic!("observer must not control lifecycle");
        }

        fn observe_debug(&self, _observation: &DebugObservation) {
            panic!("observer must not control lifecycle");
        }
    }
}
