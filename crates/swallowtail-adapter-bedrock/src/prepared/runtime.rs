#[path = "runtime/evidence.rs"]
mod evidence;
#[path = "runtime/input.rs"]
mod input;

pub use evidence::BedrockRuntimePreparedEvidence;
pub use input::{
    BedrockModelSelection, BedrockRuntimePreparationInput, BedrockRuntimeProfileInput,
};

use super::{failure, validate_execution_binding, validate_preparation};
use crate::{BedrockCredentialProvider, BedrockDirectDriver, BedrockDriverBinding, BedrockRegion};
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, CapabilityRequirement, ConfiguredInstance, Diagnostic, ExecutionHostId,
    HostServiceKind, InstanceTargetRef, PreflightContext, PreflightPlan, preflight,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage,
    PreparedAccessEvidence, RunHandle, RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone)]
/// Prepared Bedrock Runtime integration bound to one instance, host, and region.
pub struct BedrockRuntimePreparedIntegration {
    instance: ConfiguredInstance,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    services: BTreeSet<HostServiceKind>,
    region: BedrockRegion,
    credential_provider: BedrockCredentialProvider,
}

impl BedrockRuntimePreparedIntegration {
    #[must_use]
    /// Returns the configured Runtime instance.
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    #[must_use]
    /// Returns the delegated cloud-identity access profile.
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access
    }

    #[must_use]
    /// Returns access evidence together with its provenance.
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the exact AWS region.
    pub const fn region(&self) -> &BedrockRegion {
        &self.region
    }

    #[must_use]
    /// Returns the qualified Runtime SDK crate name.
    pub const fn sdk_crate(&self) -> &'static str {
        crate::SDK_CRATE
    }

    #[must_use]
    /// Returns the qualified Runtime SDK version.
    pub const fn sdk_version(&self) -> &'static str {
        crate::SDK_VERSION
    }

    #[must_use]
    /// Returns the qualified Runtime service API.
    pub const fn service_api(&self) -> &'static str {
        crate::SERVICE_API
    }

    /// Iterates the host services present during preparation.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.services.iter().copied()
    }

    #[must_use]
    /// Returns the public low-level Runtime driver with its exact SDK binding.
    pub fn low_level_driver(&self) -> BedrockDirectDriver {
        BedrockDirectDriver::new(BedrockDriverBinding::new(
            self.instance.id().clone(),
            self.access.id().clone(),
            self.access
                .credential_reference()
                .expect("validated credential reference")
                .clone(),
            self.instance.execution_host_id().clone(),
            self.region.clone(),
            self.credential_provider.clone(),
        ))
    }

    /// Rejects execution-host or endpoint drift from the prepared binding.
    pub fn validate_execution_binding(
        &self,
        execution_host: &ExecutionHostId,
        endpoint_target: &InstanceTargetRef,
    ) -> Result<(), PreparationFailure> {
        validate_execution_binding(
            self.instance.execution_host_id(),
            self.instance.target_reference(),
            execution_host,
            endpoint_target,
            "swallowtail.bedrock.runtime.preparation.target_drift",
        )
    }

    /// Validates and prepares one SDK-native inference attempt.
    pub fn prepare_inference_attempt(
        &self,
        input: BedrockRuntimeProfileInput,
    ) -> Result<BedrockPreparedInferenceAttempt, PreparationFailure> {
        let (request_id, model, content, maximum, deadline) = input.into_parts();
        if maximum.get() > i32::MAX as u64 {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.bedrock.runtime.preparation.output_limit_invalid",
                "Bedrock Runtime output-token limit exceeds the SDK request range",
            ));
        }
        let (route_id, route_revision, model_id, provider_id) = model.into_parts();
        let activity = crate::activity::profile::activity_profile();
        let capabilities = crate::activity::profile::with_activity(
            self.instance.capabilities().clone(),
            &activity,
        );
        let route = swallowtail_core::ModelRoute::new(
            route_id,
            route_revision,
            self.instance.id().clone(),
            model_id,
            capabilities.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = crate::selection::runtime_requirements(
            self.instance.execution_host_id().clone(),
            self.access.id().clone(),
        )
        .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        }));
        let instance = instance_with_capabilities(&self.instance, capabilities);
        let plan = preflight(
            &PreflightContext::new(
                &crate::bedrock_direct_descriptor(),
                &instance,
                &self.access,
                self.evidence.status(),
                self.available_host_services(),
            )
            .with_model_route(&route),
            &requirements,
        )
        .map_err(preflight_failure)?;
        let mut request =
            StructuredRunRequest::new(request_id, content, OperationPolicy::offline())
                .with_maximum_output_tokens(maximum);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(BedrockPreparedInferenceAttempt {
            evidence: BedrockRuntimePreparedEvidence::new_with_activity(self, plan, activity)?,
            request,
            driver: self.low_level_driver(),
        })
    }
}

fn instance_with_capabilities(
    base: &ConfiguredInstance,
    capabilities: swallowtail_core::CapabilityProfile,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
}

/// Prepares Bedrock Runtime without invoking the SDK.
pub fn prepare_bedrock_runtime(
    input: BedrockRuntimePreparationInput,
    services: &HostServices,
) -> Result<BedrockRuntimePreparedIntegration, PreparationFailure> {
    let (
        instance_id,
        instance_revision,
        execution_host,
        endpoint_target,
        access,
        evidence,
        cloud_client,
    ) = input.into_parts();
    validate_preparation(
        services,
        &execution_host,
        &access,
        &evidence,
        crate::BEDROCK_RUNTIME_ACCESS_PROFILE_ID,
        crate::BEDROCK_RUNTIME_ENDPOINT_AUDIENCE,
        "swallowtail.bedrock.runtime.preparation.rejected",
    )?;
    let (region, credential_provider) = cloud_client.into_parts();
    Ok(BedrockRuntimePreparedIntegration {
        instance: crate::selection::runtime_instance(
            instance_id,
            instance_revision,
            execution_host,
            endpoint_target,
            access.id().clone(),
        ),
        access,
        evidence,
        services: services.available_kinds(),
        region,
        credential_provider,
    })
}

#[derive(Clone)]
/// Executable Bedrock Runtime attempt with exact plan and request agreement.
pub struct BedrockPreparedInferenceAttempt {
    evidence: BedrockRuntimePreparedEvidence,
    request: StructuredRunRequest,
    driver: BedrockDirectDriver,
}

impl BedrockPreparedInferenceAttempt {
    #[must_use]
    /// Returns route-specific Runtime evidence.
    pub const fn evidence(&self) -> &BedrockRuntimePreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable Runtime preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived structured-run request.
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    /// Returns the bound low-level Runtime driver.
    pub fn low_level_driver(&self) -> BedrockDirectDriver {
        self.driver.clone()
    }

    /// Starts the single SDK inference attempt.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.driver.clone();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    /// Splits the prepared attempt into evidence, plan, request, and driver.
    pub fn into_parts(
        self,
    ) -> (
        BedrockRuntimePreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
        BedrockDirectDriver,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request, self.driver)
    }
}

fn preflight_failure(error: swallowtail_core::PreflightFailure) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(error.diagnostic().clone()),
    )
}
