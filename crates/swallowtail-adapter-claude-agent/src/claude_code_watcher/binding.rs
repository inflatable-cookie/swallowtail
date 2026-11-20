use super::material::{
    MCP_CONFIG_LOCATOR, SETTINGS_LOCATOR, SKILL_LOCATOR, mcp_config, settings, skill_markdown,
};
use crate::failure::failure;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::{Arc, Mutex};
use swallowtail_core::{ResourceAccess, ResourceRepresentation, WatcherCleanupCause};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, DebugObservation, DebugObservationKind, HostServices, ResourceLease,
    RuntimeFailure, RuntimeTurnId, ScopeId, ScopedTaskService, TaskReapReservation,
    WatcherBridgeCompletionState, WatcherBridgeHostService, WatcherBridgeLease,
    WatcherBridgeOpenRequest, WorkingResourceIoService, WorkingResourceLocator,
    WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
};

const MATERIAL_BYTES: usize = 16_384;
const ROUTE: &str = "claude-code.headless";

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
///
/// Both the completion gate and cleanup are awaited by the pump task, and each
/// host callback runs on its own host thread, so no host callback blocks or
/// re-enters the executor that polls the pump. The only other thread this type
/// can start is the detached last-resort cleanup described on its `Drop`
/// implementation.
pub(crate) struct WatcherBinding {
    bridge: Arc<dyn WatcherBridgeHostService>,
    resources: Arc<dyn WorkingResourceService>,
    services: HostServices,
    scope: ScopeId,
    /// Host authority, reserved before the first acquisition, to start and
    /// transfer one cancellation cleanup task without waiting on it.
    reservation: Option<Box<dyn TaskReapReservation>>,
    /// The lease lives in a shared slot rather than in this struct, because a
    /// host callback can panic on its own thread. The slot keeps the lease
    /// reachable for cleanup on every path.
    lease: Arc<Mutex<Option<WatcherBridgeLease>>>,
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

    /// Asks the host bridge whether this turn may complete.
    ///
    /// The watcher bridge host answers through a nested `block_on` inside its
    /// own callback, so that callback runs on a dedicated host thread and this
    /// future only awaits the reply. The lease stays in its slot on every path,
    /// including a panicking host callback, so cleanup can still close it.
    ///
    /// The `&mut` receiver keeps the awaited future `Send` without requiring
    /// the bridge lease to be `Sync` for the whole binding.
    pub(crate) async fn completion_gate(
        &mut self,
    ) -> Result<WatcherBridgeCompletionState, RuntimeFailure> {
        let slot = Arc::clone(&self.lease);
        let bridge = Arc::clone(&self.bridge);
        host_call(move || {
            let held = slot
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            match held.as_ref() {
                Some(lease) => futures_executor::block_on(bridge.completion_gate(lease)),
                None => Err(closed_binding()),
            }
        })
        .await?
    }

    /// Closes the bridge lease and releases the private material, in that order.
    pub(crate) async fn close(mut self, cause: WatcherCleanupCause) -> CleanupOutcome {
        let lease = self.take_lease();
        let resource = self.resource.take();
        // The explicit path needs no reap authority; an unused grant releases
        // its reservation when it is dropped here.
        self.reservation = None;
        cleanup(
            Arc::clone(&self.bridge),
            Arc::clone(&self.resources),
            lease,
            resource,
            cause,
        )
        .await
    }

    /// Takes the lease out of its slot for cleanup.
    ///
    /// A panicking host callback can poison the slot while it holds the guard,
    /// and that must not stop cleanup from reaching the lease it still owns, so
    /// poisoning is ignored here.
    fn take_lease(&self) -> Option<WatcherBridgeLease> {
        self.lease
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
    }

