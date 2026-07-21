use super::access::AccessLease;
use super::binding::BedrockCatalogueBinding;
use super::failure::projection_failure;
use super::projection::project_output;
use super::sdk::{AwsCatalogueSdkExecutor, CatalogueInvocation, CatalogueSdkExecutor};
use crate::failure::failure;
use aws_sdk_bedrock::operation::list_foundation_models::ListFoundationModelsOutput;
use futures_channel::oneshot;
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan,
    TransportFamilyId,
};
use swallowtail_runtime::{
    BlockingJob, BoxFuture, CleanupOutcome, Deadline, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};
use tokio::sync::watch;

const DRIVER_ID: &str = "swallowtail.amazon-bedrock.catalogue";

#[derive(Clone)]
pub struct BedrockCatalogueDriver {
    binding: BedrockCatalogueBinding,
    executor: Arc<dyn CatalogueSdkExecutor>,
}

impl BedrockCatalogueDriver {
    #[must_use]
    pub fn new(binding: BedrockCatalogueBinding) -> Self {
        Self {
            binding,
            executor: Arc::new(AwsCatalogueSdkExecutor),
        }
    }

    #[cfg(test)]
    pub(super) fn with_executor(
        binding: BedrockCatalogueBinding,
        executor: Arc<dyn CatalogueSdkExecutor>,
    ) -> Self {
        Self { binding, executor }
    }

    fn validate_plan(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.bedrock.catalogue_plan_driver_mismatch",
                "Preflight plan is bound to a different catalogue driver",
            ));
        }
        if plan.instance_id() != self.binding.instance()
            || plan.access_profile_id() != self.binding.access_profile()
            || plan.execution_host_id() != self.binding.execution_host()
            || plan.credential_reference() != Some(self.binding.credential())
            || plan.credential_mechanism() != &CredentialMechanism::CloudProviderIdentity
        {
            return Err(failure(
                "swallowtail.bedrock.catalogue_binding_mismatch",
                "Bedrock catalogue binding did not match preflight",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn bedrock_catalogue_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("amazon-bedrock").expect("static family id is valid"),
        TransportFamilyId::new("rust-sdk-control-plane").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::ModelCatalog])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
}

impl ModelCatalogDriver for BedrockCatalogueDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<swallowtail_core::ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services)?;
            if request.deadline().is_some_and(|deadline| {
                services.time().expect("validated").now() >= deadline.instant()
            }) {
                return Err(failure(
                    "swallowtail.bedrock.catalogue_deadline_elapsed",
                    "Bedrock catalogue deadline elapsed before provider work",
                ));
            }
            let scope = operation_scope(request.request_id().as_str())?;
            let mut access =
                AccessLease::acquire(&plan, &self.binding, scope.clone(), &services).await?;
            let invocation = CatalogueInvocation {
                endpoint: access.endpoint.clone(),
                region: self.binding.region().clone(),
                provider: self.binding.provider().clone(),
            };
            let (cancel_sender, cancel_receiver) = watch::channel(false);
            let (output_sender, output_receiver) = oneshot::channel();
            let executor = Arc::clone(&self.executor);
            let job = Box::new(move || {
                let result = run_sdk(executor, invocation, cancel_receiver);
                output_sender.send(result).map_err(|_| {
                    failure(
                        "swallowtail.bedrock.catalogue_receiver_closed",
                        "Bedrock catalogue result receiver closed before completion",
                    )
                })
            }) as BlockingJob;
            let blocking = services.blocking_work().expect("validated").run(scope, job);
            let work = async move {
                blocking.await?;
                output_receiver.await.map_err(|_| {
                    failure(
                        "swallowtail.bedrock.catalogue_executor_disconnected",
                        "Bedrock catalogue private executor disconnected",
                    )
                })?
            };
            let result =
                complete_before_deadline(work, request.deadline(), &services, cancel_sender)
                    .await
                    .and_then(|output| project_output(&output).map_err(projection_failure));
            let cleanup = access.release(&services).await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), _) => Err(failure(
                    "swallowtail.bedrock.catalogue_cleanup_failed",
                    "Bedrock catalogue credential cleanup failed",
                )),
            }
        })
    }
}

fn run_sdk(
    executor: Arc<dyn CatalogueSdkExecutor>,
    invocation: CatalogueInvocation,
    cancelled: watch::Receiver<bool>,
) -> Result<ListFoundationModelsOutput, RuntimeFailure> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| {
            failure(
                "swallowtail.bedrock.catalogue_executor_start_failed",
                "Bedrock catalogue private executor could not start",
            )
        })?;
    runtime.block_on(executor.execute(invocation, cancelled))
}

async fn complete_before_deadline<T, F>(
    work: F,
    deadline: Option<Deadline>,
    services: &HostServices,
    cancel: watch::Sender<bool>,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let Some(deadline) = deadline else {
        return work.await;
    };
    let mut work = Box::pin(work);
    let mut wait = services.time().expect("validated").wait_until(deadline);
    let mut timed_out = false;
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if !timed_out && wait.as_mut().poll(context).is_ready() {
            timed_out = true;
            let _ = cancel.send(true);
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    if timed_out {
        Err(failure(
            "swallowtail.bedrock.catalogue_timed_out",
            "Bedrock catalogue operation timed out",
        ))
    } else {
        result
    }
}

fn require_services(services: &HostServices) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        Err(failure(
            "swallowtail.bedrock.catalogue_host_service_missing",
            "Bedrock catalogue requires blocking-work, time, network, and credential services",
        ))
    } else {
        Ok(())
    }
}

fn operation_scope(id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("bedrock-catalogue:list:{id}")).map_err(|_| {
        failure(
            "swallowtail.bedrock.catalogue_scope_invalid",
            "Bedrock catalogue operation scope was invalid",
        )
    })
}

#[cfg(test)]
mod tests;
