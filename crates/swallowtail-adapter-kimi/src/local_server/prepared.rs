#[path = "prepared/import.rs"]
mod import;
#[path = "prepared/input.rs"]
mod input;
#[path = "prepared/instance.rs"]
mod instance;
#[path = "prepared/operation.rs"]
mod operation;
#[path = "prepared/preparation.rs"]
mod preparation;
#[path = "prepared/probe.rs"]
mod probe;
#[path = "prepared/topology.rs"]
mod topology;
#[path = "prepared/validation.rs"]
mod validation;

pub use catalogue::KimiLocalServerPreparedCatalogue;
pub use import::KimiLocalServerPreparedBindingImport;
pub use input::{
    KimiLocalServerAttachedInput, KimiLocalServerBindingImportInput,
    KimiLocalServerBindingImportTarget, KimiLocalServerCatalogueInput, KimiLocalServerOwnedInput,
    KimiLocalServerPreparationProbe, KimiLocalServerSessionManagementInput,
};
pub(crate) use operation::lifecycle_capabilities;
pub use operation::{KimiLocalServerPreparedArchive, KimiLocalServerPreparedRestore};
pub use preparation::prepare_kimi_local_server_attached;
pub use topology::{KimiLocalServerOwnedHandle, start_kimi_local_server_owned};

use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, HostServiceKind, InstanceTargetRef,
    InterfaceCompatibilityAssessment, InterfaceVersionBinding,
};
use swallowtail_runtime::{PreparedAccessEvidence, WorkingResourceRef};

fn runtime_preparation_failure(
    stage: swallowtail_runtime::PreparationStage,
    error: swallowtail_runtime::RuntimeFailure,
) -> swallowtail_runtime::PreparationFailure {
    swallowtail_runtime::PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
    )
}

fn preparation_failure(
    stage: swallowtail_runtime::PreparationStage,
    code: &'static str,
    message: &'static str,
) -> swallowtail_runtime::PreparationFailure {
    swallowtail_runtime::PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerObservation {
    binding: InterfaceVersionBinding,
    compatibility: InterfaceCompatibilityAssessment,
}

impl KimiLocalServerObservation {
    pub(super) const fn new(
        binding: InterfaceVersionBinding,
        compatibility: InterfaceCompatibilityAssessment,
    ) -> Self {
        Self {
            binding,
            compatibility,
        }
    }

    #[must_use]
    pub const fn binding(&self) -> &InterfaceVersionBinding {
        &self.binding
    }

    #[must_use]
    pub const fn compatibility(&self) -> &InterfaceCompatibilityAssessment {
        &self.compatibility
    }

    #[must_use]
    pub const fn is_qualified(&self) -> bool {
        matches!(
            self.compatibility,
            InterfaceCompatibilityAssessment::Qualified(_)
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    server: KimiLocalServerObservation,
    state_root: WorkingResourceRef,
    executable_target: Option<InstanceTargetRef>,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl KimiLocalServerPreparedIntegration {
    #[must_use]
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    #[must_use]
    pub const fn server(&self) -> &KimiLocalServerObservation {
        &self.server
    }

    #[must_use]
    pub const fn state_root(&self) -> &WorkingResourceRef {
        &self.state_root
    }

    #[must_use]
    pub const fn executable_target(&self) -> Option<&InstanceTargetRef> {
        self.executable_target.as_ref()
    }

    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    pub fn low_level_driver(&self) -> super::KimiLocalServerDriver {
        super::KimiLocalServerDriver::new()
    }
}
#[path = "prepared/catalogue.rs"]
mod catalogue;
