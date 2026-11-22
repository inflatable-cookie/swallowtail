use super::{
    ConnectionLifecycleStore, ConnectionLifecycleStoreFailure, SignInAuthorityBinding,
    SignInFailureKind, SignInMethod, SignInStartRequest, SignInStatus, cancel_sign_in,
    cancel_sign_in_async, complete_sign_in, complete_sign_in_async, poll_sign_in,
    poll_sign_in_async, start_sign_in, start_sign_in_async, submit_sign_in_credential_field,
};
use crate::{
    ApprovedUrlRef, BoxFuture, CleanupOutcome, CredentialRef, Deadline, DeviceAuthorizationId,
    DeviceAuthorizationReceipt, DeviceCodeDisplayService, DeviceCodePrompt, ExecutableRef,
    HostServices, LoopbackCallbackId, LoopbackCallbackLease, LoopbackCallbackReceipt,
    LoopbackCallbackService, MonotonicInstant, ProcessExit, ProcessHandle, ProcessInputChunk,
    ProcessOutputChunk, ProcessRequest, ProcessService, RuntimeFailure, ScopeId, TimeService,
    UrlOpenService,
};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, AddableRouteId, AdmittedInstanceRecord,
    ConfiguredInstanceId, CredentialFieldDescriptor, CredentialFieldId, CredentialFieldVisibility,
    CredentialMechanism, EndpointAudience, EntitlementMetering, ExecutionHostId, FieldLabel,
    HostServiceKind, IntegrationFamilyId, OverlayMarker, RouteTopology, SignInAction,
};

struct MemoryStore {
    instances: Mutex<BTreeMap<String, AdmittedInstanceRecord>>,
}

impl MemoryStore {
    fn new() -> Self {
        Self {
            instances: Mutex::new(BTreeMap::new()),
        }
    }
}

impl ConnectionLifecycleStore for MemoryStore {
    fn put_instance(
        &self,
        record: AdmittedInstanceRecord,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        self.instances
            .lock()
            .expect("store lock poisoned")
            .insert(record.id().as_str().to_owned(), record);
        Ok(())
    }

    fn get_instance(
        &self,
        id: &ConfiguredInstanceId,
    ) -> Result<Option<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .instances
            .lock()
            .expect("store lock poisoned")
            .get(id.as_str())
            .cloned())
    }

    fn list_instances(
        &self,
    ) -> Result<Vec<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .instances
            .lock()
            .expect("store lock poisoned")
            .values()
            .cloned()
            .collect())
    }

    fn put_overlay_marker(
        &self,
        _marker: OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        Ok(())
    }

    fn list_overlay_markers(&self) -> Result<Vec<OverlayMarker>, ConnectionLifecycleStoreFailure> {
        Ok(Vec::new())
    }
}

struct MockSignInPorts {
    calls: Mutex<Vec<&'static str>>,
    loopback_ready: Mutex<bool>,
    device_ready: Mutex<bool>,
    credential: CredentialRef,
    audience: EndpointAudience,
}

impl MockSignInPorts {
    fn new(credential: CredentialRef, audience: EndpointAudience) -> Arc<Self> {
        Arc::new(Self {
            calls: Mutex::new(Vec::new()),
            loopback_ready: Mutex::new(false),
            device_ready: Mutex::new(false),
            credential,
            audience,
        })
    }

    fn record(&self, call: &'static str) {
        self.calls.lock().expect("lock").push(call);
    }

    fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().expect("lock").clone()
    }
}

impl UrlOpenService for MockSignInPorts {
    fn open(
        &self,
        _scope: ScopeId,
        _url: ApprovedUrlRef,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.record("url_open");
        Box::pin(async { Ok(()) })
    }
}

impl LoopbackCallbackService for MockSignInPorts {
    fn bind(
        &self,
        scope: ScopeId,
    ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
        self.record("loopback_bind");
        Box::pin(async move {
            Ok(LoopbackCallbackLease::new(
                scope,
                LoopbackCallbackId::new("fixture.loopback").expect("id is valid"),
            ))
        })
    }

    fn poll(
        &self,
        lease: &LoopbackCallbackLease,
    ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
        self.record("loopback_poll");
        let ready = *self.loopback_ready.lock().expect("lock");
        let receipt = ready.then(|| LoopbackCallbackReceipt::new(lease.callback_id().clone()));
        Box::pin(async move { Ok(receipt) })
    }

    fn materialize_credential(
        &self,
        _receipt: &LoopbackCallbackReceipt,
        audience: &EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure> {
        self.record("loopback_materialize");
        if audience != &self.audience {
            return Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.audience_mismatch",
                "Mock loopback credential is bound to a different audience",
            )));
        }
        Ok(self.credential.clone())
    }

    fn release(&self, _lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
        self.record("loopback_release");
        Box::pin(async { CleanupOutcome::Clean })
    }
}

impl DeviceCodeDisplayService for MockSignInPorts {
    fn display(
        &self,
        _scope: ScopeId,
        _prompt: DeviceCodePrompt,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.record("device_display");
        Box::pin(async { Ok(()) })
    }

    fn poll_authorization(
        &self,
        _scope: &ScopeId,
    ) -> BoxFuture<'static, Result<Option<DeviceAuthorizationReceipt>, RuntimeFailure>> {
        self.record("device_poll");
        let ready = *self.device_ready.lock().expect("lock");
        let receipt = ready.then(|| {
            DeviceAuthorizationReceipt::new(
                DeviceAuthorizationId::new("fixture.device").expect("id is valid"),
            )
        });
        Box::pin(async move { Ok(receipt) })
    }

    fn materialize_credential(
        &self,
        _receipt: &DeviceAuthorizationReceipt,
        audience: &EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure> {
        self.record("device_materialize");
        if audience != &self.audience {
            return Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.audience_mismatch",
                "Mock device credential is bound to a different audience",
            )));
        }
        Ok(self.credential.clone())
    }
}

struct FixedTime(u64);

impl TimeService for FixedTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(self.0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, crate::DeadlineObservation> {
        Box::pin(async move { crate::DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

struct ImmediateProcess;

impl ProcessHandle for ImmediateProcess {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

impl ProcessService for ImmediateProcess {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        Box::pin(async { Ok(Box::new(ImmediateProcess) as Box<dyn ProcessHandle>) })
    }
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("host.local").expect("host id is valid")
}

fn scope() -> ScopeId {
    ScopeId::new("sign-in").expect("scope is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new("instance-work").expect("instance id is valid")
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("fixture-hosted").expect("family is valid")
}

fn route_id() -> AddableRouteId {
    AddableRouteId::new("fixture-hosted-messages").expect("route id is valid")
}

fn audience() -> EndpointAudience {
    EndpointAudience::new("api.example.test").expect("audience is valid")
}

fn field_id() -> CredentialFieldId {
    CredentialFieldId::new("api_key").expect("field id is valid")
}

fn credential() -> CredentialRef {
    CredentialRef::new("stored-credential-ref").expect("credential is valid")
}

fn interactive_services(ports: Arc<MockSignInPorts>) -> HostServices {
    HostServices::new(host_id())
        .with_url_open(ports.clone())
        .with_loopback_callback(ports)
        .with_time(Arc::new(FixedTime(1)))
}

fn interactive_request() -> SignInStartRequest {
    SignInStartRequest::new(
        scope(),
        instance_id(),
        family(),
        route_id(),
        SignInAuthorityBinding::new(
            CredentialMechanism::InteractiveOauth,
            audience(),
            EntitlementMetering::SubscriptionAllowance,
        ),
        SignInMethod::InteractiveOauth {
            authorize_url: ApprovedUrlRef::new(
                "https://login.example.test/authorize?secret=token-bytes",
            )
            .expect("url is valid"),
            credential_field: field_id(),
        },
    )
}

fn admitted(store: &MemoryStore) {
    let record = AdmittedInstanceRecord::new(
        instance_id(),
        family(),
        route_id(),
        AdapterIdentity::new(
            AdapterId::new("swallowtail-adapter-fixture-hosted").expect("adapter id is valid"),
            AdapterVersion::new("0.0.0").expect("version is valid"),
        ),
        RouteTopology::Hosted,
    );
    store.put_instance(record).expect("admit fixture instance");
}

#[test]
fn interactive_oauth_start_poll_complete_cancel_and_timeout() {
    let ports = MockSignInPorts::new(credential(), audience());
    let services = interactive_services(ports.clone());
    let mut session = start_sign_in(&services, interactive_request()).expect("start succeeds");
    assert_eq!(session.status(), SignInStatus::Started);
    assert_eq!(ports.calls(), vec!["loopback_bind", "url_open"]);
    assert!(!format!("{session:?}").contains("token-bytes"));

    assert_eq!(
        poll_sign_in(&mut session, &services).expect("poll waits"),
        SignInStatus::InProgress
    );
    *ports.loopback_ready.lock().expect("lock") = true;
    assert_eq!(
        poll_sign_in(&mut session, &services).expect("poll ready"),
        SignInStatus::ReadyToComplete
    );

    let outcome =
        complete_sign_in(session, &services, None).expect("complete materializes a reference");
    assert_eq!(
        outcome
            .credential_refs()
            .next()
            .map(|(_, reference)| reference.clone()),
        Some(credential())
    );
    assert_eq!(outcome.audience(), &audience());
    assert_eq!(outcome.mechanism(), &CredentialMechanism::InteractiveOauth);
    assert!(!format!("{outcome:?}").contains("token-bytes"));
    assert!(ports.calls().contains(&"loopback_materialize"));
    assert!(!ports.calls().contains(&"authenticate"));

    let session = start_sign_in(&services, interactive_request()).expect("restart");
    assert_eq!(
        cancel_sign_in(session, &services).expect("cancel succeeds"),
        SignInStatus::Cancelled
    );

    let mut timed_out = start_sign_in(
        &services,
        interactive_request().with_deadline(Deadline::at(MonotonicInstant::from_ticks(0))),
    )
    .expect("start with elapsed deadline");
    assert_eq!(
        poll_sign_in(&mut timed_out, &services).expect("timeout is explicit"),
        SignInStatus::TimedOut
    );
}

#[test]
fn sign_in_action_is_not_permission_to_execute() {
    let _advertisement = SignInAction::Interactive;
    let ports = MockSignInPorts::new(credential(), audience());
    let services = interactive_services(ports);
    start_sign_in(&services, interactive_request())
        .expect("SignInAction is an advertisement; start runs from an explicit loop request");
}

#[test]
fn device_and_delegated_loops_use_matching_ports_not_authenticate() {
    let ports = MockSignInPorts::new(credential(), audience());
    let services = HostServices::new(host_id())
        .with_device_code_display(ports.clone())
        .with_process(Arc::new(ImmediateProcess))
        .with_time(Arc::new(FixedTime(1)));

    let mut device = start_sign_in(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::DeviceOauth,
                audience(),
                EntitlementMetering::SubscriptionAllowance,
            ),
            SignInMethod::DeviceOauth {
                prompt: DeviceCodePrompt::new("WDJB-MJHT").expect("code is valid"),
                credential_field: field_id(),
            },
        ),
    )
    .expect("device start displays a code");
    assert!(ports.calls().contains(&"device_display"));
    *ports.device_ready.lock().expect("lock") = true;
    assert_eq!(
        poll_sign_in(&mut device, &services).expect("device ready"),
        SignInStatus::ReadyToComplete
    );
    complete_sign_in(device, &services, None).expect("device complete");

    let mut delegated = start_sign_in(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    )
    .expect("delegated start uses process authority");
    assert_eq!(
        poll_sign_in(&mut delegated, &services).expect("helper exit"),
        SignInStatus::ReadyToComplete
    );
    complete_sign_in(delegated, &services, None).expect("delegated complete");
    assert!(!ports.calls().contains(&"authenticate"));
}

#[test]
fn mechanism_or_account_change_fails_closed() {
    let ports = MockSignInPorts::new(credential(), audience());
    let services = interactive_services(ports.clone());
    let mismatch = start_sign_in(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::ApiKey,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::InteractiveOauth {
                authorize_url: ApprovedUrlRef::new("https://login.example.test/authorize")
                    .expect("url is valid"),
                credential_field: field_id(),
            },
        ),
    )
    .expect_err("kind/mechanism mismatch fails closed");
    assert_eq!(mismatch.kind(), SignInFailureKind::MechanismMismatch);

    let mut session = start_sign_in(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::InteractiveOauth,
                audience(),
                EntitlementMetering::SubscriptionAllowance,
            )
            .with_existing_credential(
                CredentialRef::new("existing-account").expect("existing ref is valid"),
            ),
            SignInMethod::InteractiveOauth {
                authorize_url: ApprovedUrlRef::new("https://login.example.test/authorize")
                    .expect("url is valid"),
                credential_field: field_id(),
            },
        ),
    )
    .expect("start with bound account");
    *ports.loopback_ready.lock().expect("lock") = true;
    poll_sign_in(&mut session, &services).expect("ready");
    let account = complete_sign_in(session, &services, None)
        .map_err(|(failure, _)| failure)
        .expect_err("replacing the bound credential fails closed");
    assert_eq!(account.kind(), SignInFailureKind::AccountMismatch);
}

