use super::launch::catalogue_arguments;
use super::validation::validate_catalogue;
use crate::catalogue::parse_catalogue;
use crate::connection::{CommandResult, OhMyPiConnection};
use crate::failure::failure;
use serde_json::json;
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelCatalogEntry, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, ExecutableRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, ProcessHandle, ProcessRequest, RuntimeFailure, ScopeId,
};

impl ModelCatalogDriver for super::OhMyPiRpcDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate_catalogue(&plan, &services)?;
            if request.deadline().is_some_and(|deadline| {
                services
                    .time()
                    .expect("validated OhMyPi time service")
                    .now()
                    >= deadline.instant()
            }) {
                return Err(failure(
                    "swallowtail.oh_my_pi.rpc.catalogue_deadline_elapsed",
                    "OhMyPi RPC model catalogue deadline elapsed before startup",
                ));
            }
            self.list_models_with_process(plan, request, services).await
        })
    }
}

impl super::OhMyPiRpcDriver {
    async fn list_models_with_process(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "oh-my-pi-rpc:catalogue:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.oh_my_pi.rpc.scope_invalid",
                "OhMyPi RPC scope was invalid",
            )
        })?;
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(catalogue_arguments())
        .with_environment([self.environment.clone()]);
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated OhMyPi process service")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => return Err(error),
        };
        let connection = OhMyPiConnection::new(
            Arc::clone(&process),
            services
                .task()
                .cloned()
                .expect("validated OhMyPi task service"),
            services
                .time()
                .cloned()
                .expect("validated OhMyPi time service"),
        );
        let pump = Arc::clone(&connection);
        let pump_task = match services
            .task()
            .expect("validated OhMyPi task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                return Err(error);
            }
        };
        let response = match super::startup::negotiate(&connection).await {
            Ok(()) => {
                let id = format!("catalogue:{}", request.request_id().as_str());
                let command = connection.command(
                    id.clone(),
                    "get_available_models",
                    json!({"id": id, "type": "get_available_models"}),
                );
                match request.deadline() {
                    Some(deadline) => {
                        let wait = services
                            .time()
                            .cloned()
                            .expect("validated OhMyPi time service")
                            .wait_until(deadline);
                        command_before_deadline(command, wait).await
                    }
                    None => command.await,
                }
            }
            Err(error) => Err(error),
        };
        let result = response.and_then(parse_catalogue);
        connection.begin_close().await;
        let process_cleanup = match pump_task.join().await {
            Ok(()) => connection.cleanup_outcome(),
            Err(_) => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.oh_my_pi.rpc.catalogue_cleanup_failed",
                "OhMyPi RPC model catalogue protocol task did not join cleanly",
            )),
        };
        match (result, process_cleanup) {
            (Err(error), _) => Err(error),
            (Ok(models), CleanupOutcome::Clean) => Ok(models),
            (Ok(_), _) => Err(failure(
                "swallowtail.oh_my_pi.rpc.catalogue_cleanup_failed",
                "OhMyPi RPC model catalogue cleanup failed",
            )),
        }
    }
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
                "swallowtail.oh_my_pi.rpc.catalogue_timed_out",
                "OhMyPi RPC model catalogue discovery timed out",
            )));
        }
        Poll::Pending
    })
    .await
}
