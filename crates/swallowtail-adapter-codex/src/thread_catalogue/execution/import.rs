use super::super::protocol::project_revalidation;
use super::control::{Controlled, wait_controlled};
use super::resource::ScopedResource;
use super::{
    cancel_and_release, cleanup_or, close_and_release, control_before_dispatch, control_failure,
    deadline_wait, finish, from_runtime, require_catalogue_version,
};
use crate::CodexAppServerDriver;
use crate::app_server::scope;
use crate::session_replay::project_thread_history;
use swallowtail_runtime::{
    HostServices, ProviderSessionImportOutcome, ProviderSessionImportPlan,
    ProviderSessionImportRequest, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage,
};

impl CodexAppServerDriver {
    pub(crate) async fn execute_thread_import(
        &self,
        plan: ProviderSessionImportPlan,
        request: ProviderSessionImportRequest,
        services: HostServices,
    ) -> Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure> {
        require_catalogue_version(plan.preflight())?;
        let behavior = self.validate_plan(plan.preflight()).map_err(|error| {
            from_runtime(ProviderSessionOperationFailureStage::BeforeDispatch, error)
        })?;
        control_before_dispatch(
            request.cancellation().as_ref(),
            request.agreement().deadline(),
            &services,
        )?;
        let operation_scope = scope("thread-import", request.request_id());
        let resource =
            ScopedResource::resolve_import(&plan, operation_scope.clone(), &services).await?;
        let cwd = resource.root().to_owned();
        let connection = self
            .start_connection(
                plan.preflight(),
                behavior,
                operation_scope,
                Some(resource.reference().clone()),
                false,
                &services,
            )
            .await;
        let (connection, task) = match connection {
            Ok(connection) => connection,
            Err(error) => {
                let cleanup = resource.release().await;
                return Err(cleanup_or(
                    cleanup,
                    from_runtime(
                        ProviderSessionOperationFailureStage::ImportRevalidation,
                        error,
                    ),
                ));
            }
        };

        if let Err(control) = control_before_dispatch(
            request.cancellation().as_ref(),
            request.agreement().deadline(),
            &services,
        ) {
            let cleanup = cancel_and_release(&connection, task, resource).await;
            return Err(cleanup_or(cleanup, control));
        }

        let response = connection
            .dispatch_request(
                "thread/read",
                serde_json::json!({
                    "threadId": request.provider_session_ref().as_provider_value(),
                    "includeTurns": true
                }),
            )
            .await;
        let response = match response {
            Ok(response) => {
                wait_controlled(
                    response,
                    request.cancellation().as_ref(),
                    deadline_wait(request.agreement().deadline(), &services)?,
                )
                .await
            }
            Err(error) => Controlled::Completed(Err(error)),
        };
        let revalidation = match &response {
            Controlled::Completed(Ok(response)) => {
                project_thread_history(response, request.provider_session_ref()).map_err(
                    |error| {
                        from_runtime(
                            ProviderSessionOperationFailureStage::ImportRevalidation,
                            error,
                        )
                    },
                )?;
                project_revalidation(
                    response,
                    request.provider_session_ref(),
                    &cwd,
                    plan.agreement().candidate().updated_at_unix_milliseconds(),
                    plan.agreement().candidate_id().clone(),
                    plan.agreement().working_resource().clone(),
                )
            }
            Controlled::Completed(Err(error)) => Err(from_runtime(
                ProviderSessionOperationFailureStage::ImportRevalidation,
                error.clone(),
            )),
            Controlled::Cancelled => Err(control_failure(
                ProviderSessionOperationFailureStage::Cancelled,
                "swallowtail.codex.thread_import.cancelled",
                "Codex thread import was cancelled",
            )),
            Controlled::Deadline => Err(control_failure(
                ProviderSessionOperationFailureStage::TimedOut,
                "swallowtail.codex.thread_import.timed_out",
                "Codex thread import timed out",
            )),
        };
        let cleanup = if matches!(response, Controlled::Cancelled | Controlled::Deadline) {
            cancel_and_release(&connection, task, resource).await
        } else {
            close_and_release(&connection, task, resource).await
        };
        let revalidation = finish(revalidation, cleanup.clone())?;
        ProviderSessionImportOutcome::new(&plan, &request, revalidation, cleanup)
    }
}