#[test]
fn missing_url_loopback_or_device_port_fails_the_matching_loop() {
    let ports = MockSignInPorts::new(credential(), audience());
    let no_url = HostServices::new(host_id()).with_loopback_callback(ports.clone());
    assert!(!no_url.available_kinds().contains(&HostServiceKind::UrlOpen));
    let missing_url = start_sign_in(&no_url, interactive_request()).expect_err("url required");
    assert_eq!(missing_url.kind(), SignInFailureKind::MissingHostPort);

    let no_loopback = HostServices::new(host_id()).with_url_open(ports.clone());
    let missing_loopback =
        start_sign_in(&no_loopback, interactive_request()).expect_err("loopback required");
    assert_eq!(missing_loopback.kind(), SignInFailureKind::MissingHostPort);

    let no_device = HostServices::new(host_id());
    let missing_device = start_sign_in(
        &no_device,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::DeviceOauth,
                audience(),
                EntitlementMetering::SubscriptionAllowance,
            ),
            SignInMethod::DeviceOauth {
                prompt: DeviceCodePrompt::new("WDJB-MJHT").expect("code is valid"),
                credential_field: field_id(),
            },
        ),
    )
    .expect_err("device-code port required");
    assert_eq!(missing_device.kind(), SignInFailureKind::MissingHostPort);
}

#[test]
fn api_key_collection_stores_a_reference_not_the_secret() {
    let store = MemoryStore::new();
    admitted(&store);
    let services = HostServices::new(host_id());
    let descriptor = CredentialFieldDescriptor::new(
        field_id(),
        FieldLabel::new("API key").expect("label is valid"),
        CredentialFieldVisibility::Secret,
    );
    let mut session = start_sign_in(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::ApiKey,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::ApiKeyCollection {
                fields: vec![descriptor],
            },
        ),
    )
    .expect("api-key collection starts without browser ports");
    assert_eq!(
        submit_sign_in_credential_field(&mut session, field_id(), credential())
            .expect("submit stores a reference"),
        SignInStatus::ReadyToComplete
    );
    let unknown = submit_sign_in_credential_field(
        &mut session,
        CredentialFieldId::new("other").expect("field is valid"),
        credential(),
    )
    .expect_err("unknown field fails");
    assert_eq!(unknown.kind(), SignInFailureKind::UnknownCredentialField);

    let outcome = complete_sign_in(session, &services, Some(&store)).expect("complete persists");
    let stored = store
        .get_instance(&instance_id())
        .expect("store reads")
        .expect("instance present");
    assert_eq!(
        stored
            .credential_refs()
            .next()
            .map(|(_, reference)| reference.clone()),
        Some(credential())
    );
    assert!(!format!("{stored:?}").contains("sk-secret-bytes"));
    assert!(!format!("{outcome:?}").contains("sk-secret-bytes"));
    assert!(services.credential().is_none());
}

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use std::task::{Context, Poll, Waker};
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    for _ in 0..128 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
    panic!("fixture future did not resolve within bounded polls");
}

fn yield_once_then<T: Send + 'static>(
    produce: impl FnOnce() -> T + Send + 'static,
) -> BoxFuture<'static, T> {
    let mut yielded = false;
    let mut produce = Some(produce);
    Box::pin(std::future::poll_fn(move |context| {
        if !yielded {
            yielded = true;
            context.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            let f = produce.take().expect("future polled after completion");
            std::task::Poll::Ready(f())
        }
    }))
}

struct AsyncSignInPorts {
    calls: Mutex<Vec<&'static str>>,
    open_fails: bool,
    materialize_fails: bool,
    pending_polls_before_ready: usize,
    credential: CredentialRef,
    audience: EndpointAudience,
}

impl AsyncSignInPorts {
    fn new(credential: CredentialRef, audience: EndpointAudience) -> Arc<Self> {
        Arc::new(Self {
            calls: Mutex::new(Vec::new()),
            open_fails: false,
            materialize_fails: false,
            pending_polls_before_ready: 1,
            credential,
            audience,
        })
    }

    fn record(&self, call: &'static str) {
        self.calls.lock().expect("lock").push(call);
    }

    fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().expect("lock").clone()
    }
}

impl UrlOpenService for AsyncSignInPorts {
    fn open(
        &self,
        _scope: ScopeId,
        _url: ApprovedUrlRef,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.record("url_open");
        let fails = self.open_fails;
        yield_once_then(move || {
            if fails {
                Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.url_open_failed",
                    "URL open failed",
                )))
            } else {
                Ok(())
            }
        })
    }
}

impl LoopbackCallbackService for AsyncSignInPorts {
    fn bind(
        &self,
        scope: ScopeId,
    ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
        self.record("loopback_bind");
        yield_once_then(move || {
            Ok(LoopbackCallbackLease::new(
                scope,
                LoopbackCallbackId::new("fixture.async.loopback").expect("id is valid"),
            ))
        })
    }

    fn poll(
        &self,
        lease: &LoopbackCallbackLease,
    ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
        self.record("loopback_poll");
        let callback_id = lease.callback_id().clone();
        let mut remaining_pending = self.pending_polls_before_ready;
        Box::pin(std::future::poll_fn(move |context| {
            if remaining_pending > 0 {
                remaining_pending -= 1;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            } else {
                std::task::Poll::Ready(Ok(Some(LoopbackCallbackReceipt::new(callback_id.clone()))))
            }
        }))
    }

    fn materialize_credential(
        &self,
        _receipt: &LoopbackCallbackReceipt,
        audience: &EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure> {
        self.record("loopback_materialize");
        if self.materialize_fails || audience != &self.audience {
            return Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.materialize_failed",
                "Loopback materialization failed",
            )));
        }
        Ok(self.credential.clone())
    }

    fn release(&self, _lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
        self.record("loopback_release");
        yield_once_then(|| CleanupOutcome::Clean)
    }
}

impl DeviceCodeDisplayService for AsyncSignInPorts {
    fn display(
        &self,
        _scope: ScopeId,
        _prompt: DeviceCodePrompt,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.record("device_display");
        yield_once_then(|| Ok(()))
    }

