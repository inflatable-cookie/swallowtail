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
    AccessProfile, AdmittedInstanceRecord, AttachedModelObservationScope, AttachedModelTag,
    ConfigFieldId, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    EntitlementMetering, ExecutionHostId, HostServiceKind, InstanceRevision, InstanceTargetRef,
    ModelManifestDigest, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EndpointRef, HostServices, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, ScopeId,
};

const ENDPOINT_AUDIENCE: &str = "ollama.attached";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs that bind one attached runtime to an exact local model artifact.
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
    /// Creates preparation inputs with explicit endpoint, model tag, and digest.
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

    /// Builds preparation input from one admitted attached-runtime record.
    ///
    /// The host-owned endpoint remains opaque until the selected network
    /// service resolves it during the bounded runtime probe.
    #[allow(clippy::too_many_arguments)]
    pub fn from_admitted(
        admitted: &AdmittedInstanceRecord,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
        model: crate::prepared_profile::OllamaModelSelection,
        selected_model_tag: AttachedModelTag,
        selected_manifest_digest: ModelManifestDigest,
    ) -> Result<Self, PreparationFailure> {
        if admitted.route_id().as_str() != crate::OLLAMA_ATTACHED_ADDABLE_ROUTE_ID {
            return Err(failure(
                PreparationStage::TargetSelection,
                "swallowtail.ollama.preparation.route_mismatch",
                "Ollama preparation requires the admitted attached route",
            ));
        }
        let endpoint_field_id = ConfigFieldId::new(crate::OLLAMA_ATTACHED_ENDPOINT_FIELD_ID)
            .expect("static Ollama config field id is valid");
        let endpoint = admitted.config_ref(&endpoint_field_id).ok_or_else(|| {
            failure(
                PreparationStage::TargetSelection,
                "swallowtail.ollama.preparation.endpoint_ref_missing",
                "Ollama preparation requires the admitted endpoint reference",
            )
        })?;
        Ok(Self::new(
            admitted.id().clone(),
            instance_revision,
            execution_host_id,
            InstanceTargetRef::from_config_field(endpoint),
            access_profile,
            access_evidence,
            model,
            selected_model_tag,
            selected_manifest_digest,
        ))
    }
}

#[derive(Clone, Debug)]
/// Caller-owned scope, deadline, and cancellation controls for runtime probing.
pub struct OllamaPreparationProbe {
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl OllamaPreparationProbe {
    /// Creates one bounded attached-runtime probe.
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
/// Observed attached Ollama runtime before operation-specific preflight.
pub struct OllamaPreparedIntegration {
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    runtime: OllamaPreparedRuntimeObservation,
    model: crate::prepared_profile::OllamaModelSelection,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl OllamaPreparedIntegration {
    /// Returns the local-unauthenticated access profile.
    #[must_use]
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    /// Returns the admitted access evidence.
    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    /// Returns the configured attached runtime instance.
    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Returns the runtime, inventory, and selected-model observations.
    #[must_use]
    pub const fn runtime(&self) -> &OllamaPreparedRuntimeObservation {
        &self.runtime
    }

    /// Returns the exact model-route selection bound during preparation.
    #[must_use]
    pub const fn model_selection(&self) -> &crate::prepared_profile::OllamaModelSelection {
        &self.model
    }

    /// Iterates host services available when preparation completed.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    /// Creates the stateless low-level native HTTP driver.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::OllamaNativeAttachedDriver {
        crate::OllamaNativeAttachedDriver::new()
    }

    /// Rejects execution after the selected host or endpoint has drifted.
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

/// Probes and admits one exact externally managed Ollama runtime and model.
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
