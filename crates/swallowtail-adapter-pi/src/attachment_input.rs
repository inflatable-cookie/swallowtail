//! Inline image attachment input shared by the Pi RPC and SDK-sidecar drivers.
//!
//! The lease, bounded read, and release mechanics live in the runtime. This
//! module maps their refusals onto each driver's own diagnostic namespace, so
//! the two drivers keep distinct codes without two copies of the flow.

use crate::failure::failure;
use base64::Engine;
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentMaterializationFailure, AttachmentReadFailure, HostServices,
    RuntimeFailure, ScopeId, SharedAttachmentMaterialization, materialize_bounded_attachment,
};

const MAXIMUM_IMAGE_BYTES: u64 = 1024 * 1024;

/// The diagnostic code and owner name one driver reports for an attachment.
#[derive(Clone, Copy)]
pub(crate) struct AttachmentFailureNamespace {
    code: &'static str,
    owner: &'static str,
}

impl AttachmentFailureNamespace {
    /// The Pi RPC driver's namespace.
    pub(crate) const RPC: Self = Self {
        code: "swallowtail.pi.rpc.attachment_materialization_failed",
        owner: "Pi RPC",
    };

    /// The Pi SDK sidecar driver's namespace.
    pub(crate) const SDK_SIDECAR: Self = Self {
        code: "swallowtail.pi.sdk-sidecar.attachment_materialization_failed",
        owner: "Pi SDK sidecar",
    };

    fn failure(self, reason: &'static str) -> RuntimeFailure {
        failure(self.code, format!("{} {reason}", self.owner))
    }
}

/// One materialized image, encoded for transport, with its release handle.
pub(crate) struct PiAttachmentInput {
    encoded: String,
    materialization: SharedAttachmentMaterialization,
}

impl PiAttachmentInput {
    pub(crate) fn encoded(&self) -> &str {
        &self.encoded
    }

    pub(crate) fn materialization(&self) -> SharedAttachmentMaterialization {
        self.materialization.clone()
    }
}

/// Materializes the first admitted attachment as a base64 image for one driver.
pub(crate) async fn prepare_attachment(
    descriptor: Option<AttachmentDescriptor>,
    services: &HostServices,
    scope: ScopeId,
    namespace: AttachmentFailureNamespace,
) -> Result<Option<PiAttachmentInput>, RuntimeFailure> {
    let Some(descriptor) = descriptor else {
        return Ok(None);
    };
    let (bytes, materialization) =
        materialize_bounded_attachment(services, scope, descriptor, MAXIMUM_IMAGE_BYTES)
            .await
            .map_err(|reason| refusal(namespace, reason))?;
    Ok(Some(PiAttachmentInput {
        encoded: base64::engine::general_purpose::STANDARD.encode(bytes),
        materialization,
    }))
}

