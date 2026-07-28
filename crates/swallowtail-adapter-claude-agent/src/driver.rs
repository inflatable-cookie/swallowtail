use crate::connection::AcpConnection;
use crate::failure::{failure, malformed, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, CredentialMechanism,
    CredentialRef, DriverDescriptor, DriverRole, ExecutionLayer, HarnessConfigurationPosture,
    HarnessIsolation, HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan,
    ResourceAccess, ResourceRepresentation, SessionAccessPolicy, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    CredentialLease, EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, OpenSessionRequest, ProcessHandle, ProcessRequest,
    RequestId, ResourceLease, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId,
    SessionLifecycleOperation, TerminalOutcome, TurnHandle, TurnRequest,
    prepare_negotiated_reasoning_setup, validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.claude-agent.acp";
const ENDPOINT_AUDIENCE: &str = "api.anthropic.com";

use self::validation::{validate_open, validate_plan};

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
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("session resume")) })
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
