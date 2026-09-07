//! Prepared-session facade for the Claude Agent SDK sidecar route.
//!
//! The prepared session binds the five exact interface-version points, the
//! host-approved launch recipe target, the delegated subscription credential
//! reference, the exact model route, and the read-only ambient resource
//! posture. Preparation always creates a fresh route binding; the resulting
//! prepared session can separately attach an explicitly supplied resume
//! binding, but never turns listing metadata into authority.

mod build;

use super::driver::{ClaudeAgentSdkDriver, ClaudeAgentSdkSessionHandle};
use super::mcp::{ClaudeAgentSdkMcpBinding, ClaudeAgentSdkMcpServer};
use super::profile::ClaudeAgentSdkSessionProfile;
use super::registered_tool::ClaudeAgentSdkRegisteredToolBinding;
use swallowtail_core::{
    AccessProfileId, ConfigFieldId, ConfiguredInstanceId, CredentialFieldId, CredentialRef,
    Diagnostic, ExecutionHostId, InstanceRevision, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, PreflightPlan,
};
use swallowtail_host_local::LocalHostServices;
use swallowtail_runtime::{
    BoxFuture, Deadline, EnvironmentRef, HostServices, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparationStage, RegisteredToolPreparation, RequestId,
    ResumeSessionRequest, RuntimeFailure, SessionOptions, SessionResumeBinding, WorkingResourceRef,
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
    pub(crate) profile: ClaudeAgentSdkSessionProfile,
    pub(crate) mcp_servers: Vec<ClaudeAgentSdkMcpServer>,
    pub(crate) registered_tools: Option<ClaudeAgentSdkRegisteredToolBinding>,
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
            profile: ClaudeAgentSdkSessionProfile::read_only(),
            mcp_servers: Vec::new(),
            registered_tools: None,
        }
    }

    /// Replaces the admitted tool set and opening permission mode.
    ///
    /// Omitting this keeps the unchanged read-only profile, so an existing
    /// caller's prepared plan, request, and sidecar options are identical to
    /// `v0.4.0`. A profile admitting a write tool binds a read-write
    /// working-resource lease into the plan, and the host's own lease must
    /// match it before the sidecar starts.
    #[must_use]
    pub const fn with_session_profile(mut self, profile: ClaudeAgentSdkSessionProfile) -> Self {
        self.profile = profile;
        self
    }

    /// Binds declared stdio MCP servers alongside the Copy session profile.
    #[must_use]
    pub fn with_mcp_binding(mut self, binding: ClaudeAgentSdkMcpBinding) -> Self {
        self.profile = binding.session_profile();
        self.mcp_servers = binding.servers().to_vec();
        self
    }

    /// Qualifies one registered-tool preparation and binds the host that
    /// resolves the approved courier path and environment.
    ///
    /// This is the openable consumer entry. Absence preserves every previous
    /// open. The binding is a separate path from card 084 consumer-declared
    /// servers: neither may present the other's reserved name.
    pub fn with_registered_tools(
        mut self,
        preparation: RegisteredToolPreparation,
        host: LocalHostServices,
    ) -> Result<Self, PreparationFailure> {
        self.registered_tools = Some(
            ClaudeAgentSdkRegisteredToolBinding::qualify(preparation)
                .map_err(|error| {
                    PreparationFailure::new(
                        PreparationStage::Preflight,
                        Diagnostic::new(error.diagnostic().clone()),
                    )
                })?
                .with_host(host),
        );
        Ok(self)
    }

    /// Binds one already-qualified registered-tool selection.
    #[must_use]
    pub fn with_registered_tool_binding(
        mut self,
        binding: ClaudeAgentSdkRegisteredToolBinding,
    ) -> Self {
        self.registered_tools = Some(binding);
        self
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

type RouteSessionFuture = BoxFuture<'static, Result<ClaudeAgentSdkSessionHandle, RuntimeFailure>>;

/// One bounded provider-owned session record returned by the route-local
/// listing query. It is metadata only and is never resume authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkSessionListing {
    provider_session_ref: swallowtail_core::SessionRef,
    cwd: String,
    created_at_unix_milliseconds: Option<u64>,
    last_modified_unix_milliseconds: u64,
    title: Option<String>,
}

impl ClaudeAgentSdkSessionListing {
    pub(crate) fn from_parts(entry: super::driver::SessionListing) -> Self {
        Self {
            provider_session_ref: entry.provider_session_ref,
            cwd: entry.cwd,
            created_at_unix_milliseconds: entry.created_at_unix_milliseconds,
            last_modified_unix_milliseconds: entry.last_modified_unix_milliseconds,
            title: entry.title,
        }
    }

    /// Returns the opaque provider session identity.
    #[must_use]
    pub const fn provider_session_ref(&self) -> &swallowtail_core::SessionRef {
        &self.provider_session_ref
    }

    /// Returns the leased cwd used for this listing.
    #[must_use]
    pub fn cwd(&self) -> &str {
        &self.cwd
    }

    /// Returns the provider-reported creation time when available.
    #[must_use]
    pub const fn created_at_unix_milliseconds(&self) -> Option<u64> {
        self.created_at_unix_milliseconds
    }

    /// Returns the provider-reported last-modified time.
    #[must_use]
    pub const fn last_modified_unix_milliseconds(&self) -> u64 {
        self.last_modified_unix_milliseconds
    }

