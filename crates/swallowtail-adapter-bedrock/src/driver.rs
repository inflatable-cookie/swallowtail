use crate::binding::BedrockDriverBinding;
use crate::failure::{failure, unsupported};
use crate::sdk::{AwsSdkExecutor, SdkExecutor, SdkInvocation};
use crate::stream::StreamUpdate;
use futures_channel::mpsc;
use futures_core::Stream;
use std::future::poll_fn;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, Capability, CredentialMechanism,
    DriverDescriptor, DriverRole, ExecutionLayer, ExternalNetworkPolicy, ExternalSearchPolicy,
    HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan, RunRef, SafeDiagnostic,
    TransportFamilyId,
};
use swallowtail_runtime::{
    BlockingJob, BoxEventStream, BoxFuture, CleanupOutcome, CredentialLease, DeadlineObservation,
    EndpointRef, HostServices, JoinedTask, OperationContent, ProviderObservation, RequestId,
    RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId, ScopeId,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus,
    TokenUsage as RuntimeTokenUsage, runtime_event_channel, terminal_outcome_channel,
};
use tokio::sync::watch;

const DRIVER_ID: &str = "swallowtail.amazon-bedrock.direct";
const EVENT_CAPACITY: usize = 64;

#[derive(Clone)]
pub struct BedrockDirectDriver {
    binding: BedrockDriverBinding,
    executor: Arc<dyn SdkExecutor>,
}

impl BedrockDirectDriver {
    #[must_use]
    pub fn new(binding: BedrockDriverBinding) -> Self {
        Self {
            binding,
            executor: Arc::new(AwsSdkExecutor),
        }
    }

    #[cfg(test)]
    fn with_executor(binding: BedrockDriverBinding, executor: Arc<dyn SdkExecutor>) -> Self {
        Self { binding, executor }
    }

    fn validate_plan(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.bedrock.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.instance_id() != self.binding.instance()
            || plan.access_profile_id() != self.binding.access_profile()
            || plan.execution_host_id() != self.binding.execution_host()
            || plan.credential_reference() != Some(self.binding.credential())
            || plan.credential_mechanism() != &CredentialMechanism::CloudProviderIdentity
        {
            return Err(failure(
                "swallowtail.bedrock.binding_mismatch",
                "Bedrock Runtime binding did not match preflight",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn bedrock_direct_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("amazon-bedrock").expect("static family id is valid"),
        TransportFamilyId::new("rust-sdk-eventstream").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::StructuredRun])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
}

impl StructuredRunDriver for BedrockDirectDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services)?;
            validate_run(&plan, &request, &services)?;
            let scope = operation_scope(request.request_id().as_str())?;
            let access =
                AccessLease::acquire(&plan, &self.binding, scope.clone(), &services).await?;
            let maximum = i32::try_from(request.maximum_output_tokens().expect("validated").get())
                .map_err(|_| {
                    failure(
                        "swallowtail.bedrock.output_limit_invalid",
                        "Bedrock Runtime output-token limit exceeded the SDK range",
                    )
                })?;
            let invocation = SdkInvocation {
                endpoint: access.endpoint.clone(),
                region: self.binding.region().clone(),
                provider: self.binding.provider().clone(),
                model: plan.model_id().expect("validated").as_str().to_owned(),
                prompt: request.content().as_str().to_owned(),
                maximum_output_tokens: maximum,
            };
            let (update_sender, update_receiver) = mpsc::channel(EVENT_CAPACITY);
            let (cancel_sender, cancel_receiver) = watch::channel(false);
            let cancellation = Arc::new(RunCancellation {
                requested: AtomicBool::new(false),
                signal: cancel_sender,
            });
            let executor = Arc::clone(&self.executor);
            let job =
                Box::new(move || run_sdk(executor, invocation, update_sender, cancel_receiver))
                    as BlockingJob;
            let (events_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            events_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
            let (terminal_sender, terminal_future) = terminal_outcome_channel();
            let deadline = request
                .deadline()
                .map(|deadline| services.time().expect("validated").wait_until(deadline));
            let run_services = services.clone();
            let run_cancellation = Arc::clone(&cancellation);
            let pending = Arc::new(Mutex::new(Some(PendingRun { access, job })));
            let task_pending = Arc::clone(&pending);
            let task_scope = scope.clone();
            let task = services.task().expect("validated").spawn(
                scope,
                Box::pin(async move {
                    let pending = task_pending
                        .lock()
                        .expect("Bedrock pending run lock poisoned")
                        .take()
                        .expect("Bedrock pending run is available");
                    let mut access = pending.access;
                    let blocking = run_services
                        .blocking_work()
                        .expect("validated")
                        .run(task_scope, pending.job);
                    let outcome = pump_run(
                        update_receiver,
                        blocking,
                        &mut access,
                        run_services,
                        events_sender.clone(),
                        run_cancellation,
                        deadline,
                    )
                    .await;
                    let _ = terminal_sender.complete(outcome);
                    events_sender.mark_terminal();
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    cancellation.request_signal();
                    let pending = pending
                        .lock()
                        .expect("Bedrock pending run lock poisoned")
                        .take();
                    if let Some(mut pending) = pending {
                        let _ = pending.access.release(&services).await;
                    }
                    return Err(error);
                }
            };
            let run_id =
                RuntimeRunId::new(format!("bedrock-direct:{}", request.request_id().as_str()))
                    .map_err(|_| {
                        failure(
                            "swallowtail.bedrock.run_id_invalid",
                            "Bedrock Runtime run id was invalid",
                        )
                    })?;
            Ok(Box::new(BedrockRunHandle {
                request_id: request.request_id().clone(),
                run_id,
                events: Some(Box::pin(event_stream)),
                terminal: Some(Box::pin(terminal_future)),
                cancellation,
                task,
            }) as Box<dyn RunHandle>)
        })
    }
}

include!("driver/validation.rs");
include!("driver/access.rs");
include!("driver/pump.rs");
include!("driver/handle.rs");

#[cfg(test)]
mod tests;
