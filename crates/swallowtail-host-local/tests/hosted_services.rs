use std::future::Future;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_core::EndpointAudience;
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, CredentialRef, CredentialService, DelegatedCredential,
    EndpointRef, NetworkPolicyService, RuntimeFailure, ScopeId,
};

#[test]
fn approved_endpoint_and_secret_are_scope_bound_redacted_and_released() {
    let endpoint = EndpointRef::new("anthropic-api").expect("endpoint is valid");
    let credential = CredentialRef::new("anthropic-key").expect("credential is valid");
    let audience = EndpointAudience::new("api.anthropic.com").expect("audience is valid");
    let scope = ScopeId::new("request-1").expect("scope is valid");
    let raw_endpoint = "https://api.anthropic.com/v1";
    let raw_secret = b"private-api-key";
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_endpoint(endpoint.clone(), audience.clone(), raw_endpoint)
        .approve_secret_credential(credential.clone(), audience.clone(), raw_secret.to_vec())
        .build();

    let grant = block_on(host.authorize(scope.clone(), endpoint.clone(), audience.clone()))
        .expect("approved endpoint resolves");
    assert_eq!(grant.scope(), &scope);
    assert_eq!(grant.endpoint(), &endpoint);
    assert_eq!(grant.audience(), &audience);
    assert_eq!(grant.authorized().as_driver_value(), raw_endpoint);
    assert!(!format!("{grant:?}").contains(raw_endpoint));

    let lease = block_on(host.acquire(scope.clone(), credential.clone(), audience.clone()))
        .expect("approved credential resolves");
    assert_eq!(lease.scope(), &scope);
    assert_eq!(lease.reference(), &credential);
    assert_eq!(lease.audience(), &audience);
    let CredentialLease::Secret(secret) = &lease else {
        panic!("approved secret remains a secret lease");
    };
    assert_eq!(secret.expose_secret(), raw_secret);
    assert!(!format!("{lease:?}").contains("private-api-key"));
    assert_eq!(block_on(host.release(lease)), CleanupOutcome::Clean);
}

#[test]
fn audience_mismatch_fails_before_endpoint_or_credential_exposure() {
    let endpoint = EndpointRef::new("service-endpoint").expect("endpoint is valid");
    let credential = CredentialRef::new("delegated-login").expect("credential is valid");
    let approved = EndpointAudience::new("approved-audience").expect("audience is valid");
    let wrong = EndpointAudience::new("wrong-audience").expect("audience is valid");
    let scope = ScopeId::new("request-2").expect("scope is valid");
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_endpoint(endpoint.clone(), approved.clone(), "http://127.0.0.1:4096")
        .approve_delegated_credential(credential.clone(), approved)
        .build();

    assert_failure_code(
        block_on(host.authorize(scope.clone(), endpoint, wrong.clone())),
        "swallowtail.local_network.audience_mismatch",
    );
    assert_failure_code(
        block_on(host.acquire(scope, credential, wrong)),
        "swallowtail.local_credential.audience_mismatch",
    );
}

#[test]
fn delegated_auth_exposes_no_secret_and_foreign_release_fails() {
    let credential = CredentialRef::new("opencode-login").expect("credential is valid");
    let audience = EndpointAudience::new("opencode-server").expect("audience is valid");
    let scope = ScopeId::new("request-3").expect("scope is valid");
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_delegated_credential(credential.clone(), audience.clone())
        .build();

    let lease = block_on(host.acquire(scope.clone(), credential, audience.clone()))
        .expect("delegated credential resolves");
    assert!(matches!(lease, CredentialLease::Delegated(_)));
    assert_eq!(block_on(host.release(lease)), CleanupOutcome::Clean);

    let foreign = CredentialLease::Delegated(DelegatedCredential::new(
        ScopeId::new("foreign-scope").expect("scope is valid"),
        CredentialRef::new("opencode-login").expect("credential is valid"),
        audience,
    ));
    let CleanupOutcome::Failed(diagnostic) = block_on(host.release(foreign)) else {
        panic!("foreign lease must fail cleanup");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.local_credential.lease_not_owned"
    );
}

fn assert_failure_code<T>(result: Result<T, RuntimeFailure>, expected: &str) {
    let failure = result.err().expect("operation must fail");
    assert_eq!(failure.diagnostic().code(), expected);
}

struct ThreadWake(thread::Thread);

impl Wake for ThreadWake {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWake(thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        assert!(Instant::now() < deadline, "fixture future timed out");
        thread::park_timeout(Duration::from_millis(10));
    }
}
