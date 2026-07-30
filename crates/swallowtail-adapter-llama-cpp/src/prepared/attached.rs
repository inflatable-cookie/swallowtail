#[path = "attached/evidence.rs"]
mod evidence;
#[path = "attached/input.rs"]
mod input;

pub use evidence::LlamaCppAttachedPreparedEvidence;
pub use input::{
    LlamaCppAttachedPreparationInput, LlamaCppCatalogueProfileInput, LlamaCppInferenceProfileInput,
    LlamaCppModelSelection,
};

use super::{preflight_failure, validate_preparation};
use crate::LlamaCppAttachedDriver;
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, CapabilityRequirement, ConfiguredInstance, DriverRole, HostServiceKind,
    ModelCatalogEntry, PreflightContext, PreflightPlan, preflight,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, OperationPolicy,
    PreparationFailure, PreparedAccessEvidence, RunHandle, RuntimeFailure, StructuredRunDriver,
    StructuredRunRequest,
};

#[derive(Clone)]
pub struct LlamaCppAttachedPreparedIntegration {
    instance: ConfiguredInstance,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    services: BTreeSet<HostServiceKind>,
}

impl LlamaCppAttachedPreparedIntegration {
    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    #[must_use]
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access
    }

    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn expected_build(&self) -> &'static str {
        crate::LLAMA_CPP_ATTACHED_BUILD
    }

    #[must_use]
    pub const fn expected_commit(&self) -> &'static str {
        crate::LLAMA_CPP_ATTACHED_COMMIT
    }

    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.services.iter().copied()
    }

    #[must_use]
    pub fn low_level_driver(&self) -> LlamaCppAttachedDriver {
        LlamaCppAttachedDriver::new()
    }

    pub fn prepare_catalogue(
        &self,
        input: LlamaCppCatalogueProfileInput,
    ) -> Result<LlamaCppPreparedCatalogue, PreparationFailure> {
        let requirements = crate::selection::attached_requirements(
            self.instance.execution_host_id().clone(),
            self.access.id().clone(),
            DriverRole::ModelCatalog,
        );
        let plan = preflight(
            &PreflightContext::new(
                &crate::llama_cpp_attached_descriptor(),
                &self.instance,
                &self.access,
                self.evidence.status(),
                self.available_host_services(),
            ),
            &requirements,
        )
        .map_err(preflight_failure)?;
        let (request_id, deadline) = input.into_parts();
        let mut request = ModelCatalogRequest::new(request_id);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(LlamaCppPreparedCatalogue {
            evidence: LlamaCppAttachedPreparedEvidence::new(self, plan)?,
            request,
            driver: self.low_level_driver(),
        })
    }

    pub fn prepare_inference_attempt(
        &self,
        input: LlamaCppInferenceProfileInput,
    ) -> Result<LlamaCppPreparedInferenceAttempt, PreparationFailure> {
        let (request_id, selection, content, maximum, deadline) = input.into_parts();
        let (route_id, route_revision, model_id) = selection.into_parts();
        let activity = crate::activity::profile::activity_profile();
        let capabilities = crate::activity::profile::with_activity(
            swallowtail_core::CapabilityProfile::new(crate::selection::attached_capabilities(
                DriverRole::StructuredRun,
            )),
            &activity,
        );
        let capability_requirements = capabilities
            .iter()
            .map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            })
            .collect::<Vec<_>>();
        let route = crate::selection::model_route(
            self.instance.id().clone(),
            route_id,
            route_revision,
            model_id,
            capability_requirements.clone(),
        );
        let requirements = crate::selection::attached_requirements(
            self.instance.execution_host_id().clone(),
            self.access.id().clone(),
            DriverRole::StructuredRun,
        )
        .with_capabilities(capability_requirements)
        .require_model_route();
        let instance = instance_with_capabilities(&self.instance, capabilities);
        let plan = preflight(
            &PreflightContext::new(
                &crate::llama_cpp_attached_descriptor(),
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
        Ok(LlamaCppPreparedInferenceAttempt {
            evidence: LlamaCppAttachedPreparedEvidence::new_with_activity(self, plan, activity)?,
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

pub fn prepare_llama_cpp_attached(
    input: LlamaCppAttachedPreparationInput,
    services: &HostServices,
) -> Result<LlamaCppAttachedPreparedIntegration, PreparationFailure> {
    let (instance_id, revision, host, target, access, evidence) = input.into_parts();
    validate_preparation(
        services,
        &host,
        &access,
        &evidence,
        crate::LLAMA_CPP_ATTACHED_ACCESS_PROFILE_ID,
        crate::LLAMA_CPP_ATTACHED_ENDPOINT_AUDIENCE,
        "swallowtail.llama_cpp.attached.preparation.rejected",
    )?;
    Ok(LlamaCppAttachedPreparedIntegration {
        instance: crate::selection::attached_instance(
            instance_id,
            revision,
            host,
            target,
            access.id().clone(),
        ),
        access,
        evidence,
        services: services.available_kinds(),
    })
}

#[derive(Clone)]
pub struct LlamaCppPreparedCatalogue {
    evidence: LlamaCppAttachedPreparedEvidence,
    request: ModelCatalogRequest,
    driver: LlamaCppAttachedDriver,
}

impl LlamaCppPreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &LlamaCppAttachedPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> LlamaCppAttachedDriver {
        self.driver.clone()
    }

    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        let driver = self.driver.clone();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }
}

#[derive(Clone)]
pub struct LlamaCppPreparedInferenceAttempt {
    evidence: LlamaCppAttachedPreparedEvidence,
    request: StructuredRunRequest,
    driver: LlamaCppAttachedDriver,
}

impl LlamaCppPreparedInferenceAttempt {
    #[must_use]
    pub const fn evidence(&self) -> &LlamaCppAttachedPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> LlamaCppAttachedDriver {
        self.driver.clone()
    }

    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.driver.clone();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }
}
