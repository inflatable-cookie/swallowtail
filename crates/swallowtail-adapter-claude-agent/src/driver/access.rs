use super::*;
use crate::driver::handle::SessionCancellation;
use crate::driver::session::{ClaudeAgentSessionHandle, cleanup_failure, merge_cleanup};
use crate::driver::validation::validate_initialize;
use crate::selection::ClaudeAgentPlanSelection;
use swallowtail_core::{HarnessMode, ReasoningMode};

use super::{
    ClaudeAgentOpenRejection, ClaudeAgentReasoningAcknowledgement, config::ReasoningConfirmation,
};

pub(super) struct PendingSession {
    pub(super) connection: Arc<AcpConnection>,
    pump_task: Option<Box<dyn JoinedTask>>,
    resource: Option<ResourceLease>,
    credential: Option<CredentialLease>,
    pub(super) cwd: String,
}

impl ClaudeAgentAcpDriver {
    pub(super) async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        selected: ClaudeAgentPlanSelection,
        reasoning: Option<ReasoningMode>,
    ) -> Result<
        (
            ClaudeAgentSessionHandle,
            ClaudeAgentReasoningAcknowledgement,
        ),
        ClaudeAgentOpenRejection,
    > {
        let working_resource = request
            .working_resource()
            .expect("validated working resource")
            .clone();
        let resource_access = request
            .access_policy()
            .resource_access()
            .expect("validated working-resource access");
        let mut pending = self
            .start_attachment(
                plan,
                request.request_id(),
                working_resource,
                request.access_policy(),
                services,
            )
            .await?;
        let opened: Result<_, ClaudeAgentOpenRejection> = async {
            let mut acknowledgement = ClaudeAgentReasoningAcknowledgement::NotRequested;
            let initialized = pending
                .connection
                .initialize()
                .await
                .map_err(ClaudeAgentOpenRejection::from)?;
            let lifecycle = validate_initialize(&initialized, &selected)?;
            let owned_session_cleanup =
                plan.requirements().operation_shape() == OperationShape::StructuredRun
                    && crate::driver::validation::run_owns_session_cleanup(plan)?;
            if owned_session_cleanup
                && (!lifecycle.close || !lifecycle.delete || !selected.is_qualified())
            {
                return Err(failure(
                    "swallowtail.claude_agent.acp.owned_cleanup_unavailable",
                    "Claude Agent did not negotiate the qualified close and delete lifecycle required by this run",
                )
                .into());
            }
            let model = plan.model_id().expect("validated model").as_str();
            let response = pending
                .connection
                .new_session(pending.cwd.clone(), model)
                .await?;
            let provider_id = crate::driver::config::parse_session_id(&response)?;
            pending.connection.set_session_id(provider_id.clone())?;
            if selected.behavior().supports_config_options() {
                crate::driver::config::validate_model_option(&response)?;
                let configured = pending
                    .connection
                    .set_config_option(&provider_id, "model", model)
                    .await?;
                crate::driver::config::confirm_model(&configured, model)?;
                if let Some(reasoning) = reasoning.as_ref() {
                    crate::driver::config::validate_reasoning_option(&configured, reasoning)?;
                    let confirmed = pending
                        .connection
                        .set_config_option(&provider_id, "effort", reasoning.as_str())
                        .await?;
                    acknowledgement = match crate::driver::config::confirm_reasoning(
                        &confirmed,
                        reasoning,
                    )? {
                        ReasoningConfirmation::Effective(value) => {
                            ClaudeAgentReasoningAcknowledgement::Effective(value)
                        }
                        ReasoningConfirmation::Rejected(value) => {
                            return Err(ClaudeAgentOpenRejection::rejected(
                                failure(
                                    "swallowtail.claude_agent.acp.reasoning_mismatch",
                                    "Claude Agent reasoning confirmation does not match the requested mode",
                                ),
                                value,
                            ));
                        }
                    };
                }
                if request.options().harness_mode() == Some(HarnessMode::Plan) {
                    crate::driver::config::validate_plan_mode_option(&configured)?;
                    let confirmed = pending
                        .connection
                        .set_config_option(&provider_id, "mode", "plan")
                        .await?;
                    crate::driver::config::confirm_plan_mode(&confirmed)?;
                }
            } else {
                crate::driver::config::validate_legacy_model(&response, model)?;
            }
            if resource_access == ResourceAccess::ReadWrite {
                crate::driver::config::validate_write_mode(&response)?;
                pending
                    .connection
                    .set_session_mode(&provider_id, "acceptEdits")
                    .await?;
            }
            let provider_ref = SessionRef::new(&provider_id).map_err(|_| malformed())?;
            let binding = SessionResumeBinding::new(
                provider_ref.clone(),
                plan.instance_id().clone(),
                plan.execution_host_id().clone(),
                plan.model_route_id().expect("validated route").clone(),
                plan.model_id().expect("validated model").clone(),
                request
                    .working_resource()
                    .expect("validated resource")
                    .clone(),
                request.access_policy().clone(),
            );
            let handle = pending.take_handle(
                SessionHandleInput {
                    request_id: request.request_id().clone(),
                    provider_ref,
                    binding,
                    provider_requests: request.access_policy().provider_requests().clone(),
                    execution_host_id: plan.execution_host_id().clone(),
                    native_close: lifecycle.close && selected.is_qualified(),
                    native_delete: lifecycle.delete && selected.is_qualified(),
                },
                services,
            )?;
            Ok::<_, ClaudeAgentOpenRejection>((handle, acknowledgement))
        }
        .await;
        match opened {
            Ok(opened) => Ok(opened),
            Err(error) => {
                let _ = pending.abort(services).await;
                Err(error)
            }
        }
    }

    pub(super) async fn start_attachment(
        &self,
        plan: &PreflightPlan,
        request_id: &RequestId,
        working_resource: swallowtail_runtime::WorkingResourceRef,
        access_policy: &SessionAccessPolicy,
        services: &HostServices,
    ) -> Result<PendingSession, RuntimeFailure> {
        let scope = ScopeId::new(format!("claude-agent-acp:session:{}", request_id.as_str()))
            .map_err(|_| malformed())?;
        let mut credential = match self.credential.as_ref() {
            Some(reference) => {
                let service = services
                    .credential()
                    .cloned()
                    .expect("validated credential service");
                let lease = service
                    .acquire(
                        scope.clone(),
                        reference.clone(),
                        plan.endpoint_audience().clone(),
                    )
                    .await?;
                if !matches!(&lease, CredentialLease::Secret(_))
                    || lease.scope() != &scope
                    || lease.reference() != reference
                    || lease.audience() != plan.endpoint_audience()
                {
                    let _ = service.release(lease).await;
                    return Err(failure(
                        "swallowtail.claude_agent.acp.credential_lease_rejected",
                        "Claude Agent ACP requires a matching API-key secret lease",
                    ));
                }
                Some(lease)
            }
            None => None,
        };
        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated resource service");
        let access = access_policy
            .resource_access()
            .expect("validated working-resource access");
        let mut resource = match resource_service
            .resolve(
                scope.clone(),
                working_resource.clone(),
                access,
                ResourceRepresentation::Filesystem,
            )
            .await
        {
            Ok(resource) => Some(resource),
            Err(error) => {
                let _ = release_credential(credential.take(), services).await;
                return Err(error);
            }
        };
        if let Err(error) = validate_session_resource_lease(
            access_policy,
            &working_resource,
            resource.as_ref().expect("resource was resolved"),
        ) {
            let _ = resource_service
                .release(resource.take().expect("resource was resolved"))
                .await;
            let _ = release_credential(credential.take(), services).await;
            return Err(error);
        }
        let cwd = resource
            .as_ref()
            .and_then(ResourceLease::filesystem)
            .expect("validated filesystem lease")
            .as_driver_value()
            .to_owned();
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated process service")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => {
                release_all(resource.take(), credential.take(), services).await;
                return Err(error);
            }
        };
        let connection = AcpConnection::new(
            Arc::clone(&process),
            resource.as_ref().expect("resource remains held").clone(),
            services
                .working_resource_io()
                .cloned()
                .expect("validated resource I/O service"),
            services.clone(),
        );
        let pump = Arc::clone(&connection);
        let pump_task = match services
            .task()
            .expect("validated task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                release_all(resource.take(), credential.take(), services).await;
                return Err(error);
            }
        };
        Ok(PendingSession {
            connection,
            pump_task: Some(pump_task),
            resource,
            credential,
            cwd,
        })
    }
}