    fn poll_authorization(
        &self,
        _scope: &ScopeId,
    ) -> BoxFuture<'static, Result<Option<DeviceAuthorizationReceipt>, RuntimeFailure>> {
        self.record("device_poll");
        let mut remaining_pending = self.pending_polls_before_ready;
        Box::pin(std::future::poll_fn(move |context| {
            if remaining_pending > 0 {
                remaining_pending -= 1;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            } else {
                std::task::Poll::Ready(Ok(Some(DeviceAuthorizationReceipt::new(
                    DeviceAuthorizationId::new("fixture.async.device").expect("id is valid"),
                ))))
            }
        }))
    }

    fn materialize_credential(
        &self,
        _receipt: &DeviceAuthorizationReceipt,
        _audience: &EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure> {
        self.record("device_materialize");
        Ok(self.credential.clone())
    }
}

struct TrackingProcess {
    calls: Mutex<Vec<&'static str>>,
    exited: std::sync::atomic::AtomicBool,
    async_cleanup: bool,
}

impl TrackingProcess {
    fn new(async_cleanup: bool) -> Arc<Self> {
        Arc::new(Self {
            calls: Mutex::new(Vec::new()),
            exited: std::sync::atomic::AtomicBool::new(false),
            async_cleanup,
        })
    }

    fn record(&self, call: &'static str) {
        self.calls.lock().expect("lock").push(call);
    }

    fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().expect("lock").clone()
    }
}

struct TrackingProcessHandle {
    shared: Arc<TrackingProcess>,
}

impl ProcessHandle for TrackingProcessHandle {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.shared.record("process_request_stop");
        if self.shared.async_cleanup {
            yield_once_then(|| Ok(()))
        } else {
            Box::pin(async { Ok(()) })
        }
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.shared.record("process_force_stop");
        self.shared
            .exited
            .store(true, std::sync::atomic::Ordering::SeqCst);
        if self.shared.async_cleanup {
            yield_once_then(|| Ok(()))
        } else {
            Box::pin(async { Ok(()) })
        }
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.shared.record("process_wait");
        let shared = Arc::clone(&self.shared);
        Box::pin(std::future::poll_fn(move |context| {
            if shared.exited.load(std::sync::atomic::Ordering::SeqCst) {
                if shared.async_cleanup {
                    shared
                        .exited
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                    context.waker().wake_by_ref();
                    // One pending tick after exit before reporting reaped.
                    std::task::Poll::Pending
                } else {
                    std::task::Poll::Ready(Ok(ProcessExit::new(true, Some(0))))
                }
            } else if shared.async_cleanup
                && shared
                    .calls
                    .lock()
                    .expect("lock")
                    .contains(&"process_force_stop")
            {
                std::task::Poll::Ready(Ok(ProcessExit::new(true, Some(0))))
            } else {
                std::task::Poll::Pending
            }
        }))
    }
}

impl ProcessService for TrackingProcess {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        self.record("process_start");
        let shared = Arc::clone(self.calls_arc_workaround());
        if self.async_cleanup {
            yield_once_then(move || {
                Ok(Box::new(TrackingProcessHandle { shared }) as Box<dyn ProcessHandle>)
            })
        } else {
            Box::pin(async move {
                Ok(Box::new(TrackingProcessHandle { shared }) as Box<dyn ProcessHandle>)
            })
        }
    }
}

struct SharedProcessService(Arc<TrackingProcess>);

impl ProcessService for SharedProcessService {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        self.0.record("process_start");
        let shared = Arc::clone(&self.0);
        if shared.async_cleanup {
            yield_once_then(move || {
                Ok(Box::new(TrackingProcessHandle { shared }) as Box<dyn ProcessHandle>)
            })
        } else {
            Box::pin(async move {
                Ok(Box::new(TrackingProcessHandle { shared }) as Box<dyn ProcessHandle>)
            })
        }
    }
}

impl TrackingProcess {
    fn calls_arc_workaround(&self) -> &Arc<Self> {
        panic!("use SharedProcessService wrapper")
    }
}

#[test]
fn async_sign_in_ports_complete_and_cancel_without_poll_now_panic() {
    let ports = AsyncSignInPorts::new(credential(), audience());
    let process = TrackingProcess::new(true);
    let services = HostServices::new(host_id())
        .with_url_open(ports.clone())
        .with_loopback_callback(ports.clone())
        .with_device_code_display(ports.clone())
        .with_process(Arc::new(SharedProcessService(Arc::clone(&process))))
        .with_time(Arc::new(FixedTime(1)));

    let mut session = block_on(start_sign_in_async(&services, interactive_request()))
        .expect("async interactive start succeeds across Poll::Pending");
    assert_eq!(
        block_on(poll_sign_in_async(&mut session, &services)).expect("async poll succeeds"),
        SignInStatus::ReadyToComplete
    );
    let outcome = block_on(complete_sign_in_async(session, &services, None))
        .expect("async complete succeeds");
    assert_eq!(outcome.audience(), &audience());
    assert!(ports.calls().contains(&"loopback_release"));

    let mut device = block_on(start_sign_in_async(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::DeviceOauth,
                audience(),
                EntitlementMetering::SubscriptionAllowance,
            ),
            SignInMethod::DeviceOauth {
                prompt: DeviceCodePrompt::new("WDJB-MJHT").expect("code is valid"),
                credential_field: field_id(),
            },
        ),
    ))
    .expect("async device start succeeds");
    assert_eq!(
        block_on(poll_sign_in_async(&mut device, &services)).expect("async device poll succeeds"),
        SignInStatus::ReadyToComplete
    );
    block_on(complete_sign_in_async(device, &services, None)).expect("async device complete");

    let delegated = block_on(start_sign_in_async(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    ))
    .expect("async delegated start succeeds");
    assert_eq!(
        block_on(cancel_sign_in_async(delegated, &services)).expect("async cancel succeeds"),
        SignInStatus::Cancelled
    );
    assert_eq!(
        process.calls(),
        vec![
            "process_start",
            "process_request_stop",
            "process_force_stop",
            "process_wait",
        ]
    );
}

