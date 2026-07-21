use crate::host::{LocalProcessHost, LocalProcessHostBuilder};
use crate::output::failure;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use swallowtail_core::{ExecutionHostId, ModelArtifactBinding, ModelArtifactRef};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, MaterializedModelArtifactRef, ModelArtifactLease,
    ModelArtifactService, RuntimeFailure, ScopeId,
};

pub(crate) struct LocalModelArtifactApproval {
    binding: ModelArtifactBinding,
    path: PathBuf,
}

impl LocalModelArtifactApproval {
    pub(crate) const fn new(binding: ModelArtifactBinding, path: PathBuf) -> Self {
        Self { binding, path }
    }
}

impl LocalProcessHostBuilder {
    /// Binds host-scoped services to one registered execution-host identity.
    #[must_use]
    pub fn bind_execution_host(mut self, execution_host_id: ExecutionHostId) -> Self {
        self.execution_host_id = Some(execution_host_id);
        self
    }

    /// Approves one exact, pre-existing model artifact without transferring ownership.
    #[must_use]
    pub fn approve_model_artifact(
        mut self,
        binding: ModelArtifactBinding,
        path: impl Into<PathBuf>,
    ) -> Self {
        self.approvals.model_artifacts.insert(
            binding.reference().clone(),
            LocalModelArtifactApproval::new(binding, path.into()),
        );
        self
    }
}

#[derive(Default)]
pub(crate) struct LocalModelArtifactLeaseState {
    issued: Mutex<HashMap<(ScopeId, ExecutionHostId, ModelArtifactRef), usize>>,
}

impl LocalModelArtifactLeaseState {
    fn issue(&self, lease: &ModelArtifactLease) {
        let mut issued = self
            .issued
            .lock()
            .expect("local model-artifact lease lock poisoned");
        *issued
            .entry((
                lease.scope().clone(),
                lease.execution_host_id().clone(),
                lease.binding().reference().clone(),
            ))
            .or_default() += 1;
    }

    fn release(&self, lease: &ModelArtifactLease) -> bool {
        let mut issued = self
            .issued
            .lock()
            .expect("local model-artifact lease lock poisoned");
        let key = (
            lease.scope().clone(),
            lease.execution_host_id().clone(),
            lease.binding().reference().clone(),
        );
        let Some(count) = issued.get_mut(&key) else {
            return false;
        };
        *count -= 1;
        if *count == 0 {
            issued.remove(&key);
        }
        true
    }
}

impl LocalProcessHost {
    fn acquire_model_artifact(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        binding: ModelArtifactBinding,
    ) -> Result<ModelArtifactLease, RuntimeFailure> {
        self.require_bound_execution_host(&execution_host_id)?;
        let approved = self
            .approvals
            .model_artifacts
            .get(binding.reference())
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_model_artifact.not_approved",
                    "Local model artifact reference is not approved",
                )
            })?;
        if approved.binding != binding {
            return Err(failure(
                "swallowtail.local_model_artifact.descriptor_mismatch",
                "Local model artifact descriptor does not match its approval",
            ));
        }
        verify_regular_file(&approved.path)?;
        verify_sha256(
            &approved.path,
            approved.binding.descriptor().digest().as_str(),
        )?;
        let path = approved.path.to_str().ok_or_else(|| {
            failure(
                "swallowtail.local_model_artifact.path_unavailable",
                "Local model artifact path cannot be represented for the driver",
            )
        })?;
        let materialized = MaterializedModelArtifactRef::new(path).map_err(|_| {
            failure(
                "swallowtail.local_model_artifact.path_unavailable",
                "Local model artifact path cannot be represented for the driver",
            )
        })?;
        let lease = ModelArtifactLease::read_only(scope, execution_host_id, binding, materialized);
        self.model_artifact_leases.issue(&lease);
        Ok(lease)
    }

    pub(crate) fn require_bound_execution_host(
        &self,
        requested: &ExecutionHostId,
    ) -> Result<(), RuntimeFailure> {
        match &self.execution_host_id {
            Some(bound) if bound == requested => Ok(()),
            Some(_) => Err(failure(
                "swallowtail.local_host.execution_host_mismatch",
                "Local service request targets a different execution host",
            )),
            None => Err(failure(
                "swallowtail.local_host.execution_host_unbound",
                "Local host-scoped service has no execution-host binding",
            )),
        }
    }
}

impl ModelArtifactService for LocalProcessHost {
    fn acquire(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        binding: ModelArtifactBinding,
    ) -> BoxFuture<'static, Result<ModelArtifactLease, RuntimeFailure>> {
        let result = self.acquire_model_artifact(scope, execution_host_id, binding);
        Box::pin(async move { result })
    }

    fn release(&self, lease: ModelArtifactLease) -> BoxFuture<'static, CleanupOutcome> {
        let owned = self
            .approvals
            .model_artifacts
            .get(lease.binding().reference())
            .is_some_and(|approved| approved.binding == *lease.binding())
            && self
                .execution_host_id
                .as_ref()
                .is_some_and(|bound| bound == lease.execution_host_id())
            && self.model_artifact_leases.release(&lease);
        let outcome = if owned {
            // Approved artifacts predate the lease. The host drops authority but never the file.
            CleanupOutcome::NotApplicable
        } else {
            CleanupOutcome::Failed(
                failure(
                    "swallowtail.local_model_artifact.lease_not_owned",
                    "Model artifact lease is not owned by this local host",
                )
                .diagnostic()
                .clone(),
            )
        };
        Box::pin(async move { outcome })
    }
}

fn verify_regular_file(path: &Path) -> Result<(), RuntimeFailure> {
    let metadata = path.symlink_metadata().map_err(|_| {
        failure(
            "swallowtail.local_model_artifact.unavailable",
            "Local model artifact is unavailable",
        )
    })?;
    if !metadata.file_type().is_file() {
        return Err(failure(
            "swallowtail.local_model_artifact.not_regular_file",
            "Local model artifact must be a regular file",
        ));
    }
    Ok(())
}

fn verify_sha256(path: &Path, expected: &str) -> Result<(), RuntimeFailure> {
    let expected = expected.strip_prefix("sha256:").ok_or_else(|| {
        failure(
            "swallowtail.local_model_artifact.digest_unsupported",
            "Local model artifact digest algorithm is unsupported",
        )
    })?;
    if expected.len() != 64
        || !expected
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(failure(
            "swallowtail.local_model_artifact.digest_unsupported",
            "Local model artifact digest format is unsupported",
        ));
    }
    let mut file = File::open(path).map_err(|_| {
        failure(
            "swallowtail.local_model_artifact.unavailable",
            "Local model artifact is unavailable",
        )
    })?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer).map_err(|_| {
            failure(
                "swallowtail.local_model_artifact.read_failed",
                "Local model artifact could not be verified",
            )
        })?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    let actual = format!("{:x}", digest.finalize());
    if actual != expected {
        return Err(failure(
            "swallowtail.local_model_artifact.digest_mismatch",
            "Local model artifact digest does not match its approval",
        ));
    }
    Ok(())
}
