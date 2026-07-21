use crate::{BoxFuture, CleanupOutcome, MaterializedModelArtifactRef, RuntimeFailure, ScopeId};
use swallowtail_core::{ExecutionHostId, ModelArtifactBinding};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModelArtifactAccess {
    ReadOnly,
}

/// Read-only driver access to one host-resolved model artifact.
#[derive(Debug, Eq, PartialEq)]
pub struct ModelArtifactLease {
    scope: ScopeId,
    execution_host_id: ExecutionHostId,
    binding: ModelArtifactBinding,
    access: ModelArtifactAccess,
    materialized: MaterializedModelArtifactRef,
}

impl ModelArtifactLease {
    #[must_use]
    pub const fn read_only(
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        binding: ModelArtifactBinding,
        materialized: MaterializedModelArtifactRef,
    ) -> Self {
        Self {
            scope,
            execution_host_id,
            binding,
            access: ModelArtifactAccess::ReadOnly,
            materialized,
        }
    }

    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn binding(&self) -> &ModelArtifactBinding {
        &self.binding
    }

    #[must_use]
    pub const fn access(&self) -> ModelArtifactAccess {
        self.access
    }

    /// Passes the host-authorized materialization to a serving driver.
    #[must_use]
    pub const fn materialized(&self) -> &MaterializedModelArtifactRef {
        &self.materialized
    }
}

pub trait ModelArtifactService: Send + Sync {
    fn acquire(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        binding: ModelArtifactBinding,
    ) -> BoxFuture<'static, Result<ModelArtifactLease, RuntimeFailure>>;

    fn release(&self, lease: ModelArtifactLease) -> BoxFuture<'static, CleanupOutcome>;
}

#[cfg(test)]
mod tests {
    use super::{ModelArtifactAccess, ModelArtifactLease};
    use crate::{MaterializedModelArtifactRef, ScopeId};
    use swallowtail_core::{
        ExecutionHostId, ModelArtifactBinding, ModelArtifactDescriptor, ModelArtifactDigest,
        ModelArtifactFormat, ModelArtifactId, ModelArtifactRef, ModelArtifactRevision,
    };

    #[test]
    fn lease_binds_read_only_scope_host_and_redacted_materialization() {
        let raw = "/private/models/fixture.gguf";
        let lease = ModelArtifactLease::read_only(
            ScopeId::new("serving-scope").expect("scope is valid"),
            ExecutionHostId::new("host.local").expect("host is valid"),
            artifact_binding("artifact-ref"),
            MaterializedModelArtifactRef::new(raw).expect("materialization is valid"),
        );

        assert_eq!(lease.access(), ModelArtifactAccess::ReadOnly);
        assert_eq!(
            lease.binding().descriptor().digest().as_str(),
            "sha256:fixture"
        );
        assert_eq!(lease.materialized().as_driver_value(), raw);
        assert!(!format!("{lease:?}").contains(raw));
    }

    fn artifact_binding(reference: &str) -> ModelArtifactBinding {
        ModelArtifactBinding::new(
            ModelArtifactRef::new(reference).expect("reference is valid"),
            ModelArtifactDescriptor::new(
                ModelArtifactId::new("artifact-1").expect("id is valid"),
                ModelArtifactFormat::new("gguf").expect("format is valid"),
                ModelArtifactRevision::new("revision-1").expect("revision is valid"),
                ModelArtifactDigest::new("sha256:fixture").expect("digest is valid"),
            ),
        )
    }
}
