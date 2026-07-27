use swallowtail_core::{
    AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    InterfaceVersionBinding,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, PreparedAccessEvidence, ProviderSessionManagementBinding,
    RequestId, ScopeId, WorkingResourceRef,
};

use super::{KimiLocalServerObservation, KimiLocalServerPreparedIntegration};
use crate::KimiAcpSessionImportAuthority;
use swallowtail_core::ConfiguredInstance;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerAttachedInput {
    pub(super) instance_id: ConfiguredInstanceId,
    pub(super) instance_revision: InstanceRevision,
    pub(super) execution_host_id: ExecutionHostId,
    pub(super) endpoint_target: InstanceTargetRef,
    pub(super) access_profile: AccessProfile,
    pub(super) access_evidence: PreparedAccessEvidence,
    pub(super) state_root: WorkingResourceRef,
    pub(super) executable_version: InterfaceVersionBinding,
}

impl KimiLocalServerAttachedInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
        state_root: WorkingResourceRef,
        executable_version: InterfaceVersionBinding,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            endpoint_target,
            access_profile,
            access_evidence,
            state_root,
            executable_version,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerOwnedInput {
    pub(super) attached: KimiLocalServerAttachedInput,
    pub(super) executable_target: InstanceTargetRef,
}

impl KimiLocalServerOwnedInput {
    #[must_use]
    pub const fn new(
        attached: KimiLocalServerAttachedInput,
        executable_target: InstanceTargetRef,
    ) -> Self {
        Self {
            attached,
            executable_target,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KimiLocalServerPreparationProbe {
    pub(super) scope_id: ScopeId,
    pub(super) deadline: Deadline,
    pub(super) cancellation: DiscoveryCancellation,
}

impl KimiLocalServerPreparationProbe {
    #[must_use]
    pub const fn new(
        scope_id: ScopeId,
        deadline: Deadline,
        cancellation: DiscoveryCancellation,
    ) -> Self {
        Self {
            scope_id,
            deadline,
            cancellation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerSessionManagementInput {
    pub(super) request_id: RequestId,
    pub(super) binding: ProviderSessionManagementBinding,
    pub(super) deadline: Option<Deadline>,
    pub(super) allow_unverified_newer: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerCatalogueInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
    allow_unverified_newer: bool,
}

impl KimiLocalServerCatalogueInput {
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
            allow_unverified_newer: false,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub const fn allow_unverified_newer(mut self) -> Self {
        self.allow_unverified_newer = true;
        self
    }

    pub(super) fn into_parts(self) -> (RequestId, Option<Deadline>, bool) {
        (self.request_id, self.deadline, self.allow_unverified_newer)
    }
}

/// Exact local-server route snapshot that must agree with the prepared target
/// selected for a cross-transport import.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerBindingImportTarget {
    pub(super) instance: ConfiguredInstance,
    pub(super) access_profile: AccessProfile,
    pub(super) access_evidence: PreparedAccessEvidence,
    pub(super) server: KimiLocalServerObservation,
    pub(super) state_root: WorkingResourceRef,
}

impl KimiLocalServerPreparedIntegration {
    #[must_use]
    pub fn binding_import_target(&self) -> KimiLocalServerBindingImportTarget {
        KimiLocalServerBindingImportTarget {
            instance: self.instance().clone(),
            access_profile: self.access_profile().clone(),
            access_evidence: self.access_evidence().clone(),
            server: self.server().clone(),
            state_root: self.state_root().clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KimiLocalServerBindingImportInput {
    pub(super) request_id: RequestId,
    pub(super) scope_id: ScopeId,
    pub(super) source: KimiAcpSessionImportAuthority,
    pub(super) target: KimiLocalServerBindingImportTarget,
    pub(super) deadline: Deadline,
    pub(super) cancellation: DiscoveryCancellation,
    pub(super) allow_unverified_newer: bool,
}

impl KimiLocalServerBindingImportInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        scope_id: ScopeId,
        source: KimiAcpSessionImportAuthority,
        target: KimiLocalServerBindingImportTarget,
        deadline: Deadline,
        cancellation: DiscoveryCancellation,
    ) -> Self {
        Self {
            request_id,
            scope_id,
            source,
            target,
            deadline,
            cancellation,
            allow_unverified_newer: false,
        }
    }

    #[must_use]
    pub const fn allow_unverified_newer(mut self) -> Self {
        self.allow_unverified_newer = true;
        self
    }
}

impl KimiLocalServerSessionManagementInput {
    #[must_use]
    pub const fn new(request_id: RequestId, binding: ProviderSessionManagementBinding) -> Self {
        Self {
            request_id,
            binding,
            deadline: None,
            allow_unverified_newer: false,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub const fn allow_unverified_newer(mut self) -> Self {
        self.allow_unverified_newer = true;
        self
    }
}