impl PendingSession {
    pub(super) fn take_handle(
        &mut self,
        input: SessionHandleInput,
        services: &HostServices,
    ) -> Result<ClaudeAgentSessionHandle, RuntimeFailure> {
        let SessionHandleInput {
            request_id,
            provider_ref,
            binding,
            provider_requests,
            execution_host_id,
            native_close,
            native_delete,
        } = input;
        let runtime_id = RuntimeSessionId::new(format!("claude-agent-acp:{}", request_id.as_str()))
            .map_err(|_| malformed())?;
        let provider_id = provider_ref.as_provider_value().to_owned();
        let active = Arc::new(Mutex::new(None));
        Ok(ClaudeAgentSessionHandle {
            request_id,
            runtime_id,
            provider_ref,
            provider_id,
            binding,
            execution_host_id,
            native_close,
            native_delete,
            provider_requests,
            connection: Arc::clone(&self.connection),
            cancellation: SessionCancellation::new(Arc::clone(&self.connection)),
            pump_task: self.pump_task.take(),
            services: services.clone(),
            resource: self.resource.take(),
            credential: self.credential.take(),
            active,
        })
    }

    pub(super) async fn abort(&mut self, services: &HostServices) -> CleanupOutcome {
        self.connection.begin_close().await;
        let task = match self.pump_task.take() {
            Some(task) => match task.join().await {
                Ok(()) => self.connection.cleanup_outcome(),
                Err(_) => cleanup_failure(
                    "task_join_failed",
                    "Claude Agent ACP protocol task did not join",
                ),
            },
            None => CleanupOutcome::NotApplicable,
        };
        let resource = release_resource(self.resource.take(), services).await;
        let credential = release_credential(self.credential.take(), services).await;
        merge_cleanup(merge_cleanup(task, resource), credential)
    }
}

pub(super) struct SessionHandleInput {
    pub(super) request_id: RequestId,
    pub(super) provider_ref: SessionRef,
    pub(super) binding: SessionResumeBinding,
    pub(super) provider_requests: swallowtail_core::ProviderRequestPolicy,
    pub(super) execution_host_id: swallowtail_core::ExecutionHostId,
    pub(super) native_close: bool,
    pub(super) native_delete: bool,
}

async fn release_all(
    resource: Option<ResourceLease>,
    credential: Option<CredentialLease>,
    services: &HostServices,
) {
    let _ = release_resource(resource, services).await;
    let _ = release_credential(credential, services).await;
}

pub(super) async fn release_resource(
    lease: Option<ResourceLease>,
    services: &HostServices,
) -> CleanupOutcome {
    match (lease, services.working_resource()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "resource_release_failed",
            "Claude Agent working-resource service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

pub(super) async fn release_credential(
    lease: Option<CredentialLease>,
    services: &HostServices,
) -> CleanupOutcome {
    match (lease, services.credential()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "credential_release_failed",
            "Claude Agent credential service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}
