//! OpenHands owned loopback Agent Server structured-run driver.

mod activity;
mod events;
mod handle;
mod pump;
mod validation;

use crate::{command::arguments, failure::failure};
use handle::{OpenHandsCancellation, OpenHandsRunHandle};
use serde_json::Value;
use std::sync::Arc;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    IntegrationFamilyId, OperationShape, PreflightPlan, ResourceAccess, ResourceRepresentation,
    TransportFamilyId,
};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, EnvironmentRef, HostServices, ProcessHandle, ProcessRequest,
    RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId, ScopeId,
    StructuredRunDriver, StructuredRunRequest, runtime_event_channel, terminal_outcome_channel,
    validate_session_resource_lease,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.openhands.agent-server";
const EVENT_CAPACITY: usize = 4098;

/// Low-level structured-run driver for an owned OpenHands Agent Server child.
pub struct OpenHandsAgentServerDriver {
    environment: EnvironmentRef,
    scripted_events: Option<Vec<Value>>,
}

impl OpenHandsAgentServerDriver {
    /// Binds the isolated launch environment. Credentials stay host-owned.
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self {
            environment,
            scripted_events: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_scripted_events(environment: EnvironmentRef, events: Vec<Value>) -> Self {
        Self {
            environment,
            scripted_events: Some(events),
        }
    }

    /// Returns the approved process environment.
    #[must_use]
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }
}

/// Describes owned-loopback OpenHands discovery and structured-run roles.
#[must_use]
pub fn openhands_agent_server_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("openhands").expect("static family id is valid"),
        TransportFamilyId::new("openhands-agent-server-http-ws")
            .expect("static transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::StructuredRun])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
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
    .with_interface_compatibility(crate::openhands_agent_server_claim())
}

impl StructuredRunDriver for OpenHandsAgentServerDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl OpenHandsAgentServerDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        self.validate_plan(&plan)?;
        validation::validate(&plan, &request, &services)?;
        let scripted = self.scripted_events.clone().ok_or_else(|| {
            failure(
                "swallowtail.openhands.agent_server.live_http_unwired",
                "OpenHands Agent Server live HTTP/WebSocket conversation is admitted by identity and decoded in fixtures; this card proves spawn, authority, decode, and joined cleanup without a live server",
            )
        })?;
        services.require_execution_host(plan.execution_host_id())?;
        let task_service = services.task().cloned().expect("validated task service");
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let time_service = services.time().cloned().expect("validated time service");
        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated working-resource service");
        let working_resource = request
            .working_resource()
            .cloned()
            .expect("validated working resource");
        let deadline = request.deadline().expect("validated deadline");
        let run_id = runtime_run_id(request.request_id().as_str())?;
        let scope = runtime_scope(request.request_id().as_str())?;
        let resource_access = ResourceAccess::Read;
        let resource = resource_service
            .resolve(
                scope.clone(),
                working_resource.clone(),
                resource_access,
                ResourceRepresentation::Filesystem,
            )
            .await?;
        if let Err(error) = validate_session_resource_lease(
            &swallowtail_core::SessionAccessPolicy::ambient_harness(resource_access),
            &working_resource,
            &resource,
        ) {
            let _ = resource_service.release(resource).await;
            return Err(error);
        }
        if resource.filesystem().is_none() {
            let _ = resource_service.release(resource).await;
            return Err(failure(
                "swallowtail.openhands.agent_server.working_resource_rejected",
                "OpenHands Agent Server requires a materialized filesystem working resource",
            ));
        }
        let (event_sender, event_stream) = match runtime_event_channel(EVENT_CAPACITY) {
            Ok(channel) => channel,
            Err(error) => {
                let _ = resource_service.release(resource).await;
                return Err(error);
            }
        };
        let process_request = ProcessRequest::new(
            swallowtail_runtime::ExecutableRef::from_instance_target(plan.instance_target_ref()),
        )
        .with_arguments(arguments())
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process = match process_service.start(scope.clone(), process_request).await {
            Ok(process) => process,
            Err(error) => {
                let _ = resource_service.release(resource).await;
                return Err(error);
            }
        };
        let process: Arc<dyn ProcessHandle> = Arc::from(process);
        if let Err(error) = process.close_stdin().await {
            pump::cleanup_failed_start(process.as_ref()).await;
            let _ = resource_service.release(resource).await;
            return Err(error);
        }
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            pump::cleanup_failed_start(process.as_ref()).await;
            let _ = resource_service.release(resource).await;
            return Err(error);
        }
        let _ = resource_service.release(resource).await;
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(OpenHandsCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let operation_id = ActivityOperationId::Run(run_id.clone());
                let services = services.clone();
                async move {
                    let outcome = pump::pump(
                        process,
                        event_sender.clone(),
                        cancellation,
                        time_service.wait_until(deadline),
                        operation_id,
                        services,
                        scripted,
                    )
                    .await;
                    let _ = terminal_sender.complete(outcome);
                    event_sender.mark_terminal();
                }
            }),
        );
        let task = match task {
            Ok(task) => task,
            Err(error) => {
                pump::cleanup_failed_start(process.as_ref()).await;
                return Err(error);
            }
        };
        Ok(Box::new(OpenHandsRunHandle::new(
            request.request_id().clone(),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
        )))
    }

    fn validate_plan(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.openhands.agent_server.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
            || plan.endpoint_audience().as_str() != crate::OPENHANDS_LOCAL_ACCOUNT_AUDIENCE
        {
            return Err(failure(
                "swallowtail.openhands.agent_server.access_profile_rejected",
                "OpenHands Agent Server requires its host-owned local-loopback profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.openhands.agent_server.configuration_posture_rejected",
                "OpenHands Agent Server requires explicit ambient configuration inside its selected environment",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.openhands.agent_server.isolation_rejected",
                "OpenHands Agent Server requires explicit ambient-host isolation posture",
            ));
        }
        crate::selection::select_openhands_agent_server_plan(plan).map(|_| ())
    }
}

fn runtime_run_id(request_id: &str) -> Result<RuntimeRunId, RuntimeFailure> {
    RuntimeRunId::new(format!("openhands-agent-server:{request_id}")).map_err(|_| invalid_request())
}

fn runtime_scope(request_id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("openhands-agent-server:{request_id}")).map_err(|_| invalid_request())
}

fn invalid_request() -> RuntimeFailure {
    failure(
        "swallowtail.openhands.agent_server.invalid_request",
        "OpenHands Agent Server request identity is invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::openhands_agent_server_descriptor;
    use swallowtail_core::{DriverRole, ExecutionLayer, OperationShape};

    #[test]
    fn descriptor_is_structured_run_only_and_not_acp() {
        let descriptor = openhands_agent_server_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "openhands");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "openhands-agent-server-http-ws"
        );
        assert!(descriptor.supports_role(DriverRole::Discovery));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(!descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(!descriptor.supports_operation_shape(OperationShape::InteractiveSession));
    }
}
