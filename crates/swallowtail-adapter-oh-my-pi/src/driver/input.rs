use base64::Engine;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentFileLease, AttachmentService, BlockingJob, CleanupOutcome,
    HostServices, RuntimeFailure, ScopeId,
};

const MAXIMUM_IMAGE_BYTES: u64 = 1024 * 1024;

pub(super) struct OhMyPiAttachmentInput {
    encoded: String,
    materialization: SharedAttachmentMaterialization,
}

impl OhMyPiAttachmentInput {
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
) -> Result<Option<OhMyPiAttachmentInput>, RuntimeFailure> {
    let Some(descriptor) = descriptor else {
        return Ok(None);
    };
    let service = services
        .attachment()
        .cloned()
        .ok_or_else(|| failure("attachment service was unavailable"))?;
    let lease = service.materialize_file(scope.clone(), descriptor).await?;
    let path = lease.file().as_driver_value().to_owned();
    let output = Arc::new(Mutex::new(None));
    let job_output = Arc::clone(&output);
    let job = Box::new(move || {
        let bytes =
            std::fs::read(path).map_err(|_| failure("materialized image was unreadable"))?;
        if bytes.len() as u64 > MAXIMUM_IMAGE_BYTES {
            return Err(failure("materialized image exceeded the one MiB bound"));
        }
        *job_output.lock().unwrap_or_else(|error| error.into_inner()) = Some(bytes);
        Ok(())
    }) as BlockingJob;
    let result = services
        .blocking_work()
        .ok_or_else(|| failure("blocking-work service was unavailable"))?
        .run(scope, job)
        .await;
    if let Err(error) = result {
        let _ = service.release_file(lease).await;
        return Err(error);
    }
    let bytes = output
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .take()
        .ok_or_else(|| failure("materialized image read produced no bytes"))?;
    Ok(Some(OhMyPiAttachmentInput {
        encoded: base64::engine::general_purpose::STANDARD.encode(bytes),
        materialization: SharedAttachmentMaterialization::new(service, lease),
    }))
}

fn failure(reason: &'static str) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.oh_my_pi.rpc.attachment_materialization_failed",
        format!("OhMyPi RPC {reason}"),
    )
}
