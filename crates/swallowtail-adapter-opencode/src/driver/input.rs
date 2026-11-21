use base64::Engine;
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentMaterializationFailure, AttachmentReadFailure, HostServices,
    RuntimeFailure, ScopeId, SharedAttachmentMaterialization, materialize_bounded_attachment,
};

const MAXIMUM_IMAGE_BYTES: u64 = 1024 * 1024;

pub(crate) struct FilePart {
    pub(crate) media_type: String,
    pub(crate) filename: &'static str,
    pub(crate) data_url: String,
}

/// The materialized attachment's lease, released once after the turn.
pub(super) type SharedAttachment = SharedAttachmentMaterialization;

pub(super) async fn prepare(
    descriptor: Option<AttachmentDescriptor>,
    services: &HostServices,
    scope: ScopeId,
) -> Result<(Option<FilePart>, SharedAttachment), RuntimeFailure> {
    let Some(descriptor) = descriptor else {
        return Ok((None, SharedAttachment::default()));
    };
    let media_type = descriptor.media_type().to_owned();
    let (bytes, materialization) =
        materialize_bounded_attachment(services, scope, descriptor, MAXIMUM_IMAGE_BYTES)
            .await
            .map_err(refusal)?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok((
        Some(FilePart {
            media_type: media_type.clone(),
            filename: "approved-image.png",
            data_url: format!("data:{media_type};base64,{encoded}"),
        }),
        materialization,
    ))
}

fn refusal(reason: AttachmentMaterializationFailure) -> RuntimeFailure {
    match reason {
        AttachmentMaterializationFailure::Host(error) => error,
        AttachmentMaterializationFailure::AttachmentServiceUnavailable => {
            failure("attachment service was unavailable")
        }
        AttachmentMaterializationFailure::BlockingWorkUnavailable => {
            failure("blocking-work service was unavailable")
        }
        AttachmentMaterializationFailure::Read(AttachmentReadFailure::Unreadable) => {
            failure("materialized image was unreadable")
        }
        AttachmentMaterializationFailure::Read(AttachmentReadFailure::TooLarge) => {
            failure("materialized image exceeded the one MiB bound")
        }
        AttachmentMaterializationFailure::NoBytes => {
            failure("materialized image read produced no bytes")
        }
    }
}

fn failure(reason: &'static str) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.opencode.attachment_materialization_failed",
        format!("OpenCode {reason}"),
    )
}

#[cfg(test)]
mod tests {
    use super::prepare;
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
    const FAILURE_CODE: &str = "swallowtail.opencode.attachment_materialization_failed";

    struct FixtureDirectory(PathBuf);

    impl FixtureDirectory {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "swallowtail-opencode-input-{}-{name}",
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
        ScopeId::new("opencode.input.unit").expect("scope id is valid")
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
        HostServices::new(ExecutionHostId::new("opencode.input.unit").expect("host id is valid"))
            .with_attachment(Arc::new(FixtureAttachment {
                path,
                state: Arc::clone(state),
            }))
    }

    fn prepare_error(services: &HostServices) -> RuntimeFailure {
        match block_on(prepare(Some(descriptor()), services, scope())) {
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
        // The shared bounded read must refuse it instead of draining it.
        let state = Arc::new(AttachmentState::default());
        let services = services(&state, "/dev/zero".to_owned())
            .with_blocking_work(Arc::new(InlineBlockingWork));

        let error = prepare_error(&services);

        assert_eq!(error.diagnostic().code(), FAILURE_CODE);
        assert_eq!(state.releases.load(Ordering::SeqCst), 1);
    }
}
