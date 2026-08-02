#[path = "prepared/instance.rs"]
mod instance;
#[path = "prepared/lifecycle.rs"]
mod lifecycle;
#[path = "prepared/observation.rs"]
mod observation;
#[path = "prepared/probe.rs"]
mod probe;
#[path = "prepared/validation.rs"]
mod validation;

use instance::configured_instance;
pub(crate) use instance::{all_capabilities, run_capabilities, working_resource_capability};
pub use observation::OpenCodePreparedServerObservation;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, ExecutionHostId, HostServiceKind,
    InstanceRevision, InstanceTargetRef,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, HostServices, PreparationFailure, PreparationStage,
    PreparedAccessEvidence, ScopeId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodePreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl OpenCodePreparationInput {
    #[must_use]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            endpoint_target,
            access_profile,
            access_evidence,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OpenCodePreparationProbe {
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl OpenCodePreparationProbe {
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
pub struct OpenCodePreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    server: OpenCodePreparedServerObservation,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl OpenCodePreparedIntegration {
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
    pub const fn server(&self) -> &OpenCodePreparedServerObservation {
        &self.server
    }

    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    pub fn low_level_driver(&self) -> crate::OpenCodeHttpDriver {
        crate::OpenCodeHttpDriver::new()
    }

    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        endpoint_target: &InstanceTargetRef,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.instance.execution_host_id()
            || endpoint_target != self.instance.target_reference()
        {
            return Err(failure(
                PreparationStage::TargetSelection,
                "swallowtail.opencode.preparation.target_drift",
                "Prepared OpenCode host or endpoint target no longer matches",
            ));
        }
        Ok(())
    }
}

pub async fn prepare_opencode_attached(
    input: OpenCodePreparationInput,
    probe: OpenCodePreparationProbe,
    services: HostServices,
) -> Result<OpenCodePreparedIntegration, PreparationFailure> {
    validation::validate_input(&input, &services)?;
    validation::validate_probe(&probe, &services)?;
    let available_host_services = services.available_kinds();
    let server = probe::observe_server(&input, &probe, &services).await?;
    let instance = configured_instance(&input, server.binding());
    Ok(OpenCodePreparedIntegration {
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        server,
        available_host_services,
    })
}

fn runtime_failure(
    stage: PreparationStage,
    error: swallowtail_runtime::RuntimeFailure,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
    )
}

fn failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