#[test]
fn sync_sign_in_handles_poll_pending_without_panicking() {
    let async_ports = AsyncSignInPorts::new(credential(), audience());
    let async_services = HostServices::new(host_id())
        .with_url_open(async_ports.clone())
        .with_loopback_callback(async_ports.clone())
        .with_time(Arc::new(FixedTime(1)));
    let pending_start = start_sign_in(&async_services, interactive_request())
        .expect_err("pending start future fails closed instead of panicking");
    assert_eq!(pending_start.kind(), SignInFailureKind::HostPort);

    struct ImmediateStartAsyncPollPorts {
        inner: Arc<AsyncSignInPorts>,
        calls: Mutex<Vec<&'static str>>,
    }
    impl UrlOpenService for ImmediateStartAsyncPollPorts {
        fn open(
            &self,
            _scope: ScopeId,
            _url: ApprovedUrlRef,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            self.calls.lock().expect("lock").push("url_open");
            Box::pin(async { Ok(()) })
        }
    }
    impl LoopbackCallbackService for ImmediateStartAsyncPollPorts {
        fn bind(
            &self,
            scope: ScopeId,
        ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
            self.calls.lock().expect("lock").push("loopback_bind");
            Box::pin(async move {
                Ok(LoopbackCallbackLease::new(
                    scope,
                    LoopbackCallbackId::new("fixture.sync.loopback").expect("id is valid"),
                ))
            })
        }
        fn poll(
            &self,
            lease: &LoopbackCallbackLease,
        ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
            self.inner.poll(lease)
        }
        fn materialize_credential(
            &self,
            receipt: &LoopbackCallbackReceipt,
            audience: &EndpointAudience,
        ) -> Result<CredentialRef, RuntimeFailure> {
            LoopbackCallbackService::materialize_credential(&*self.inner, receipt, audience)
        }
        fn release(&self, lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
            self.inner.release(lease)
        }
    }
    impl DeviceCodeDisplayService for ImmediateStartAsyncPollPorts {
        fn display(
            &self,
            _scope: ScopeId,
            _prompt: DeviceCodePrompt,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn poll_authorization(
            &self,
            scope: &ScopeId,
        ) -> BoxFuture<'static, Result<Option<DeviceAuthorizationReceipt>, RuntimeFailure>>
        {
            self.inner.poll_authorization(scope)
        }
        fn materialize_credential(
            &self,
            receipt: &DeviceAuthorizationReceipt,
            audience: &EndpointAudience,
        ) -> Result<CredentialRef, RuntimeFailure> {
            DeviceCodeDisplayService::materialize_credential(&*self.inner, receipt, audience)
        }
    }

    let ports = Arc::new(ImmediateStartAsyncPollPorts {
        inner: AsyncSignInPorts::new(credential(), audience()),
        calls: Mutex::new(Vec::new()),
    });
    let services = HostServices::new(host_id())
        .with_url_open(ports.clone())
        .with_loopback_callback(ports.clone())
        .with_device_code_display(ports.clone())
        .with_time(Arc::new(FixedTime(1)));

    let mut session = start_sign_in(&services, interactive_request()).expect("start succeeds");
    assert_eq!(
        poll_sign_in(&mut session, &services).expect("first pending poll does not panic"),
        SignInStatus::InProgress
    );
    assert_eq!(
        poll_sign_in(&mut session, &services).expect("second poll advances pending future"),
        SignInStatus::ReadyToComplete
    );
    let (cancel_err, session) = cancel_sign_in(session, &services)
        .expect_err("sync cancel with pending release must fail closed");
    assert_eq!(cancel_err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        cancel_err.diagnostic().code(),
        "swallowtail.sign_in.host_port_pending"
    );
    assert!(ports.inner.calls().contains(&"loopback_release"));

    // The returned session keeps the pending release, so async cleanup finishes it.
    assert_eq!(
        block_on(cancel_sign_in_async(session, &services))
            .expect("async cancel completes the retained release"),
        SignInStatus::Cancelled
    );
}

#[test]
fn sync_sign_in_cleanup_fails_closed_when_host_cleanup_is_pending() {
    struct ImmediateStartPendingReleasePorts {
        release_completed: Arc<std::sync::atomic::AtomicBool>,
    }
    impl UrlOpenService for ImmediateStartPendingReleasePorts {
        fn open(
            &self,
            _scope: ScopeId,
            _url: ApprovedUrlRef,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
    }
    impl LoopbackCallbackService for ImmediateStartPendingReleasePorts {
        fn bind(
            &self,
            scope: ScopeId,
        ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
            Box::pin(async move {
                Ok(LoopbackCallbackLease::new(
                    scope,
                    LoopbackCallbackId::new("fixture.pending.release").expect("id is valid"),
                ))
            })
        }
        fn poll(
            &self,
            lease: &LoopbackCallbackLease,
        ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
            let callback_id = lease.callback_id().clone();
            Box::pin(async move { Ok(Some(LoopbackCallbackReceipt::new(callback_id))) })
        }
        fn materialize_credential(
            &self,
            _receipt: &LoopbackCallbackReceipt,
            _audience: &EndpointAudience,
        ) -> Result<CredentialRef, RuntimeFailure> {
            Ok(credential())
        }
        fn release(&self, _lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
            let completed = Arc::clone(&self.release_completed);
            let mut yielded = false;
            Box::pin(std::future::poll_fn(move |context| {
                if !yielded {
                    yielded = true;
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                } else {
                    completed.store(true, std::sync::atomic::Ordering::SeqCst);
                    std::task::Poll::Ready(CleanupOutcome::Clean)
                }
            }))
        }
    }

    struct ImmediateStartPendingCleanupProcess {
        stop_completed: Arc<std::sync::atomic::AtomicBool>,
        wait_completed: Arc<std::sync::atomic::AtomicBool>,
    }
    struct PendingCleanupHandle {
        stop_completed: Arc<std::sync::atomic::AtomicBool>,
        wait_completed: Arc<std::sync::atomic::AtomicBool>,
    }
    impl ProcessHandle for PendingCleanupHandle {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            let stop_completed = Arc::clone(&self.stop_completed);
            yield_once_then(move || {
                stop_completed.store(true, std::sync::atomic::Ordering::SeqCst);
                Ok(())
            })
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            yield_once_then(|| Ok(()))
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            let wait_completed = Arc::clone(&self.wait_completed);
            yield_once_then(move || {
                wait_completed.store(true, std::sync::atomic::Ordering::SeqCst);
                Ok(ProcessExit::new(true, Some(0)))
            })
        }
    }
    impl ProcessService for ImmediateStartPendingCleanupProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            let stop_completed = Arc::clone(&self.stop_completed);
            let wait_completed = Arc::clone(&self.wait_completed);
            Box::pin(async move {
                Ok(Box::new(PendingCleanupHandle {
                    stop_completed,
                    wait_completed,
                }) as Box<dyn ProcessHandle>)
            })
        }
    }

    let delegated_request = || {
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        )
    };

    // 1. poll_sign_in timeout with pending loopback release fails closed and retains cleanup for async completion.
    let release_completed = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let loopback_ports = Arc::new(ImmediateStartPendingReleasePorts {
        release_completed: Arc::clone(&release_completed),
    });
    let loopback_services = HostServices::new(host_id())
        .with_url_open(loopback_ports.clone())
        .with_loopback_callback(loopback_ports.clone())
        .with_time(Arc::new(FixedTime(1)));

    let mut timed_out = start_sign_in(
        &loopback_services,
        interactive_request().with_deadline(Deadline::at(MonotonicInstant::from_ticks(0))),
    )
    .expect("start succeeds");
    let err = poll_sign_in(&mut timed_out, &loopback_services)
        .expect_err("sync timeout with pending loopback release must fail closed");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.sign_in.host_port_pending"
    );
    assert_ne!(timed_out.status(), SignInStatus::TimedOut);
    assert!(!release_completed.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(
        block_on(poll_sign_in_async(&mut timed_out, &loopback_services))
            .expect("async follow-up completes retained release"),
        SignInStatus::TimedOut
    );
    assert!(release_completed.load(std::sync::atomic::Ordering::SeqCst));

    // 2. complete_sign_in on Incomplete and ReadyToComplete with pending loopback release fails closed without mutating store.
    let incomplete_session =
        start_sign_in(&loopback_services, interactive_request()).expect("start succeeds");
    let err = complete_sign_in(incomplete_session, &loopback_services, None)
        .map_err(|(failure, _)| failure)
        .expect_err("sync complete error path with pending release must fail closed");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.sign_in.host_port_pending"
    );

    let store = MemoryStore::new();
    admitted(&store);
    let mut ready_session =
        start_sign_in(&loopback_services, interactive_request()).expect("start succeeds");
    assert_eq!(
        poll_sign_in(&mut ready_session, &loopback_services).expect("ready"),
        SignInStatus::ReadyToComplete
    );
    let err = complete_sign_in(ready_session, &loopback_services, Some(&store))
        .map_err(|(failure, _)| failure)
        .expect_err("sync complete with pending release must fail closed");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.sign_in.host_port_pending"
    );
    assert!(
        store
            .get_instance(&instance_id())
            .expect("store read")
            .expect("instance exists")
            .credential_refs()
            .next()
            .is_none(),
        "store must not be mutated when sync cleanup fails closed"
    );

    // 3. Delegated process pending cleanup fails closed on cancel_sign_in, poll_sign_in timeout, and complete_sign_in.
    let stop_completed = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let wait_completed = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let proc_services = HostServices::new(host_id())
        .with_process(Arc::new(ImmediateStartPendingCleanupProcess {
            stop_completed: Arc::clone(&stop_completed),
            wait_completed: Arc::clone(&wait_completed),
        }))
        .with_time(Arc::new(FixedTime(1)));

    let delegated_cancel =
        start_sign_in(&proc_services, delegated_request()).expect("delegated start");
    let err = cancel_sign_in(delegated_cancel, &proc_services)
        .map_err(|(failure, _)| failure)
        .expect_err("sync cancel with pending process cleanup must fail closed");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.sign_in.host_port_pending"
    );

    let delegated_complete =
        start_sign_in(&proc_services, delegated_request()).expect("delegated start");
    let err = complete_sign_in(delegated_complete, &proc_services, None)
        .map_err(|(failure, _)| failure)
        .expect_err("sync complete error path with pending process cleanup must fail closed");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.sign_in.host_port_pending"
    );

    let mut delegated_timeout = start_sign_in(
        &proc_services,
        delegated_request().with_deadline(Deadline::at(MonotonicInstant::from_ticks(0))),
    )
    .expect("delegated start");
    let err = poll_sign_in(&mut delegated_timeout, &proc_services)
        .expect_err("sync timeout with pending process cleanup must fail closed");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.sign_in.host_port_pending"
    );
    assert_ne!(delegated_timeout.status(), SignInStatus::TimedOut);
    assert!(!wait_completed.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(
        block_on(poll_sign_in_async(&mut delegated_timeout, &proc_services))
            .expect("async follow-up completes process cleanup"),
        SignInStatus::TimedOut
    );
    assert!(stop_completed.load(std::sync::atomic::Ordering::SeqCst));
    assert!(wait_completed.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn async_delegated_poll_awaits_stateful_pending_wait_future() {
    struct StatefulWaitProcess {
        wait_invocations: Arc<std::sync::atomic::AtomicUsize>,
        exit_success: bool,
    }
    struct StatefulWaitHandle {
        wait_invocations: Arc<std::sync::atomic::AtomicUsize>,
        exit_success: bool,
    }
    impl ProcessHandle for StatefulWaitHandle {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            self.wait_invocations
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let exit_success = self.exit_success;
            let mut remaining_pending = 2usize;
            Box::pin(std::future::poll_fn(move |context| {
                if remaining_pending > 0 {
                    remaining_pending -= 1;
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                } else if exit_success {
                    std::task::Poll::Ready(Ok(ProcessExit::new(true, Some(0))))
                } else {
                    std::task::Poll::Ready(Ok(ProcessExit::new(false, Some(1))))
                }
            }))
        }
    }
    impl ProcessService for StatefulWaitProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            let wait_invocations = Arc::clone(&self.wait_invocations);
            let exit_success = self.exit_success;
            yield_once_then(move || {
                Ok(Box::new(StatefulWaitHandle {
                    wait_invocations,
                    exit_success,
                }) as Box<dyn ProcessHandle>)
            })
        }
    }

    let request = || {
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        )
    };

    let ok_invocations = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let ok_services = HostServices::new(host_id())
        .with_process(Arc::new(StatefulWaitProcess {
            wait_invocations: Arc::clone(&ok_invocations),
            exit_success: true,
        }))
        .with_time(Arc::new(FixedTime(1)));

    let mut session =
        block_on(start_sign_in_async(&ok_services, request())).expect("async start succeeds");
    assert_eq!(
        block_on(poll_sign_in_async(&mut session, &ok_services))
            .expect("async delegated poll awaits stateful wait future"),
        SignInStatus::ReadyToComplete
    );
    assert_eq!(
        ok_invocations.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "poll_sign_in_async must await a single wait() future without dropping it"
    );
    let outcome = block_on(complete_sign_in_async(session, &ok_services, None))
        .expect("async complete succeeds");
    assert_eq!(outcome.audience(), &audience());

    let fail_invocations = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let fail_services = HostServices::new(host_id())
        .with_process(Arc::new(StatefulWaitProcess {
            wait_invocations: Arc::clone(&fail_invocations),
            exit_success: false,
        }))
        .with_time(Arc::new(FixedTime(1)));

    let mut failed_session =
        block_on(start_sign_in_async(&fail_services, request())).expect("async start succeeds");
    let err = block_on(poll_sign_in_async(&mut failed_session, &fail_services))
        .expect_err("non-zero helper exit must fail");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.sign_in.login_helper_failed"
    );
    assert_eq!(
        fail_invocations.load(std::sync::atomic::Ordering::SeqCst),
        1
    );
}

