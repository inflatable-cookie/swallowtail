use base64::Engine;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentFileLease, AttachmentService, BlockingJob, CleanupOutcome,
    HostServices, RuntimeFailure, ScopeId,
};

const MAXIMUM_IMAGE_BYTES: u64 = 1024 * 1024;

pub(super) struct PiAttachmentInput {
    encoded: String,
    materialization: SharedAttachmentMaterialization,
}

impl PiAttachmentInput {
    pub(super) fn encoded(&self) -> &str {
        &self.encoded
    }

    pub(super) fn materialization(&self) -> SharedAttachmentMaterialization {
        self.materialization.clone()
    }
}

#[derive(Clone, Default)]
pub(super) struct SharedAttachmentMaterialization(Arc<Mutex<Option<AttachmentMaterialization>>>);

struct AttachmentMaterialization {
    service: Arc<dyn AttachmentService>,
    lease: AttachmentFileLease,
}

impl SharedAttachmentMaterialization {
    fn new(service: Arc<dyn AttachmentService>, lease: AttachmentFileLease) -> Self {
        Self(Arc::new(Mutex::new(Some(AttachmentMaterialization {
            service,
            lease,
        }))))
    }

    pub(super) async fn release(&self) -> CleanupOutcome {
        let materialization = self
            .0
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take();
        match materialization {
            Some(materialization) => {
                materialization
                    .service
                    .release_file(materialization.lease)
                    .await
            }
            None => CleanupOutcome::NotApplicable,
        }
    }
}

pub(super) async fn prepare_attachment(
    descriptor: Option<AttachmentDescriptor>,
    services: &HostServices,
    scope: ScopeId,
) -> Result<Option<PiAttachmentInput>, RuntimeFailure> {
    let Some(descriptor) = descriptor else {
        return Ok(None);
    };
    let service = services
        .attachment()
        .cloned()
        .ok_or_else(|| failure("attachment service was unavailable"))?;
    // Resolve the blocking-work service before materializing anything: a host
    // without it must fail before it holds a file lease it cannot read.
    let blocking_work = services
        .blocking_work()
        .cloned()
        .ok_or_else(|| failure("blocking-work service was unavailable"))?;
    let lease = service.materialize_file(scope.clone(), descriptor).await?;
    let path = lease.file().as_driver_value().to_owned();
    let output = Arc::new(Mutex::new(None));
    let job_output = Arc::clone(&output);
    let job = Box::new(move || {
        let bytes = read_bounded_image(&path)?;
        *job_output.lock().unwrap_or_else(|error| error.into_inner()) = Some(bytes);
        Ok(())
    }) as BlockingJob;
    if let Err(error) = blocking_work.run(scope, job).await {
        let _ = service.release_file(lease).await;
        return Err(error);
    }
    let bytes = output
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .take();
    let Some(bytes) = bytes else {
        let _ = service.release_file(lease).await;
        return Err(failure("materialized image read produced no bytes"));
    };
    Ok(Some(PiAttachmentInput {
        encoded: base64::engine::general_purpose::STANDARD.encode(bytes),
        materialization: SharedAttachmentMaterialization::new(service, lease),
    }))
}

/// Reads one materialized image without ever holding more than the bound.
fn read_bounded_image(path: &str) -> Result<Vec<u8>, RuntimeFailure> {
    swallowtail_runtime::read_bounded_attachment(path, MAXIMUM_IMAGE_BYTES).map_err(|reason| {
        match reason {
            swallowtail_runtime::AttachmentReadFailure::Unreadable => {
                failure("materialized image was unreadable")
            }
            swallowtail_runtime::AttachmentReadFailure::TooLarge => {
                failure("materialized image exceeded the one MiB bound")
            }
        }
    })
}

fn failure(reason: &'static str) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.pi.rpc.attachment_materialization_failed",
        format!("Pi RPC {reason}"),
    )
}

