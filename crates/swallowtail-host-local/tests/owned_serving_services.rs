use sha2::{Digest, Sha256};
use std::fs;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_core::{
    EndpointAudience, ExecutionHostId, ModelArtifactBinding, ModelArtifactDescriptor,
    ModelArtifactDigest, ModelArtifactFormat, ModelArtifactId, ModelArtifactRef,
    ModelArtifactRevision,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    CleanupOutcome, MaterializedModelArtifactRef, ModelArtifactAccess, ModelArtifactLease,
    ModelArtifactService, NetworkPolicyService, ObservedServingEndpoint, RuntimeFailure, ScopeId,
    ServingEndpointBinding, ServingEndpointLease, ServingEndpointService,
};

#[path = "owned_serving_services/serving_endpoint.rs"]
mod serving_endpoint;

#[test]
fn exact_artifact_is_read_only_redacted_and_not_deleted_on_release() {
    let fixture = TempFixture::new();
    let artifact_path = fixture.path().join("fixture.gguf");
    let bytes = b"deterministic gguf fixture";
    fs::write(&artifact_path, bytes).expect("fixture artifact can be written");
    let binding = artifact_binding("artifact-ref", "revision-1", bytes);
    let host_id = host_id();
    let scope = scope();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .bind_execution_host(host_id.clone())
        .approve_model_artifact(binding.clone(), artifact_path.clone())
        .build();

    let lease = block_on(host.acquire(scope.clone(), host_id.clone(), binding.clone()))
        .expect("exact approved artifact resolves");
    assert_eq!(lease.scope(), &scope);
    assert_eq!(lease.execution_host_id(), &host_id);
    assert_eq!(lease.binding(), &binding);
    assert_eq!(lease.access(), ModelArtifactAccess::ReadOnly);
    assert_eq!(
        lease.materialized().as_driver_value(),
        artifact_path.to_str().unwrap()
    );
    assert!(!format!("{lease:?}").contains(artifact_path.to_str().unwrap()));

    assert_eq!(
        block_on(ModelArtifactService::release(&host, lease)),
        CleanupOutcome::NotApplicable
    );
    assert_eq!(fs::read(&artifact_path).expect("artifact remains"), bytes);
}

#[test]
fn artifact_descriptor_digest_and_file_type_are_checked_before_lease_issue() {
    let fixture = TempFixture::new();
    let artifact_path = fixture.path().join("fixture.gguf");
    let bytes = b"approved bytes";
    fs::write(&artifact_path, bytes).expect("fixture artifact can be written");
    let approved = artifact_binding("artifact-ref", "revision-1", bytes);
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .bind_execution_host(host_id())
        .approve_model_artifact(approved.clone(), artifact_path)
        .approve_model_artifact(
            artifact_binding("directory-ref", "revision-1", bytes),
            fixture.path(),
        )
        .build();

    assert_failure_code(
        block_on(host.acquire(
            scope(),
            host_id(),
            artifact_binding("artifact-ref", "revision-2", bytes),
        )),
        "swallowtail.local_model_artifact.descriptor_mismatch",
    );
    assert_failure_code(
        block_on(host.acquire(
            scope(),
            host_id(),
            artifact_binding("artifact-ref", "revision-1", b"other bytes"),
        )),
        "swallowtail.local_model_artifact.descriptor_mismatch",
    );
    assert_failure_code(
        block_on(host.acquire(
            scope(),
            host_id(),
            artifact_binding("directory-ref", "revision-1", bytes),
        )),
        "swallowtail.local_model_artifact.not_regular_file",
    );

    fs::write(fixture.path().join("fixture.gguf"), b"mutated")
        .expect("fixture artifact can be mutated");
    assert_failure_code(
        block_on(host.acquire(scope(), host_id(), approved)),
        "swallowtail.local_model_artifact.digest_mismatch",
    );
}

#[test]
fn artifact_service_rejects_unbound_mismatched_and_foreign_leases() {
    let fixture = TempFixture::new();
    let artifact_path = fixture.path().join("fixture.gguf");
    let bytes = b"artifact bytes";
    fs::write(&artifact_path, bytes).expect("fixture artifact can be written");
    let binding = artifact_binding("artifact-ref", "revision-1", bytes);
    let unbound = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_model_artifact(binding.clone(), artifact_path.clone())
        .build();
    assert_failure_code(
        block_on(unbound.acquire(scope(), host_id(), binding.clone())),
        "swallowtail.local_host.execution_host_unbound",
    );

    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .bind_execution_host(host_id())
        .approve_model_artifact(binding.clone(), artifact_path.clone())
        .build();
    let other_host = ExecutionHostId::new("host.other").expect("host id is valid");
    assert_failure_code(
        block_on(host.acquire(scope(), other_host.clone(), binding.clone())),
        "swallowtail.local_host.execution_host_mismatch",
    );
    let foreign = ModelArtifactLease::read_only(
        scope(),
        other_host,
        binding,
        MaterializedModelArtifactRef::new(artifact_path.to_string_lossy())
            .expect("materialized reference is valid"),
    );
    assert_failed_cleanup(
        block_on(ModelArtifactService::release(&host, foreign)),
        "swallowtail.local_model_artifact.lease_not_owned",
    );
}

fn artifact_binding(reference: &str, revision: &str, bytes: &[u8]) -> ModelArtifactBinding {
    ModelArtifactBinding::new(
        ModelArtifactRef::new(reference).expect("artifact reference is valid"),
        ModelArtifactDescriptor::new(
            ModelArtifactId::new("artifact-1").expect("artifact id is valid"),
            ModelArtifactFormat::new("gguf").expect("format is valid"),
            ModelArtifactRevision::new(revision).expect("revision is valid"),
            ModelArtifactDigest::new(format!("sha256:{:x}", Sha256::digest(bytes)))
                .expect("digest is valid"),
        ),
    )
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("host.local").expect("host id is valid")
}

fn scope() -> ScopeId {
    ScopeId::new("owned-serving-scope").expect("scope is valid")
}

fn audience() -> EndpointAudience {
    EndpointAudience::new("llama.cpp.local").expect("audience is valid")
}

fn assert_failure_code<T>(result: Result<T, RuntimeFailure>, expected: &str) {
    let failure = result.err().expect("operation must fail");
    assert_eq!(failure.diagnostic().code(), expected);
}

fn assert_failed_cleanup(outcome: CleanupOutcome, expected: &str) {
    let CleanupOutcome::Failed(diagnostic) = outcome else {
        panic!("cleanup must fail");
    };
    assert_eq!(diagnostic.code(), expected);
}

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TempFixture(PathBuf);

impl TempFixture {
    fn new() -> Self {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "swallowtail-owned-serving-test-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("fixture directory can be created");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempFixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("fixture directory can be removed");
    }
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
