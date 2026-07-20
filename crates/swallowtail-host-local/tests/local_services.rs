use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use swallowtail_host_local::{LocalMaterializationLimits, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, AttachmentService, CleanupOutcome,
    Deadline, ResourceAccess, ResourceLease, ResourceRepresentation, RuntimeFailure,
    SchemaDocument, SchemaRef, SchemaService, ScopeId, TimeService, WorkingResourceRef,
    WorkingResourceService,
};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn approved_materializations_are_bounded_redacted_and_explicitly_released() {
    let root = temporary_root();
    let sources = root.join("sources");
    let materialized = root.join("materialized");
    std::fs::create_dir_all(&sources).expect("source directory is created");
    let attachment_path = sources.join("private-attachment.txt");
    let schema_path = sources.join("private-schema.json");
    std::fs::write(&attachment_path, b"attachment-secret").expect("attachment source is written");
    std::fs::write(&schema_path, br#"{"type":"object"}"#).expect("schema source is written");

    let attachment = attachment_ref();
    let schema = schema_ref();
    let scope = scope("materialize");
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_temporary_root(&materialized)
        .with_materialization_limits(LocalMaterializationLimits::new(64, 64))
        .approve_attachment(attachment.clone(), &attachment_path)
        .approve_schema(schema.clone(), &schema_path)
        .build();

    let descriptor = AttachmentDescriptor::new(attachment, "text/plain", AttachmentRole::Input)
        .expect("attachment descriptor is valid")
        .with_known_length(17);
    let attachment_lease = block_on(AttachmentService::materialize_file(
        &host,
        scope.clone(),
        descriptor,
    ))
    .expect("approved attachment materializes");
    let attachment_file = PathBuf::from(attachment_lease.file().as_driver_value());
    assert_eq!(
        std::fs::read(&attachment_file).expect("attachment copy is readable"),
        b"attachment-secret"
    );
    assert!(!format!("{attachment_lease:?}").contains("private-attachment"));
    assert_eq!(
        block_on(AttachmentService::release_file(&host, attachment_lease)),
        CleanupOutcome::Clean
    );
    assert!(!attachment_file.exists());
    assert!(attachment_path.exists(), "consumer source must remain");

    let schema_lease = block_on(SchemaService::materialize_file(
        &host,
        scope,
        SchemaDocument::reference(schema),
    ))
    .expect("approved schema materializes");
    let schema_file = PathBuf::from(schema_lease.file().as_driver_value());
    assert_eq!(
        std::fs::read(&schema_file).expect("schema copy is readable"),
        br#"{"type":"object"}"#
    );
    assert!(!format!("{schema_lease:?}").contains("private-schema"));
    assert_eq!(
        block_on(SchemaService::release_file(&host, schema_lease)),
        CleanupOutcome::Clean
    );
    assert!(!schema_file.exists());
    assert!(schema_path.exists(), "consumer source must remain");

    drop(host);
    std::fs::remove_dir_all(root).expect("fixture root is removed");
}

#[test]
fn temporary_resources_are_scope_bound_and_cleanup_authority_is_enforced() {
    let root = temporary_root();
    let borrowed = root.join("borrowed");
    let materialized = root.join("materialized");
    std::fs::create_dir_all(&borrowed).expect("borrowed resource is created");
    let borrowed_ref = working_resource_ref("borrowed");
    let owner = scope("owner");
    let other = scope("other");
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_temporary_root(&materialized)
        .approve_working_resource(borrowed_ref.clone(), &borrowed)
        .build();

    let borrowed_lease = block_on(host.resolve(
        owner.clone(),
        borrowed_ref,
        ResourceAccess::Read,
        ResourceRepresentation::TemporaryFile,
    ))
    .expect("approved borrowed resource resolves");
    assert_eq!(
        block_on(host.release(borrowed_lease)),
        CleanupOutcome::NotApplicable
    );
    assert!(borrowed.exists());

    let temporary = block_on(host.create_temporary(
        owner.clone(),
        ResourceAccess::ReadWrite,
        ResourceRepresentation::TemporaryFile,
    ))
    .expect("temporary resource is created");
    assert_eq!(entry_count(&materialized), 1);
    assert_failure_code(
        block_on(host.resolve(
            other.clone(),
            temporary.reference().clone(),
            ResourceAccess::Read,
            ResourceRepresentation::TemporaryFile,
        )),
        "swallowtail.local_materialization.working_resource_not_approved",
    );

    let wrong_scope_lease = ResourceLease::operation_scoped(
        other,
        temporary.reference().clone(),
        ResourceAccess::ReadWrite,
        ResourceRepresentation::TemporaryFile,
    );
    assert_cleanup_failure(
        block_on(host.release(wrong_scope_lease)),
        "swallowtail.local_materialization.scope_mismatch",
    );
    assert_eq!(entry_count(&materialized), 1);
    assert_eq!(block_on(host.release(temporary)), CleanupOutcome::Clean);
    assert_eq!(entry_count(&materialized), 0);

    drop(host);
    std::fs::remove_dir_all(root).expect("fixture root is removed");
}

#[test]
fn unapproved_and_oversized_inputs_fail_without_leaving_materializations() {
    let root = temporary_root();
    let sources = root.join("sources");
    let materialized = root.join("materialized");
    std::fs::create_dir_all(&sources).expect("source directory is created");
    let attachment_path = sources.join("large.bin");
    let schema_path = sources.join("large.json");
    std::fs::write(&attachment_path, b"12345").expect("attachment source is written");
    std::fs::write(&schema_path, b"12345").expect("schema source is written");
    let attachment = attachment_ref();
    let schema = schema_ref();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_temporary_root(&materialized)
        .with_materialization_limits(LocalMaterializationLimits::new(4, 4))
        .approve_attachment(attachment.clone(), attachment_path)
        .approve_schema(schema.clone(), schema_path)
        .build();

    let oversized = AttachmentDescriptor::new(
        attachment,
        "application/octet-stream",
        AttachmentRole::Input,
    )
    .expect("attachment descriptor is valid");
    assert_failure_code(
        block_on(AttachmentService::materialize_file(
            &host,
            scope("limits"),
            oversized,
        )),
        "swallowtail.local_materialization.attachment_limit_exceeded",
    );
    assert_failure_code(
        block_on(SchemaService::materialize_file(
            &host,
            scope("limits"),
            SchemaDocument::reference(schema),
        )),
        "swallowtail.local_materialization.schema_limit_exceeded",
    );
    let unknown = AttachmentDescriptor::new(
        AttachmentRef::new("unknown").expect("attachment reference is valid"),
        "application/octet-stream",
        AttachmentRole::Input,
    )
    .expect("attachment descriptor is valid");
    assert_failure_code(
        block_on(AttachmentService::materialize_file(
            &host,
            scope("limits"),
            unknown,
        )),
        "swallowtail.local_materialization.attachment_not_approved",
    );
    assert_eq!(entry_count(&materialized), 0);

    drop(host);
    std::fs::remove_dir_all(root).expect("fixture root is removed");
}

#[test]
fn monotonic_deadlines_observe_expiry_and_cancel_without_detached_waiters() {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    let before = host.now();
    thread::sleep(Duration::from_millis(1));
    assert!(host.now() >= before);

    let deadline = Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
        host.now().ticks().saturating_add(5_000_000),
    ));
    let observation = block_on(host.wait_until(deadline));
    assert_eq!(observation.deadline(), deadline);
    assert!(observation.observed_at() >= deadline.instant());

    let distant = Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
        host.now().ticks().saturating_add(60_000_000_000),
    ));
    let started = Instant::now();
    drop(host.wait_until(distant));
    assert!(started.elapsed() < Duration::from_secs(1));
}

