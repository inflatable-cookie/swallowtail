use crate::host::LocalProcessHost;
use crate::output::failure;
use std::fs;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, LeaseCleanupAuthority, MaterializedResourceRef, ResourceAccess,
    ResourceLease, ResourceRepresentation, RuntimeFailure, ScopeId, WorkingResourceRef,
    WorkingResourceService,
};

impl WorkingResourceService for LocalProcessHost {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let approved = self.approvals.working_resources.get(&reference).cloned();
        let scoped = self
            .materialization
            .working_resource_path(&scope, &reference);
        let result = if let Some(path) = approved {
            filesystem_lease(
                ResourceLease::consumer_owned(scope, reference, access, representation),
                representation,
                path,
            )
        } else if let Some(path) = scoped {
            filesystem_lease(
                ResourceLease::operation_scoped(scope, reference, access, representation),
                representation,
                path,
            )
        } else {
            Err(failure(
                "swallowtail.local_materialization.working_resource_not_approved",
                "Local working-resource reference is not approved for this scope",
            ))
        };
        Box::pin(async move { result })
    }

    fn create_temporary(
        &self,
        scope: ScopeId,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let result = if !matches!(
            representation,
            ResourceRepresentation::TemporaryFile | ResourceRepresentation::Filesystem
        ) {
            Err(failure(
                "swallowtail.local_materialization.representation_unsupported",
                "Local temporary working resources require a filesystem representation",
            ))
        } else {
            self.materialization
                .create_directory("resource")
                .and_then(|path| {
                    let reference = self
                        .materialization
                        .insert_working_resource(scope.clone(), path.clone());
                    if reference.is_err() {
                        let _ = fs::remove_dir_all(&path);
                    }
                    reference.map(|reference| (reference, path))
                })
                .and_then(|(reference, path)| {
                    filesystem_lease(
                        ResourceLease::operation_scoped(scope, reference, access, representation),
                        representation,
                        path,
                    )
                })
        };
        Box::pin(async move { result })
    }

    fn release(&self, lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = if lease.cleanup_authority() == LeaseCleanupAuthority::Consumer {
            CleanupOutcome::NotApplicable
        } else {
            self.materialization
                .release_working_resource(lease.scope(), lease.reference())
        };
        Box::pin(async move { outcome })
    }
}

fn filesystem_lease(
    lease: ResourceLease,
    representation: ResourceRepresentation,
    path: std::path::PathBuf,
) -> Result<ResourceLease, RuntimeFailure> {
    if representation != ResourceRepresentation::Filesystem {
        return Ok(lease);
    }
    let value = path.to_str().ok_or_else(|| {
        failure(
            "swallowtail.local_materialization.resource_path_not_utf8",
            "Local working-resource path cannot be passed to the selected driver",
        )
    })?;
    let materialized = MaterializedResourceRef::new(value).map_err(|_| {
        failure(
            "swallowtail.local_materialization.resource_path_invalid",
            "Local working-resource path cannot be passed to the selected driver",
        )
    })?;
    Ok(lease.with_filesystem(materialized))
}
