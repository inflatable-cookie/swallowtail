impl CredentialService for FixtureHost {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.credentials.fetch_add(1, Ordering::SeqCst);
        let lease = CredentialLease::Delegated(DelegatedCredential::new(
            scope, reference, audience,
        ));
        Box::pin(async move { Ok(lease) })
    }

    fn release(&self, _lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        self.credential_releases.fetch_add(1, Ordering::SeqCst);
        self.cleanup_events
            .lock()
            .expect("fixture cleanup-event lock poisoned")
            .push(CleanupEvent::CredentialRelease);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

impl WorkingResourceService for FixtureHost {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let lease = ResourceLease::consumer_owned(scope, reference, access, representation)
            .with_filesystem(
                swallowtail_runtime::MaterializedResourceRef::new("/fixture/workspace")
                    .expect("fixture path is valid"),
            );
        Box::pin(async move { Ok(lease) })
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
        self.resource_releases.fetch_add(1, Ordering::SeqCst);
        self.cleanup_events
            .lock()
            .expect("fixture cleanup-event lock poisoned")
            .push(CleanupEvent::ResourceRelease);
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}

impl WorkingResourceIoService for FixtureHost {
    fn read_text(
        &self,
        _lease: &ResourceLease,
        request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        let result = WorkingResourceText::new("fixture".to_owned(), request.maximum_bytes())
            .map_err(|_| fixture_failure());
        Box::pin(async move { result })
    }

    fn write_text(
        &self,
        lease: &ResourceLease,
        request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        if lease.access() != ResourceAccess::ReadWrite {
            return Box::pin(async { Err(fixture_failure()) });
        }
        self.resource_writes
            .lock()
            .expect("fixture writes lock poisoned")
            .push((
                request.locator().as_host_value().to_owned(),
                request.content().as_driver_value().to_owned(),
            ));
        Box::pin(async { Ok(()) })
    }
}
