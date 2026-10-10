use crate::{
    command::arguments,
    connection::AcpConnection,
    failure::{failure, malformed, unsupported},
    mcp::{CopilotCliAcpRemoteMcpPlacement, production_mcp_servers},
    turn::ActiveTurn,
};
use serde_json::{Value, json};
use std::future::{Future, poll_fn};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, CredentialMechanism,
    DriverDescriptor, DriverRole, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    Deadline, EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, OpenSessionRequest, ProcessHandle, ProcessRequest,
    RequestId, ResourceLease, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId,
    TerminalOutcome, TimeService, TurnHandle, TurnRequest, validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.copilot-cli.acp";

/// Low-level interactive driver for the installed Copilot CLI ACP agent.
pub struct CopilotCliAcpDriver {
    isolated_environment: EnvironmentRef,
    http_mcp: Option<CopilotCliAcpRemoteMcpPlacement>,
    admission: crate::selection::CopilotCliPlanAdmission,
}

impl CopilotCliAcpDriver {
    /// Binds the isolated launch environment. Credentials stay host-owned.
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef) -> Self {
        Self {
            isolated_environment,
            http_mcp: None,
            admission: crate::selection::CopilotCliPlanAdmission::PublicQualified,
        }
    }

    pub(crate) const fn with_admission(
        isolated_environment: EnvironmentRef,
        admission: crate::selection::CopilotCliPlanAdmission,
    ) -> Self {
        Self {
            isolated_environment,
            http_mcp: None,
            admission,
        }
    }

    pub(crate) const fn admission(&self) -> crate::selection::CopilotCliPlanAdmission {
        self.admission
    }

    /// Admits one route-owned streamable-HTTP MCP declaration onto production `session/new`.
    pub fn with_http_mcp_placement(
        self,
        server: CopilotCliAcpRemoteMcpPlacement,
    ) -> Result<Self, RuntimeFailure> {
        let _ = server.to_acp_http_value()?;
        Ok(self.with_prepared_http_mcp(Some(server)))
    }

    pub(crate) fn with_prepared_http_mcp(
        mut self,
        server: Option<CopilotCliAcpRemoteMcpPlacement>,
    ) -> Self {
        self.http_mcp = server;
        self
    }

    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<crate::selection::CopilotCliPlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.copilot-cli.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
            || plan.endpoint_audience().as_str() != crate::COPILOT_CLI_HOST_ACCOUNT_AUDIENCE
        {
            return Err(failure(
                "swallowtail.copilot-cli.acp.access_profile_rejected",
                "Copilot CLI ACP requires its host-owned GitHub-login or BYOK profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.copilot-cli.acp.configuration_posture_rejected",
                "Copilot CLI ACP requires explicit ambient configuration inside its selected environment",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.copilot-cli.acp.isolation_rejected",
                "Copilot CLI ACP requires explicit ambient-host isolation posture",
            ));
        }
        crate::selection::select_copilot_cli_acp_plan_for(plan, self.admission)
    }
}

/// Describes the installed Copilot CLI ACP discovery and session roles.
#[must_use]
pub fn copilot_cli_acp_descriptor() -> DriverDescriptor {
    copilot_cli_acp_descriptor_for(crate::selection::CopilotCliPlanAdmission::PublicQualified)
}

pub(crate) fn copilot_cli_acp_descriptor_for(
    admission: crate::selection::CopilotCliPlanAdmission,
) -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("copilot-cli").expect("static family id is valid"),
        TransportFamilyId::new("acp-v1-stdio").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::selection::copilot_cli_acp_claim_for(admission))
}

