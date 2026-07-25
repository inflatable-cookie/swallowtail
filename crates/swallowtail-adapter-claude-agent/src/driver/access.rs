use super::*;
use crate::driver::handle::SessionCancellation;
use crate::driver::session::{ClaudeAgentSessionHandle, cleanup_failure, merge_cleanup};
use crate::driver::validation::{parse_session, validate_initialize};
use crate::selection::ClaudeAgentPlanSelection;

struct PendingSession {
    connection: Arc<AcpConnection>,
    pump_task: Option<Box<dyn JoinedTask>>,
    resource: Option<ResourceLease>,
    credential: Option<CredentialLease>,
}

impl ClaudeAgentAcpDriver {
    pub(super) async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        selected: ClaudeAgentPlanSelection,
    ) -> Result<ClaudeAgentSessionHandle, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "claude-agent-acp:session:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| malformed())?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated credential service");
        let mut credential = Some(
            credential_service
                .acquire(
                    scope.clone(),
                    self.credential.clone(),
                    plan.endpoint_audience().clone(),
                )
                .await?,
        );
        if !matches!(credential.as_ref(), Some(CredentialLease::Secret(_)))
            || credential.as_ref().is_some_and(|lease| {
                lease.scope() != &scope
                    || lease.reference() != &self.credential
                    || lease.audience() != plan.endpoint_audience()
            })
        {
            let _ = credential_service
                .release(credential.take().expect("credential was acquired"))
                .await;
            return Err(failure(
                "swallowtail.claude_agent.acp.credential_lease_rejected",
                "Claude Agent ACP requires a matching API-key secret lease",
            ));
        }

        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated resource service");
        let working_resource = request
            .working_resource()
            .expect("validated working resource")
            .clone();
        let mut resource = match resource_service
            .resolve(
                scope.clone(),
                working_resource.clone(),
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            )
            .await
        {
            Ok(resource) => Some(resource),
            Err(error) => {
                let _ = credential_service
                    .release(credential.take().expect("credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        if let Err(error) = validate_session_resource_lease(
            request.access_policy(),
            &working_resource,
            resource.as_ref().expect("resource was resolved"),
        ) {
            let _ = resource_service
                .release(resource.take().expect("resource was resolved"))
                .await;
            let _ = credential_service
                .release(credential.take().expect("credential was acquired"))
                .await;
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
        let mut pending = PendingSession {
            connection,
            pump_task: Some(pump_task),
            resource,
            credential,
        };
        let opened = async {
            let initialized = pending.connection.initialize().await?;
            validate_initialize(&initialized, &selected)?;
            let model = plan.model_id().expect("validated model").as_str();
            let response = pending.connection.new_session(cwd, model).await?;
            let provider_id = parse_session(&response, model)?;
            pending.connection.set_session_id(provider_id.clone())?;
            let provider_ref = SessionRef::new(&provider_id).map_err(|_| malformed())?;
            pending.take_handle(
                request.request_id().clone(),
                provider_ref,
                provider_id,
                plan.execution_host_id().clone(),
                services,
            )
        }
        .await;
        match opened {
            Ok(handle) => Ok(handle),
            Err(error) => {
                let _ = pending.abort(services).await;
                Err(error)
            }
        }
    }
}

impl PendingSession {
    fn take_handle(
        &mut self,
        request_id: RequestId,
        provider_ref: SessionRef,
        provider_id: String,
        execution_host_id: swallowtail_core::ExecutionHostId,
        services: &HostServices,
    ) -> Result<ClaudeAgentSessionHandle, RuntimeFailure> {
        let runtime_id = RuntimeSessionId::new(format!("claude-agent-acp:{}", request_id.as_str()))
            .map_err(|_| malformed())?;
        let active = Arc::new(Mutex::new(None));
        Ok(ClaudeAgentSessionHandle {
            request_id,
            runtime_id,
            provider_ref,
            provider_id,
            execution_host_id,
            connection: Arc::clone(&self.connection),
            cancellation: SessionCancellation::new(Arc::clone(&self.connection)),
            pump_task: self.pump_task.take(),
            services: services.clone(),
            resource: self.resource.take(),
            credential: self.credential.take(),
            active,
        })
    }

    async fn abort(&mut self, services: &HostServices) -> CleanupOutcome {
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
