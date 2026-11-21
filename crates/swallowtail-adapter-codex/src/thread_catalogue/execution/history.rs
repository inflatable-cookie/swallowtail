use super::pipeline::{Controls, Operation, run_controlled_rpc};
use super::resource::ScopedResource;
use super::{control_before_dispatch, finish, require_catalogue_version, runtime_from_operation};
use crate::CodexAppServerDriver;
use crate::app_server::scope;
use crate::session_replay::project_thread_history;
use serde_json::json;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    HostServices, ProviderSessionHistoryPage, ProviderSessionHistoryPlan,
    ProviderSessionHistoryRequest, ProviderSessionHistoryTotal,
    ProviderSessionOperationFailureStage, RuntimeFailure, page_provider_session_history_window,
};

/// History reports runtime failures, so its dispatch stage is only a label:
/// the stage is dropped when the failure crosses back to the runtime.
const OPERATION: Operation = Operation::new(
    (
        "swallowtail.codex.thread_history.cancelled",
        "Codex thread history was cancelled",
    ),
    (
        "swallowtail.codex.thread_history.timed_out",
        "Codex thread history timed out",
    ),
    ProviderSessionOperationFailureStage::CatalogueDispatch,
);

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
            OPERATION,
            request.cancellation().as_ref(),
            agreement.deadline(),
            &services,
        )
        .map_err(runtime_from_operation)?;
        let operation_scope = scope("thread-history", request.request_id());
        let resource = ScopedResource::resolve_history(&plan, operation_scope.clone(), &services)
            .await
            .map_err(runtime_from_operation)?;
        let settled = run_controlled_rpc(
            self,
            plan.preflight(),
            behavior,
            operation_scope,
            resource,
            Controls {
                operation: OPERATION,
                cancellation: request.cancellation().as_ref(),
                deadline: agreement.deadline(),
            },
            &services,
            "thread/read",
            json!({
                "threadId": agreement.binding().provider_session_ref().as_provider_value(),
                "includeTurns": true
            }),
            |response| {
                let replay =
                    project_thread_history(response, agreement.binding().provider_session_ref())
                        .map_err(|error| OPERATION.runtime(error))?;
                let total = u32::try_from(replay.len()).map_err(|_| {
                    OPERATION.runtime(RuntimeFailure::new(SafeDiagnostic::new(
                        "swallowtail.codex.thread_history.total_invalid",
                        "Codex thread history exceeds portable cardinality",
                    )))
                })?;
                page_provider_session_history_window(
                    &plan,
                    &request,
                    replay,
                    ProviderSessionHistoryTotal::Exact(total),
                )
                .map_err(|error| OPERATION.runtime(error))
            },
        )
        .await
        .map_err(runtime_from_operation)?;
        let window =
            finish(settled.projected, settled.cleanup.clone()).map_err(runtime_from_operation)?;
        ProviderSessionHistoryPage::new(&plan, &request, window, settled.cleanup)
    }
}