impl InteractiveSessionDriver for CopilotCliAcpDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let selected = self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_open(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "copilot-cli-acp:session:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let resource_service = services
                .working_resource()
                .cloned()
                .expect("validated working-resource service");
            let resource_access = session_resource_access(&plan)?;
            let resource = resource_service
                .resolve(
                    scope.clone(),
                    request
                        .working_resource()
                        .expect("validated resource")
                        .clone(),
                    resource_access,
                    ResourceRepresentation::Filesystem,
                )
                .await?;
            if let Err(error) = validate_session_resource_lease(
                request.access_policy(),
                request.working_resource().expect("validated resource"),
                &resource,
            ) {
                let _ = resource_service.release(resource).await;
                return Err(error);
            }
            let result = self
                .start_session(&plan, &request, &services, scope, resource, selected, None)
                .await;
            match result {
                Ok(session) => Ok(Box::new(session) as Box<dyn InteractiveSessionHandle>),
                Err(pair) => {
                    let (error, resource) = *pair;
                    let _ = resource_service.release(resource).await;
                    Err(error)
                }
            }
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

impl CopilotCliAcpDriver {
    #[cfg(test)]
    pub(crate) fn open_assessment_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
        action_deadline: Deadline,
        cleanup_deadline: Deadline,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let selected = self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_open(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "copilot-cli-acp:session:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let resource_service = services
                .working_resource()
                .cloned()
                .expect("validated working-resource service");
            let resource_access = session_resource_access(&plan)?;
            let time = services.time().cloned().ok_or_else(|| {
                failure(
                    "swallowtail.copilot-cli.acp.assessment.time_service_missing",
                    "Private Copilot assessment requires the selected host monotonic clock",
                )
            })?;
            if time.now() >= action_deadline.instant() {
                return Err(assessment_deadline_failure("session_open_deadline"));
            }
            let resolved = resource_service.resolve(
                scope.clone(),
                request
                    .working_resource()
                    .expect("validated resource")
                    .clone(),
                resource_access,
                ResourceRepresentation::Filesystem,
            );
            let resource = match await_before(resolved, action_deadline, Arc::clone(&time)).await {
                Ok(Ok(resource)) => resource,
                Ok(Err(error)) => return Err(error),
                Err(()) => return Err(assessment_deadline_failure("session_open_deadline")),
            };
            if let Err(error) = validate_session_resource_lease(
                request.access_policy(),
                request.working_resource().expect("validated resource"),
                &resource,
            ) {
                let released = await_before(
                    resource_service.release(resource),
                    cleanup_deadline,
                    Arc::clone(&time),
                )
                .await;
                return if matches!(
                    released,
                    Ok(CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
                ) {
                    Err(error)
                } else {
                    Err(assessment_deadline_failure("session_open_cleanup_failed"))
                };
            }
            let result = self
                .start_session(
                    &plan,
                    &request,
                    &services,
                    scope,
                    resource,
                    selected,
                    Some((action_deadline, cleanup_deadline)),
                )
                .await;
            match result {
                Ok(session) => Ok(Box::new(session) as Box<dyn InteractiveSessionHandle>),
                Err(pair) => {
                    let (error, resource) = *pair;
                    let released =
                        await_before(resource_service.release(resource), cleanup_deadline, time)
                            .await;
                    if matches!(
                        released,
                        Ok(CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
                    ) {
                        Err(error)
                    } else {
                        Err(assessment_deadline_failure("session_open_cleanup_failed"))
                    }
                }
            }
        })
    }

    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        scope: ScopeId,
        resource: ResourceLease,
        selected: crate::selection::CopilotCliPlanSelection,
        deadlines: Option<(Deadline, Deadline)>,
    ) -> Result<CopilotCliSessionHandle, Box<(RuntimeFailure, ResourceLease)>> {
        let cwd = resource
            .filesystem()
            .expect("validated filesystem lease")
            .as_driver_value()
            .to_owned();
        let mcp_servers = match production_mcp_servers(self.http_mcp.as_ref()) {
            Ok(value) => value,
            Err(error) => return Err(Box::new((error, resource))),
        };
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments())
        .with_environment([self.isolated_environment.clone()])
        .with_working_resource(
            request
                .working_resource()
                .expect("validated resource")
                .clone(),
        );
        if let Some((action_deadline, _)) = deadlines {
            let Some(time) = services.time() else {
                return Err(Box::new((
                    failure(
                        "swallowtail.copilot-cli.acp.assessment.time_service_missing",
                        "Private Copilot assessment requires the selected host monotonic clock",
                    ),
                    resource,
                )));
            };
            if time.now() >= action_deadline.instant() {
                return Err(Box::new((
                    assessment_deadline_failure("session_open_deadline"),
                    resource,
                )));
            }
        }
        let process_start = if let Some((action_deadline, _)) = deadlines {
            let Some(time) = services.time().cloned() else {
                return Err(Box::new((
                    failure(
                        "swallowtail.copilot-cli.acp.assessment.time_service_missing",
                        "Private Copilot assessment requires the selected host monotonic clock",
                    ),
                    resource,
                )));
            };
            match await_before(
                process_service.start(scope.clone(), process_request),
                action_deadline,
                time,
            )
            .await
            {
                Ok(result) => result,
                Err(()) => {
                    return Err(Box::new((
                        assessment_deadline_failure("session_open_deadline"),
                        resource,
                    )));
                }
            }
        } else {
            process_service.start(scope.clone(), process_request).await
        };
        let process: Arc<dyn ProcessHandle> = match process_start {
            Ok(process) => Arc::from(process),
            Err(error) => return Err(Box::new((error, resource))),
        };
        let connection = AcpConnection::new(Arc::clone(&process), services.clone());
        let pump_connection = Arc::clone(&connection);
        let task_service = services.task().cloned().expect("validated task service");
        let pump_task = match task_service
            .spawn(scope, Box::pin(async move { pump_connection.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                if let Some((_, cleanup_deadline)) = deadlines {
                    let Some(time) = services.time().cloned() else {
                        return Err(Box::new((
                            failure(
                                "swallowtail.copilot-cli.acp.assessment.time_service_missing",
                                "Private Copilot assessment requires the selected host monotonic clock",
                            ),
                            resource,
                        )));
                    };
                    let stop_process = Arc::clone(&process);
                    let process_cleanup = async {
                        let (stopped, exited) =
                            join_pair(stop_process.force_stop(), process.wait()).await;
                        matches!(stopped, Ok(())) && matches!(exited, Ok(_))
                    };
                    let process_clean = await_before(process_cleanup, cleanup_deadline, time).await;
                    if !matches!(process_clean, Ok(true)) {
                        return Err(Box::new((
                            assessment_deadline_failure("session_open_cleanup_failed"),
                            resource,
                        )));
                    }
                } else {
                    let _ = process.force_stop().await;
                    let _ = process.wait().await;
                }
                return Err(Box::new((error, resource)));
            }
        };
        let opened = async {
            let initialize = connection.initialize().await?;
            validate_initialize(&initialize, selected.version())?;
            connection
                .request(
                    "session/new",
                    json!({"cwd": cwd, "mcpServers": mcp_servers}),
                )
                .await
                .and_then(parse_new_session)
        };
        let opened = if let Some((action_deadline, cleanup_deadline)) = deadlines {
            let time = services.time().cloned().ok_or_else(|| {
                Box::new((
                    failure(
                        "swallowtail.copilot-cli.acp.assessment.time_service_missing",
                        "Private Copilot assessment requires the selected host monotonic clock",
                    ),
                    resource.clone(),
                ))
            })?;
            match await_before(opened, action_deadline, Arc::clone(&time)).await {
                Ok(opened) => opened,
                Err(()) => {
                    let timeout = failure(
                        "swallowtail.copilot-cli.acp.assessment.session_open_deadline",
                        "Prepared Copilot session open exceeded its proof action cutoff",
                    );
                    let cleanup_ok =
                        cleanup_assessment_open(&connection, pump_task, cleanup_deadline, time)
                            .await;
                    return if cleanup_ok {
                        Err(Box::new((timeout, resource)))
                    } else {
                        Err(Box::new((
                            failure(
                                "swallowtail.copilot-cli.acp.assessment.session_open_cleanup_failed",
                                "Prepared Copilot session open cleanup did not finish within its reserve",
                            ),
                            resource,
                        )))
                    };
                }
            }
        } else {
            opened.await
        };
        let provider_id = match opened {
            Ok(provider_id) => provider_id,
            Err(error) => {
                if let Some((_, cleanup_deadline)) = deadlines {
                    let time = services
                        .time()
                        .cloned()
                        .expect("time service validated above");
                    if !cleanup_assessment_open(&connection, pump_task, cleanup_deadline, time)
                        .await
                    {
                        return Err(Box::new((
                            failure(
                                "swallowtail.copilot-cli.acp.assessment.session_open_cleanup_failed",
                                "Prepared Copilot session open cleanup did not finish within its reserve",
                            ),
                            resource,
                        )));
                    }
                } else {
                    connection.begin_close().await;
                    let _ = pump_task.join().await;
                }
                return Err(Box::new((error, resource)));
            }
        };
        if let Err(error) = connection.set_session_id(provider_id.clone()) {
            if let Some((_, cleanup_deadline)) = deadlines {
                let time = services
                    .time()
                    .cloned()
                    .expect("time service validated above");
                if !cleanup_assessment_open(&connection, pump_task, cleanup_deadline, time).await {
                    return Err(Box::new((
                        assessment_deadline_failure("session_open_cleanup_failed"),
                        resource,
                    )));
                }
            } else {
                connection.begin_close().await;
                let _ = pump_task.join().await;
            }
            return Err(Box::new((error, resource)));
        }
        let provider_ref = match SessionRef::new(&provider_id) {
            Ok(provider_ref) => provider_ref,
            Err(_) => {
                if let Some((_, cleanup_deadline)) = deadlines {
                    let time = services
                        .time()
                        .cloned()
                        .expect("time service validated above");
                    if !cleanup_assessment_open(&connection, pump_task, cleanup_deadline, time)
                        .await
                    {
                        return Err(Box::new((
                            assessment_deadline_failure("session_open_cleanup_failed"),
                            resource,
                        )));
                    }
                } else {
                    connection.begin_close().await;
                    let _ = pump_task.join().await;
                }
                return Err(Box::new((malformed(), resource)));
            }
        };
        let runtime_id = match RuntimeSessionId::new(format!(
            "copilot-cli-acp:{}",
            request.request_id().as_str()
        )) {
            Ok(runtime_id) => runtime_id,
            Err(_) => {
                if let Some((_, cleanup_deadline)) = deadlines {
                    let time = services
                        .time()
                        .cloned()
                        .expect("time service validated above");
                    if !cleanup_assessment_open(&connection, pump_task, cleanup_deadline, time)
                        .await
                    {
                        return Err(Box::new((
                            assessment_deadline_failure("session_open_cleanup_failed"),
                            resource,
                        )));
                    }
                } else {
                    connection.begin_close().await;
                    let _ = pump_task.join().await;
                }
                return Err(Box::new((malformed(), resource)));
            }
        };
        let active = Arc::new(Mutex::new(None));
        Ok(CopilotCliSessionHandle {
            request_id: request.request_id().clone(),
            runtime_id,
            provider_ref,
            provider_id,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&connection),
            cancellation: SessionCancellation::new(connection),
            pump_task: Some(pump_task),
            services: services.clone(),
            resource: Some(resource),
            active,
            #[cfg(test)]
            assessment_permission_action: deadlines
                .map(|_| crate::assessment::CopilotCliAssessmentAction::sentinel_edit()),
        })
    }
}

