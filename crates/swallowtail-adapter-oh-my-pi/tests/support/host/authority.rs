use super::{CleanupEvent, FixtureHost, fixture_failure};
use swallowtail_core::{ResourceAccess, ResourceRepresentation};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, MaterializedResourceRef, ResourceLease, RuntimeFailure, ScopeId,
    WorkingResourceRef, WorkingResourceService,
};

impl WorkingResourceService for FixtureHost {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async move {
            Ok(
                ResourceLease::consumer_owned(scope, reference, access, representation)
                    .with_filesystem(
                        MaterializedResourceRef::new("/fixture/pi-workspace").expect("valid path"),
                    ),
            )
        })
    }

    fn create_temporary(
        &self,
        _scope: ScopeId,
        _access: ResourceAccess,
        _representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        self.shared
            .cleanup
            .lock()
            .expect("OhMyPi fixture cleanup lock poisoned")
            .push(CleanupEvent::ResourceRelease);
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}
