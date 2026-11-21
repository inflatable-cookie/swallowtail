//! Shared mechanics for materializing one attachment as bounded bytes.
//!
//! Every adapter that sends an attachment inline runs the same sequence:
//! resolve the host services, have the attachment service materialize a file
//! lease, read the file on the blocking pool under a byte bound, and release
//! the lease on every refusal. This module owns that sequence once. Adapters
//! keep their own provider codes and messages by mapping
//! [`AttachmentMaterializationFailure`] at their boundary.

use super::{AttachmentDescriptor, AttachmentReadFailure, read_bounded_attachment};
use crate::{
    AttachmentFileLease, AttachmentService, BlockingJob, CleanupOutcome, HostServices,
    RuntimeFailure, ScopeId,
};
use std::sync::{Arc, Mutex};

/// Why an attachment could not be materialized as bounded bytes.
#[derive(Debug)]
pub enum AttachmentMaterializationFailure {
    /// The host provides no attachment service.
    AttachmentServiceUnavailable,
    /// The host provides no blocking-work service to read the file on.
    BlockingWorkUnavailable,
    /// A host service refused the attachment or its blocking read.
    Host(RuntimeFailure),
    /// The materialized file could not be read within the bound.
    Read(AttachmentReadFailure),
    /// The blocking read completed without producing bytes.
    NoBytes,
}

/// A materialized attachment whose file lease is released exactly once.
///
/// Clones share one lease, so an adapter can keep a handle for cleanup while
/// the request holds another. The first [`release`](Self::release) takes the
/// lease and later calls report [`CleanupOutcome::NotApplicable`].
#[derive(Clone, Default)]
pub struct SharedAttachmentMaterialization(Arc<Mutex<Option<HeldAttachment>>>);

impl std::fmt::Debug for SharedAttachmentMaterialization {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The lease names a host file, so Debug output stays opaque.
        formatter.write_str("SharedAttachmentMaterialization { .. }")
    }
}

struct HeldAttachment {
    service: Arc<dyn AttachmentService>,
    lease: AttachmentFileLease,
}

impl SharedAttachmentMaterialization {
    /// Holds one materialized lease for the service that issued it.
    #[must_use]
    pub fn new(service: Arc<dyn AttachmentService>, lease: AttachmentFileLease) -> Self {
        Self(Arc::new(Mutex::new(Some(HeldAttachment {
            service,
            lease,
        }))))
    }

    /// Releases the held lease, if no earlier release already took it.
    pub async fn release(&self) -> CleanupOutcome {
        let held = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        match held {
            Some(held) => held.service.release_file(held.lease).await,
            None => CleanupOutcome::NotApplicable,
        }
    }
}

