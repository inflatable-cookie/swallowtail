//! Persistent-session prepared facade for the Pi SDK sidecar route.
//!
//! The prepared session binds the four exact interface-version points, the
//! host-approved launch recipe target, the delegated harness credential, the
//! exact provider and model route, and the durable provider-state posture.
//! From it, consumers open new sessions (which return the initial durable
//! binding), or load/resume previously bound provider sessions through the
//! runtime's binding machinery.

use super::driver::PiSdkSidecarDriver;
use super::selection::{
    pi_sdk_sidecar_node_binding, pi_sdk_sidecar_package_binding, pi_sdk_sidecar_sidecar_binding,
    pi_sdk_sidecar_wire_binding,
};
use super::{
    PI_SDK_SIDECAR_NODE_RUNTIME, PI_SDK_SIDECAR_SDK_VERSION, PI_SDK_SIDECAR_SOURCE_TAG,
    PI_SDK_SIDECAR_WIRE,
};
use std::num::NonZeroU32;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfigFieldId,
    ConfiguredInstance, ConfiguredInstanceId, CredentialFieldId, CredentialMechanism,
    CredentialRef, CredentialState, DriverRole, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer, ExtensionNamespace,
    HarnessConfigurationPosture, HarnessIsolation, HarnessRpcPolicy, HarnessSchedulingBounds,
    HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PreflightPlan, ProtocolFacadeId, ProviderId, ResourceAccess, ResourceRepresentation,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority,
};
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    LoadSessionRequest, LoadedSession, OpenSessionRequest, PreparationFailure, PreparationStage,
    RequestId, ResumeSessionRequest, RuntimeFailure, SessionOptions, SessionResumeBinding,
    WorkingResourceRef,
};

/// Explicit inputs for preparing one persistent sidecar session.
pub struct PiSdkSidecarSessionPreparation {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstanceTargetRef,
    environment: EnvironmentRef,
    credential: CredentialRef,
    access_profile_id: AccessProfileId,
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider: ProviderId,
    model: ModelId,
    working_resource: WorkingResourceRef,
    request_id: RequestId,
    image_attachments: bool,
}

impl PiSdkSidecarSessionPreparation {
    /// Creates a session preparation from explicit application-approved
    /// identity, launch, access, model, and resource inputs.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        target: InstanceTargetRef,
        environment: EnvironmentRef,
        credential: CredentialRef,
        access_profile_id: AccessProfileId,
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        provider: ProviderId,
        model: ModelId,
        working_resource: WorkingResourceRef,
        request_id: RequestId,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            target,
            environment,
            credential,
            access_profile_id,
            route_id,
            route_revision,
            provider,
            model,
            working_resource,
            request_id,
            image_attachments: false,
        }
    }

    /// Enables the route's bounded PNG image-attachment capability.
    #[must_use]
    pub const fn with_image_attachments(mut self) -> Self {
        self.image_attachments = true;
        self
    }

    /// Builds session preparation input from one admitted sidecar route
    /// record plus the explicit per-session model, resource, and request.
    ///
    /// The admitted launch recipe, environment, and credential references
    /// stay opaque; the selected host services resolve them during use.
    #[allow(clippy::too_many_arguments)]
    pub fn from_admitted(
        admitted: &swallowtail_core::AdmittedInstanceRecord,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        access_profile_id: AccessProfileId,
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        provider: ProviderId,
        model: ModelId,
        working_resource: WorkingResourceRef,
        request_id: RequestId,
    ) -> Result<Self, PreparationFailure> {
        if admitted.route_id().as_str() != super::PI_SDK_SIDECAR_ADDABLE_ROUTE_ID {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.preparation.route_mismatch",
                "Pi SDK sidecar preparation requires the admitted SDK sidecar route",
            ));
        }
        if admitted.driver() != &super::pi_sdk_sidecar_descriptor().identity().clone() {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.preparation.driver_mismatch",
                "Pi SDK sidecar preparation requires the sidecar driver identity",
            ));
        }
        let launch_field = ConfigFieldId::new(super::PI_SDK_SIDECAR_LAUNCH_RECIPE_FIELD_ID)
            .expect("static config field id is valid");
        let launch_recipe = admitted.config_ref(&launch_field).ok_or_else(|| {
            failure(
                "swallowtail.pi.sdk-sidecar.preparation.launch_recipe_missing",
                "Pi SDK sidecar preparation requires the admitted launch recipe reference",
            )
        })?;
        let environment_field = ConfigFieldId::new(super::PI_SDK_SIDECAR_ENVIRONMENT_FIELD_ID)
            .expect("static config field id is valid");
        let environment = admitted.config_ref(&environment_field).ok_or_else(|| {
            failure(
                "swallowtail.pi.sdk-sidecar.preparation.environment_ref_missing",
                "Pi SDK sidecar preparation requires the admitted environment reference",
            )
        })?;
        let credential_field = CredentialFieldId::new(super::PI_SDK_SIDECAR_CREDENTIAL_FIELD_ID)
            .expect("static credential field id is valid");
        let credential = admitted.credential_ref(&credential_field).ok_or_else(|| {
            failure(
                "swallowtail.pi.sdk-sidecar.preparation.credential_ref_missing",
                "Pi SDK sidecar preparation requires the admitted credential reference",
            )
        })?;
        Ok(Self::new(
            admitted.id().clone(),
            instance_revision,
            execution_host_id,
            InstanceTargetRef::from_config_field(launch_recipe),
            EnvironmentRef::from_config_field(environment),
            credential.clone(),
            access_profile_id,
            route_id,
            route_revision,
            provider,
            model,
            working_resource,
            request_id,
        ))
    }
}

type OpenSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
type LoadSessionFuture = BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;

/// A prepared persistent sidecar session: validated plan plus bound request.
pub struct PiSdkSidecarPreparedSession {
    plan: PreflightPlan,
    request: OpenSessionRequest,
    environment: EnvironmentRef,
    credential: CredentialRef,
}

impl PiSdkSidecarPreparedSession {
    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        &self.plan
    }

    /// Returns the bound session-open request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Creates the low-level sidecar driver bound to this session.
    #[must_use]
    pub fn low_level_driver(&self) -> PiSdkSidecarDriver {
        PiSdkSidecarDriver::new(self.environment.clone(), self.credential.clone())
    }

    /// Opens a new provider session with caller-supplied host services. The
    /// returned handle carries the initial durable resume binding.
    pub fn open_session(&self, services: HostServices) -> OpenSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan.clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    /// Builds an exact provider-session load request with bounded replay.
    pub fn load_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<LoadSessionRequest, PreparationFailure> {
        Ok(LoadSessionRequest::from_plan(
            &self.plan,
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared sidecar session binds a working resource")
                .clone(),
            self.request.deadline(),
        )?
        .with_options(self.request.options().clone()))
    }

    /// Builds an exact provider-session resume request without replay.
    pub fn resume_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<ResumeSessionRequest, PreparationFailure> {
        Ok(ResumeSessionRequest::from_plan(
            &self.plan,
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared sidecar session binds a working resource")
                .clone(),
            self.request.deadline(),
        )?
        .with_options(self.request.options().clone()))
    }

    /// Loads a bound provider session, returning typed replay plus the
    /// interactive handle.
    pub fn load_session(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<LoadSessionFuture, PreparationFailure> {
        let request = self.load_request(request_id, binding)?;
        let driver = self.low_level_driver();
        let plan = self.plan.clone();
        Ok(Box::pin(async move {
            driver.load_session(plan, request, services).await
        }))
    }

    /// Resumes a bound provider session without replaying prior content.
    pub fn resume_session(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<OpenSessionFuture, PreparationFailure> {
        let request = self.resume_request(request_id, binding)?;
        let driver = self.low_level_driver();
        let plan = self.plan.clone();
        Ok(Box::pin(async move {
            driver.resume_session(plan, request, services).await
        }))
    }
}

pub(super) fn preparation_failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    preparation_failure(PreparationStage::TargetSelection, code, message)
}