async fn cleanup_assessment_open(
    connection: &AcpConnection,
    pump_task: Box<dyn JoinedTask>,
    deadline: Deadline,
    time: Arc<dyn TimeService>,
) -> bool {
    let close_and_stop = async {
        let ((), stopped) = join_pair(
            connection.begin_close(),
            connection.force_stop_owned_process(),
        )
        .await;
        stopped
    };
    let closed_and_stopped = await_before(close_and_stop, deadline, Arc::clone(&time)).await;
    let joined = await_before(pump_task.join(), deadline, time).await;
    matches!(closed_and_stopped, Ok(Ok(()))) && matches!(joined, Ok(Ok(())))
}

async fn join_pair<First: Future, Second: Future>(
    first: First,
    second: Second,
) -> (First::Output, Second::Output) {
    let mut first = Box::pin(first);
    let mut second = Box::pin(second);
    let mut first_output = None;
    let mut second_output = None;
    poll_fn(|context| {
        if first_output.is_none()
            && let Poll::Ready(output) = first.as_mut().poll(context)
        {
            first_output = Some(output);
        }
        if second_output.is_none()
            && let Poll::Ready(output) = second.as_mut().poll(context)
        {
            second_output = Some(output);
        }
        match (first_output.take(), second_output.take()) {
            (Some(first), Some(second)) => Poll::Ready((first, second)),
            (first, second) => {
                first_output = first;
                second_output = second;
                Poll::Pending
            }
        }
    })
    .await
}