    /// Hands one cleanup job to the host's reap lane without waiting for it.
    ///
    /// The reservation was taken before the first acquisition, so the host has
    /// already committed the lane. `Err` returns the still-owned cleanup future,
    /// so a host without a reap lane loses no cleanup.
    fn hand_off(&mut self, cleanup: BoxFuture<'static, ()>) -> Result<(), BoxFuture<'static, ()>> {
        let Some(service) = self.services.task().cloned() else {
            return Err(cleanup);
        };
        let Some(reservation) = self.reservation.take() else {
            return Err(cleanup);
        };
        match service.spawn_reapable(reservation, cleanup) {
            Ok(task) => {
                let mut task = Some(task);
                if ScopedTaskService::relinquish(service.as_ref(), &self.scope, &mut task).is_err()
                {
                    self.observe_abandoned("watcher_cleanup=relinquish_refused");
                }
                Ok(())
            }
            Err(_) => {
                self.observe_abandoned("watcher_cleanup=spawn_refused");
                Ok(())
            }
        }
    }

    fn observe_abandoned(&self, detail: &'static str) {
        self.services.emit_debug_observation(
            &DebugObservation::new(DebugObservationKind::Cleanup, detail)
                .with_scope_id(self.scope.clone())
                .with_route(ROUTE)
                .with_stage("claude-code.headless.watcher.cancel"),
        );
    }

    #[cfg(test)]
    fn for_test(
        services: HostServices,
        bridge: Arc<dyn WatcherBridgeHostService>,
        resources: Arc<dyn WorkingResourceService>,
        lease: WatcherBridgeLease,
        resource: ResourceLease,
    ) -> Self {
        let scope = lease.scope().clone();
        let reservation = services
            .task()
            .and_then(|task| task.reserve_reap(scope.clone()).ok());
        Self {
            bridge,
            resources,
            services,
            scope,
            reservation,
            lease: Arc::new(Mutex::new(Some(lease))),
            resource: Some(resource),
            files: WatcherCommandFiles {
                mcp_config: "mcp.json".to_owned(),
                settings: "settings.json".to_owned(),
                add_dir: "add-dir".to_owned(),
            },
        }
    }
}

/// Dropping an unclosed binding still releases what it holds, and never blocks
/// the dropping thread.
///
/// The pump closes its binding on every terminal path, so this is the
/// cancellation fallback: the pump future was dropped before it could run its
/// own cleanup. The work is handed to the host's reap lane, where it runs to
/// completion without being joined here. Hosts that cannot pre-admit a reap
/// (a fixture or standalone task service) get one detached host thread running
/// the same cleanup, which is the only remaining way to release the leases
/// without a synchronous join.
impl Drop for WatcherBinding {
    fn drop(&mut self) {
        let lease = self.take_lease();
        let resource = self.resource.take();
        if lease.is_none() && resource.is_none() {
            return;
        }
        let bridge = Arc::clone(&self.bridge);
        let resources = Arc::clone(&self.resources);
        let cleanup: BoxFuture<'static, ()> = Box::pin(async move {
            let _ = cleanup(
                bridge,
                resources,
                lease,
                resource,
                WatcherCleanupCause::Failed,
            )
            .await;
        });
        let cleanup = match self.hand_off(cleanup) {
            Ok(()) => return,
            Err(cleanup) => cleanup,
        };
        // No reap lane: one detached host thread runs the same cleanup. It is
        // never joined, so the dropping thread cannot block on host work.
        self.observe_abandoned("watcher_cleanup=detached_thread");
        let _ = std::thread::Builder::new()
            .name("swallowtail-claude-watcher-drop".to_owned())
            .spawn(move || futures_executor::block_on(cleanup));
    }
}