/// Prepares one persistent Pi SDK sidecar session from explicit inputs.
pub fn prepare_pi_sdk_sidecar_session(
    input: PiSdkSidecarSessionPreparation,
    options: SessionOptions,
) -> Result<PiSdkSidecarPreparedSession, PreparationFailure> {
    super::reasoning::validate_options(&options)?;
    let mut capability_requirements = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::LoadSession,
            [
                CapabilityConstraint::ReplayMaximumItems(
                    super::replay::MAXIMUM_REPLAY_ITEMS as u32,
                ),
                CapabilityConstraint::ReplayMaximumBytes(
                    super::replay::MAXIMUM_REPLAY_BYTES as u64,
                ),
            ],
        ),
        CapabilityRequirement::new(Capability::Resume, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if input.image_attachments {
        capability_requirements.push(CapabilityRequirement::new(
            Capability::Attachments,
            [
                CapabilityConstraint::attachment_media_type("image/png")
                    .expect("static media type is valid"),
                CapabilityConstraint::AttachmentMaximumBytes(1024 * 1024),
                CapabilityConstraint::AttachmentMaximumCount(1),
            ],
        ));
    }
    if let Some(reasoning) = options.reasoning_mode() {
        super::reasoning::validate_preparation(&input.provider, &input.model, reasoning)?;
        capability_requirements.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(reasoning.clone())],
        ));
    }
    let capabilities = CapabilityProfile::new(capability_requirements.clone());
    let versions = [
        pi_sdk_sidecar_package_binding(PI_SDK_SIDECAR_SDK_VERSION),
        pi_sdk_sidecar_node_binding(PI_SDK_SIDECAR_NODE_RUNTIME),
        pi_sdk_sidecar_wire_binding(PI_SDK_SIDECAR_WIRE),
        pi_sdk_sidecar_sidecar_binding(PI_SDK_SIDECAR_SOURCE_TAG),
    ];
    let versions = versions
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .expect("static sidecar version bindings are valid");
    let one = NonZeroU32::new(1).expect("one is non-zero");
    let rpc_policy = HarnessRpcPolicy::restrictive(HarnessSchedulingBounds::new(
        one,
        NonZeroU32::new(2).expect("two is non-zero"),
        one,
        one,
    ));
    let descriptor = super::pi_sdk_sidecar_descriptor();
    let instance = ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision,
        descriptor.identity().id().clone(),
        input.execution_host_id.clone(),
        input.target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("pi-sdk-sidecar-jsonl-v1").expect("static facade is valid"),
        InstancePolicyId::new("pi-sdk-sidecar-ambient-read").expect("static policy is valid"),
        capabilities.clone(),
    )
    .with_interface_versions(versions.clone())
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_harness_rpc_policy(rpc_policy.clone());
    let route = ModelRoute::new(
        input.route_id,
        input.route_revision,
        input.instance_id,
        input.model,
        capabilities,
    )
    .with_provider_id(input.provider);
    let access = AccessProfile::new(
        input.access_profile_id.clone(),
        CredentialMechanism::ProviderSpecific(
            ExtensionNamespace::new("pi/delegated-harness-auth").expect("static namespace"),
        ),
        EntitlementMetering::Unknown,
        EndpointAudience::new("pi-harness").expect("static audience"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(input.credential.clone());
    let status = AccessStatus::new(
        input.access_profile_id,
        CredentialState::Ready,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let services = [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
        HostServiceKind::Time,
    ];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        input.execution_host_id.clone(),
        AccessRequirement::new(access.id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services)
    .with_capabilities(capability_requirements)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_interface_versions(versions)
    .with_harness_rpc_policy(rpc_policy)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    .with_session_provider_state_policy(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    .require_model_route();
    let plan = swallowtail_runtime::build_plan(
        &descriptor,
        &instance,
        Some(&route),
        &requirements,
        &access,
        &status,
        services,
    )?;
    let request =
        OpenSessionRequest::from_plan(&plan, input.request_id, input.working_resource, None)?
            .with_options(options);
    Ok(PiSdkSidecarPreparedSession {
        plan,
        request,
        environment: input.environment,
        credential: input.credential,
    })
}
