use crate::{RecordingHostServices, RuntimePreflightFixture, poll_immediate};
use std::sync::Arc;
use swallowtail_core::{
    DiscoveryOutcome, DiscoveryStatus, DriverRole, ExecutionHostId, PreflightPlan, SafeDiagnostic,
};
use swallowtail_runtime::{
    AttachServingRequest, AttachedServingHandle, BoxFuture, CleanupOutcome, DiscoveryDriver,
    DiscoveryRequest, DriverRegistration, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, ModelCatalogDriver, ModelCatalogRequest,
    OpenRealtimeMediaSessionRequest, OpenSessionRequest, OperationContent, OperationPolicy,
    OwnedServingHandle, RealtimeMediaSessionDriver, RealtimeMediaSessionHandle, RequestId,
    ResumeSessionRequest, RunHandle, RuntimeFailure, ServingInstanceDriver, ServingInstanceId,
    StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

struct RecordingRejectingDriver {
    calls: Arc<std::sync::atomic::AtomicUsize>,
}

impl RecordingRejectingDriver {
    fn reject<T>(&self) -> BoxFuture<'_, Result<T, RuntimeFailure>> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Box::pin(async {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "fixture.role_rejected",
                "Fixture role rejected",
            )))
        })
    }
}

impl DiscoveryDriver for RecordingRejectingDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Box::pin(async { Ok(vec![DiscoveryOutcome::new(DiscoveryStatus::Absent, None)]) })
    }
}

impl StructuredRunDriver for RecordingRejectingDriver {
    fn start_run(
        &self,
        _plan: PreflightPlan,
        _request: StructuredRunRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        self.reject()
    }
}

impl ModelCatalogDriver for RecordingRejectingDriver {
    fn list_models(
        &self,
        _plan: PreflightPlan,
        _request: ModelCatalogRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<swallowtail_core::ModelCatalogEntry>, RuntimeFailure>> {
        self.reject()
    }
}

impl InteractiveSessionDriver for RecordingRejectingDriver {
    fn open_session(
        &self,
        _plan: PreflightPlan,
        _request: OpenSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        self.reject()
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        self.reject()
    }
}

impl RealtimeMediaSessionDriver for RecordingRejectingDriver {
    fn open_realtime_media_session(
        &self,
        _plan: PreflightPlan,
        _request: OpenRealtimeMediaSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RealtimeMediaSessionHandle>, RuntimeFailure>> {
        self.reject()
    }
}

impl ServingInstanceDriver for RecordingRejectingDriver {
    fn attach(
        &self,
        _plan: PreflightPlan,
        _request: AttachServingRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn AttachedServingHandle>, RuntimeFailure>> {
        self.reject()
    }

    fn start(
        &self,
        _plan: PreflightPlan,
        _request: swallowtail_runtime::StartServingRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn OwnedServingHandle>, RuntimeFailure>> {
        self.reject()
    }
}

pub fn assert_dynamic_role_registration_and_calls() {
    let fixture = RuntimePreflightFixture::canonical();
    let driver = Arc::new(RecordingRejectingDriver {
        calls: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    });
    let calls = Arc::clone(&driver.calls);
    let descriptor = fixture.driver_descriptor().clone().with_roles([
        DriverRole::Discovery,
        DriverRole::ModelCatalog,
        DriverRole::StructuredRun,
        DriverRole::InteractiveSession,
        DriverRole::RealtimeMediaSession,
        DriverRole::ServingInstanceLifecycle,
    ]);
    let registration = DriverRegistration::new(descriptor)
        .with_discovery(driver.clone())
        .expect("descriptor declares discovery")
        .with_model_catalog(driver.clone())
        .expect("descriptor declares model catalog")
        .with_structured_run(driver.clone())
        .expect("descriptor declares structured run")
        .with_interactive_session(driver.clone())
        .expect("descriptor declares interactive session")
        .with_realtime_media_session(driver.clone())
        .expect("descriptor declares realtime media session")
        .with_serving_instance(driver)
        .expect("descriptor declares serving lifecycle");
    let services = RecordingHostServices::default().services().clone();
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host id is valid");

    let discovered = poll_immediate(
        registration
            .discovery()
            .expect("discovery role is registered")
            .discover(DiscoveryRequest::new(host_id), services.clone()),
    )
    .expect("discovery fixture succeeds");
    assert_eq!(discovered[0].status(), DiscoveryStatus::Absent);

    let plan = fixture.preflight().expect("fixture preflight succeeds");
    let catalog_result = poll_immediate(
        registration
            .model_catalog()
            .expect("model catalog role is registered")
            .list_models(
                plan.clone(),
                ModelCatalogRequest::new(valid_request("request-catalog")),
                services.clone(),
            ),
    );
    assert!(catalog_result.is_err());
    let run_result = poll_immediate(
        registration
            .structured_run()
            .expect("run role is registered")
            .start_run(
                plan.clone(),
                StructuredRunRequest::new(
                    valid_request("request-run"),
                    OperationContent::new("fixture prompt").expect("content is valid"),
                    OperationPolicy::offline(),
                )
                .with_working_resource(
                    WorkingResourceRef::new("fixture-resource").expect("resource is valid"),
                ),
                services.clone(),
            ),
    );
    assert!(run_result.is_err());
    let session_result = poll_immediate(
        registration
            .interactive_session()
            .expect("session role is registered")
            .open_session(
                plan.clone(),
                OpenSessionRequest::new(
                    valid_request("request-session"),
                    WorkingResourceRef::new("fixture-resource").expect("resource is valid"),
                    None,
                ),
                services.clone(),
            ),
    );
    assert!(session_result.is_err());
    let media_result = poll_immediate(
        registration
            .realtime_media_session()
            .expect("realtime media role is registered")
            .open_realtime_media_session(
                plan.clone(),
                OpenRealtimeMediaSessionRequest::new(
                    valid_request("request-media"),
                    crate::realtime_media_fixture::realtime_media_config(),
                    None,
                ),
                services.clone(),
            ),
    );
    assert!(media_result.is_err());
    let serving_result = poll_immediate(
        registration
            .serving_instance()
            .expect("serving role is registered")
            .attach(
                plan,
                AttachServingRequest::new(
                    ServingInstanceId::new("serving-1").expect("serving id is valid"),
                ),
                services,
            ),
    );
    assert!(serving_result.is_err());

    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 6);
}

pub fn assert_missing_roles_are_explicit() {
    let fixture = RuntimePreflightFixture::canonical();
    let registration = DriverRegistration::new(fixture.driver_descriptor().clone());

    assert!(registration.discovery().is_none());
    assert!(registration.model_catalog().is_none());
    assert!(registration.structured_run().is_none());
    assert!(registration.interactive_session().is_none());
    assert!(registration.realtime_media_session().is_none());
    assert!(registration.serving_instance().is_none());

    let driver = Arc::new(RecordingRejectingDriver {
        calls: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    });
    let undeclared =
        DriverRegistration::new(fixture.driver_descriptor().clone()).with_discovery(driver);
    assert!(matches!(
        undeclared,
        Err(failure) if failure.role() == DriverRole::Discovery
    ));
}

pub fn assert_cleanup_states_remain_distinct() {
    let failure = SafeDiagnostic::new("fixture.cleanup", "Cleanup failed");
    assert_ne!(CleanupOutcome::Clean, CleanupOutcome::Failed(failure));
    assert_ne!(CleanupOutcome::NotApplicable, CleanupOutcome::Clean);
}

fn valid_request(value: &str) -> RequestId {
    RequestId::new(value).expect("fixture request id is valid")
}
