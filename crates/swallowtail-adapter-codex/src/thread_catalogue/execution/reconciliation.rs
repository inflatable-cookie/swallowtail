use super::super::protocol::project_reconciliation_activity;
use super::control::{Controlled, wait_controlled};
use super::resource::ScopedResource;
use super::{
    cancel_and_release, close_and_release, control_before_dispatch, deadline_wait,
    require_catalogue_version, runtime_from_operation,
};
use crate::CodexAppServerDriver;
use crate::app_server::scope;
use crate::session_replay::{project_interrupted_turn_state, project_thread_history};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, HostServices, ProviderSessionReconciliationOutcome,
    ProviderSessionReconciliationPlan, ProviderSessionReconciliationRequest, RuntimeFailure,
};

impl CodexAppServerDriver {
    pub(crate) async fn execute_thread_reconciliation(
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
