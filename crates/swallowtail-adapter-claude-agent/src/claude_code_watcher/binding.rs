use super::material::{
    MCP_CONFIG_LOCATOR, SETTINGS_LOCATOR, SKILL_LOCATOR, mcp_config, settings, skill_markdown,
};
use crate::failure::failure;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Arc;
use swallowtail_core::{ResourceAccess, ResourceRepresentation, WatcherCleanupCause};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, ResourceLease, RuntimeFailure, RuntimeTurnId, ScopeId,
    WatcherBridgeCompletionState, WatcherBridgeHostService, WatcherBridgeLease,
    WatcherBridgeOpenRequest, WorkingResourceIoService, WorkingResourceLocator,
    WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
};

const MATERIAL_BYTES: usize = 16_384;

/// Driver-only file paths for one opted-in Claude Code watcher command.
pub(crate) struct WatcherCommandFiles {
    pub(crate) mcp_config: String,
    pub(crate) settings: String,
    pub(crate) add_dir: String,
}

impl std::fmt::Debug for WatcherCommandFiles {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WatcherCommandFiles")
            .field("mcp_config", &"<redacted>")
            .field("settings", &"<redacted>")
            .field("add_dir", &"<redacted>")
            .finish()
    }
}

/// Open bridge lease and private working-resource material for one run.
pub(crate) struct WatcherBinding {
    bridge: Arc<dyn WatcherBridgeHostService>,
    resources: Arc<dyn WorkingResourceService>,
    lease: Option<WatcherBridgeLease>,
    resource: Option<ResourceLease>,
    files: WatcherCommandFiles,
}

impl std::fmt::Debug for WatcherBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WatcherBinding")
            .field("lease", &self.lease)
            .field("files", &self.files)
            .finish_non_exhaustive()
    }
}

impl WatcherBinding {
    pub(crate) fn files(&self) -> &WatcherCommandFiles {
        &self.files
    }

    pub(crate) fn completion_gate(
        &mut self,
    ) -> Result<WatcherBridgeCompletionState, RuntimeFailure> {
        let lease = self.lease.take().ok_or_else(closed_binding)?;
        let bridge = Arc::clone(&self.bridge);
        let (result, lease) = host_thread(move || {
            let result = futures_executor::block_on(bridge.completion_gate(&lease));
            (result, lease)
        })?;
        self.lease = Some(lease);
        result
    }

    pub(crate) fn close(mut self, cause: WatcherCleanupCause) -> CleanupOutcome {
        let lease = self.lease.take();
        let resource = self.resource.take();
        let bridge = Arc::clone(&self.bridge);
        let resources = Arc::clone(&self.resources);
        host_thread(move || {
            let bridge = match lease {
                Some(lease) => futures_executor::block_on(bridge.close(lease, cause))
                    .unwrap_or_else(|error| CleanupOutcome::Failed(error.diagnostic().clone())),
                None => CleanupOutcome::NotApplicable,
            };
            let resource = match resource {
                Some(lease) => futures_executor::block_on(resources.release(lease)),
                None => CleanupOutcome::NotApplicable,
            };
            merge_cleanup(bridge, resource)
        })
        .unwrap_or_else(|error| CleanupOutcome::Failed(error.diagnostic().clone()))
    }

    #[cfg(test)]
    fn for_test(
        bridge: Arc<dyn WatcherBridgeHostService>,
        resources: Arc<dyn WorkingResourceService>,
        lease: WatcherBridgeLease,
        resource: ResourceLease,
    ) -> Self {
        Self {
            bridge,
            resources,
            lease: Some(lease),
            resource: Some(resource),
            files: WatcherCommandFiles {
                mcp_config: "mcp.json".to_owned(),
                settings: "settings.json".to_owned(),
                add_dir: "add-dir".to_owned(),
            },
        }
    }
}

impl Drop for WatcherBinding {
    fn drop(&mut self) {
        if self.lease.is_none() && self.resource.is_none() {
            return;
        }
        let lease = self.lease.take();
        let resource = self.resource.take();
        let bridge = Arc::clone(&self.bridge);
        let resources = Arc::clone(&self.resources);
        let _ = host_thread(move || {
            if let Some(lease) = lease {
                let _ = futures_executor::block_on(
                    bridge.close(lease, swallowtail_core::WatcherCleanupCause::Failed),
                );
            }
            if let Some(resource) = resource {
                let _ = futures_executor::block_on(resources.release(resource));
            }
        });
    }
}

