use super::resource::ScopedResource;
use super::{
    abort_runtime, control_before_dispatch, deadline_wait, finish_runtime, release_runtime,
    request_controlled, require_catalogue_version, runtime_from_operation, settle,
};
use crate::CodexAppServerDriver;
use crate::app_server::scope;
use crate::control_wait::Controlled;
use crate::process_close::close_budget;
use crate::session_replay::project_thread_history;
use serde_json::json;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    HostServices, ProviderSessionHistoryPage, ProviderSessionHistoryPlan,
    ProviderSessionHistoryRequest, ProviderSessionHistoryTotal, RuntimeFailure,
    page_provider_session_history_window,
};

impl CodexAppServerDriver {
    pub(crate) async fn execute_thread_history(
        &self,
        plan: ProviderSessionHistoryPlan,
        request: ProviderSessionHistoryRequest,
        services: HostServices,
    ) -> Result<ProviderSessionHistoryPage, RuntimeFailure> {
        require_catalogue_version(plan.preflight()).map_err(runtime_from_operation)?;
        let behavior = self.validate_plan(plan.preflight())?;
        let agreement = plan.agreement();
        control_before_dispatch(
            request.cancellation().as_ref(),
            agreement.deadline(),
            &services,
        )
        .map_err(runtime_from_operation)?;
        let operation_scope = scope("thread-history", request.request_id());
        let resource = ScopedResource::resolve_history(&plan, operation_scope.clone(), &services)
            .await
            .map_err(runtime_from_operation)?;
        let (connection, task) = match self
            .start_connection(
                plan.preflight(),
                behavior,
                operation_scope,
                Some(resource.reference().clone()),
                false,
                &services,
            )
            .await
        {
            Ok(started) => started,
            Err(error) => return Err(release_runtime(resource, error).await),
        };
        if let Err(control) = control_before_dispatch(
            request.cancellation().as_ref(),
            agreement.deadline(),
            &services,
        ) {
            return Err(abort_runtime(
                &connection,
                task,
                resource,
                runtime_from_operation(control),
            )
            .await);
        }
        let deadline_wait = match deadline_wait(agreement.deadline(), &services) {
            Ok(deadline_wait) => deadline_wait,
            Err(error) => {
                return Err(abort_runtime(
                    &connection,
                    task,
                    resource,
                    runtime_from_operation(error),
                )
                .await);
            }
        };
        let response = request_controlled(
            &connection,
            "thread/read",
            json!({
                "threadId": agreement.binding().provider_session_ref().as_provider_value(),
                "includeTurns": true
            }),
            request.cancellation().as_ref(),
            deadline_wait,
        )
        .await;
        let interrupted = matches!(&response, Controlled::Cancelled | Controlled::Deadline);
        let projected = match response {
            Controlled::Completed(Ok(response)) => (|| {
                let replay =
                    project_thread_history(&response, agreement.binding().provider_session_ref())?;
                let total = u32::try_from(replay.len()).map_err(|_| {
                    RuntimeFailure::new(SafeDiagnostic::new(
                        "swallowtail.codex.thread_history.total_invalid",
                        "Codex thread history exceeds portable cardinality",
                    ))
                })?;
                page_provider_session_history_window(
                    &plan,
                    &request,
                    replay,
                    ProviderSessionHistoryTotal::Exact(total),
                )
            })(),
            Controlled::Completed(Err(error)) => Err(error),
            Controlled::Cancelled => Err(RuntimeFailure::new(SafeDiagnostic::new(
                "swallowtail.codex.thread_history.cancelled",
                "Codex thread history was cancelled",
            ))),
            Controlled::Deadline => Err(RuntimeFailure::new(SafeDiagnostic::new(
                "swallowtail.codex.thread_history.timed_out",
                "Codex thread history timed out",
            ))),
        };
        let cleanup = settle(
            &connection,
            task,
            resource,
            interrupted,
            close_budget(&services, agreement.deadline()),
        )
        .await;
        let window = finish_runtime(projected, &cleanup)?;
        ProviderSessionHistoryPage::new(&plan, &request, window, cleanup)
    }
}
