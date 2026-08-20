use super::{
    ConnectionLifecycleStore, ConnectionLifecycleStoreFailure, SignInAuthorityBinding,
    SignInFailureKind, SignInMethod, SignInStartRequest, SignInStatus, cancel_sign_in,
    complete_sign_in, poll_sign_in, start_sign_in, submit_sign_in_credential_field,
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
