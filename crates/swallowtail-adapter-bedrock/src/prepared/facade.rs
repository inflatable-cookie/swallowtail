use super::{
    BedrockCataloguePreparationInput, BedrockCataloguePreparedIntegration,
    BedrockRuntimePreparationInput, BedrockRuntimePreparedIntegration, failure,
    prepare_bedrock_catalogue, prepare_bedrock_runtime,
};
use crate::BedrockCloudClientConfig;
use swallowtail_core::{
    AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, InstanceTargetRef,
};
use swallowtail_runtime::{
    HostServices, PreparationFailure, PreparationStage, PreparedAccessEvidence,
};

/// Shared Bedrock inputs which do not select a catalogue or Runtime route.
#[derive(Clone, Debug)]
pub struct BedrockFacadePreparationInput {
    execution_host: ExecutionHostId,
    cloud_client: BedrockCloudClientConfig,
}

impl BedrockFacadePreparationInput {
    #[must_use]
    /// Creates shared facade input without selecting a Bedrock route.
    pub const fn new(
        execution_host: ExecutionHostId,
        cloud_client: BedrockCloudClientConfig,
    ) -> Self {
        Self {
            execution_host,
            cloud_client,
        }
    }

    fn into_parts(self) -> (ExecutionHostId, BedrockCloudClientConfig) {
        (self.execution_host, self.cloud_client)
    }
}

/// Route-specific catalogue inputs. Shared host and cloud-client inputs come
/// from [`BedrockFacade`].
#[derive(Clone)]
pub struct BedrockCatalogueRouteInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    endpoint_target: InstanceTargetRef,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
}

impl BedrockCatalogueRouteInput {
    #[must_use]
    /// Creates explicit control-plane catalogue route input.
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        endpoint_target: InstanceTargetRef,
        access: AccessProfile,
        evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            endpoint_target,
            access,
            evidence,
        }
    }

    fn into_parts(
        self,
    ) -> (
        ConfiguredInstanceId,
        InstanceRevision,
        InstanceTargetRef,
        AccessProfile,
        PreparedAccessEvidence,
    ) {
        (
            self.instance_id,
            self.instance_revision,
            self.endpoint_target,
            self.access,
            self.evidence,
        )
    }
}

/// Route-specific Runtime inputs. Shared host and cloud-client inputs come
/// from [`BedrockFacade`].
#[derive(Clone)]
pub struct BedrockRuntimeRouteInput {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    endpoint_target: InstanceTargetRef,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
}

impl BedrockRuntimeRouteInput {
    #[must_use]
    /// Creates explicit Runtime inference route input.
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        endpoint_target: InstanceTargetRef,
        access: AccessProfile,
        evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            endpoint_target,
            access,
            evidence,
        }
    }

    fn into_parts(
        self,
    ) -> (
        ConfiguredInstanceId,
        InstanceRevision,
        InstanceTargetRef,
        AccessProfile,
        PreparedAccessEvidence,
    ) {
        (
            self.instance_id,
            self.instance_revision,
            self.endpoint_target,
            self.access,
            self.evidence,
        )
    }
}

/// One Bedrock provider facade with separate typed catalogue and Runtime
/// branches.
///
/// This value shares only the execution host and explicit AWS SDK client
/// configuration. Each branch still prepares one exact Swallowtail route.
#[derive(Clone, Debug)]
pub struct BedrockFacade {
    execution_host: ExecutionHostId,
    cloud_client: BedrockCloudClientConfig,
}

impl BedrockFacade {
    #[must_use]
    /// Returns the execution host shared by both facade branches.
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host
    }

    #[must_use]
    /// Returns the explicit AWS region shared by both facade branches.
    pub const fn region(&self) -> &crate::BedrockRegion {
        self.cloud_client.region()
    }

    /// Prepares the control-plane catalogue branch.
    pub fn catalogue(
        &self,
        input: BedrockCatalogueRouteInput,
        services: &HostServices,
    ) -> Result<BedrockCataloguePreparedIntegration, PreparationFailure> {
        let (instance_id, instance_revision, endpoint_target, access, evidence) =
            input.into_parts();
        prepare_bedrock_catalogue(
            BedrockCataloguePreparationInput::new(
                instance_id,
                instance_revision,
                self.execution_host.clone(),
                endpoint_target,
                access,
                evidence,
                self.cloud_client.clone(),
            ),
            services,
        )
    }

    /// Prepares the Runtime inference branch.
    pub fn runtime(
        &self,
        input: BedrockRuntimeRouteInput,
        services: &HostServices,
    ) -> Result<BedrockRuntimePreparedIntegration, PreparationFailure> {
        let (instance_id, instance_revision, endpoint_target, access, evidence) =
            input.into_parts();
        prepare_bedrock_runtime(
            BedrockRuntimePreparationInput::new(
                instance_id,
                instance_revision,
                self.execution_host.clone(),
                endpoint_target,
                access,
                evidence,
                self.cloud_client.clone(),
            ),
            services,
        )
    }
}

/// Prepare shared Bedrock provider context without selecting a route.
///
/// Catalogue and Runtime route preparation remain explicit through the
/// returned typed facade.
pub fn prepare_bedrock(
    input: BedrockFacadePreparationInput,
    services: &HostServices,
) -> Result<BedrockFacade, PreparationFailure> {
    let (execution_host, cloud_client) = input.into_parts();
    if services.execution_host_id() != &execution_host {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.bedrock.facade.preparation.host_mismatch",
            "Bedrock facade services belong to a different execution host",
        ));
    }
    Ok(BedrockFacade {
        execution_host,
        cloud_client,
    })
}
