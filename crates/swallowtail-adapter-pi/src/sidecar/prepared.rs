//! Persistent-session prepared facade for the Pi SDK sidecar route.
//!
//! The prepared session binds the four exact interface-version points, the
//! host-approved launch recipe target, the delegated harness credential, the
//! exact provider and model route, and the durable provider-state posture.
//! From it, consumers open new sessions (which return the initial durable
//! binding), or load/resume previously bound provider sessions through the
//! runtime's binding machinery.

mod build;

use super::driver::PiSdkSidecarDriver;
use swallowtail_core::{
    AccessProfileId, ConfigFieldId, ConfiguredInstanceId, CredentialFieldId, CredentialRef,
    ExecutionHostId, InstanceRevision, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, PreflightPlan, ProviderId,
};
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    LoadSessionRequest, LoadedSession, OpenSessionRequest, PreparationFailure, PreparationStage,
    RequestId, ResumeSessionRequest, RuntimeFailure, SessionOptions, SessionResumeBinding,
    WorkingResourceRef,
};

/// Explicit inputs for preparing one persistent sidecar session.
pub struct PiSdkSidecarSessionPreparation {
    pub(crate) instance_id: ConfiguredInstanceId,
    pub(crate) instance_revision: InstanceRevision,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) target: InstanceTargetRef,
    pub(crate) environment: EnvironmentRef,
    pub(crate) credential: CredentialRef,
    pub(crate) access_profile_id: AccessProfileId,
    pub(crate) route_id: ModelRouteId,
    pub(crate) route_revision: ModelRouteRevision,
    pub(crate) provider: ProviderId,
    pub(crate) model: ModelId,
    pub(crate) working_resource: WorkingResourceRef,
    pub(crate) request_id: RequestId,
    pub(crate) image_attachments: bool,
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
    build::prepare(input, options)
}