/// Closes the bridge lease, then releases the private material, off the polling
/// thread.
///
/// The local watcher bridge performs the joined teardown synchronously when
/// `close` is called, and its teardown drives watcher work through a nested
/// `block_on`. The executor refuses a nested `block_on` on the same thread, so
/// the call runs on its own host thread; the caller awaits the reply instead of
/// joining anything.
fn cleanup(
    bridge: Arc<dyn WatcherBridgeHostService>,
    resources: Arc<dyn WorkingResourceService>,
    lease: Option<WatcherBridgeLease>,
    resource: Option<ResourceLease>,
    cause: WatcherCleanupCause,
) -> BoxFuture<'static, CleanupOutcome> {
    Box::pin(async move {
        host_call(move || {
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
        .await
        .unwrap_or_else(|error| CleanupOutcome::Failed(error.diagnostic().clone()))
    })
}

/// Runs one host callback on a dedicated thread and awaits its reply.
///
/// The reply channel is the only wait: this future never joins the thread it
/// started, so a cancelled caller cannot block on host teardown.
async fn host_call<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T, RuntimeFailure> {
    let (sender, receiver) = futures_channel::oneshot::channel();
    std::thread::Builder::new()
        .name("swallowtail-claude-watcher-host".to_owned())
        .spawn(move || {
            let _ = sender.send(work());
        })
        .map_err(|_| host_worker_failure())?;
    receiver.await.map_err(|_| host_worker_failure())
}

fn host_worker_failure() -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.headless.watcher_host_worker_failed",
        "Claude Code watcher host worker failed",
    )
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
    // Reserved before the bridge lease exists, so cancelling the open later
    // cannot leave a lease the host refuses to reap.
    let reservation = services
        .task()
        .and_then(|task| task.reserve_reap(scope.clone()).ok());
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
            services: services.clone(),
            scope: lease.scope().clone(),
            reservation,
            lease: Arc::new(Mutex::new(Some(lease))),
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
    use futures_executor::block_on;
    use std::future::{Future, poll_fn};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::Poll;
    use std::time::{Duration, Instant};
    use swallowtail_core::{
        ExecutionHostId, ResourceAccess, ResourceRepresentation, SafeDiagnostic,
        WatcherCleanupCause,
    };
    use swallowtail_runtime::{
        BoxFuture, CleanupOutcome, HostServices, JoinedTask, ResourceLease, RuntimeFailure,
        RuntimeTurnId, ScopeId, ScopedTaskService, TaskReapReservation, TaskRelinquishOutcome,
        WatcherBridgeAdmission, WatcherBridgeBearer, WatcherBridgeCompletionState,
        WatcherBridgeEndpoint, WatcherBridgeGeneration, WatcherBridgeHostService,
        WatcherBridgeLease, WatcherBridgeOpenRequest, WorkingResourceRef, WorkingResourceService,
    };

    fn unused(what: &str) -> RuntimeFailure {
        RuntimeFailure::new(SafeDiagnostic::new(
            "fixture.unused",
            format!("{what} is unused"),
        ))
    }

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
            Box::pin(async { Err(unused("open")) })
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

    #[derive(Default)]
    struct PanickingBridge {
        close_calls: AtomicUsize,
    }

    impl WatcherBridgeHostService for PanickingBridge {
        fn open(
            &self,
            _request: WatcherBridgeOpenRequest,
        ) -> BoxFuture<'_, Result<WatcherBridgeLease, RuntimeFailure>> {
            Box::pin(async { Err(unused("open")) })
        }

        fn completion_gate(
            &self,
            _lease: &WatcherBridgeLease,
        ) -> BoxFuture<'_, Result<WatcherBridgeCompletionState, RuntimeFailure>> {
            Box::pin(async { panic!("fixture completion gate panicked") })
        }

        fn close(
            &self,
            _lease: WatcherBridgeLease,
            _cause: WatcherCleanupCause,
        ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
            self.close_calls.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(CleanupOutcome::Clean) })
        }
    }

    /// One bridge whose close only finishes when the test releases it.
    struct GatedBridge {
        close_calls: Arc<AtomicUsize>,
        gate: Mutex<Option<futures_channel::oneshot::Receiver<()>>>,
    }

    impl GatedBridge {
        fn new(close_calls: Arc<AtomicUsize>) -> (Self, futures_channel::oneshot::Sender<()>) {
            let (sender, receiver) = futures_channel::oneshot::channel();
            (
                Self {
                    close_calls,
                    gate: Mutex::new(Some(receiver)),
                },
                sender,
            )
        }
    }

    impl WatcherBridgeHostService for GatedBridge {
        fn open(
            &self,
            _request: WatcherBridgeOpenRequest,
        ) -> BoxFuture<'_, Result<WatcherBridgeLease, RuntimeFailure>> {
            Box::pin(async { Err(unused("open")) })
        }

        fn completion_gate(
            &self,
            _lease: &WatcherBridgeLease,
        ) -> BoxFuture<'_, Result<WatcherBridgeCompletionState, RuntimeFailure>> {
            Box::pin(async { Err(unused("completion gate")) })
        }

        fn close(
            &self,
            _lease: WatcherBridgeLease,
            _cause: WatcherCleanupCause,
        ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
            self.close_calls.fetch_add(1, Ordering::SeqCst);
            let gate = self.gate.lock().expect("gate lock").take();
            Box::pin(async move {
                if let Some(gate) = gate {
                    let _ = gate.await;
                }
                Ok(CleanupOutcome::Clean)
            })
        }
    }

    struct PendingOnceResources {
        pending_release: AtomicBool,
        release_calls: Arc<AtomicUsize>,
    }

    impl PendingOnceResources {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                pending_release: AtomicBool::new(true),
                release_calls: Arc::new(AtomicUsize::new(0)),
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
            Box::pin(async { Err(unused("resolve")) })
        }

        fn create_temporary(
            &self,
            _scope: ScopeId,
            _access: ResourceAccess,
            _representation: ResourceRepresentation,
        ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
            Box::pin(async { Err(unused("create_temporary")) })
        }

        fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
            self.release_calls.fetch_add(1, Ordering::SeqCst);
            let pending = self.pending_release.swap(false, Ordering::SeqCst);
            Box::pin(pending_once(pending, CleanupOutcome::Clean))
        }
    }

    struct FixtureReservation;

    impl std::fmt::Debug for FixtureReservation {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("FixtureReservation")
        }
    }

    impl TaskReapReservation for FixtureReservation {
        fn into_any(self: Box<Self>) -> Box<dyn std::any::Any + Send> {
            self
        }
    }

    struct FixtureJoinedTask {
        handed_back: AtomicBool,
    }

    impl JoinedTask for FixtureJoinedTask {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn is_finished(&self) -> bool {
            self.handed_back.load(Ordering::SeqCst)
        }

        fn relinquish_to_host(
            &mut self,
            _execution_host_id: &ExecutionHostId,
            _scope: &ScopeId,
        ) -> Result<(), RuntimeFailure> {
            self.handed_back.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    /// Host reap lane fixture: records the handoff and keeps the cleanup future
    /// so a test can prove it was not run on the dropping thread.
    #[derive(Default)]
    struct RecordingTasks {
        reserved: AtomicUsize,
        spawned: AtomicUsize,
        relinquished: AtomicUsize,
        handed: Mutex<Option<BoxFuture<'static, ()>>>,
    }

    impl RecordingTasks {
        fn services(self: &Arc<Self>) -> HostServices {
            HostServices::new(ExecutionHostId::new("fixture.host").expect("host"))
                .with_task(self.clone())
        }
    }

    impl ScopedTaskService for RecordingTasks {
        fn spawn(
            &self,
            _scope: ScopeId,
            _task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            Err(unused("spawn"))
        }

        fn reserve_reap(
            &self,
            _scope: ScopeId,
        ) -> Result<Box<dyn TaskReapReservation>, RuntimeFailure> {
            self.reserved.fetch_add(1, Ordering::SeqCst);
            Ok(Box::new(FixtureReservation))
        }

        fn spawn_reapable(
            &self,
            _reservation: Box<dyn TaskReapReservation>,
            task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            self.spawned.fetch_add(1, Ordering::SeqCst);
            *self.handed.lock().expect("handoff lock") = Some(task);
            Ok(Box::new(FixtureJoinedTask {
                handed_back: AtomicBool::new(false),
            }))
        }

        fn relinquish(
            &self,
            scope: &ScopeId,
            task: &mut Option<Box<dyn JoinedTask>>,
        ) -> Result<TaskRelinquishOutcome, RuntimeFailure> {
            let mut held = task.take().ok_or_else(|| unused("relinquish"))?;
            held.relinquish_to_host(&ExecutionHostId::new("fixture.host").expect("host"), scope)?;
            self.relinquished.fetch_add(1, Ordering::SeqCst);
            Ok(TaskRelinquishOutcome::AcceptedForReap)
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

    fn fixture_state() -> WatcherBridgeCompletionState {
        WatcherBridgeCompletionState::new(WatcherBridgeAdmission::Open, Vec::new())
    }

    fn host_without_tasks() -> HostServices {
        HostServices::new(ExecutionHostId::new("fixture.host").expect("host"))
    }

    fn wait_for(condition: impl Fn() -> bool) {
        let deadline = Instant::now() + Duration::from_secs(10);
        while !condition() {
            assert!(Instant::now() < deadline, "fixture condition timed out");
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    #[test]
    fn completion_gate_awaits_a_pending_host_future() {
        let state = fixture_state();
        let bridge = PendingOnceBridge::new(state.clone());
        let resources = PendingOnceResources::new();
        let mut binding = WatcherBinding::for_test(
            host_without_tasks(),
            bridge,
            resources,
            fixture_lease(),
            fixture_resource(),
        );
        let observed = block_on(binding.completion_gate()).expect("gate completes");
        assert_eq!(observed, state);
    }

    #[test]
    fn a_panicking_completion_gate_keeps_the_lease_for_cleanup() {
        let bridge = Arc::new(PanickingBridge::default());
        let resources = PendingOnceResources::new();
        let mut binding = WatcherBinding::for_test(
            host_without_tasks(),
            bridge.clone(),
            resources,
            fixture_lease(),
            fixture_resource(),
        );
        let error = block_on(binding.completion_gate())
            .expect_err("the panicking host callback fails the gate");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.claude_code.headless.watcher_host_worker_failed"
        );
        assert_eq!(
            block_on(binding.close(WatcherCleanupCause::Failed)),
            CleanupOutcome::Clean,
            "the lease survives the panic and is still closable"
        );
        assert_eq!(bridge.close_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn close_awaits_pending_bridge_and_resource_futures_and_keeps_cleanup_clean() {
        let bridge = PendingOnceBridge::new(WatcherBridgeCompletionState::new(
            WatcherBridgeAdmission::Frozen,
            Vec::new(),
        ));
        let resources = PendingOnceResources::new();
        let binding = WatcherBinding::for_test(
            host_without_tasks(),
            bridge.clone(),
            resources.clone(),
            fixture_lease(),
            fixture_resource(),
        );
        let outcome = block_on(binding.close(WatcherCleanupCause::Cancelled));
        assert_eq!(outcome, CleanupOutcome::Clean);
        assert_eq!(bridge.close_calls.load(Ordering::SeqCst), 1);
        assert_eq!(resources.release_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn drop_hands_cleanup_to_the_host_reap_lane_without_running_it_inline() {
        let bridge = PendingOnceBridge::new(fixture_state());
        let resources = PendingOnceResources::new();
        let tasks = Arc::new(RecordingTasks::default());
        let binding = WatcherBinding::for_test(
            tasks.services(),
            bridge.clone(),
            resources.clone(),
            fixture_lease(),
            fixture_resource(),
        );
        drop(binding);

        assert_eq!(tasks.reserved.load(Ordering::SeqCst), 1);
        assert_eq!(tasks.spawned.load(Ordering::SeqCst), 1);
        assert_eq!(tasks.relinquished.load(Ordering::SeqCst), 1);
        assert_eq!(
            bridge.close_calls.load(Ordering::SeqCst),
            0,
            "the handed-off cleanup has not run yet"
        );
        let cleanup = tasks
            .handed
            .lock()
            .expect("handoff lock")
            .take()
            .expect("the cleanup future was handed to the host");
        block_on(cleanup);
        assert_eq!(bridge.close_calls.load(Ordering::SeqCst), 1);
        assert_eq!(resources.release_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn drop_without_a_reap_lane_still_releases_on_a_detached_thread() {
        let close_calls = Arc::new(AtomicUsize::new(0));
        let (bridge, release) = GatedBridge::new(Arc::clone(&close_calls));
        let bridge = Arc::new(bridge);
        let resources = PendingOnceResources::new();
        let binding = WatcherBinding::for_test(
            host_without_tasks(),
            bridge.clone(),
            resources.clone(),
            fixture_lease(),
            fixture_resource(),
        );

        let (dropped, dropped_rx) = std::sync::mpsc::channel();
        let worker = std::thread::spawn(move || {
            drop(binding);
            let _ = dropped.send(());
        });
        assert!(
            dropped_rx.recv_timeout(Duration::from_secs(5)).is_ok(),
            "dropping an unclosed binding must not wait on host cleanup"
        );
        release.send(()).expect("the gated close is waiting");
        wait_for(|| resources.release_calls.load(Ordering::SeqCst) == 1);
        worker.join().expect("drop worker finishes");
        assert_eq!(close_calls.load(Ordering::SeqCst), 1);
    }
}
