#[path = "catalogue/evidence.rs"]
mod evidence;
#[path = "catalogue/input.rs"]
mod input;

pub use evidence::BedrockCataloguePreparedEvidence;
pub use input::{BedrockCataloguePreparationInput, BedrockCatalogueProfileInput};

use super::{validate_execution_binding, validate_preparation};
use crate::{
    BedrockCatalogueBinding, BedrockCatalogueDriver, BedrockCredentialProvider, BedrockRegion,
};
use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, Diagnostic, ExecutionHostId, HostServiceKind,
    InstanceTargetRef, ModelCatalogEntry, PreflightContext, PreflightPlan, preflight,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RuntimeFailure,
};

#[derive(Clone)]
pub struct BedrockCataloguePreparedIntegration {
    instance: ConfiguredInstance,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    services: BTreeSet<HostServiceKind>,
    region: BedrockRegion,
    credential_provider: BedrockCredentialProvider,
}

impl BedrockCataloguePreparedIntegration {
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
    pub const fn region(&self) -> &BedrockRegion {
        &self.region
    }

    #[must_use]
    pub const fn sdk_crate(&self) -> &'static str {
        crate::CATALOGUE_SDK_CRATE
    }

    #[must_use]
    pub const fn sdk_version(&self) -> &'static str {
        crate::CATALOGUE_SDK_VERSION
    }

    #[must_use]
    pub const fn service_api(&self) -> &'static str {
        crate::CATALOGUE_SERVICE_API
    }

    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.services.iter().copied()
    }

    #[must_use]
    pub fn low_level_driver(&self) -> BedrockCatalogueDriver {
        BedrockCatalogueDriver::new(BedrockCatalogueBinding::new(
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
            "swallowtail.bedrock.catalogue.preparation.target_drift",
        )
    }

    pub fn prepare_catalogue(
        &self,
        input: BedrockCatalogueProfileInput,
    ) -> Result<BedrockPreparedCatalogue, PreparationFailure> {
        let requirements = crate::selection::catalogue_requirements(
            self.instance.execution_host_id().clone(),
            self.access.id().clone(),
        );
        let plan = preflight(
            &PreflightContext::new(
                &crate::bedrock_catalogue_descriptor(),
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
        Ok(BedrockPreparedCatalogue {
            evidence: BedrockCataloguePreparedEvidence::new(self, plan)?,
            request,
            driver: self.low_level_driver(),
        })
    }
}

pub fn prepare_bedrock_catalogue(
    input: BedrockCataloguePreparationInput,
    services: &HostServices,
) -> Result<BedrockCataloguePreparedIntegration, PreparationFailure> {
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
        crate::BEDROCK_CATALOGUE_ACCESS_PROFILE_ID,
        crate::BEDROCK_CONTROL_PLANE_ENDPOINT_AUDIENCE,
        "swallowtail.bedrock.catalogue.preparation.rejected",
    )?;
    let (region, credential_provider) = cloud_client.into_parts();
    Ok(BedrockCataloguePreparedIntegration {
        instance: crate::selection::catalogue_instance(
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
pub struct BedrockPreparedCatalogue {
    evidence: BedrockCataloguePreparedEvidence,
    request: ModelCatalogRequest,
    driver: BedrockCatalogueDriver,
}

impl BedrockPreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &BedrockCataloguePreparedEvidence {
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
    pub fn low_level_driver(&self) -> BedrockCatalogueDriver {
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

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        BedrockCataloguePreparedEvidence,
        PreflightPlan,
        ModelCatalogRequest,
        BedrockCatalogueDriver,
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
