use base64::Engine;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentFileLease, AttachmentService, BlockingJob, CleanupOutcome,
    HostServices, RuntimeFailure, ScopeId,
};

const MAXIMUM_IMAGE_BYTES: u64 = 1024 * 1024;

pub(crate) struct FilePart {
    pub(crate) media_type: String,
    pub(crate) filename: &'static str,
    pub(crate) data_url: String,
}

#[derive(Clone, Default)]
pub(super) struct SharedAttachment(Arc<Mutex<Option<Materialization>>>);

struct Materialization {
    service: Arc<dyn AttachmentService>,
    lease: AttachmentFileLease,
}

impl SharedAttachment {
    fn new(service: Arc<dyn AttachmentService>, lease: AttachmentFileLease) -> Self {
        Self(Arc::new(Mutex::new(Some(Materialization {
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

pub(super) async fn prepare(
    descriptor: Option<AttachmentDescriptor>,
    services: &HostServices,
    scope: ScopeId,
) -> Result<(Option<FilePart>, SharedAttachment), RuntimeFailure> {
    let Some(descriptor) = descriptor else {
        return Ok((None, SharedAttachment::default()));
    };
    let media_type = descriptor.media_type().to_owned();
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
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok((
        Some(FilePart {
            media_type: media_type.clone(),
            filename: "approved-image.png",
            data_url: format!("data:{media_type};base64,{encoded}"),
        }),
        SharedAttachment::new(service, lease),
    ))
}

fn failure(reason: &'static str) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.opencode.attachment_materialization_failed",
        format!("OpenCode {reason}"),
    )
}
