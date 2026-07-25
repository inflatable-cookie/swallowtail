#[path = "owned/evidence.rs"]
mod evidence;
#[path = "owned/input.rs"]
mod input;

pub use evidence::LlamaCppOwnedPreparedEvidence;
pub use input::{LlamaCppOwnedPreparationInput, LlamaCppOwnedServingSelection};

use super::{preflight_failure, validate_preparation};
use crate::{LlamaCppModelSelection, LlamaCppOwnedDriver};
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, HostServiceKind, ModelArtifactBinding, PreflightContext,
    PreflightPlan, preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OwnedServingHandle, PreparationFailure,
    PreparedAccessEvidence, RuntimeFailure, ScopeId, ServingInstanceDriver, ServingInstanceId,
    StartServingRequest,
};

#[derive(Clone)]
pub struct LlamaCppOwnedPreparedIntegration {
    instance: ConfiguredInstance,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    services: BTreeSet<HostServiceKind>,
    artifact: ModelArtifactBinding,
    selection: LlamaCppModelSelection,
}

impl LlamaCppOwnedPreparedIntegration {
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
    pub const fn artifact(&self) -> &ModelArtifactBinding {
        &self.artifact
    }

    #[must_use]
    pub const fn expected_build(&self) -> &'static str {
        crate::LLAMA_CPP_OWNED_BUILD
    }

    #[must_use]
    pub const fn expected_commit(&self) -> &'static str {
        crate::LLAMA_CPP_OWNED_COMMIT
    }

    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.services.iter().copied()
    }

    #[must_use]
    pub fn low_level_driver(&self) -> LlamaCppOwnedDriver {
        LlamaCppOwnedDriver::new()
    }

    pub fn prepare_serving_start(
        &self,
        scope: ScopeId,
        serving_instance_id: ServingInstanceId,
        deadline: Deadline,
    ) -> Result<LlamaCppPreparedServingStart, PreparationFailure> {
        let (route_id, route_revision, model_id) = self.selection.clone().into_parts();
        let route = crate::selection::model_route(
            self.instance.id().clone(),
            route_id,
            route_revision,
            model_id,
            crate::selection::owned_capabilities(),
        );
        let requirements = crate::selection::owned_requirements(
            self.instance.execution_host_id().clone(),
            self.access.id().clone(),
        );
        let plan = preflight(
            &PreflightContext::new(
                &crate::llama_cpp_owned_descriptor(),
                &self.instance,
                &self.access,
                self.evidence.status(),
                self.available_host_services(),
            )
            .with_model_route(&route)
            .with_model_artifact(&self.artifact),
            &requirements,
        )
        .map_err(preflight_failure)?;
        let request =
            StartServingRequest::new(scope, serving_instance_id, self.artifact.clone(), deadline);
        Ok(LlamaCppPreparedServingStart {
            evidence: LlamaCppOwnedPreparedEvidence::new(self, plan)?,
            request,
            driver: self.low_level_driver(),
        })
    }
}

pub fn prepare_llama_cpp_owned(
    input: LlamaCppOwnedPreparationInput,
    services: &HostServices,
) -> Result<LlamaCppOwnedPreparedIntegration, PreparationFailure> {
    let (instance_id, revision, host, target, access, evidence, serving) = input.into_parts();
    let (artifact, selection) = serving.into_parts();
    validate_preparation(
        services,
        &host,
        &access,
        &evidence,
        crate::LLAMA_CPP_OWNED_ACCESS_PROFILE_ID,
        crate::LLAMA_CPP_OWNED_ENDPOINT_AUDIENCE,
        "swallowtail.llama_cpp.owned.preparation.rejected",
    )?;
    Ok(LlamaCppOwnedPreparedIntegration {
        instance: crate::selection::owned_instance(
            instance_id,
            revision,
            host,
            target,
            access.id().clone(),
        ),
        access,
        evidence,
        services: services.available_kinds(),
        artifact,
        selection,
    })
}

#[derive(Clone)]
pub struct LlamaCppPreparedServingStart {
    evidence: LlamaCppOwnedPreparedEvidence,
    request: StartServingRequest,
    driver: LlamaCppOwnedDriver,
}

impl LlamaCppPreparedServingStart {
    #[must_use]
    pub const fn evidence(&self) -> &LlamaCppOwnedPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StartServingRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> LlamaCppOwnedDriver {
        self.driver.clone()
    }

    pub fn start(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn OwnedServingHandle>, RuntimeFailure>> {
        let driver = self.driver.clone();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start(plan, request, services).await })
    }
}
