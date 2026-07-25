#[path = "prepared/instance.rs"]
mod instance;
#[path = "prepared/lifecycle.rs"]
mod lifecycle;
#[path = "prepared/observation.rs"]
mod observation;
#[path = "prepared/probe.rs"]
mod probe;

use crate::protocol::{
    ObservationBinding, Request, parse_inventory, parse_model_detail, parse_version,
};
use crate::transport::CurlTransport;
use instance::configured_instance;
use lifecycle::complete_probe_work;
pub use observation::OllamaPreparedRuntimeObservation;
use probe::observe_runtime;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{
    AccessProfile, AttachedModelObservationScope, AttachedModelTag, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, EntitlementMetering, ExecutionHostId,
    HostServiceKind, InstanceRevision, InstanceTargetRef, ModelManifestDigest, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EndpointRef, HostServices, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, ScopeId,
};

const ENDPOINT_AUDIENCE: &str = "ollama.attached";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OllamaPreparationInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    endpoint_target: InstanceTargetRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    model: crate::prepared_profile::OllamaModelSelection,
    selected_model_tag: AttachedModelTag,
    selected_manifest_digest: ModelManifestDigest,
}

impl OllamaPreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        endpoint_target: InstanceTargetRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
        model: crate::prepared_profile::OllamaModelSelection,
        selected_model_tag: AttachedModelTag,
        selected_manifest_digest: ModelManifestDigest,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            endpoint_target,
            access_profile,
            access_evidence,
            model,
            selected_model_tag,
            selected_manifest_digest,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OllamaPreparationProbe {
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl OllamaPreparationProbe {
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
pub struct OllamaPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    runtime: OllamaPreparedRuntimeObservation,
    model: crate::prepared_profile::OllamaModelSelection,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl OllamaPreparedIntegration {
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
    pub const fn runtime(&self) -> &OllamaPreparedRuntimeObservation {
        &self.runtime
    }

    #[must_use]
    pub const fn model_selection(&self) -> &crate::prepared_profile::OllamaModelSelection {
        &self.model
    }

    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    #[must_use]
    pub fn low_level_driver(&self) -> crate::OllamaNativeAttachedDriver {
        crate::OllamaNativeAttachedDriver::new()
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
                "swallowtail.ollama.preparation.target_drift",
                "Prepared Ollama host or endpoint target no longer matches",
            ));
        }
        Ok(())
    }
}

pub async fn prepare_ollama_attached(
    input: OllamaPreparationInput,
    probe: OllamaPreparationProbe,
    services: HostServices,
) -> Result<OllamaPreparedIntegration, PreparationFailure> {
    validate_input(&input, &services)?;
    validate_probe(&probe, &services)?;
    let available_host_services = services.available_kinds();
    let runtime = observe_runtime(&input, &probe, &services).await?;
    let instance = configured_instance(&input, runtime.runtime_version());
    Ok(OllamaPreparedIntegration {
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        runtime,
        model: input.model,
        available_host_services,
    })
}

include!("prepared/validation.rs");
