use super::super::protocol::project_revalidation;
use super::pipeline::{Controls, Operation, run_controlled_rpc};
use super::resource::ScopedResource;
use super::{control_before_dispatch, finish, from_runtime, require_catalogue_version};
use crate::CodexAppServerDriver;
use crate::app_server::scope;
use crate::session_replay::project_thread_history;
use serde_json::json;
use swallowtail_runtime::{
    HostServices, ProviderSessionImportOutcome, ProviderSessionImportPlan,
    ProviderSessionImportRequest, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage,
};

const OPERATION: Operation = Operation::new(
    (
        "swallowtail.codex.thread_import.cancelled",
        "Codex thread import was cancelled",
    ),
    (
        "swallowtail.codex.thread_import.timed_out",
        "Codex thread import timed out",
    ),
    ProviderSessionOperationFailureStage::ImportRevalidation,
);

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
            OPERATION,
            request.cancellation().as_ref(),
            request.agreement().deadline(),
            &services,
        )?;
        let operation_scope = scope("thread-import", request.request_id());
        let resource =
            ScopedResource::resolve_import(&plan, operation_scope.clone(), &services).await?;
        let cwd = resource.root().to_owned();
        let settled = run_controlled_rpc(
            self,
            plan.preflight(),
            behavior,
            operation_scope,
            resource,
            Controls {
                operation: OPERATION,
                cancellation: request.cancellation().as_ref(),
                deadline: request.agreement().deadline(),
            },
            &services,
            "thread/read",
            json!({
                "threadId": request.provider_session_ref().as_provider_value(),
                "includeTurns": true
            }),
            |response| {
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
            },
        )
        .await?;
        let revalidation = finish(settled.projected, settled.cleanup.clone())?;
        ProviderSessionImportOutcome::new(&plan, &request, revalidation, settled.cleanup)
    }
}
