use std::sync::Arc;
use swallowtail_core::{EndpointAudience, ExecutionHostId, HostServiceKind};
use swallowtail_host_local::{
    LocalProcessHost, LocalProcessLimits, LocalSignInCall, LocalSignInPorts,
};
use swallowtail_runtime::{
    ApprovedUrlRef, CredentialRef, DeviceCodePrompt, ExecutableRef, HostServices,
    LoopbackCallbackService, ProcessRequest, ScopeId, UrlOpenService,
};

fn poll_now<T>(future: impl std::future::Future<Output = T>) -> T {
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};

    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());
    match Pin::as_mut(&mut future).poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("fixture future must be immediately ready"),
    }
}

#[test]
fn sign_in_ports_are_optional_and_registration_does_not_open_or_display() {
    let ports = LocalSignInPorts::new();
    let services = HostServices::new(host_id())
        .with_url_open(Arc::new(ports.clone()))
        .with_loopback_callback(Arc::new(ports.clone()))
        .with_device_code_display(Arc::new(ports.clone()));

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
    assert!(ports.calls().is_empty());
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
fn local_composition_does_not_imply_sign_in_ports() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default()).build_services(host_id());
    let kinds = local.services().available_kinds();
    assert!(!kinds.contains(&HostServiceKind::UrlOpen));
    assert!(!kinds.contains(&HostServiceKind::LoopbackCallback));
    assert!(!kinds.contains(&HostServiceKind::DeviceCodeDisplay));
}

#[test]
fn ports_never_return_secret_bytes_and_login_helpers_stay_process() {
    let ports = LocalSignInPorts::new();
    let url = ApprovedUrlRef::new("https://login.example.test/authorize?secret=token-bytes")
        .expect("url is valid");
    let prompt = DeviceCodePrompt::new("WDJB-MJHT").expect("code is valid");
    poll_now(UrlOpenService::open(
        &ports,
        ScopeId::new("sign-in").expect("scope is valid"),
        url.clone(),
    ))
    .expect("open records without returning a token");
    assert!(!format!("{url:?}").contains("token-bytes"));
    assert!(!format!("{prompt:?}").contains("WDJB-MJHT"));
    assert_eq!(ports.count(LocalSignInCall::UrlOpen), 1);

    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(
            ExecutableRef::new("approved.login.helper").expect("executable is valid"),
            "/private/login-helper",
        )
        .build();
    let process_services = HostServices::new(host_id()).with_process(Arc::new(host));
    assert!(
        process_services
            .available_kinds()
            .contains(&HostServiceKind::Process)
    );
    assert!(
        !process_services
            .available_kinds()
            .contains(&HostServiceKind::UrlOpen)
    );
    let _ = process_services.process();
    let _request = ProcessRequest::new(
        ExecutableRef::new("approved.login.helper").expect("executable is valid"),
    );
}

#[test]
fn delivered_callback_materializes_a_reference_for_the_bound_audience() {
    let ports = LocalSignInPorts::new();
    let audience = EndpointAudience::new("api.example.test").expect("audience is valid");
    let credential = CredentialRef::new("stored-ref").expect("credential is valid");
    ports.deliver_loopback(audience.clone(), credential.clone());
    let lease = poll_now(ports.bind(ScopeId::new("sign-in").expect("scope is valid")))
        .expect("bind succeeds");
    let receipt = poll_now(ports.poll(&lease))
        .expect("poll succeeds")
        .expect("callback arrived");
    let materialized = ports
        .materialize_credential(&receipt, &audience)
        .expect("materialize stays a reference");
    assert_eq!(materialized, credential);
    assert!(!format!("{receipt:?}").contains("sk-secret"));
    assert!(!format!("{materialized:?}").contains("sk-secret"));
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("host.local").expect("host id is valid")
}