pub(crate) async fn open_binding(
    services: &HostServices,
    scope: ScopeId,
    turn: RuntimeTurnId,
) -> Result<WatcherBinding, RuntimeFailure> {
    let bridge = services
        .watcher_bridge()
        .cloned()
        .ok_or_else(|| missing_service("watcher bridge"))?;
    let resources = services
        .working_resource()
        .cloned()
        .ok_or_else(|| missing_service("working resource"))?;
    let io = services
        .working_resource_io()
        .cloned()
        .ok_or_else(|| missing_service("working-resource I/O"))?;
    let lease = bridge
        .open(WatcherBridgeOpenRequest::new(scope.clone(), turn))
        .await?;
    let resource = match resources
        .create_temporary(
            scope,
            ResourceAccess::ReadWrite,
            ResourceRepresentation::Filesystem,
        )
        .await
    {
        Ok(resource) => resource,
        Err(error) => {
            let _ = bridge.close(lease, WatcherCleanupCause::Failed).await;
            return Err(error);
        }
    };
    let mcp = mcp_config(lease.endpoint().expose(), lease.bearer());
    let settings = settings();
    let skill = skill_markdown();
    match materialize(&io, &resource, &mcp, &settings, skill).await {
        Ok(files) => Ok(WatcherBinding {
            bridge,
            resources,
            lease: Some(lease),
            resource: Some(resource),
            files,
        }),
        Err(error) => {
            let _ = resources.release(resource).await;
            let _ = bridge.close(lease, WatcherCleanupCause::Failed).await;
            Err(error)
        }
    }
}

async fn materialize(
    io: &Arc<dyn WorkingResourceIoService>,
    resource: &ResourceLease,
    mcp: &str,
    settings: &str,
    skill: &str,
) -> Result<WatcherCommandFiles, RuntimeFailure> {
    let root = resource
        .filesystem()
        .ok_or_else(|| {
            failure(
                "swallowtail.claude_code.headless.watcher_material_missing",
                "Claude Code watcher private material has no filesystem path",
            )
        })?
        .as_driver_value()
        .to_owned();
    write_file(io, resource, MCP_CONFIG_LOCATOR, mcp).await?;
    write_file(io, resource, SETTINGS_LOCATOR, settings).await?;
    write_file(io, resource, SKILL_LOCATOR, skill).await?;
    Ok(WatcherCommandFiles {
        mcp_config: join_utf8(&root, MCP_CONFIG_LOCATOR)?,
        settings: join_utf8(&root, SETTINGS_LOCATOR)?,
        add_dir: root,
    })
}

async fn write_file(
    io: &Arc<dyn WorkingResourceIoService>,
    lease: &ResourceLease,
    locator: &str,
    content: &str,
) -> Result<(), RuntimeFailure> {
    let maximum = NonZeroUsize::new(MATERIAL_BYTES).expect("material bound is non-zero");
    let locator = WorkingResourceLocator::new(locator).map_err(|_| {
        failure(
            "swallowtail.claude_code.headless.watcher_material_invalid",
            "Claude Code watcher private material locator was invalid",
        )
    })?;
    let content = WorkingResourceText::new(content.to_owned(), maximum).map_err(|_| {
        failure(
            "swallowtail.claude_code.headless.watcher_material_invalid",
            "Claude Code watcher private material exceeded its bound",
        )
    })?;
    io.write_text(lease, WorkingResourceWriteRequest::new(locator, content))
        .await
}

fn join_utf8(root: &str, relative: &str) -> Result<String, RuntimeFailure> {
    Path::new(root)
        .join(relative)
        .into_os_string()
        .into_string()
        .map_err(|_| {
            failure(
                "swallowtail.claude_code.headless.watcher_material_missing",
                "Claude Code watcher private material path is not UTF-8",
            )
        })
}

fn host_thread<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T, RuntimeFailure> {
    std::thread::spawn(work).join().map_err(|_| {
        failure(
            "swallowtail.claude_code.headless.watcher_host_worker_failed",
            "Claude Code watcher host worker failed",
        )
    })
}

fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(diagnostic), _) | (_, CleanupOutcome::Failed(diagnostic)) => {
            CleanupOutcome::Failed(diagnostic)
        }
        (CleanupOutcome::Degraded(diagnostic), _) | (_, CleanupOutcome::Degraded(diagnostic)) => {
            CleanupOutcome::Degraded(diagnostic)
        }
        (CleanupOutcome::Clean, CleanupOutcome::Clean)
        | (CleanupOutcome::Clean, CleanupOutcome::NotApplicable)
        | (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}

fn missing_service(name: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.headless.host_service_missing",
        format!("Claude Code headless requires the preflight-bound {name} service"),
    )
}

fn closed_binding() -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.headless.watcher_binding_closed",
        "Claude Code watcher binding is no longer open",
    )
}

#[cfg(test)]
mod tests {
    use super::WatcherBinding;
    use std::future::{Future, poll_fn};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::task::Poll;
    use swallowtail_core::{
        ExecutionHostId, ResourceAccess, ResourceRepresentation, WatcherCleanupCause,
    };
    use swallowtail_runtime::{
        BoxFuture, CleanupOutcome, ResourceLease, RuntimeFailure, RuntimeTurnId, ScopeId,
        WatcherBridgeAdmission, WatcherBridgeBearer, WatcherBridgeCompletionState,
        WatcherBridgeEndpoint, WatcherBridgeGeneration, WatcherBridgeHostService,
        WatcherBridgeLease, WatcherBridgeOpenRequest, WorkingResourceRef, WorkingResourceService,
    };

