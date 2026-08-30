use super::material::{
    MCP_CONFIG_LOCATOR, SETTINGS_LOCATOR, SKILL_LOCATOR, mcp_config, settings, skill_markdown,
};
use crate::failure::failure;
use std::future::Future;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
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
            let result = poll_ready(bridge.completion_gate(&lease));
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
                Some(lease) => poll_ready(bridge.close(lease, cause))
                    .unwrap_or_else(|error| CleanupOutcome::Failed(error.diagnostic().clone())),
                None => CleanupOutcome::NotApplicable,
            };
            let resource = match resource {
                Some(lease) => poll_ready(resources.release(lease)),
                None => CleanupOutcome::NotApplicable,
            };
            merge_cleanup(bridge, resource)
        })
        .unwrap_or_else(|error| CleanupOutcome::Failed(error.diagnostic().clone()))
    }
}

impl Drop for WatcherBinding {
    fn drop(&mut self) {
        if let Some(lease) = self.lease.take() {
            let _ = std::thread::spawn(move || drop(lease)).join();
        }
        if let Some(resource) = self.resource.take() {
            let resources = Arc::clone(&self.resources);
            let _ = std::thread::spawn(move || {
                let _ = poll_ready(resources.release(resource));
            })
            .join();
        }
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

fn poll_ready<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("watcher host call was expected to be ready"),
    }
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
