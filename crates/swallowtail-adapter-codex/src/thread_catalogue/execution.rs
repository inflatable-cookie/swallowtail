use super::protocol::{project_page, project_reconciliation_activity, project_revalidation};
#[path = "execution/control.rs"]
mod control;
#[path = "execution/resource.rs"]
mod resource;
use crate::app_server::{close_connection, scope};
use crate::selection::supports_thread_catalogue_version;
use crate::session_replay::{project_interrupted_turn_state, project_thread_history};
use crate::{CodexAppServerDriver, rpc::RpcConnection};
use control::{Controlled, wait_controlled};
use resource::ScopedResource;
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, HostServices, JoinedTask,
    ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest,
    ProviderSessionImportOutcome, ProviderSessionImportPlan, ProviderSessionImportRequest,
    ProviderSessionOperationFailure, ProviderSessionOperationFailureStage,
    ProviderSessionReconciliationOutcome, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest, RuntimeFailure,
};

impl CodexAppServerDriver {
    pub(super) async fn execute_thread_catalogue(
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

    pub(super) async fn execute_thread_import(
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

    pub(super) async fn execute_thread_reconciliation(
        &self,
        plan: ProviderSessionReconciliationPlan,
        request: ProviderSessionReconciliationRequest,
        services: HostServices,
    ) -> Result<ProviderSessionReconciliationOutcome, RuntimeFailure> {
        require_catalogue_version(plan.preflight()).map_err(runtime_from_operation)?;
        let behavior = self.validate_plan(plan.preflight())?;
        let agreement = plan.agreement();
        control_before_dispatch(
            request.cancellation().as_ref(),
            agreement.deadline(),
            &services,
        )
        .map_err(runtime_from_operation)?;
        let operation_scope = scope("thread-reconciliation", request.request_id());
        let resource =
            ScopedResource::resolve_reconciliation(&plan, operation_scope.clone(), &services)
                .await
                .map_err(runtime_from_operation)?;
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
                return match cleanup {
                    CleanupOutcome::Clean | CleanupOutcome::NotApplicable => Err(error),
                    CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
                        Err(RuntimeFailure::new(diagnostic))
                    }
                };
            }
        };
        if let Err(control) = control_before_dispatch(
            request.cancellation().as_ref(),
            agreement.deadline(),
            &services,
        ) {
            let cleanup = cancel_and_release(&connection, task, resource).await;
            return match cleanup {
                CleanupOutcome::Clean | CleanupOutcome::NotApplicable => {
                    Err(runtime_from_operation(control))
                }
                CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
                    Err(RuntimeFailure::new(diagnostic))
                }
            };
        }
        let deadline_wait = match deadline_wait(agreement.deadline(), &services) {
            Ok(deadline_wait) => deadline_wait,
            Err(error) => {
                let cleanup = cancel_and_release(&connection, task, resource).await;
                return match cleanup {
                    CleanupOutcome::Clean | CleanupOutcome::NotApplicable => {
                        Err(runtime_from_operation(error))
                    }
                    CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
                        Err(RuntimeFailure::new(diagnostic))
                    }
                };
            }
        };
        let response = connection
            .dispatch_request(
                "thread/read",
                serde_json::json!({
                    "threadId": agreement.binding().provider_session_ref().as_provider_value(),
                    "includeTurns": true
                }),
            )
            .await;
        let response = match response {
            Ok(response) => {
                wait_controlled(response, request.cancellation().as_ref(), deadline_wait).await
            }
            Err(error) => Controlled::Completed(Err(error)),
        };
        let interrupted = matches!(&response, Controlled::Cancelled | Controlled::Deadline);
        let projected = match response {
            Controlled::Completed(Ok(response)) => (|| {
                let activity = project_reconciliation_activity(
                    &response,
                    agreement.binding().provider_session_ref(),
                    &cwd,
                )
                .map_err(runtime_from_operation)?;
                let replay =
                    project_thread_history(&response, agreement.binding().provider_session_ref())?;
                let (replay, complete) = swallowtail_runtime::bound_provider_session_replay_tail(
                    replay,
                    agreement.bounds(),
                );
                Ok(match agreement.provider_turn_ref() {
                    Some(provider_turn_ref) => {
                        swallowtail_runtime::ProviderSessionReconciliationObservation::exact_turn(
                            project_interrupted_turn_state(&response, provider_turn_ref)?,
                            provider_turn_ref.clone(),
                            replay,
                            complete,
                        )
                    }
                    None => {
                        swallowtail_runtime::ProviderSessionReconciliationObservation::session_scoped(
                            match activity {
                                swallowtail_core::ProviderSessionActivityState::Active => {
                                    swallowtail_runtime::InterruptedTurnState::Active
                                }
                                swallowtail_core::ProviderSessionActivityState::Inactive => {
                                    swallowtail_runtime::InterruptedTurnState::InactiveUnresolved
                                }
                                swallowtail_core::ProviderSessionActivityState::Unknown => {
                                    swallowtail_runtime::InterruptedTurnState::Unknown
                                }
                            },
                            replay,
                            complete,
                        )
                    }
                })
            })(),
            Controlled::Completed(Err(error)) => Err(error),
            Controlled::Cancelled => Err(RuntimeFailure::new(SafeDiagnostic::new(
                "swallowtail.codex.thread_reconciliation.cancelled",
                "Codex thread reconciliation was cancelled",
            ))),
            Controlled::Deadline => Err(RuntimeFailure::new(SafeDiagnostic::new(
                "swallowtail.codex.thread_reconciliation.timed_out",
                "Codex thread reconciliation timed out",
            ))),
        };
        let cleanup = if interrupted {
            cancel_and_release(&connection, task, resource).await
        } else {
            close_and_release(&connection, task, resource).await
        };
        let observation = match (projected, &cleanup) {
            (Ok(observation), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => observation,
            (Err(error), _) => return Err(error),
            (Ok(_), CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic)) => {
                return Err(RuntimeFailure::new(diagnostic.clone()));
            }
        };
        ProviderSessionReconciliationOutcome::new(&plan, &request, observation, cleanup)
    }
}

