use crate::connection::AcpConnection;
use crate::failure::{failure, malformed, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, Capability,
    CapabilityConstraint, CredentialMechanism, CredentialRef, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan,
    ResourceAccess, ResourceRepresentation, SessionAccessPolicy, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    CredentialLease, EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, LoadSessionRequest, LoadedSession,
    NegotiatedSessionModelOption, NegotiatedSessionModelOptions, OpenSessionRequest, ProcessHandle,
    ProcessRequest, RequestId, ResourceLease, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, ScopeId, SessionLifecycleOperation, SessionResumeBinding, TerminalOutcome,
    TurnHandle, TurnRequest, prepare_negotiated_reasoning_setup, validate_session_access_plan,
    validate_session_plan_agreement, validate_session_resource_lease,
};

impl InteractiveSessionDriver for KimiAcpDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_session_plan_agreement(&plan, request.plan_agreement())?;
            let selected = self.validate_plan(&plan)?;
            let reasoning = prepare_negotiated_reasoning_setup(
                &plan,
                SessionLifecycleOperation::Open,
                request.options(),
            )?;
            validate_request(
                &plan,
                request.access_policy(),
                request.deadline(),
                request.options(),
                &services,
            )?;
            let request_id = request.request_id().clone();
            let working_resource = request
                .working_resource()
                .ok_or_else(|| unsupported("resource-free session"))?
                .clone();
            let access_policy = request.access_policy().clone();
            let mut attachment = self
                .start_attachment(&plan, &request_id, working_resource.clone(), &services)
                .await?;
            let opened = async {
                let initialize = attachment.connection.initialize().await?;
                validate_initialize(&initialize, selected.version().as_str())?;
                let response = attachment
                    .connection
                    .new_session(attachment.cwd.clone())
                    .await?;
                let (provider_id, model_options) =
                    parse_session(&response, plan.model_id().expect("validated model"))?;
                attachment.connection.set_session_id(provider_id.clone())?;
                if let Some(reasoning) = reasoning {
                    let selection = reasoning::prepare_reasoning_selection(
                        &response,
                        selected.behavior(),
                        reasoning,
                    )?;
                    let confirmation = attachment
                        .connection
                        .set_config_option(&provider_id, selection.provider_value())
                        .await?;
                    let _ = selection.confirm(&confirmation, selected.behavior())?;
                }
                let provider_ref = SessionRef::new(&provider_id).map_err(|_| malformed())?;
                let binding = SessionResumeBinding::new(
                    provider_ref.clone(),
                    plan.instance_id().clone(),
                    plan.execution_host_id().clone(),
                    plan.model_route_id().expect("validated route").clone(),
                    plan.model_id().expect("validated model").clone(),
                    working_resource,
                    access_policy,
                );
                attachment.take_session(
                    request_id,
                    provider_ref,
                    provider_id,
                    binding,
                    model_options,
                    &services,
                )
            }
            .await;
            match opened {
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
            validate_session_plan_agreement(&plan, request.plan_agreement())?;
            let selected = self.validate_plan(&plan)?;
            let _ = prepare_negotiated_reasoning_setup(
                &plan,
                SessionLifecycleOperation::Load,
                request.options(),
            )?;
            require_capability(&plan, Capability::LoadSession)?;
            require_constraint(
                &plan,
                Capability::LoadSession,
                &CapabilityConstraint::ReplayMaximumItems(crate::MAXIMUM_REPLAY_ITEMS as u32),
            )?;
            require_constraint(
                &plan,
                Capability::LoadSession,
                &CapabilityConstraint::ReplayMaximumBytes(crate::MAXIMUM_REPLAY_BYTES as u64),
            )?;
            validate_bound_request(
                &plan,
                request.resume_binding(),
                request.working_resource(),
                request.access_policy(),
            )?;
            validate_request(
                &plan,
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
                    &services,
                )
                .await?;
            let loaded = async {
                validate_initialize(
                    &attachment.connection.initialize().await?,
                    selected.version().as_str(),
                )?;
                let provider_ref = request.provider_session_ref().clone();
                let (response, replay) = attachment
                    .connection
                    .load_session(provider_ref.clone(), attachment.cwd.clone())
                    .await?;
                validate_session_configuration(
                    &response,
                    plan.model_id().expect("validated model"),
                )?;
                let model_options = parse_model_options(&response)?;
                let session = attachment.take_session(
                    request.request_id().clone(),
                    provider_ref.clone(),
                    provider_ref.as_provider_value().to_owned(),
                    request.resume_binding().clone(),
                    model_options,
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

    fn resume_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_session_plan_agreement(&plan, request.plan_agreement())?;
            let selected = self.validate_plan(&plan)?;
            let _ = prepare_negotiated_reasoning_setup(
                &plan,
                SessionLifecycleOperation::Resume,
                request.options(),
            )?;
            require_capability(&plan, Capability::Resume)?;
            validate_bound_request(
                &plan,
                request.resume_binding(),
                request.working_resource(),
                request.access_policy(),
            )?;
            validate_request(
                &plan,
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
                    &services,
                )
                .await?;
            let resumed = async {
                validate_initialize(
                    &attachment.connection.initialize().await?,
                    selected.version().as_str(),
                )?;
                let provider_ref = request.provider_session_ref().clone();
                let response = attachment
                    .connection
                    .resume_session(provider_ref.clone(), attachment.cwd.clone())
                    .await?;
                validate_session_configuration(
                    &response,
                    plan.model_id().expect("validated model"),
                )?;
                let model_options = parse_model_options(&response)?;
                attachment.take_session(
                    request.request_id().clone(),
                    provider_ref.clone(),
                    provider_ref.as_provider_value().to_owned(),
                    request.resume_binding().clone(),
                    model_options,
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
}

include!("driver/access.rs");
include!("driver/descriptor.rs");
include!("driver/plan.rs");
include!("driver/session.rs");
include!("driver/validation.rs");
