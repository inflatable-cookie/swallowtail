use crate::connection::AcpConnection;
use crate::failure::{failure, malformed, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, Capability,
    CapabilityConstraint, CredentialMechanism, CredentialRef, DriverDescriptor, DriverRole,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    IntegrationFamilyId, OperationShape, PreflightPlan, ResourceAccess, ResourceRepresentation,
    SessionAccessPolicy, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    CredentialLease, EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, LoadSessionRequest, LoadedSession, OpenSessionRequest,
    ProcessHandle, ProcessRequest, RequestId, ResourceLease, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, ScopeId, SessionLifecycleOperation, SessionResumeBinding, TerminalOutcome,
    TurnHandle, TurnRequest, prepare_negotiated_reasoning_setup, validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.claude-agent.acp";
const ENDPOINT_AUDIENCE: &str = "api.anthropic.com";

use self::validation::{validate_attachment, validate_open, validate_plan};

pub struct ClaudeAgentAcpDriver {
    environment: EnvironmentRef,
    credential: Option<CredentialRef>,
}

impl ClaudeAgentAcpDriver {
    /// Uses one host-approved Anthropic API-key credential reference.
    #[must_use]
    pub const fn new(environment: EnvironmentRef, credential: CredentialRef) -> Self {
        Self {
            environment,
            credential: Some(credential),
        }
    }

    /// Uses authentication already held by the locally installed Claude harness.
    #[must_use]
    pub const fn with_local_auth(environment: EnvironmentRef) -> Self {
        Self {
            environment,
            credential: None,
        }
    }
}

impl InteractiveSessionDriver for ClaudeAgentAcpDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let selected = validate_plan(&plan, self.credential.as_ref())?;
            let reasoning = prepare_negotiated_reasoning_setup(
                &plan,
                SessionLifecycleOperation::Open,
                request.options(),
            )?
            .map(|setup| setup.requested().clone());
            if reasoning.is_some() && !selected.behavior().supports_config_options() {
                return Err(unsupported("reasoning selection for this adapter version"));
            }
            validate_open(&plan, &request, &services)?;
            let session = self
                .start_session(&plan, &request, &services, selected, reasoning)
                .await?;
            Ok(Box::new(session) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn resume_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let selected = validate_plan(&plan, self.credential.as_ref())?;
            require_continuity(&plan, Capability::Resume)?;
            validate_attachment(
                &plan,
                request.resume_binding(),
                request.working_resource(),
                request.access_policy(),
                request.deadline(),
                request.options(),
                &services,
            )?;
            let mut attachment = self
                .start_attachment(
                    &plan,
                    request.request_id(),
                    request.working_resource().clone(),
                    request.access_policy(),
                    &services,
                )
                .await?;
            let resumed = async {
                let lifecycle = validation::validate_initialize(
                    &attachment.connection.initialize().await?,
                    &selected,
                )?;
                let provider_ref = request.provider_session_ref().clone();
                let response = attachment
                    .connection
                    .resume_session(provider_ref.clone(), attachment.cwd.clone())
                    .await?;
                validate_attached_response(&response, &plan, &provider_ref, &selected)?;
                attachment.take_handle(
                    access::SessionHandleInput {
                        request_id: request.request_id().clone(),
                        provider_ref: provider_ref.clone(),
                        binding: request.resume_binding().clone(),
                        provider_requests: request.access_policy().provider_requests().clone(),
                        execution_host_id: plan.execution_host_id().clone(),
                        native_close: lifecycle.close && selected.is_qualified(),
                        native_delete: lifecycle.delete && selected.is_qualified(),
                    },
                    &services,
                )
            }
            .await;
            match resumed {
                Ok(session) => Ok(Box::new(session) as Box<dyn InteractiveSessionHandle>),
                Err(error) => {
                    let _ = attachment.abort(&services).await;
                    Err(error)
                }
            }
        })
    }

    fn load_session(
        &self,
        plan: PreflightPlan,
        request: LoadSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        Box::pin(async move {
            let selected = validate_plan(&plan, self.credential.as_ref())?;
            require_continuity(&plan, Capability::LoadSession)?;
            let working_resource = request
                .working_resource()
                .ok_or_else(|| unsupported("a resource-free session"))?;
            validate_attachment(
                &plan,
                request.resume_binding(),
                working_resource,
                request.access_policy(),
                request.deadline(),
                request.options(),
                &services,
            )?;
            let mut attachment = self
                .start_attachment(
                    &plan,
                    request.request_id(),
                    working_resource.clone(),
                    request.access_policy(),
                    &services,
                )
                .await?;
            let loaded = async {
                let lifecycle = validation::validate_initialize(
                    &attachment.connection.initialize().await?,
                    &selected,
                )?;
                let provider_ref = request.provider_session_ref().clone();
                let (response, replay) = attachment
                    .connection
                    .load_session(provider_ref.clone(), attachment.cwd.clone())
                    .await?;
                validate_attached_response(&response, &plan, &provider_ref, &selected)?;
                let session = attachment.take_handle(
                    access::SessionHandleInput {
                        request_id: request.request_id().clone(),
                        provider_ref: provider_ref.clone(),
                        binding: request.resume_binding().clone(),
                        provider_requests: request.access_policy().provider_requests().clone(),
                        execution_host_id: plan.execution_host_id().clone(),
                        native_close: lifecycle.close && selected.is_qualified(),
                        native_delete: lifecycle.delete && selected.is_qualified(),
                    },
                    &services,
                )?;
                Ok(LoadedSession::new(replay, Box::new(session)))
            }
            .await;
            match loaded {
                Ok(session) => Ok(session),
                Err(error) => {
                    let _ = attachment.abort(&services).await;
                    Err(error)
                }
            }
        })
    }
}

fn require_continuity(plan: &PreflightPlan, capability: Capability) -> Result<(), RuntimeFailure> {
    let requirement = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == capability)
        .ok_or_else(|| {
            failure(
                "swallowtail.claude_agent.acp.continuity_capability_mismatch",
                "Claude Agent continuity capability does not match its preflight plan",
            )
        })?;
    if capability == Capability::LoadSession
        && (!requirement.constraints().any(|constraint| {
            constraint
                == &CapabilityConstraint::ReplayMaximumItems(crate::MAXIMUM_REPLAY_ITEMS as u32)
        }) || !requirement.constraints().any(|constraint| {
            constraint
                == &CapabilityConstraint::ReplayMaximumBytes(crate::MAXIMUM_REPLAY_BYTES as u64)
        }))
    {
        return Err(failure(
            "swallowtail.claude_agent.acp.continuity_capability_mismatch",
            "Claude Agent continuity bounds do not match its preflight plan",
        ));
    }
    Ok(())
}

fn validate_attached_response(
    response: &Value,
    plan: &PreflightPlan,
    provider_ref: &SessionRef,
    selected: &crate::selection::ClaudeAgentPlanSelection,
) -> Result<(), RuntimeFailure> {
    if config::parse_session_id(response)?.as_str() != provider_ref.as_provider_value() {
        return Err(failure(
            "swallowtail.claude_agent.acp.session_mismatch",
            "Claude Agent attached a different provider session",
        ));
    }
    let model = plan.model_id().expect("validated model").as_str();
    if selected.behavior().supports_config_options() {
        config::confirm_model(response, model)
    } else {
        config::validate_legacy_model(response, model)
    }
}

mod access;
mod config;
mod descriptor;
mod handle;
mod run;
mod session;
mod session_management;
mod validation;

pub use descriptor::claude_agent_acp_descriptor;
