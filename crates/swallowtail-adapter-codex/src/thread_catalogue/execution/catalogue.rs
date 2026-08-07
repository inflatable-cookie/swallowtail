use super::control::{Controlled, wait_controlled};
use super::resource::ScopedResource;
use super::{
    cancel_and_release, cleanup_or, close_and_release, control_before_dispatch, control_failure,
    deadline_wait, finish, from_runtime, require_catalogue_version,
};
use crate::app_server::scope;
use crate::CodexAppServerDriver;
use super::super::protocol::project_page;
use swallowtail_runtime::{
    HostServices, ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage,
};

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
            request.cancellation().as_ref(),
            request.agreement().deadline(),
            &services,
        )?;
        let operation_scope = scope("thread-catalogue", request.request_id());
        let resource = ScopedResource::resolve(&plan, operation_scope.clone(), &services).await?;
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
                        ProviderSessionOperationFailureStage::CatalogueDispatch,
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
                "thread/list",
                serde_json::json!({
                    "cursor": request.cursor().map(|cursor| cursor.as_provider_value()),
                    "limit": plan.agreement().bounds().maximum_page_size().get(),
                    "sourceKinds": ["cli", "vscode", "appServer"],
                    "archived": false,
                    "cwd": cwd
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
        let interrupted = matches!(&response, Controlled::Cancelled | Controlled::Deadline);
        let projected = match response {
            Controlled::Completed(Ok(response)) => project_page(
                &plan,
                &response,
                &cwd,
                request
                    .cursor()
                    .map_or(0, |cursor| cursor.observed_candidates()),
            ),
            Controlled::Completed(Err(error)) => Err(from_runtime(
                ProviderSessionOperationFailureStage::CatalogueDispatch,
                error,
            )),
            Controlled::Cancelled => Err(control_failure(
                ProviderSessionOperationFailureStage::Cancelled,
                "swallowtail.codex.thread_catalogue.cancelled",
                "Codex thread catalogue was cancelled",
            )),
            Controlled::Deadline => Err(control_failure(
                ProviderSessionOperationFailureStage::TimedOut,
                "swallowtail.codex.thread_catalogue.timed_out",
                "Codex thread catalogue timed out",
            )),
        };
        let cleanup = if interrupted {
            cancel_and_release(&connection, task, resource).await
        } else {
            close_and_release(&connection, task, resource).await
        };
        let page = finish(projected, cleanup.clone())?;
        ProviderSessionCatalogueOutcome::new(
            &plan,
            &request,
            page.candidates,
            page.next_cursor,
            cleanup,
        )
    }

}