#[test]
fn sync_delegated_poll_retains_pending_wait_future_across_polls() {
    struct StatefulWaitHandle {
        wait_invocations: Arc<std::sync::atomic::AtomicUsize>,
    }
    impl ProcessHandle for StatefulWaitHandle {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            self.wait_invocations
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let mut remaining_pending = 2usize;
            Box::pin(std::future::poll_fn(move |_context| {
                if remaining_pending > 0 {
                    remaining_pending -= 1;
                    std::task::Poll::Pending
                } else {
                    std::task::Poll::Ready(Ok(ProcessExit::new(true, Some(0))))
                }
            }))
        }
    }
    struct ReadyStartProcess {
        wait_invocations: Arc<std::sync::atomic::AtomicUsize>,
    }
    impl ProcessService for ReadyStartProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            Box::pin(std::future::ready(Ok(Box::new(StatefulWaitHandle {
                wait_invocations: Arc::clone(&self.wait_invocations),
            })
                as Box<dyn ProcessHandle>)))
        }
    }

    let invocations = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let services = HostServices::new(host_id())
        .with_process(Arc::new(ReadyStartProcess {
            wait_invocations: Arc::clone(&invocations),
        }))
        .with_time(Arc::new(FixedTime(1)));
    let request = SignInStartRequest::new(
        scope(),
        instance_id(),
        family(),
        route_id(),
        SignInAuthorityBinding::new(
            CredentialMechanism::GatewayHelper,
            audience(),
            EntitlementMetering::PayAsYouGo,
        ),
        SignInMethod::DelegatedCliLogin {
            process: ProcessRequest::new(
                ExecutableRef::new("approved.login.helper").expect("executable is valid"),
            ),
            credential: credential(),
            credential_field: field_id(),
        },
    );
    let mut session = start_sign_in(&services, request).expect("sync start succeeds");

    assert_eq!(
        poll_sign_in(&mut session, &services).expect("first pending poll"),
        SignInStatus::InProgress
    );
    assert_eq!(
        poll_sign_in(&mut session, &services).expect("second pending poll"),
        SignInStatus::InProgress
    );
    assert_eq!(
        poll_sign_in(&mut session, &services).expect("completed poll"),
        SignInStatus::ReadyToComplete
    );
    assert_eq!(
        invocations.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "a pending sync poll must keep its wait() future instead of re-invoking wait() each time"
    );
}

#[test]
fn sign_in_releases_loopback_and_stops_reaps_process_on_timeout_and_errors() {
    // 1. start_interactive releases bound loopback lease if url_open.open fails.
    let failing_open = Arc::new(AsyncSignInPorts {
        calls: Mutex::new(Vec::new()),
        open_fails: true,
        materialize_fails: false,
        pending_polls_before_ready: 0,
        credential: credential(),
        audience: audience(),
    });
    let services_open_fail = HostServices::new(host_id())
        .with_url_open(failing_open.clone())
        .with_loopback_callback(failing_open.clone())
        .with_time(Arc::new(FixedTime(1)));
    let error = block_on(start_sign_in_async(
        &services_open_fail,
        interactive_request(),
    ))
    .expect_err("url_open error must fail start");
    assert_eq!(error.kind(), SignInFailureKind::HostPort);
    assert!(
        failing_open.calls().contains(&"loopback_release"),
        "start_interactive must release bound loopback lease on url_open error"
    );

    // 2. poll_sign_in timeout releases loopback and stops/reaps process.
    let ports = MockSignInPorts::new(credential(), audience());
    let interactive_services = interactive_services(ports.clone());
    let mut timed_out_interactive = start_sign_in(
        &interactive_services,
        interactive_request().with_deadline(Deadline::at(MonotonicInstant::from_ticks(0))),
    )
    .expect("start");
    assert_eq!(
        poll_sign_in(&mut timed_out_interactive, &interactive_services).expect("timeout"),
        SignInStatus::TimedOut
    );
    assert!(
        ports.calls().contains(&"loopback_release"),
        "poll_sign_in timeout must release session.loopback"
    );

    let timeout_process = TrackingProcess::new(false);
    let delegated_services = HostServices::new(host_id())
        .with_process(Arc::new(SharedProcessService(Arc::clone(&timeout_process))))
        .with_time(Arc::new(FixedTime(1)));
    let mut timed_out_delegated = start_sign_in(
        &delegated_services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        )
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(0))),
    )
    .expect("delegated start");
    assert_eq!(
        poll_sign_in(&mut timed_out_delegated, &delegated_services).expect("timeout"),
        SignInStatus::TimedOut
    );
    assert_eq!(
        timeout_process.calls(),
        vec![
            "process_start",
            "process_request_stop",
            "process_force_stop",
            "process_wait",
        ]
    );

    // 3. complete_sign_in error paths release loopback and stop/reap process.
    let ports_incomplete = MockSignInPorts::new(credential(), audience());
    let services_incomplete = interactive_services_fn(ports_incomplete.clone());
    let incomplete_session =
        start_sign_in(&services_incomplete, interactive_request()).expect("start");
    let err = complete_sign_in(incomplete_session, &services_incomplete, None)
        .map_err(|(failure, _)| failure)
        .expect_err("incomplete fails");
    assert_eq!(err.kind(), SignInFailureKind::Incomplete);
    assert!(
        ports_incomplete.calls().contains(&"loopback_release"),
        "complete_sign_in must release loopback on Incomplete error"
    );

    let ports_store_err = MockSignInPorts::new(credential(), audience());
    let services_store_err = interactive_services_fn(ports_store_err.clone());
    let mut store_err_session =
        start_sign_in(&services_store_err, interactive_request()).expect("start");
    *ports_store_err.loopback_ready.lock().expect("lock") = true;
    poll_sign_in(&mut store_err_session, &services_store_err).expect("ready");
    let empty_store = MemoryStore::new();
    let err = complete_sign_in(store_err_session, &services_store_err, Some(&empty_store))
        .map_err(|(failure, _)| failure)
        .expect_err("absent instance fails");
    assert_eq!(err.kind(), SignInFailureKind::InstanceAbsent);
    assert!(
        ports_store_err.calls().contains(&"loopback_release"),
        "complete_sign_in must release loopback on store error"
    );

    let incomplete_process = TrackingProcess::new(false);
    let services_proc_err = HostServices::new(host_id())
        .with_process(Arc::new(SharedProcessService(Arc::clone(
            &incomplete_process,
        ))))
        .with_time(Arc::new(FixedTime(1)));
    let proc_session = start_sign_in(
        &services_proc_err,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    )
    .expect("start");
    let err = complete_sign_in(proc_session, &services_proc_err, None)
        .map_err(|(failure, _)| failure)
        .expect_err("incomplete delegated fails");
    assert_eq!(err.kind(), SignInFailureKind::Incomplete);
    assert_eq!(
        incomplete_process.calls(),
        vec![
            "process_start",
            "process_request_stop",
            "process_force_stop",
            "process_wait",
        ]
    );
}