    /// Returns the bounded display title when available.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

/// A prepared fresh sidecar session: validated plan plus bound request.
pub struct ClaudeAgentSdkPreparedSession {
    plan: PreflightPlan,
    request: OpenSessionRequest,
    environment: EnvironmentRef,
    credential: CredentialRef,
    profile: ClaudeAgentSdkSessionProfile,
    mcp_servers: Vec<ClaudeAgentSdkMcpServer>,
    registered_tools: Option<ClaudeAgentSdkRegisteredToolBinding>,
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

    /// Returns the admitted tool set and opening permission mode this session
    /// was prepared with.
    ///
    /// This is prepared evidence: it is exactly what the sidecar receives as
    /// `tools` and `permissionMode`, and what open re-verifies from the
    /// sidecar's own echo.
    #[must_use]
    pub const fn session_profile(&self) -> ClaudeAgentSdkSessionProfile {
        self.profile
    }

    /// Returns the declared stdio MCP servers this session was prepared with.
    #[must_use]
    pub fn mcp_servers(&self) -> &[ClaudeAgentSdkMcpServer] {
        &self.mcp_servers
    }

    /// Returns the qualified registered-tool binding, when this session opted in.
    #[must_use]
    pub const fn registered_tools(&self) -> Option<&ClaudeAgentSdkRegisteredToolBinding> {
        self.registered_tools.as_ref()
    }

    /// Creates the low-level sidecar driver bound to this session.
    #[must_use]
    pub fn low_level_driver(&self) -> ClaudeAgentSdkDriver {
        let driver = ClaudeAgentSdkDriver::new(self.environment.clone(), self.credential.clone())
            .with_session_profile(self.profile)
            .with_mcp_servers(self.mcp_servers.clone());
        match self.registered_tools.clone() {
            Some(binding) => driver.with_registered_tools(binding),
            None => driver,
        }
    }

    /// Opens a fresh provider session with caller-supplied host services.
    pub fn open_session(&self, services: HostServices) -> OpenSessionFuture {
        let opened = self.open_route_session(services);
        Box::pin(async move {
            opened
                .await
                .map(|handle| Box::new(handle) as Box<dyn InteractiveSessionHandle>)
        })
    }

    /// Opens the same fresh session and returns the route-local handle.
    ///
    /// The shared trait object carries the ordinary session surface. This
    /// concrete handle adds the route-local mid-session permission-mode
    /// control, which no provider-neutral trait declares.
    pub fn open_route_session(&self, services: HostServices) -> RouteSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan.clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_route_session(plan, request, services).await })
    }

    /// Builds an exact Contract 017 resume request without replay.
    pub fn resume_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<ResumeSessionRequest, PreparationFailure> {
        ResumeSessionRequest::from_plan(
            &self.plan,
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Claude Agent SDK session binds a working resource")
                .clone(),
            self.request.deadline(),
        )
        .map_err(|_| {
            preparation_failure(
                PreparationStage::Preflight,
                "swallowtail.claude-agent.sdk.preparation.resume_request_invalid",
                "Claude Agent SDK resume request did not match the prepared session",
            )
        })
    }

    /// Resumes a retained provider session without replaying its transcript.
    pub fn resume_session(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<RouteSessionFuture, PreparationFailure> {
        let request = self.resume_request(request_id, binding)?;
        let driver = self.low_level_driver();
        let plan = self.plan.clone();
        Ok(Box::pin(async move {
            driver
                .resume_route_session(plan, request, services, None)
                .await
        }))
    }

    /// Resumes at one additive message boundary without replaying prior
    /// transcript content.
    pub fn resume_session_at(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        message_boundary: impl Into<String>,
        services: HostServices,
    ) -> Result<RouteSessionFuture, PreparationFailure> {
        let request = self.resume_request(request_id, binding)?;
        let message_boundary = message_boundary.into();
        let driver = self.low_level_driver();
        let plan = self.plan.clone();
        Ok(Box::pin(async move {
            driver
                .resume_route_session(plan, request, services, Some(message_boundary))
                .await
        }))
    }

    /// Lists bounded provider-owned session metadata for the leased cwd.
    /// Listing is not a resume or attachment authority.
    pub fn list_sessions(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<ClaudeAgentSdkSessionListing>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan.clone();
        let request_id = self.request.request_id().clone();
        let working_resource = self
            .request
            .working_resource()
            .expect("prepared Claude Agent SDK session binds a working resource")
            .clone();
        let deadline = self.request.deadline().expect("prepared session deadline");
        Box::pin(async move {
            driver
                .list_sessions(plan, request_id, working_resource, deadline, services)
                .await
        })
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
    profile: ClaudeAgentSdkSessionProfile,
    mcp_servers: Vec<ClaudeAgentSdkMcpServer>,
    registered_tools: Option<ClaudeAgentSdkRegisteredToolBinding>,
) -> ClaudeAgentSdkPreparedSession {
    ClaudeAgentSdkPreparedSession {
        plan,
        request,
        environment,
        credential,
        profile,
        mcp_servers,
        registered_tools,
    }
}

/// Prepares one fresh Claude Agent SDK sidecar session from explicit inputs.
pub fn prepare_claude_agent_sdk_session(
    input: ClaudeAgentSdkSessionPreparation,
    options: SessionOptions,
) -> Result<ClaudeAgentSdkPreparedSession, PreparationFailure> {
    build::prepare(input, options)
}
