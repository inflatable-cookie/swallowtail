//! Prepared-session facade for the Claude Agent SDK sidecar route.
//!
//! The prepared session binds the five exact interface-version points, the
//! host-approved launch recipe target, the delegated subscription credential
//! reference, the exact model route, and the read-only ambient resource
//! posture. Only fresh sessions are prepared here: resume, fork, and session
//! management are later layers and are not smuggled in through preparation.

mod build;

use super::driver::ClaudeAgentSdkDriver;
use swallowtail_core::{
    AccessProfileId, ConfigFieldId, ConfiguredInstanceId, CredentialFieldId, CredentialRef,
    ExecutionHostId, InstanceRevision, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, EnvironmentRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, PreparationFailure, PreparationStage, RequestId,
    RuntimeFailure, SessionOptions, WorkingResourceRef,
};

/// Explicit inputs for preparing one fresh Claude Agent SDK sidecar session.
///
/// `deadline` is caller-supplied and mandatory: it bounds open and every
/// startup await against the host clock. Close carries no caller deadline on
/// the shared session seam, and monotonic tick units are host-defined, so this
/// route does not derive a close bound from it.
pub struct ClaudeAgentSdkSessionPreparation {
    pub(crate) instance_id: ConfiguredInstanceId,
    pub(crate) instance_revision: InstanceRevision,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) target: InstanceTargetRef,
    pub(crate) environment: EnvironmentRef,
    pub(crate) credential: CredentialRef,
    pub(crate) access_profile_id: AccessProfileId,
    pub(crate) route_id: ModelRouteId,
    pub(crate) route_revision: ModelRouteRevision,
    pub(crate) model: ModelId,
    pub(crate) working_resource: WorkingResourceRef,
    pub(crate) request_id: RequestId,
    pub(crate) deadline: Deadline,
}

impl ClaudeAgentSdkSessionPreparation {
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
        model: ModelId,
        working_resource: WorkingResourceRef,
        request_id: RequestId,
        deadline: Deadline,
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
            model,
            working_resource,
            request_id,
            deadline,
        }
    }

    /// Builds session preparation input from one admitted SDK sidecar route
    /// record plus the explicit per-session model, resource, and request.
    #[allow(clippy::too_many_arguments)]
    pub fn from_admitted(
        admitted: &swallowtail_core::AdmittedInstanceRecord,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        access_profile_id: AccessProfileId,
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model: ModelId,
        working_resource: WorkingResourceRef,
        request_id: RequestId,
        deadline: Deadline,
    ) -> Result<Self, PreparationFailure> {
        if admitted.route_id().as_str() != super::CLAUDE_AGENT_SDK_ADDABLE_ROUTE_ID {
            return Err(failure(
                "swallowtail.claude-agent.sdk.preparation.route_mismatch",
                "Claude Agent SDK preparation requires the admitted SDK sidecar route",
            ));
        }
        if admitted.driver() != super::claude_agent_sdk_descriptor().identity() {
            return Err(failure(
                "swallowtail.claude-agent.sdk.preparation.driver_mismatch",
                "Claude Agent SDK preparation requires the sidecar driver identity",
            ));
        }
        let launch_field = ConfigFieldId::new(super::CLAUDE_AGENT_SDK_LAUNCH_RECIPE_FIELD_ID)
            .expect("static config field id is valid");
        let launch_recipe = admitted.config_ref(&launch_field).ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.preparation.launch_recipe_missing",
                "Claude Agent SDK preparation requires the admitted launch recipe reference",
            )
        })?;
        let environment_field = ConfigFieldId::new(super::CLAUDE_AGENT_SDK_ENVIRONMENT_FIELD_ID)
            .expect("static config field id is valid");
        let environment = admitted.config_ref(&environment_field).ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.preparation.environment_ref_missing",
                "Claude Agent SDK preparation requires the admitted environment reference",
            )
        })?;
        let credential_field = CredentialFieldId::new(super::CLAUDE_AGENT_SDK_CREDENTIAL_FIELD_ID)
            .expect("static credential field id is valid");
        let credential = admitted.credential_ref(&credential_field).ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.preparation.credential_ref_missing",
                "Claude Agent SDK preparation requires the admitted credential reference",
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
            model,
            working_resource,
            request_id,
            deadline,
        ))
    }
}

type OpenSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;

/// A prepared fresh sidecar session: validated plan plus bound request.
pub struct ClaudeAgentSdkPreparedSession {
    plan: PreflightPlan,
    request: OpenSessionRequest,
    environment: EnvironmentRef,
    credential: CredentialRef,
}

impl ClaudeAgentSdkPreparedSession {
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
    pub fn low_level_driver(&self) -> ClaudeAgentSdkDriver {
        ClaudeAgentSdkDriver::new(self.environment.clone(), self.credential.clone())
    }

    /// Opens a fresh provider session with caller-supplied host services.
    pub fn open_session(&self, services: HostServices) -> OpenSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan.clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
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

pub(super) fn build_prepared(
    plan: PreflightPlan,
    request: OpenSessionRequest,
    environment: EnvironmentRef,
    credential: CredentialRef,
) -> ClaudeAgentSdkPreparedSession {
    ClaudeAgentSdkPreparedSession {
        plan,
        request,
        environment,
        credential,
    }
}

/// Prepares one fresh Claude Agent SDK sidecar session from explicit inputs.
pub fn prepare_claude_agent_sdk_session(
    input: ClaudeAgentSdkSessionPreparation,
    options: SessionOptions,
) -> Result<ClaudeAgentSdkPreparedSession, PreparationFailure> {
    build::prepare(input, options)
}