    fn pending_once<T: Send + 'static>(yield_once: bool, value: T) -> impl Future<Output = T> {
        let mut pending = yield_once;
        let mut value = Some(value);
        poll_fn(move |context| {
            if pending {
                pending = false;
                context.waker().wake_by_ref();
                return Poll::Pending;
            }
            Poll::Ready(value.take().expect("pending-once is ready once"))
        })
    }

    struct PendingOnceBridge {
        pending_gate: AtomicBool,
        pending_close: AtomicBool,
        close_calls: AtomicUsize,
        state: WatcherBridgeCompletionState,
    }

    impl PendingOnceBridge {
        fn new(state: WatcherBridgeCompletionState) -> Arc<Self> {
            Arc::new(Self {
                pending_gate: AtomicBool::new(true),
                pending_close: AtomicBool::new(true),
                close_calls: AtomicUsize::new(0),
                state,
            })
        }
    }

    impl WatcherBridgeHostService for PendingOnceBridge {
        fn open(
            &self,
            _request: WatcherBridgeOpenRequest,
        ) -> BoxFuture<'_, Result<WatcherBridgeLease, RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.unused",
                    "open is unused",
                )))
            })
        }

        fn completion_gate(
            &self,
            _lease: &WatcherBridgeLease,
        ) -> BoxFuture<'_, Result<WatcherBridgeCompletionState, RuntimeFailure>> {
            let pending = self.pending_gate.swap(false, Ordering::SeqCst);
            Box::pin(pending_once(pending, Ok(self.state.clone())))
        }

        fn close(
            &self,
            _lease: WatcherBridgeLease,
            _cause: WatcherCleanupCause,
        ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
            self.close_calls.fetch_add(1, Ordering::SeqCst);
            let pending = self.pending_close.swap(false, Ordering::SeqCst);
            Box::pin(pending_once(pending, Ok(CleanupOutcome::Clean)))
        }
    }

    struct PendingOnceResources {
        pending_release: AtomicBool,
        release_calls: AtomicUsize,
    }

    impl PendingOnceResources {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                pending_release: AtomicBool::new(true),
                release_calls: AtomicUsize::new(0),
            })
        }
    }

    impl WorkingResourceService for PendingOnceResources {
        fn resolve(
            &self,
            _scope: ScopeId,
            _reference: WorkingResourceRef,
            _access: ResourceAccess,
            _representation: ResourceRepresentation,
        ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.unused",
                    "resolve is unused",
                )))
            })
        }

        fn create_temporary(
            &self,
            _scope: ScopeId,
            _access: ResourceAccess,
            _representation: ResourceRepresentation,
        ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.unused",
                    "create_temporary is unused",
                )))
            })
        }

        fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
            self.release_calls.fetch_add(1, Ordering::SeqCst);
            let pending = self.pending_release.swap(false, Ordering::SeqCst);
            Box::pin(pending_once(pending, CleanupOutcome::Clean))
        }
    }

    fn fixture_lease() -> WatcherBridgeLease {
        WatcherBridgeLease::new(
            ExecutionHostId::new("fixture.host").expect("host"),
            ScopeId::new("fixture.scope").expect("scope"),
            RuntimeTurnId::new("fixture.turn").expect("turn"),
            WatcherBridgeGeneration::initial(),
            WatcherBridgeEndpoint::new("http://127.0.0.1:1/mcp").expect("endpoint"),
            WatcherBridgeBearer::new("fixture-bearer").expect("bearer"),
        )
    }

    fn fixture_resource() -> ResourceLease {
        ResourceLease::operation_scoped(
            ScopeId::new("fixture.scope").expect("scope"),
            WorkingResourceRef::new("fixture.resource").expect("resource"),
            ResourceAccess::ReadWrite,
            ResourceRepresentation::Filesystem,
        )
    }

    #[test]
    fn completion_gate_runs_a_pending_host_future_to_ready() {
        let state = WatcherBridgeCompletionState::new(WatcherBridgeAdmission::Open, Vec::new());
        let bridge = PendingOnceBridge::new(state.clone());
        let resources = PendingOnceResources::new();
        let mut binding = WatcherBinding::for_test(
            bridge.clone(),
            resources,
            fixture_lease(),
            fixture_resource(),
        );
        let observed = binding.completion_gate().expect("gate completes");
        assert_eq!(observed, state);
    }

    #[test]
    fn close_runs_pending_bridge_and_resource_futures_and_keeps_cleanup_clean() {
        let bridge = PendingOnceBridge::new(WatcherBridgeCompletionState::new(
            WatcherBridgeAdmission::Frozen,
            Vec::new(),
        ));
        let resources = PendingOnceResources::new();
        let binding = WatcherBinding::for_test(
            bridge.clone(),
            resources.clone(),
            fixture_lease(),
            fixture_resource(),
        );
        let outcome = binding.close(WatcherCleanupCause::Cancelled);
        assert_eq!(outcome, CleanupOutcome::Clean);
        assert_eq!(bridge.close_calls.load(Ordering::SeqCst), 1);
        assert_eq!(resources.release_calls.load(Ordering::SeqCst), 1);
    }
}