#[test]
fn async_poll_times_out_and_cleans_up_hung_delegated_and_loopback_waits() {
    struct AdvancingTime {
        now: Arc<std::sync::atomic::AtomicU64>,
    }
    impl TimeService for AdvancingTime {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(self.now.load(std::sync::atomic::Ordering::SeqCst))
        }
        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, crate::DeadlineObservation> {
            let now = Arc::clone(&self.now);
            let mut yielded = false;
            Box::pin(std::future::poll_fn(move |context| {
                if !yielded {
                    yielded = true;
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                } else {
                    now.store(
                        deadline.instant().ticks(),
                        std::sync::atomic::Ordering::SeqCst,
                    );
                    std::task::Poll::Ready(crate::DeadlineObservation::new(
                        deadline,
                        deadline.instant(),
                    ))
                }
            }))
        }
    }

    struct HungProcess {
        calls: Mutex<Vec<&'static str>>,
        force_stopped: std::sync::atomic::AtomicBool,
    }
    struct HungProcessHandle {
        shared: Arc<HungProcess>,
    }
    impl ProcessHandle for HungProcessHandle {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.shared.calls.lock().expect("lock").push("request_stop");
            yield_once_then(|| Ok(()))
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.shared.calls.lock().expect("lock").push("force_stop");
            self.shared
                .force_stopped
                .store(true, std::sync::atomic::Ordering::SeqCst);
            yield_once_then(|| Ok(()))
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            let shared = Arc::clone(&self.shared);
            shared.calls.lock().expect("lock").push("wait");
            Box::pin(std::future::poll_fn(move |_context| {
                if shared
                    .force_stopped
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    std::task::Poll::Ready(Ok(ProcessExit::new(false, None)))
                } else {
                    // Never completes on its own until deadline triggers cleanup.
                    std::task::Poll::Pending
                }
            }))
        }
    }
    struct HungProcessService(Arc<HungProcess>);
    impl ProcessService for HungProcessService {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            self.0.calls.lock().expect("lock").push("start");
            let shared = Arc::clone(&self.0);
            Box::pin(
                async move { Ok(Box::new(HungProcessHandle { shared }) as Box<dyn ProcessHandle>) },
            )
        }
    }

    // 1. Hung delegated helper wait observes deadline via wait_until and stops/reaps the child.
    let hung = Arc::new(HungProcess {
        calls: Mutex::new(Vec::new()),
        force_stopped: std::sync::atomic::AtomicBool::new(false),
    });
    let time = Arc::new(AdvancingTime {
        now: Arc::new(std::sync::atomic::AtomicU64::new(1)),
    });
    let services = HostServices::new(host_id())
        .with_process(Arc::new(HungProcessService(Arc::clone(&hung))))
        .with_time(time);

    let mut session = block_on(start_sign_in_async(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        )
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(10))),
    ))
    .expect("async delegated start succeeds");

    assert_eq!(
        block_on(poll_sign_in_async(&mut session, &services))
            .expect("hung delegated helper must time out at deadline"),
        SignInStatus::TimedOut
    );
    assert_eq!(session.status(), SignInStatus::TimedOut);
    assert_eq!(
        *hung.calls.lock().expect("lock"),
        vec!["start", "wait", "request_stop", "force_stop", "wait"]
    );
    let terminal_err = block_on(poll_sign_in_async(&mut session, &services))
        .expect_err("timed-out session must be terminal");
    assert_eq!(terminal_err.kind(), SignInFailureKind::InvalidState);

    // 2. Hung interactive loopback poll also observes deadline and releases the loopback lease.
    struct HungLoopbackPorts {
        released: Arc<std::sync::atomic::AtomicBool>,
    }
    impl UrlOpenService for HungLoopbackPorts {
        fn open(
            &self,
            _scope: ScopeId,
            _url: ApprovedUrlRef,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
    }
    impl LoopbackCallbackService for HungLoopbackPorts {
        fn bind(
            &self,
            scope: ScopeId,
        ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
            Box::pin(async move {
                Ok(LoopbackCallbackLease::new(
                    scope,
                    LoopbackCallbackId::new("fixture.hung.loopback").expect("id is valid"),
                ))
            })
        }
        fn poll(
            &self,
            _lease: &LoopbackCallbackLease,
        ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
        fn materialize_credential(
            &self,
            _receipt: &LoopbackCallbackReceipt,
            _audience: &EndpointAudience,
        ) -> Result<CredentialRef, RuntimeFailure> {
            Ok(credential())
        }
        fn release(&self, _lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
            let released = Arc::clone(&self.released);
            yield_once_then(move || {
                released.store(true, std::sync::atomic::Ordering::SeqCst);
                CleanupOutcome::Clean
            })
        }
    }

    let released = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let hung_loopback = Arc::new(HungLoopbackPorts {
        released: Arc::clone(&released),
    });
    let loopback_time = Arc::new(AdvancingTime {
        now: Arc::new(std::sync::atomic::AtomicU64::new(1)),
    });
    let loopback_services = HostServices::new(host_id())
        .with_url_open(hung_loopback.clone())
        .with_loopback_callback(hung_loopback)
        .with_time(loopback_time);

    let mut interactive = block_on(start_sign_in_async(
        &loopback_services,
        interactive_request().with_deadline(Deadline::at(MonotonicInstant::from_ticks(10))),
    ))
    .expect("async interactive start succeeds");
    assert_eq!(
        block_on(poll_sign_in_async(&mut interactive, &loopback_services))
            .expect("hung loopback poll must time out at deadline"),
        SignInStatus::TimedOut
    );
    assert!(released.load(std::sync::atomic::Ordering::SeqCst));
}

fn interactive_services_fn(ports: Arc<MockSignInPorts>) -> HostServices {
    interactive_services(ports)
}

#[test]
fn poll_delegated_async_reuses_retained_pending_poll() {
    struct CountingWaitProcess {
        wait_calls: Arc<std::sync::atomic::AtomicUsize>,
    }
    struct CountingWaitHandle {
        wait_calls: Arc<std::sync::atomic::AtomicUsize>,
    }
    impl ProcessHandle for CountingWaitHandle {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            self.wait_calls
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            yield_once_then(|| Ok(ProcessExit::new(true, Some(0))))
        }
    }
    impl ProcessService for CountingWaitProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            let wait_calls = Arc::clone(&self.wait_calls);
            Box::pin(async move {
                Ok(Box::new(CountingWaitHandle { wait_calls }) as Box<dyn ProcessHandle>)
            })
        }
    }

    let wait_calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let services = HostServices::new(host_id())
        .with_process(Arc::new(CountingWaitProcess {
            wait_calls: Arc::clone(&wait_calls),
        }))
        .with_time(Arc::new(FixedTime(1)));
    let mut session = start_sign_in(
        &services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    )
    .expect("delegated start succeeds");
    assert_eq!(
        poll_sign_in(&mut session, &services).expect("first sync poll yields InProgress"),
        SignInStatus::InProgress
    );
    assert_eq!(wait_calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(
        block_on(poll_sign_in_async(&mut session, &services))
            .expect("async poll completes retained wait"),
        SignInStatus::ReadyToComplete
    );
    assert_eq!(
        wait_calls.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "poll_delegated_async must reuse the retained wait future instead of calling wait() twice"
    );
}

