use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{ResourceAccess, ResourceRepresentation, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, MaterializedResourceRef, ResourceLease, RuntimeFailure, ScopeId,
    WorkingResourceRef, WorkingResourceService,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedWorkingResource {
    pub reference: String,
    pub access: ResourceAccess,
    pub representation: ResourceRepresentation,
    pub path: String,
}

#[derive(Default)]
pub struct WorkingResourceState {
    resolved: Mutex<Option<ResolvedWorkingResource>>,
    releases: AtomicUsize,
}

impl WorkingResourceState {
    pub fn resolved(&self) -> Option<ResolvedWorkingResource> {
        self.resolved
            .lock()
            .expect("working-resource state lock is available")
            .clone()
    }

    pub fn releases(&self) -> usize {
        self.releases.load(Ordering::SeqCst)
    }
}

pub struct FakeWorkingResourceService {
    path: String,
    state: Arc<WorkingResourceState>,
}

impl FakeWorkingResourceService {
    pub fn new(path: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            path: path.into(),
            state: Arc::new(WorkingResourceState::default()),
        })
    }

    pub fn local() -> Arc<Self> {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/qwen-code-v0.19.11");
        Self::new(path.to_string_lossy().into_owned())
    }

    pub fn state(&self) -> Arc<WorkingResourceState> {
        Arc::clone(&self.state)
    }
}

impl WorkingResourceService for FakeWorkingResourceService {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let path = self.path.clone();
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            *state
                .resolved
                .lock()
                .expect("working-resource state lock is available") =
                Some(ResolvedWorkingResource {
                    reference: reference.as_host_value().to_owned(),
                    access,
                    representation,
                    path: path.clone(),
                });
            Ok(
                ResourceLease::consumer_owned(scope, reference, access, representation)
                    .with_filesystem(
                        MaterializedResourceRef::new(path)
                            .expect("fixture working-resource path is non-empty"),
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
        Box::pin(async {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "fixture.qwen.temporary_resource_unavailable",
                "Qwen test resources do not create temporary workspaces",
            )))
        })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        self.state.releases.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { CleanupOutcome::Clean })
    }
}