fn assessment_deadline_failure(stage: &'static str) -> RuntimeFailure {
    let (code, message) = match stage {
        "session_open_cleanup_failed" => (
            "swallowtail.copilot-cli.acp.assessment.session_open_cleanup_failed",
            "Prepared Copilot session open cleanup did not finish within its reserve",
        ),
        _ => (
            "swallowtail.copilot-cli.acp.assessment.session_open_deadline",
            "Prepared Copilot session open exceeded its proof action cutoff",
        ),
    };
    failure(code, message)
}

async fn await_before<F: Future>(
    future: F,
    deadline: Deadline,
    time: Arc<dyn TimeService>,
) -> Result<F::Output, ()> {
    let mut future = Box::pin(future);
    let mut wait = Box::pin(time.wait_until(deadline));
    poll_fn(move |context| {
        if time.now() >= deadline.instant() {
            return Poll::Ready(Err(()));
        }
        if let Poll::Ready(output) = future.as_mut().poll(context) {
            return Poll::Ready(Ok(output));
        }
        if wait.as_mut().poll(context).is_ready() {
            Poll::Ready(Err(()))
        } else {
            Poll::Pending
        }
    })
    .await
}

include!("driver/validation.rs");
include!("driver/cancellation.rs");
include!("driver/turn_handle.rs");
include!("driver/session.rs");