#[test]
fn cleanup_session_async_bounds_wedged_teardown_and_retains_pending_stop_and_loopback_lease() {
    // 1. Wedged helper teardown is bounded by CLEANUP_TEARDOWN_BUDGET.
    struct WedgedStopProcess;
    impl ProcessHandle for WedgedStopProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }
    impl ProcessService for WedgedStopProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            Box::pin(async { Ok(Box::new(WedgedStopProcess) as Box<dyn ProcessHandle>) })
        }
    }
    struct ImmediateBudgetTime {
        now: Arc<std::sync::atomic::AtomicU64>,
    }
    impl TimeService for ImmediateBudgetTime {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(self.now.load(std::sync::atomic::Ordering::SeqCst))
        }
        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, crate::DeadlineObservation> {
            let now = Arc::clone(&self.now);
            Box::pin(async move {
                now.store(
                    deadline.instant().ticks(),
                    std::sync::atomic::Ordering::SeqCst,
                );
                crate::DeadlineObservation::new(deadline, deadline.instant())
            })
        }
    }

    let wedged_services = HostServices::new(host_id())
        .with_process(Arc::new(WedgedStopProcess))
        .with_time(Arc::new(ImmediateBudgetTime {
            now: Arc::new(std::sync::atomic::AtomicU64::new(1)),
        }));
    let wedged_session = block_on(start_sign_in_async(
        &wedged_services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    ))
    .expect("delegated start succeeds");
    let wedged_err = block_on(cancel_sign_in_async(wedged_session, &wedged_services))
        .expect_err("bounded teardown must fail closed instead of hanging or succeeding");
    assert_eq!(wedged_err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        wedged_err.diagnostic().code(),
        "swallowtail.sign_in.cleanup_timed_out"
    );

    // 1b. Deadline-free session on a host with NO TimeService awaits genuinely async
    // cleanup ports directly instead of failing closed on the first Poll::Pending.
    struct AsyncYieldStopProcess;
    impl ProcessHandle for AsyncYieldStopProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            yield_once_then(|| Ok(()))
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            yield_once_then(|| Ok(()))
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            yield_once_then(|| Ok(ProcessExit::new(true, Some(0))))
        }
    }
    impl ProcessService for AsyncYieldStopProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            Box::pin(async { Ok(Box::new(AsyncYieldStopProcess) as Box<dyn ProcessHandle>) })
        }
    }
    let no_time_async_services =
        HostServices::new(host_id()).with_process(Arc::new(AsyncYieldStopProcess));
    let no_time_async_session = block_on(start_sign_in_async(
        &no_time_async_services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    ))
    .expect("deadline-free delegated start succeeds without TimeService");
    assert_eq!(
        block_on(cancel_sign_in_async(
            no_time_async_session,
            &no_time_async_services,
        ))
        .expect("async cleanup without TimeService awaits genuinely async ports to completion"),
        SignInStatus::Cancelled
    );

    // 2. Missing loopback_callback on cleanup HostServices still releases the lease via the
    // retained binding service across sync cancel, async cancel, and async complete.
    let ports = MockSignInPorts::new(credential(), audience());
    let full_services = interactive_services(Arc::clone(&ports));
    let no_loopback = HostServices::new(host_id()).with_time(Arc::new(FixedTime(1)));
    let session = start_sign_in(&full_services, interactive_request()).expect("start succeeds");
    assert_eq!(
        cancel_sign_in(session, &no_loopback)
            .expect("sync cancel releases lease via retained service"),
        SignInStatus::Cancelled
    );
    assert_eq!(
        ports
            .calls()
            .iter()
            .filter(|&&call| call == "loopback_release")
            .count(),
        1
    );

    let session_async_cancel = block_on(start_sign_in_async(&full_services, interactive_request()))
        .expect("async start succeeds");
    assert_eq!(
        block_on(cancel_sign_in_async(session_async_cancel, &no_loopback))
            .expect("async cancel releases lease via retained service"),
        SignInStatus::Cancelled
    );
    assert_eq!(
        ports
            .calls()
            .iter()
            .filter(|&&call| call == "loopback_release")
            .count(),
        2
    );

    // 2b. Pending loopback release on async complete fails closed without persisting credentials.
    struct HangingReleaseLoopbackPorts;
    impl UrlOpenService for HangingReleaseLoopbackPorts {
        fn open(
            &self,
            _scope: ScopeId,
            _url: ApprovedUrlRef,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
    }
    impl LoopbackCallbackService for HangingReleaseLoopbackPorts {
        fn bind(
            &self,
            scope: ScopeId,
        ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
            Box::pin(async move {
                Ok(LoopbackCallbackLease::new(
                    scope,
                    LoopbackCallbackId::new("fixture.hanging.release").expect("id is valid"),
                ))
            })
        }
        fn poll(
            &self,
            lease: &LoopbackCallbackLease,
        ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
            let callback_id = lease.callback_id().clone();
            Box::pin(async move { Ok(Some(LoopbackCallbackReceipt::new(callback_id))) })
        }
        fn materialize_credential(
            &self,
            _receipt: &LoopbackCallbackReceipt,
            _audience: &EndpointAudience,
        ) -> Result<CredentialRef, RuntimeFailure> {
            Ok(credential())
        }
        fn release(&self, _lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
            Box::pin(std::future::pending())
        }
    }

    let hanging_release_ports = Arc::new(HangingReleaseLoopbackPorts);
    let hanging_release_services = HostServices::new(host_id())
        .with_url_open(hanging_release_ports.clone())
        .with_loopback_callback(hanging_release_ports)
        .with_time(Arc::new(ImmediateBudgetTime {
            now: Arc::new(std::sync::atomic::AtomicU64::new(1)),
        }));
    let store = MemoryStore::new();
    admitted(&store);
    let mut ready_session = block_on(start_sign_in_async(
        &hanging_release_services,
        interactive_request(),
    ))
    .expect("start succeeds");
    assert_eq!(
        block_on(poll_sign_in_async(
            &mut ready_session,
            &hanging_release_services
        ))
        .expect("poll becomes ready"),
        SignInStatus::ReadyToComplete
    );
    let complete_err = block_on(complete_sign_in_async(
        ready_session,
        &hanging_release_services,
        Some(&store),
    ))
    .expect_err("expired loopback release must fail closed before persisting credentials");
    assert_eq!(complete_err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        complete_err.diagnostic().code(),
        "swallowtail.sign_in.cleanup_timed_out"
    );
    let stored = store
        .get_instance(&instance_id())
        .expect("store read")
        .expect("instance exists");
    assert_eq!(
        stored.credential_refs().len(),
        0,
        "credentials must not be persisted when loopback release did not finish"
    );

    // 2c. Cleanup when a DIFFERENT LoopbackCallbackService is registered still releases
    // through the original binder, leaving the foreign port untouched.
    let original_ports = MockSignInPorts::new(credential(), audience());
    let foreign_ports = MockSignInPorts::new(credential(), audience());
    let original_services = interactive_services(Arc::clone(&original_ports));
    let foreign_services = interactive_services(Arc::clone(&foreign_ports));
    let session = block_on(start_sign_in_async(
        &original_services,
        interactive_request(),
    ))
    .expect("start succeeds");
    assert_eq!(
        block_on(cancel_sign_in_async(session, &foreign_services))
            .expect("cancel releases via original binder"),
        SignInStatus::Cancelled
    );
    assert!(
        original_ports.calls().contains(&"loopback_release"),
        "original binder must receive the release"
    );
    assert!(
        !foreign_ports.calls().contains(&"loopback_release"),
        "foreign loopback port must not receive a lease it never bound"
    );

    // 2d. A completed CleanupOutcome::Failed from loopback.release surfaces its exact
    // diagnostic and blocks credential persistence in complete_sign_in_async.
    struct FailingReleaseLoopbackPorts;
    impl UrlOpenService for FailingReleaseLoopbackPorts {
        fn open(
            &self,
            _scope: ScopeId,
            _url: ApprovedUrlRef,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
    }
    impl LoopbackCallbackService for FailingReleaseLoopbackPorts {
        fn bind(
            &self,
            scope: ScopeId,
        ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
            Box::pin(async move {
                Ok(LoopbackCallbackLease::new(
                    scope,
                    LoopbackCallbackId::new("fixture.failing.release").expect("id is valid"),
                ))
            })
        }
        fn poll(
            &self,
            lease: &LoopbackCallbackLease,
        ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
            let callback_id = lease.callback_id().clone();
            Box::pin(async move { Ok(Some(LoopbackCallbackReceipt::new(callback_id))) })
        }
        fn materialize_credential(
            &self,
            _receipt: &LoopbackCallbackReceipt,
            _audience: &EndpointAudience,
        ) -> Result<CredentialRef, RuntimeFailure> {
            Ok(credential())
        }
        fn release(&self, _lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
            Box::pin(async {
                CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "fixture.loopback_release_failed",
                    "Loopback listener failed to close",
                ))
            })
        }
    }

    let failing_release_ports = Arc::new(FailingReleaseLoopbackPorts);
    let failing_release_services = HostServices::new(host_id())
        .with_url_open(failing_release_ports.clone())
        .with_loopback_callback(failing_release_ports)
        .with_time(Arc::new(FixedTime(1)));
    let failing_store = MemoryStore::new();
    admitted(&failing_store);
    let mut failing_session = block_on(start_sign_in_async(
        &failing_release_services,
        interactive_request(),
    ))
    .expect("start succeeds");
    assert_eq!(
        block_on(poll_sign_in_async(
            &mut failing_session,
            &failing_release_services
        ))
        .expect("poll becomes ready"),
        SignInStatus::ReadyToComplete
    );
    let release_err = block_on(complete_sign_in_async(
        failing_session,
        &failing_release_services,
        Some(&failing_store),
    ))
    .expect_err("failed loopback release must fail closed before persisting credentials");
    assert_eq!(release_err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        release_err.diagnostic().code(),
        "fixture.loopback_release_failed"
    );
    let stored_after_fail = failing_store
        .get_instance(&instance_id())
        .expect("store read")
        .expect("instance exists");
    assert_eq!(
        stored_after_fail.credential_refs().len(),
        0,
        "credentials must not be persisted when loopback release returned CleanupOutcome::Failed"
    );

    // 2e. A synchronous complete_sign_in failure on CleanupOutcome::Failed retains
    // the failure in the returned session so neither a synchronous nor an async
    // retry forgets the failure or persists credentials.
    let retry_store = MemoryStore::new();
    admitted(&retry_store);
    let mut sync_failing_session = start_sign_in(&failing_release_services, interactive_request())
        .expect("sync start succeeds");
    assert_eq!(
        poll_sign_in(&mut sync_failing_session, &failing_release_services)
            .expect("sync poll becomes ready"),
        SignInStatus::ReadyToComplete
    );
    let (first_err, returned_session) = complete_sign_in(
        sync_failing_session,
        &failing_release_services,
        Some(&retry_store),
    )
    .expect_err("initial sync complete must fail closed on CleanupOutcome::Failed");
    assert_eq!(first_err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        first_err.diagnostic().code(),
        "fixture.loopback_release_failed"
    );
    assert_eq!(
        retry_store
            .get_instance(&instance_id())
            .expect("store read")
            .expect("instance exists")
            .credential_refs()
            .len(),
        0,
        "store must stay empty after initial sync complete failure"
    );

    let (sync_retry_err, returned_session) = complete_sign_in(
        returned_session,
        &failing_release_services,
        Some(&retry_store),
    )
    .expect_err("sync retry must still surface the retained loopback release failure");
    assert_eq!(sync_retry_err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        sync_retry_err.diagnostic().code(),
        "fixture.loopback_release_failed"
    );
    assert_eq!(
        retry_store
            .get_instance(&instance_id())
            .expect("store read")
            .expect("instance exists")
            .credential_refs()
            .len(),
        0,
        "store must stay empty after sync retry"
    );

    let async_retry_err = block_on(complete_sign_in_async(
        returned_session,
        &failing_release_services,
        Some(&retry_store),
    ))
    .expect_err("async retry must still surface the retained loopback release failure");
    assert_eq!(async_retry_err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        async_retry_err.diagnostic().code(),
        "fixture.loopback_release_failed"
    );
    assert_eq!(
        retry_store
            .get_instance(&instance_id())
            .expect("store read")
            .expect("instance exists")
            .credential_refs()
            .len(),
        0,
        "store must stay empty after async retry"
    );

    // 3. cleanup_session_now retains pending stop futures instead of dropping and restarting them.
    struct CountingStopProcess {
        stop_calls: Arc<std::sync::atomic::AtomicUsize>,
    }
    struct CountingStopHandle {
        stop_calls: Arc<std::sync::atomic::AtomicUsize>,
    }
    impl ProcessHandle for CountingStopHandle {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.stop_calls
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            yield_once_then(|| Ok(()))
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }
    impl ProcessService for CountingStopProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            let stop_calls = Arc::clone(&self.stop_calls);
            Box::pin(async move {
                Ok(Box::new(CountingStopHandle { stop_calls }) as Box<dyn ProcessHandle>)
            })
        }
    }

    let stop_calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let stop_services = HostServices::new(host_id())
        .with_process(Arc::new(CountingStopProcess {
            stop_calls: Arc::clone(&stop_calls),
        }))
        .with_time(Arc::new(FixedTime(1)));
    let session = start_sign_in(
        &stop_services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    )
    .expect("delegated start succeeds");
    let (err, session) = cancel_sign_in(session, &stop_services)
        .expect_err("pending stop fails closed and retains the future");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(stop_calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(
        block_on(cancel_sign_in_async(session, &stop_services))
            .expect("async cancel drives retained stop future"),
        SignInStatus::Cancelled
    );
    assert_eq!(
        stop_calls.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "retained stop future must not be dropped and restarted"
    );
}

