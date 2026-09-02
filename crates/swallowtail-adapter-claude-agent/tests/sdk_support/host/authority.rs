use super::super::host::{CleanupEvent, FIXTURE_CWD, SdkFixtureHost, fixture_failure};
use std::sync::atomic::Ordering;
use swallowtail_core::{CredentialRef, EndpointAudience, ResourceAccess, ResourceRepresentation};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, CredentialService, DelegatedCredential,
    MaterializedResourceRef, ResourceLease, RuntimeFailure, ScopeId, WorkingResourceRef,
    WorkingResourceService,
};

impl CredentialService for SdkFixtureHost {
    /// Only a delegated lease is ever produced: the fixture models the
    /// official credential store the native binary reads for itself, so no
    /// secret value exists anywhere in this test surface.
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.shared
            .credential_acquisitions
            .fetch_add(1, Ordering::SeqCst);
        Box::pin(async move {
            Ok(CredentialLease::Delegated(DelegatedCredential::new(
                scope, reference, audience,
            )))
        })
    }

    fn release(&self, _lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        self.shared
            .cleanup
            .lock()
            .expect("SDK fixture cleanup lock poisoned")
            .push(CleanupEvent::CredentialRelease);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

impl WorkingResourceService for SdkFixtureHost {
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
                        MaterializedResourceRef::new(FIXTURE_CWD).expect("valid path"),
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
            .expect("SDK fixture cleanup lock poisoned")
            .push(CleanupEvent::ResourceRelease);
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}
