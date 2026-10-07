use std::path::{Component, Path};
use std::sync::Arc;
use swallowtail_core::{ResourceAccess, ResourceRepresentation, SafeDiagnostic};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, ResourceLease, RuntimeFailure, ScopeId, SessionAccessPolicy,
    WorkingResourceRef, WorkingResourceService, validate_session_resource_lease,
};

/// Host-resolved read-only working resource held for one Qwen child lifetime.
pub(crate) struct QwenWorkingResourceLease {
    service: Arc<dyn WorkingResourceService>,
    lease: ResourceLease,
}

impl QwenWorkingResourceLease {
    pub(crate) async fn resolve(
        services: &HostServices,
        scope: ScopeId,
        reference: WorkingResourceRef,
    ) -> Result<Self, RuntimeFailure> {
        let service = services.working_resource().cloned().ok_or_else(|| {
            failure(
                "swallowtail.qwen.headless.host_service_missing",
                "Qwen working-resource service is unavailable",
            )
        })?;
        let lease = service
            .resolve(
                scope,
                reference.clone(),
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            )
            .await?;
        if let Err(error) = validate_session_resource_lease(
            &SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
            &reference,
            &lease,
        ) {
            let _ = service.release(lease).await;
            return Err(error);
        }
        let Some(path) = lease.filesystem() else {
            let _ = service.release(lease).await;
            return Err(working_resource_rejected());
        };
        let canonical = match std::fs::canonicalize(path.as_driver_value()) {
            Ok(path) => path,
            Err(_) => {
                let _ = service.release(lease).await;
                return Err(working_resource_rejected());
            }
        };
        if is_qwen_ssh_workspace(&canonical) {
            let _ = service.release(lease).await;
            return Err(failure(
                "swallowtail.qwen.headless.ssh_workspace_rejected",
                "Qwen headless does not select provider-managed SSH workspaces",
            ));
        }
        Ok(Self { service, lease })
    }

    pub(crate) async fn release(self) -> CleanupOutcome {
        self.service.release(self.lease).await
    }
}

pub(crate) fn combine_cleanup(
    primary: CleanupOutcome,
    secondary: CleanupOutcome,
) -> CleanupOutcome {
    match (primary, secondary) {
        (CleanupOutcome::Failed(diagnostic), _) | (_, CleanupOutcome::Failed(diagnostic)) => {
            CleanupOutcome::Failed(diagnostic)
        }
        (CleanupOutcome::Degraded(diagnostic), _) | (_, CleanupOutcome::Degraded(diagnostic)) => {
            CleanupOutcome::Degraded(diagnostic)
        }
        (CleanupOutcome::NotApplicable, outcome) | (outcome, CleanupOutcome::NotApplicable) => {
            outcome
        }
        _ => CleanupOutcome::Clean,
    }
}

fn is_qwen_ssh_workspace(path: &Path) -> bool {
    let mut components = path.components().rev();
    let Some(Component::Normal(workspace)) = components.next() else {
        return false;
    };
    let Some(Component::Normal(connection_hash)) = components.next() else {
        return false;
    };
    let Some(Component::Normal(root)) = components.next() else {
        return false;
    };
    workspace
        .to_string_lossy()
        .eq_ignore_ascii_case("workspace")
        && root
            .to_string_lossy()
            .eq_ignore_ascii_case("ssh-workspaces")
        && connection_hash.to_string_lossy().len() == 64
        && connection_hash
            .to_string_lossy()
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
}

fn working_resource_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.qwen.headless.working_resource_rejected",
        "Qwen headless requires a resolvable local filesystem working resource",
    )
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
mod tests {
    use super::is_qwen_ssh_workspace;
    use std::path::Path;

    #[test]
    fn detects_only_the_reserved_qwen_workspace_shape() {
        assert!(is_qwen_ssh_workspace(Path::new(
            "/custom/qwen-home/ssh-workspaces/0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef/workspace"
        )));
        assert!(is_qwen_ssh_workspace(Path::new(
            "/custom/qwen-home/SSH-WORKSPACES/0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF/WORKSPACE"
        )));
        assert!(!is_qwen_ssh_workspace(Path::new(
            "/project/ssh-workspaces/0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef/workspace/src"
        )));
        assert!(!is_qwen_ssh_workspace(Path::new(
            "/project/ssh-workspaces/not-a-connection/workspace"
        )));
        assert!(!is_qwen_ssh_workspace(Path::new(
            "/project/ssh-workspaces/0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        )));
    }
}
