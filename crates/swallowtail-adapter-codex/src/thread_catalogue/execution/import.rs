use super::super::protocol::project_revalidation;
use super::resource::ScopedResource;
use super::{
    cancel_and_release, cleanup_or, control_before_dispatch, control_failure, deadline_wait,
    finish, from_runtime, request_controlled, require_catalogue_version, settle,
};
use crate::CodexAppServerDriver;
use crate::app_server::{INITIALIZE_TIMED_OUT, InitBound, InitControl, initialize_control, scope};
use crate::control_wait::Controlled;
use crate::process_close::close_budget;
use crate::session_replay::project_thread_history;
use serde_json::json;
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
                &services,
                InitBound {
                    experimental_api: false,
                    cancellation: Some(request.cancellation().as_ref()),
                    deadline: request.agreement().deadline(),
                    timed_out: INITIALIZE_TIMED_OUT,
                },
            )
            .await;
        let (connection, task) = match connection {
            Ok(connection) => connection,
            Err(error) => {
                let cleanup = resource.release().await;
                let failure = match initialize_control(&error) {
                    Some(InitControl::Cancelled) => control_failure(
                        ProviderSessionOperationFailureStage::Cancelled,
                        "swallowtail.codex.thread_import.cancelled",
                        "Codex thread import was cancelled",
                    ),
                    Some(InitControl::TimedOut) => control_failure(
                        ProviderSessionOperationFailureStage::TimedOut,
                        "swallowtail.codex.thread_import.timed_out",
                        "Codex thread import timed out",
                    ),
                    None => from_runtime(
                        ProviderSessionOperationFailureStage::ImportRevalidation,
                        error,
                    ),
                };
                return Err(cleanup_or(cleanup, failure));
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

        let deadline = request.agreement().deadline();
        let deadline_wait = match deadline_wait(deadline, &services) {
            Ok(wait) => wait,
            Err(error) => {
                let cleanup = cancel_and_release(&connection, task, resource).await;
                return Err(cleanup_or(cleanup, error));
            }
        };
        let response = request_controlled(
            &connection,
            "thread/read",
            json!({
                "threadId": request.provider_session_ref().as_provider_value(),
                "includeTurns": true
            }),
            request.cancellation().as_ref(),
            deadline_wait,
        )
        .await;
        let revalidation = match &response {
            Controlled::Completed(Ok(response)) => {
                project_thread_history(response, request.provider_session_ref())
                    .map_err(|error| {
                        from_runtime(
                            ProviderSessionOperationFailureStage::ImportRevalidation,
                            error,
                        )
                    })
                    .and_then(|_| {
                        project_revalidation(
                            response,
                            request.provider_session_ref(),
                            &cwd,
                            plan.agreement().candidate().updated_at_unix_milliseconds(),
                            plan.agreement().candidate_id().clone(),
                            plan.agreement().working_resource().clone(),
                        )
                    })
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
        let cleanup = settle(
            &connection,
            task,
            resource,
            matches!(response, Controlled::Cancelled | Controlled::Deadline),
            close_budget(&services, deadline),
        )
        .await;
        let revalidation = finish(revalidation, cleanup.clone())?;
        ProviderSessionImportOutcome::new(&plan, &request, revalidation, cleanup)
    }
}