/// Materializes one attachment and returns its bytes, which never exceed
/// `maximum_bytes`, together with the lease that must be released afterwards.
///
/// Service resolution happens before any lease is acquired, so a host that
/// lacks blocking work never holds a file it cannot read. Every failure after
/// acquisition releases the lease before it returns.
///
/// # Errors
///
/// Returns the first refusal in the order described on
/// [`AttachmentMaterializationFailure`].
pub async fn materialize_bounded_attachment(
    services: &HostServices,
    scope: ScopeId,
    descriptor: AttachmentDescriptor,
    maximum_bytes: u64,
) -> Result<(Vec<u8>, SharedAttachmentMaterialization), AttachmentMaterializationFailure> {
    let service = services
        .attachment()
        .cloned()
        .ok_or(AttachmentMaterializationFailure::AttachmentServiceUnavailable)?;
    let blocking_work = services
        .blocking_work()
        .cloned()
        .ok_or(AttachmentMaterializationFailure::BlockingWorkUnavailable)?;
    let lease = service
        .materialize_file(scope.clone(), descriptor)
        .await
        .map_err(AttachmentMaterializationFailure::Host)?;
    let path = lease.file().as_driver_value().to_owned();
    let output = Arc::new(Mutex::new(None));
    let job_output = Arc::clone(&output);
    let job = Box::new(move || {
        let read = read_bounded_attachment(&path, maximum_bytes);
        *job_output
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(read);
        Ok(())
    }) as BlockingJob;
    let held = SharedAttachmentMaterialization::new(service, lease);
    if let Err(error) = blocking_work.run(scope, job).await {
        let _ = held.release().await;
        return Err(AttachmentMaterializationFailure::Host(error));
    }
    let read = output
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .take();
    match read {
        Some(Ok(bytes)) => Ok((bytes, held)),
        Some(Err(reason)) => {
            let _ = held.release().await;
            Err(AttachmentMaterializationFailure::Read(reason))
        }
        None => {
            let _ = held.release().await;
            Err(AttachmentMaterializationFailure::NoBytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AttachmentMaterializationFailure, materialize_bounded_attachment};
    use crate::{
        AttachmentDescriptor, AttachmentFileLease, AttachmentRef, AttachmentRole,
        AttachmentService, BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome,
        HostServices, MaterializedFileRef, RuntimeFailure, ScopeId,
    };
    use std::future::Future;
    use std::task::{Context, Poll, Waker};

    /// Polls a future that completes without waiting on a host, so no executor
    /// is needed. The bound stops a hung future from hanging the test.
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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use swallowtail_core::ExecutionHostId;

    #[derive(Default)]
    struct Counts {
        materializations: AtomicUsize,
        releases: AtomicUsize,
    }

    struct FixtureAttachment {
        path: &'static str,
        counts: Arc<Counts>,
    }

    impl AttachmentService for FixtureAttachment {
        fn materialize_file(
            &self,
            scope: ScopeId,
            descriptor: AttachmentDescriptor,
        ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
            self.counts.materializations.fetch_add(1, Ordering::SeqCst);
            let path = self.path;
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
        ScopeId::new("attachment.materialization.unit").expect("scope id is valid")
    }

    fn descriptor() -> AttachmentDescriptor {
        AttachmentDescriptor::new(
            AttachmentRef::new("attachment-materialization").expect("reference is valid"),
            "image/png",
            AttachmentRole::Input,
        )
        .expect("descriptor is valid")
    }

    fn services(counts: &Arc<Counts>, path: &'static str) -> HostServices {
        HostServices::new(
            ExecutionHostId::new("attachment.materialization.unit").expect("host id is valid"),
        )
        .with_attachment(Arc::new(FixtureAttachment {
            path,
            counts: Arc::clone(counts),
        }))
    }

    #[test]
    fn a_host_without_blocking_work_never_materializes_a_lease() {
        let counts = Arc::new(Counts::default());
        let services = services(&counts, "/nonexistent/swallowtail-materialization");

        let failure = block_on(materialize_bounded_attachment(
            &services,
            scope(),
            descriptor(),
            8,
        ))
        .expect_err("blocking work is required");

        assert!(matches!(
            failure,
            AttachmentMaterializationFailure::BlockingWorkUnavailable
        ));
        assert_eq!(counts.materializations.load(Ordering::SeqCst), 0);
        assert_eq!(counts.releases.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn a_missing_attachment_service_fails_before_any_work() {
        let services = HostServices::new(
            ExecutionHostId::new("attachment.materialization.unit").expect("host id is valid"),
        );

        let failure = block_on(materialize_bounded_attachment(
            &services,
            scope(),
            descriptor(),
            8,
        ))
        .expect_err("an attachment service is required");

        assert!(matches!(
            failure,
            AttachmentMaterializationFailure::AttachmentServiceUnavailable
        ));
    }

    #[cfg(unix)]
    #[test]
    fn an_oversized_file_is_refused_and_its_lease_released() {
        let counts = Arc::new(Counts::default());
        let services =
            services(&counts, "/dev/zero").with_blocking_work(Arc::new(InlineBlockingWork));

        let failure = block_on(materialize_bounded_attachment(
            &services,
            scope(),
            descriptor(),
            8,
        ))
        .expect_err("an unbounded stream exceeds the bound");

        assert!(matches!(
            failure,
            AttachmentMaterializationFailure::Read(crate::AttachmentReadFailure::TooLarge)
        ));
        assert_eq!(counts.materializations.load(Ordering::SeqCst), 1);
        assert_eq!(counts.releases.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn a_lost_job_releases_its_lease_and_reports_no_bytes() {
        let counts = Arc::new(Counts::default());
        let services = services(&counts, "/nonexistent/swallowtail-materialization")
            .with_blocking_work(Arc::new(DiscardingBlockingWork));

        let failure = block_on(materialize_bounded_attachment(
            &services,
            scope(),
            descriptor(),
            8,
        ))
        .expect_err("a discarded job yields no bytes");

        assert!(matches!(failure, AttachmentMaterializationFailure::NoBytes));
        assert_eq!(counts.releases.load(Ordering::SeqCst), 1);
    }

    #[cfg(unix)]
    #[test]
    fn a_held_lease_is_released_exactly_once() {
        let counts = Arc::new(Counts::default());
        let services =
            services(&counts, "/dev/null").with_blocking_work(Arc::new(InlineBlockingWork));

        let (bytes, held) = block_on(materialize_bounded_attachment(
            &services,
            scope(),
            descriptor(),
            8,
        ))
        .expect("an empty file materializes");

        assert!(bytes.is_empty());
        assert!(matches!(block_on(held.release()), CleanupOutcome::Clean));
        assert!(matches!(
            block_on(held.release()),
            CleanupOutcome::NotApplicable
        ));
        assert_eq!(counts.releases.load(Ordering::SeqCst), 1);
    }
}
