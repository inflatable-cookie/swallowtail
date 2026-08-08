use super::launch::catalogue_arguments;
use super::session::cleanup::{merge_cleanup, release_credential};
use super::validation::validate_catalogue;
use crate::catalogue::parse_catalogue;
use crate::connection::{CommandResult, PiConnection};
use crate::failure::failure;
use serde_json::json;
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelCatalogEntry, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, ExecutableRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, ProcessHandle, ProcessRequest, RuntimeFailure, ScopeId,
};

impl ModelCatalogDriver for super::PiRpcDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate_catalogue(&plan, &services, &self.credential)?;
            if request.deadline().is_some_and(|deadline| {
                services.time().expect("validated Pi time service").now() >= deadline.instant()
            }) {
                return Err(failure(
                    "swallowtail.pi.rpc.catalogue_deadline_elapsed",
                    "Pi RPC model catalogue deadline elapsed before startup",
                ));
            }
            self.list_models_with_process(plan, request, services).await
        })
    }
}

impl super::PiRpcDriver {
    async fn list_models_with_process(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "pi-rpc:catalogue:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.pi.rpc.scope_invalid",
                "Pi RPC scope was invalid",
            )
        })?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated Pi credential service");
        let mut credential = Some(
            credential_service
                .acquire(
                    scope.clone(),
                    self.credential.clone(),
                    plan.endpoint_audience().clone(),
                )
                .await?,
        );
        if !matching_credential(
            credential.as_ref().expect("Pi credential was acquired"),
            &scope,
            &self.credential,
            plan.endpoint_audience(),
        ) {
            let _ = credential_service
                .release(credential.take().expect("Pi credential was acquired"))
                .await;
            return Err(failure(
                "swallowtail.pi.rpc.credential_lease_rejected",
                "Pi RPC requires a matching delegated credential lease",
            ));
        }
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(catalogue_arguments())
        .with_environment([self.environment.clone()]);
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated Pi process service")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => {
                let _ = credential_service
                    .release(credential.take().expect("Pi credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let connection = PiConnection::new(
            Arc::clone(&process),
            services.task().cloned().expect("validated Pi task service"),
            services.time().cloned().expect("validated Pi time service"),
            services.clone(),
        );
        let pump = Arc::clone(&connection);
        let pump_task = match services
            .task()
            .expect("validated Pi task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                let _ = credential_service
                    .release(credential.take().expect("Pi credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let id = format!("catalogue:{}", request.request_id().as_str());
        let command = connection.command(
            id.clone(),
            "get_available_models",
            json!({"id": id, "type": "get_available_models"}),
        );
        let response = match request.deadline() {
            Some(deadline) => {
                let wait = services
                    .time()
                    .cloned()
                    .expect("validated Pi time service")
                    .wait_until(deadline);
                command_before_deadline(command, wait).await
            }
            None => command.await,
        };
        let result = response.and_then(parse_catalogue);
        connection.begin_close().await;
        let process_cleanup = match pump_task.join().await {
            Ok(()) => connection.cleanup_outcome(),
            Err(_) => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.pi.rpc.catalogue_cleanup_failed",
                "Pi RPC model catalogue protocol task did not join cleanly",
            )),
        };
        let credential_cleanup = release_credential(credential.take(), &services).await;
        let cleanup = merge_cleanup(process_cleanup, credential_cleanup);
        match (result, cleanup) {
            (Err(error), _) => Err(error),
            (Ok(models), CleanupOutcome::Clean) => Ok(models),
            (Ok(_), _) => Err(failure(
                "swallowtail.pi.rpc.catalogue_cleanup_failed",
                "Pi RPC model catalogue cleanup failed",
            )),
        }
    }
}

fn matching_credential(
    credential: &CredentialLease,
    scope: &ScopeId,
    reference: &swallowtail_core::CredentialRef,
    audience: &swallowtail_core::EndpointAudience,
) -> bool {
    matches!(credential, CredentialLease::Delegated(_))
        && credential.scope() == scope
        && credential.reference() == reference
        && credential.audience() == audience
}

async fn command_before_deadline<F>(
    command: F,
    mut deadline: BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
) -> Result<CommandResult, RuntimeFailure>
where
    F: Future<Output = Result<CommandResult, RuntimeFailure>>,
{
    let mut command = Box::pin(command);
    poll_fn(|context| {
        if let Poll::Ready(result) = command.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(Err(failure(
                "swallowtail.pi.rpc.catalogue_timed_out",
                "Pi RPC model catalogue discovery timed out",
            )));
        }
        Poll::Pending
    })
    .await
}
