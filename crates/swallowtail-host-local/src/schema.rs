use crate::host::LocalProcessHost;
use crate::output::failure;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, RuntimeFailure, SchemaDocument, SchemaFileLease, SchemaService,
    ScopeId,
};

impl SchemaService for LocalProcessHost {
    fn materialize_file(
        &self,
        scope: ScopeId,
        document: SchemaDocument,
    ) -> BoxFuture<'static, Result<SchemaFileLease, RuntimeFailure>> {
        let result = (|| {
            let bytes = match document {
                SchemaDocument::Inline(bytes) => {
                    if bytes.len() as u64 > self.materialization_limits.schema_bytes() {
                        return Err(schema_limit_exceeded());
                    }
                    bytes
                }
                SchemaDocument::Reference(reference) => {
                    let source = self.approvals.schemas.get(&reference).ok_or_else(|| {
                        failure(
                            "swallowtail.local_materialization.schema_not_approved",
                            "Local schema reference is not approved",
                        )
                    })?;
                    read_bounded(source, self.materialization_limits.schema_bytes())?
                }
            };
            let directory = self.materialization.create_directory("schema")?;
            let destination = directory.join("schema.json");
            if File::create(&destination)
                .and_then(|mut file| file.write_all(&bytes))
                .is_err()
            {
                let _ = fs::remove_dir_all(&directory);
                return Err(failure(
                    "swallowtail.local_materialization.write_failed",
                    "Local schema materialization could not be written",
                ));
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
            Ok(SchemaFileLease::operation_scoped(scope, file))
        })();
        Box::pin(async move { result })
    }

    fn release_file(&self, lease: SchemaFileLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = self
            .materialization
            .release_file(lease.scope(), lease.file());
        Box::pin(async move { outcome })
    }
}

fn read_bounded(source: &Path, maximum_bytes: u64) -> Result<Vec<u8>, RuntimeFailure> {
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
        return Err(schema_limit_exceeded());
    }
    let capacity = usize::try_from(maximum_bytes.min(64 * 1024)).unwrap_or(64 * 1024);
    let mut bytes = Vec::with_capacity(capacity);
    source
        .take(maximum_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|_| {
            failure(
                "swallowtail.local_materialization.read_failed",
                "Approved local materialization source could not be read",
            )
        })?;
    if bytes.len() as u64 > maximum_bytes {
        Err(schema_limit_exceeded())
    } else {
        Ok(bytes)
    }
}

fn schema_limit_exceeded() -> RuntimeFailure {
    failure(
        "swallowtail.local_materialization.schema_limit_exceeded",
        "Local schema exceeded its host-approved size limit",
    )
}
