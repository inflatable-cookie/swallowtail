use super::super::protocol::project_page;
use super::pipeline::{Controls, Operation, run_controlled_rpc};
use super::resource::ScopedResource;
use super::{control_before_dispatch, finish, from_runtime, require_catalogue_version};
use crate::CodexAppServerDriver;
use crate::app_server::scope;
use serde_json::json;
use swallowtail_runtime::{
    HostServices, ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage,
};

const OPERATION: Operation = Operation::new(
    (
        "swallowtail.codex.thread_catalogue.cancelled",
        "Codex thread catalogue was cancelled",
    ),
    (
        "swallowtail.codex.thread_catalogue.timed_out",
        "Codex thread catalogue timed out",
    ),
    ProviderSessionOperationFailureStage::CatalogueDispatch,
);

impl CodexAppServerDriver {
    pub(crate) async fn execute_thread_catalogue(
        &self,
        plan: ProviderSessionCataloguePlan,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure> {
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
        let operation_scope = scope("thread-catalogue", request.request_id());
        let resource = ScopedResource::resolve(&plan, operation_scope.clone(), &services).await?;
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
            "thread/list",
            json!({
                "cursor": request.cursor().map(|cursor| cursor.as_provider_value()),
                "limit": plan.agreement().bounds().maximum_page_size().get(),
                "sourceKinds": ["cli", "vscode", "appServer"],
                "archived": false,
                "cwd": cwd
            }),
            |response| {
                project_page(
                    &plan,
                    response,
                    &cwd,
                    request
                        .cursor()
                        .map_or(0, |cursor| cursor.observed_candidates()),
                )
            },
        )
        .await?;
        let page = finish(settled.projected, settled.cleanup.clone())?;
        ProviderSessionCatalogueOutcome::new(
            &plan,
            &request,
            page.candidates,
            page.next_cursor,
            settled.cleanup,
        )
    }
}