fn refusal(
    namespace: AttachmentFailureNamespace,
    reason: AttachmentMaterializationFailure,
) -> RuntimeFailure {
    match reason {
        AttachmentMaterializationFailure::Host(error) => error,
        AttachmentMaterializationFailure::AttachmentServiceUnavailable => {
            namespace.failure("attachment service was unavailable")
        }
        AttachmentMaterializationFailure::BlockingWorkUnavailable => {
            namespace.failure("blocking-work service was unavailable")
        }
        AttachmentMaterializationFailure::Read(AttachmentReadFailure::Unreadable) => {
            namespace.failure("materialized image was unreadable")
        }
        AttachmentMaterializationFailure::Read(AttachmentReadFailure::TooLarge) => {
            namespace.failure("materialized image exceeded the one MiB bound")
        }
        AttachmentMaterializationFailure::NoBytes => {
            namespace.failure("materialized image read produced no bytes")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AttachmentFailureNamespace, prepare_attachment};
    use std::future::Future;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Waker};
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        AttachmentDescriptor, AttachmentFileLease, AttachmentRef, AttachmentRole,
        AttachmentService, BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome,
        HostServices, MaterializedFileRef, RuntimeFailure, ScopeId,
    };

    const OVERSIZED_BYTES: usize = 1024 * 1024 + 1;

    fn block_on<F: Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        for _ in 0..128 {
            if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
                return value;
            }
        }
        panic!("fixture future did not resolve within bounded polls");
    }

    /// A fixture directory under the process temp dir, removed on drop.
    struct FixtureDirectory(PathBuf);

    impl FixtureDirectory {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "swallowtail-pi-attachment-input-{}-{name}",
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
    struct Counts {
        materializations: AtomicUsize,
        releases: AtomicUsize,
    }

    struct FixtureAttachment {
        path: String,
        counts: Arc<Counts>,
    }

    impl AttachmentService for FixtureAttachment {
        fn materialize_file(
            &self,
            scope: ScopeId,
            descriptor: AttachmentDescriptor,
        ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
            self.counts.materializations.fetch_add(1, Ordering::SeqCst);
            let path = self.path.clone();
            let reference = descriptor.reference().clone();
            Box::pin(async move {
                Ok(AttachmentFileLease::operation_scoped(
                    scope,
                    reference,
                    MaterializedFileRef::new(path).expect("fixture path is valid"),
                ))
            })
        }

        fn release_file(&self, _lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome> {
            self.counts.releases.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { CleanupOutcome::Clean })
        }
    }

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
        ScopeId::new("pi.attachment.input.unit").expect("scope id is valid")
    }

    fn descriptor() -> AttachmentDescriptor {
        AttachmentDescriptor::new(
            AttachmentRef::new("attachment-fixture").expect("attachment reference is valid"),
            "image/png",
            AttachmentRole::Input,
        )
        .expect("attachment descriptor is valid")
    }

    fn services(counts: &Arc<Counts>, path: String) -> HostServices {
        HostServices::new(
            ExecutionHostId::new("pi.attachment.input.unit").expect("host id is valid"),
        )
        .with_attachment(Arc::new(FixtureAttachment {
            path,
            counts: Arc::clone(counts),
        }))
    }

    fn refusal(services: &HostServices, namespace: AttachmentFailureNamespace) -> RuntimeFailure {
        match block_on(prepare_attachment(
            Some(descriptor()),
            services,
            scope(),
            namespace,
        )) {
            Ok(_) => panic!("the attachment must be refused"),
            Err(error) => error,
        }
    }

    #[test]
    fn an_oversized_image_is_refused_under_each_driver_namespace_and_released() {
        for (namespace, code, owner) in [
            (
                AttachmentFailureNamespace::RPC,
                "swallowtail.pi.rpc.attachment_materialization_failed",
                "Pi RPC",
            ),
            (
                AttachmentFailureNamespace::SDK_SIDECAR,
                "swallowtail.pi.sdk-sidecar.attachment_materialization_failed",
                "Pi SDK sidecar",
            ),
        ] {
            let directory = FixtureDirectory::new(owner);
            let path = directory.file("approved-image.png", &vec![0_u8; OVERSIZED_BYTES]);
            let counts = Arc::new(Counts::default());
            let services = services(&counts, path).with_blocking_work(Arc::new(InlineBlockingWork));

            let error = refusal(&services, namespace);

            assert_eq!(error.diagnostic().code(), code);
            assert!(
                error
                    .diagnostic()
                    .message()
                    .starts_with(&format!("{owner} "))
                    && error
                        .diagnostic()
                        .message()
                        .contains("exceeded the one MiB bound"),
                "unexpected message: {}",
                error.diagnostic().message()
            );
            assert_eq!(counts.materializations.load(Ordering::SeqCst), 1);
            assert_eq!(counts.releases.load(Ordering::SeqCst), 1);
        }
    }

    #[test]
    fn a_host_without_blocking_work_never_materializes_a_lease() {
        let directory = FixtureDirectory::new("no-blocking-work");
        let path = directory.file("approved-image.png", b"\x89PNG\r\n\x1a\n");
        let counts = Arc::new(Counts::default());
        let services = services(&counts, path);

        let error = refusal(&services, AttachmentFailureNamespace::RPC);

        assert!(
            error
                .diagnostic()
                .message()
                .contains("blocking-work service was unavailable"),
            "unexpected message: {}",
            error.diagnostic().message()
        );
        assert_eq!(counts.materializations.load(Ordering::SeqCst), 0);
        assert_eq!(counts.releases.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn a_discarded_read_produced_no_bytes_and_releases_its_lease() {
        let directory = FixtureDirectory::new("discarded-job");
        let path = directory.file("approved-image.png", b"\x89PNG\r\n\x1a\n");
        let counts = Arc::new(Counts::default());
        let services = services(&counts, path).with_blocking_work(Arc::new(DiscardingBlockingWork));

        let error = refusal(&services, AttachmentFailureNamespace::SDK_SIDECAR);

        assert!(
            error.diagnostic().message().contains("produced no bytes"),
            "unexpected message: {}",
            error.diagnostic().message()
        );
        assert_eq!(counts.releases.load(Ordering::SeqCst), 1);
    }

    #[cfg(unix)]
    #[test]
    fn an_unbounded_stream_is_refused_at_the_bound() {
        // `/dev/zero` never ends; the bounded read must reject it promptly.
        let counts = Arc::new(Counts::default());
        let services = services(&counts, "/dev/zero".to_owned())
            .with_blocking_work(Arc::new(InlineBlockingWork));

        let error = refusal(&services, AttachmentFailureNamespace::RPC);

        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.rpc.attachment_materialization_failed"
        );
        assert_eq!(counts.releases.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn no_descriptor_means_no_attachment() {
        let counts = Arc::new(Counts::default());
        let services = services(&counts, String::new());

        let input = block_on(prepare_attachment(
            None,
            &services,
            scope(),
            AttachmentFailureNamespace::RPC,
        ))
        .expect("an absent attachment is not an error");

        assert!(input.is_none());
        assert_eq!(counts.materializations.load(Ordering::SeqCst), 0);
    }
}