fn attachment_ref() -> AttachmentRef {
    AttachmentRef::new("fixture-attachment").expect("attachment reference is valid")
}

fn schema_ref() -> SchemaRef {
    SchemaRef::new("fixture-schema").expect("schema reference is valid")
}

fn working_resource_ref(suffix: &str) -> WorkingResourceRef {
    WorkingResourceRef::new(format!("fixture-working-resource-{suffix}"))
        .expect("working-resource reference is valid")
}

fn scope(suffix: &str) -> ScopeId {
    ScopeId::new(format!("fixture-scope-{suffix}")).expect("scope is valid")
}

fn temporary_root() -> PathBuf {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time follows epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "swallowtail-local-services-{}-{nanos}-{sequence}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path).expect("fixture root is created");
    path
}

fn entry_count(path: &Path) -> usize {
    match std::fs::read_dir(path) {
        Ok(entries) => entries.count(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
        Err(error) => panic!("fixture directory cannot be read: {error}"),
    }
}

fn assert_failure_code<T>(result: Result<T, RuntimeFailure>, expected: &str) {
    let failure = result.err().expect("operation must fail");
    assert_eq!(failure.diagnostic().code(), expected);
    assert!(!format!("{failure:?}").contains("private-"));
}

fn assert_cleanup_failure(outcome: CleanupOutcome, expected: &str) {
    match outcome {
        CleanupOutcome::Failed(diagnostic) => assert_eq!(diagnostic.code(), expected),
        other => panic!("expected cleanup failure, received {other:?}"),
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
