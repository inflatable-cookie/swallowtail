use crate::host::LocalProcessHost;
use crate::output::failure;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentFileLease, AttachmentService, BoxFuture, CleanupOutcome,
    RuntimeFailure, ScopeId,
};

impl AttachmentService for LocalProcessHost {
    fn materialize_file(
        &self,
        scope: ScopeId,
        descriptor: AttachmentDescriptor,
    ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
        let result = (|| {
            if descriptor
                .known_length()
                .is_some_and(|length| length > self.materialization_limits.attachment_bytes())
            {
                return Err(attachment_limit_exceeded());
            }
            let source = self
                .approvals
                .attachments
                .get(descriptor.reference())
                .ok_or_else(|| {
                    failure(
                        "swallowtail.local_materialization.attachment_not_approved",
                        "Local attachment reference is not approved",
                    )
                })?;
            let directory = self.materialization.create_directory("attachment")?;
            let destination = directory.join(attachment_filename(source));
            if let Err(error) = copy_bounded(
                source,
                &destination,
                self.materialization_limits.attachment_bytes(),
            ) {
                let _ = fs::remove_dir_all(&directory);
                return Err(error);
            }
            let file = match self.materialization.insert_file(
                scope.clone(),
                destination,
                directory.clone(),
            ) {
                Ok(file) => file,
                Err(error) => {
                    let _ = fs::remove_dir_all(directory);
                    return Err(error);
                }
            };
            Ok(AttachmentFileLease::operation_scoped(
                scope,
                descriptor.reference().clone(),
                file,
            ))
        })();
        Box::pin(async move { result })
    }

    fn release_file(&self, lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = self
            .materialization
            .release_file(lease.scope(), lease.file());
        Box::pin(async move { outcome })
    }
}

fn attachment_filename(source: &Path) -> String {
    source
        .extension()
        .and_then(|extension| extension.to_str())
        .filter(|extension| {
            !extension.is_empty()
                && extension.len() <= 16
                && extension
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric())
        })
        .map_or_else(
            || "attachment.bin".to_owned(),
            |extension| format!("attachment.{extension}"),
        )
}

fn copy_bounded(
    source: &Path,
    destination: &Path,
    maximum_bytes: u64,
) -> Result<(), RuntimeFailure> {
    let source = File::open(source).map_err(|_| {
        failure(
            "swallowtail.local_materialization.source_unavailable",
            "Approved local materialization source is unavailable",
        )
    })?;
    if source
        .metadata()
        .map(|metadata| metadata.len())
        .unwrap_or(u64::MAX)
        > maximum_bytes
    {
        return Err(attachment_limit_exceeded());
    }
    let mut source = source.take(maximum_bytes.saturating_add(1));
    let mut destination = File::create(destination).map_err(|_| {
        failure(
            "swallowtail.local_materialization.write_failed",
            "Local materialization could not be written",
        )
    })?;
    let copied = std::io::copy(&mut source, &mut destination).map_err(|_| {
        failure(
            "swallowtail.local_materialization.copy_failed",
            "Approved local materialization source could not be copied",
        )
    })?;
    if copied > maximum_bytes {
        Err(attachment_limit_exceeded())
    } else {
        Ok(())
    }
}

fn attachment_limit_exceeded() -> RuntimeFailure {
    failure(
        "swallowtail.local_materialization.attachment_limit_exceeded",
        "Local attachment exceeded its host-approved size limit",
    )
}