#[test]
fn await_cleanup_budget_never_repolls_completed_timer_on_early_ready() {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::task::{Context, Poll};

    struct PanicOnRepollTimer {
        deadline: Deadline,
        completed: bool,
        advance_clock: Option<Arc<AtomicU64>>,
        polls_before_ready: usize,
    }

    impl Future for PanicOnRepollTimer {
        type Output = crate::DeadlineObservation;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            assert!(
                !self.completed,
                "completed budget timer future must never be polled a second time"
            );
            if self.polls_before_ready > 0 {
                self.polls_before_ready -= 1;
                context.waker().wake_by_ref();
                return Poll::Pending;
            }
            self.completed = true;
            if let Some(clock) = &self.advance_clock {
                clock.store(self.deadline.instant().ticks(), Ordering::SeqCst);
            }
            Poll::Ready(crate::DeadlineObservation::new(
                self.deadline,
                self.deadline.instant(),
            ))
        }
    }

    struct YieldNTimes {
        remaining: usize,
    }

    impl Future for YieldNTimes {
        type Output = Result<(), RuntimeFailure>;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            if self.remaining == 0 {
                Poll::Ready(Ok(()))
            } else {
                self.remaining -= 1;
                context.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    // Case A: Fixed clock where wait_until returns Ready without advancing time, while
    // cleanup yields multiple times before completing. Must never repoll a completed timer.
    struct FixedPanicTimerTime;
    impl TimeService for FixedPanicTimerTime {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(1)
        }
        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, crate::DeadlineObservation> {
            Box::pin(PanicOnRepollTimer {
                deadline,
                completed: false,
                advance_clock: None,
                polls_before_ready: 0,
            })
        }
    }

    struct MultiYieldStopProcess;
    impl ProcessHandle for MultiYieldStopProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(YieldNTimes { remaining: 3 })
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }
    impl ProcessService for MultiYieldStopProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            Box::pin(async { Ok(Box::new(MultiYieldStopProcess) as Box<dyn ProcessHandle>) })
        }
    }

    let fixed_services = HostServices::new(host_id())
        .with_process(Arc::new(MultiYieldStopProcess))
        .with_time(Arc::new(FixedPanicTimerTime));
    let session = block_on(start_sign_in_async(
        &fixed_services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    ))
    .expect("delegated start succeeds");
    assert_eq!(
        block_on(cancel_sign_in_async(session, &fixed_services))
            .expect("multi-yield cleanup completes without repolling completed timer"),
        SignInStatus::Cancelled
    );

    // Case B: Spurious early wake on first timer followed by a re-armed timer that
    // actually advances the clock to the deadline. Must re-arm and time out cleanly
    // without ever repolling the first completed timer.
    struct SpuriousEarlyWakeTime {
        now: Arc<AtomicU64>,
        arms: AtomicUsize,
    }
    impl TimeService for SpuriousEarlyWakeTime {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(self.now.load(Ordering::SeqCst))
        }
        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, crate::DeadlineObservation> {
            let arm = self.arms.fetch_add(1, Ordering::SeqCst);
            if arm == 0 {
                // First timer resolves early without advancing `now`.
                Box::pin(PanicOnRepollTimer {
                    deadline,
                    completed: false,
                    advance_clock: None,
                    polls_before_ready: 0,
                })
            } else {
                // Re-armed timer yields once then advances `now` to the deadline.
                Box::pin(PanicOnRepollTimer {
                    deadline,
                    completed: false,
                    advance_clock: Some(Arc::clone(&self.now)),
                    polls_before_ready: 1,
                })
            }
        }
    }

    struct HangingStopProcess;
    impl ProcessHandle for HangingStopProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }
    impl ProcessService for HangingStopProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            Box::pin(async { Ok(Box::new(HangingStopProcess) as Box<dyn ProcessHandle>) })
        }
    }

    let spurious_time = Arc::new(SpuriousEarlyWakeTime {
        now: Arc::new(AtomicU64::new(1)),
        arms: AtomicUsize::new(0),
    });
    let spurious_services = HostServices::new(host_id())
        .with_process(Arc::new(HangingStopProcess))
        .with_time(spurious_time.clone());
    let session = block_on(start_sign_in_async(
        &spurious_services,
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        ),
    ))
    .expect("delegated start succeeds");
    let err = block_on(cancel_sign_in_async(session, &spurious_services))
        .expect_err("re-armed timer must still time out wedged cleanup");
    assert_eq!(err.kind(), SignInFailureKind::HostPort);
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.sign_in.cleanup_timed_out"
    );
    assert!(
        spurious_time.arms.load(Ordering::SeqCst) >= 2,
        "spurious early wake must re-arm the budget timer"
    );
}

#[test]
fn process_cleanup_surfaces_runtime_failures_in_sync_and_async_paths() {
    use std::sync::atomic::{AtomicBool, Ordering};

    #[derive(Clone, Copy)]
    enum FailStage {
        RequestStop,
        ForceStop,
        Wait,
    }

    struct FailingCleanupProcess {
        stage: FailStage,
        force_called: Arc<AtomicBool>,
        wait_called: Arc<AtomicBool>,
    }

    impl ProcessHandle for FailingCleanupProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            if matches!(self.stage, FailStage::RequestStop) {
                Box::pin(async {
                    Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                        "fixture.request_stop_failed",
                        "request_stop failed",
                    )))
                })
            } else {
                Box::pin(async { Ok(()) })
            }
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.force_called.store(true, Ordering::SeqCst);
            if matches!(self.stage, FailStage::ForceStop) {
                Box::pin(async {
                    Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                        "fixture.force_stop_failed",
                        "force_stop failed",
                    )))
                })
            } else {
                Box::pin(async { Ok(()) })
            }
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            self.wait_called.store(true, Ordering::SeqCst);
            if matches!(self.stage, FailStage::Wait) {
                Box::pin(async {
                    Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                        "fixture.wait_failed",
                        "wait failed",
                    )))
                })
            } else {
                Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
            }
        }
    }

    impl ProcessService for FailingCleanupProcess {
        fn start(
            &self,
            _scope: ScopeId,
            _request: ProcessRequest,
        ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
            let handle = FailingCleanupProcess {
                stage: self.stage,
                force_called: Arc::clone(&self.force_called),
                wait_called: Arc::clone(&self.wait_called),
            };
            Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
        }
    }

    let delegated_request = || {
        SignInStartRequest::new(
            scope(),
            instance_id(),
            family(),
            route_id(),
            SignInAuthorityBinding::new(
                CredentialMechanism::GatewayHelper,
                audience(),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::DelegatedCliLogin {
                process: ProcessRequest::new(
                    ExecutableRef::new("approved.login.helper").expect("executable is valid"),
                ),
                credential: credential(),
                credential_field: field_id(),
            },
        )
    };

    for (stage, expected_code) in [
        (FailStage::RequestStop, "fixture.request_stop_failed"),
        (FailStage::ForceStop, "fixture.force_stop_failed"),
        (FailStage::Wait, "fixture.wait_failed"),
    ] {
        // Sync path: cancel_sign_in surfaces the failure, still runs remaining teardown
        // stages, and retains the failure across a retry.
        let force_called = Arc::new(AtomicBool::new(false));
        let wait_called = Arc::new(AtomicBool::new(false));
        let services = HostServices::new(host_id())
            .with_process(Arc::new(FailingCleanupProcess {
                stage,
                force_called: Arc::clone(&force_called),
                wait_called: Arc::clone(&wait_called),
            }))
            .with_time(Arc::new(FixedTime(1)));
        let session = start_sign_in(&services, delegated_request()).expect("sync start succeeds");
        let (err, returned_session) =
            cancel_sign_in(session, &services).expect_err("sync cancel must surface process error");
        assert_eq!(err.kind(), SignInFailureKind::HostPort);
        assert_eq!(err.diagnostic().code(), expected_code);
        assert!(force_called.load(Ordering::SeqCst), "force_stop still ran");
        assert!(wait_called.load(Ordering::SeqCst), "wait still ran");

        let (retry_err, _) = cancel_sign_in(returned_session, &services)
            .expect_err("sync retry must retain process cleanup failure");
        assert_eq!(retry_err.diagnostic().code(), expected_code);

        // Async path: cancel_sign_in_async surfaces the failure and still runs remaining stages.
        let force_called_async = Arc::new(AtomicBool::new(false));
        let wait_called_async = Arc::new(AtomicBool::new(false));
        let async_services = HostServices::new(host_id())
            .with_process(Arc::new(FailingCleanupProcess {
                stage,
                force_called: Arc::clone(&force_called_async),
                wait_called: Arc::clone(&wait_called_async),
            }))
            .with_time(Arc::new(FixedTime(1)));
        let async_session = block_on(start_sign_in_async(&async_services, delegated_request()))
            .expect("async start succeeds");
        let async_err = block_on(cancel_sign_in_async(async_session, &async_services))
            .expect_err("async cancel must surface process error");
        assert_eq!(async_err.kind(), SignInFailureKind::HostPort);
        assert_eq!(async_err.diagnostic().code(), expected_code);
        assert!(
            force_called_async.load(Ordering::SeqCst),
            "async force_stop still ran"
        );
        assert!(
            wait_called_async.load(Ordering::SeqCst),
            "async wait still ran"
        );
    }
}