#[cfg(test)]
mod tests {
    use super::{prepare_attachment, read_bounded_image};
    use futures_executor::block_on;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        AttachmentDescriptor, AttachmentFileLease, AttachmentRef, AttachmentRole,
        AttachmentService, BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome,
        HostServices, MaterializedFileRef, RuntimeFailure, ScopeId,
    };

    const OVERSIZED_BYTES: usize = 1024 * 1024 + 1;
    const FAILURE_CODE: &str = "swallowtail.pi.rpc.attachment_materialization_failed";

    struct FixtureDirectory(PathBuf);

    impl FixtureDirectory {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "swallowtail-pi-input-{}-{name}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(&path).expect("fixture directory creates");
            Self(path)
        }

        fn file(&self, name: &str, bytes: &[u8]) -> String {
            let path = self.0.join(name);
            let mut file = std::fs::File::create(&path).expect("fixture file creates");
            file.write_all(bytes).expect("fixture file writes");
            path.to_string_lossy().into_owned()
        }
    }

    impl Drop for FixtureDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[derive(Default)]
    struct AttachmentState {
        materializations: AtomicUsize,
        releases: AtomicUsize,
    }

    struct FixtureAttachment {
        path: String,
        state: Arc<AttachmentState>,
    }

    impl AttachmentService for FixtureAttachment {
        fn materialize_file(
            &self,
            scope: ScopeId,
            descriptor: AttachmentDescriptor,
        ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
            self.state.materializations.fetch_add(1, Ordering::SeqCst);
            let path = self.path.clone();
            let reference = descriptor.reference().clone();
            Box::pin(async move {
                Ok(AttachmentFileLease::operation_scoped(
                    scope,
                    reference,
                    MaterializedFileRef::new(path).expect("fixture file reference is valid"),
                ))
            })
        }

        fn release_file(&self, _lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome> {
            self.state.releases.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { CleanupOutcome::Clean })
        }
    }

    /// Runs the materialization job on the calling task, as a host that owns a
    /// blocking slot would.
    struct InlineBlockingWork;

    impl BlockingWorkService for InlineBlockingWork {
        fn run(
            &self,
            _scope: ScopeId,
            job: BlockingJob,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async move { job() })
        }
    }

    /// Reports success without ever running the job, as a host that lost the
    /// materialization would.
    struct DiscardingBlockingWork;

    impl BlockingWorkService for DiscardingBlockingWork {
        fn run(
            &self,
            _scope: ScopeId,
            _job: BlockingJob,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
    }

    fn scope() -> ScopeId {
        ScopeId::new("pi.input.unit").expect("scope id is valid")
    }

    fn descriptor() -> AttachmentDescriptor {
        AttachmentDescriptor::new(
            AttachmentRef::new("attachment-fixture").expect("attachment reference is valid"),
            "image/png",
            AttachmentRole::Input,
        )
        .expect("attachment descriptor is valid")
    }

    fn services(state: &Arc<AttachmentState>, path: String) -> HostServices {
        HostServices::new(ExecutionHostId::new("pi.input.unit").expect("host id is valid"))
            .with_attachment(Arc::new(FixtureAttachment {
                path,
                state: Arc::clone(state),
            }))
    }

    fn prepare_error(services: &HostServices) -> RuntimeFailure {
        match block_on(prepare_attachment(Some(descriptor()), services, scope())) {
            Ok(_) => panic!("the materialized image must be rejected"),
            Err(error) => error,
        }
    }

    #[test]
    fn an_oversized_image_is_rejected_and_its_lease_released() {
        let directory = FixtureDirectory::new("oversized");
        let path = directory.file("approved-image.png", &vec![0_u8; OVERSIZED_BYTES]);
        let state = Arc::new(AttachmentState::default());
        let services = services(&state, path).with_blocking_work(Arc::new(InlineBlockingWork));

        let error = prepare_error(&services);

        assert_eq!(error.diagnostic().code(), FAILURE_CODE);
        assert!(
            error
                .diagnostic()
                .message()
                .contains("exceeded the one MiB bound"),
            "unexpected message: {}",
            error.diagnostic().message()
        );
        assert_eq!(state.materializations.load(Ordering::SeqCst), 1);
        assert_eq!(
            state.releases.load(Ordering::SeqCst),
            1,
            "an oversized attachment must release the file lease it acquired"
        );
    }

    #[test]
    fn a_missing_blocking_work_service_never_materializes_a_lease() {
        let directory = FixtureDirectory::new("no-blocking-work");
        let path = directory.file("approved-image.png", b"\x89PNG\r\n\x1a\n");
        let state = Arc::new(AttachmentState::default());
        let services = services(&state, path);

        let error = prepare_error(&services);

        assert_eq!(error.diagnostic().code(), FAILURE_CODE);
        assert!(
            error
                .diagnostic()
                .message()
                .contains("blocking-work service was unavailable"),
            "unexpected message: {}",
            error.diagnostic().message()
        );
        assert_eq!(
            state.materializations.load(Ordering::SeqCst),
            0,
            "a host without blocking work must not be asked to materialize a lease"
        );
        assert_eq!(state.releases.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn a_missing_materialization_produced_no_bytes_and_releases_its_lease() {
        let directory = FixtureDirectory::new("discarded-job");
        let path = directory.file("approved-image.png", b"\x89PNG\r\n\x1a\n");
        let state = Arc::new(AttachmentState::default());
        let services = services(&state, path).with_blocking_work(Arc::new(DiscardingBlockingWork));

        let error = prepare_error(&services);

        assert_eq!(error.diagnostic().code(), FAILURE_CODE);
        assert!(
            error.diagnostic().message().contains("produced no bytes"),
            "unexpected message: {}",
            error.diagnostic().message()
        );
        assert_eq!(
            state.releases.load(Ordering::SeqCst),
            1,
            "a lost materialization must release the file lease it acquired"
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_read_of_an_unbounded_stream_stops_at_the_size_bound() {
        // `/dev/zero` never ends, so an unbounded read cannot return at all.
        // A bounded read must reject after the extra byte instead of draining
        // an arbitrary stream into memory.
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let outcome = read_bounded_image("/dev/zero")
                .map(|bytes| bytes.len())
                .map_err(|error| error.diagnostic().code().to_owned());
            let _ = sender.send(outcome);
        });
        let outcome = receiver
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("a bounded image read returns without draining the stream");
        assert_eq!(outcome, Err(FAILURE_CODE.to_owned()));
    }
}