fn require_catalogue_version(
    plan: &swallowtail_core::PreflightPlan,
) -> Result<(), ProviderSessionOperationFailure> {
    let version = plan.interface_versions().next().ok_or_else(|| {
        control_failure(
            ProviderSessionOperationFailureStage::BeforeDispatch,
            "swallowtail.codex.thread_catalogue.version_missing",
            "Codex thread catalogue requires an exact executable version",
        )
    })?;
    if supports_thread_catalogue_version(version.version()) {
        Ok(())
    } else {
        Err(control_failure(
            ProviderSessionOperationFailureStage::BeforeDispatch,
            "swallowtail.codex.thread_catalogue.version_unsupported",
            "Codex executable version does not support the qualified thread catalogue",
        ))
    }
}

fn control_before_dispatch(
    cancellation: &swallowtail_runtime::ImmediateCancellation,
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<(), ProviderSessionOperationFailure> {
    if cancellation.is_requested() {
        return Err(control_failure(
            ProviderSessionOperationFailureStage::Cancelled,
            "swallowtail.codex.thread_catalogue.cancelled",
            "Codex provider-session operation was cancelled",
        ));
    }
    if deadline.is_some_and(|deadline| {
        services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    }) {
        return Err(control_failure(
            ProviderSessionOperationFailureStage::TimedOut,
            "swallowtail.codex.thread_catalogue.timed_out",
            "Codex provider-session operation timed out",
        ));
    }
    Ok(())
}

fn deadline_wait(
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<Option<BoxFuture<'static, DeadlineObservation>>, ProviderSessionOperationFailure> {
    deadline
        .map(|deadline| {
            services
                .time()
                .ok_or_else(|| {
                    control_failure(
                        ProviderSessionOperationFailureStage::BeforeDispatch,
                        "swallowtail.codex.thread_catalogue.time_service_missing",
                        "Deadline-bound Codex provider-session operation requires a time service",
                    )
                })
                .map(|time| time.wait_until(deadline))
        })
        .transpose()
}

async fn close_and_release(
    connection: &Arc<RpcConnection>,
    task: Box<dyn JoinedTask>,
    resource: ScopedResource,
) -> CleanupOutcome {
    let connection = close_connection(connection, task).await;
    merge_cleanup(connection, resource.release().await)
}

async fn cancel_and_release(
    connection: &Arc<RpcConnection>,
    task: Box<dyn JoinedTask>,
    resource: ScopedResource,
) -> CleanupOutcome {
    let stop = connection.cancel_session().await;
    let join = task.join().await;
    let connection_cleanup = if stop.is_err() || join.is_err() {
        CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.codex.thread_catalogue.cancel_cleanup_failed",
            "Codex provider-session cancellation cleanup failed",
        ))
    } else {
        connection.cleanup_outcome()
    };
    merge_cleanup(connection_cleanup, resource.release().await)
}

fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, CleanupOutcome::NotApplicable) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}

fn finish<T>(
    result: Result<T, ProviderSessionOperationFailure>,
    cleanup: CleanupOutcome,
) -> Result<T, ProviderSessionOperationFailure> {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => result,
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            Err(ProviderSessionOperationFailure::new(
                ProviderSessionOperationFailureStage::Cleanup,
                diagnostic,
            ))
        }
    }
}

pub(super) fn cleanup_or(
    cleanup: CleanupOutcome,
    otherwise: ProviderSessionOperationFailure,
) -> ProviderSessionOperationFailure {
    finish::<()>(Err(otherwise.clone()), cleanup).unwrap_err()
}

pub(super) fn from_runtime(
    stage: ProviderSessionOperationFailureStage,
    error: RuntimeFailure,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(stage, error.diagnostic().clone())
}

fn runtime_from_operation(error: ProviderSessionOperationFailure) -> RuntimeFailure {
    RuntimeFailure::new(error.diagnostic().clone())
}

pub(super) fn control_failure(
    stage: ProviderSessionOperationFailureStage,
    code: &'static str,
    message: &'static str,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(stage, SafeDiagnostic::new(code, message))
}
