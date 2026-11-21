use super::super::protocol::project_reconciliation_activity;
use super::resource::ScopedResource;
use super::{
    abort_runtime, control_before_dispatch, deadline_wait, finish_runtime, release_runtime,
    request_controlled, require_catalogue_version, runtime_from_operation, settle,
};
use crate::CodexAppServerDriver;
use crate::app_server::{INITIALIZE_TIMED_OUT, InitBound, InitControl, initialize_control, scope};
use crate::control_wait::Controlled;
use crate::process_close::close_budget;
use crate::session_replay::{project_interrupted_turn_state, project_thread_history};
use serde_json::json;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    HostServices, InterruptedTurnState, ProviderSessionReconciliationObservation,
    ProviderSessionReconciliationOutcome, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest, RuntimeFailure, bound_provider_session_replay_tail,
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
        let (connection, task) = match self
            .start_connection(
                plan.preflight(),
                behavior,
                operation_scope,
                Some(resource.reference().clone()),
                &services,
                InitBound {
                    experimental_api: false,
                    cancellation: Some(request.cancellation().as_ref()),
                    deadline: agreement.deadline(),
                    timed_out: INITIALIZE_TIMED_OUT,
                },
            )
            .await
        {
            Ok(started) => started,
            Err(error) => {
                let error = match initialize_control(&error) {
                    Some(InitControl::Cancelled) => RuntimeFailure::new(SafeDiagnostic::new(
                        "swallowtail.codex.thread_reconciliation.cancelled",
                        "Codex thread reconciliation was cancelled",
                    )),
                    Some(InitControl::TimedOut) => RuntimeFailure::new(SafeDiagnostic::new(
                        "swallowtail.codex.thread_reconciliation.timed_out",
                        "Codex thread reconciliation timed out",
                    )),
                    None => error,
                };
                return Err(release_runtime(resource, error).await);
            }
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
                let activity = project_reconciliation_activity(
                    &response,
                    agreement.binding().provider_session_ref(),
                    &cwd,
                )
                .map_err(runtime_from_operation)?;
                let replay =
                    project_thread_history(&response, agreement.binding().provider_session_ref())?;
                let (replay, complete) =
                    bound_provider_session_replay_tail(replay, agreement.bounds());
                Ok(match agreement.provider_turn_ref() {
                    Some(provider_turn_ref) => {
                        ProviderSessionReconciliationObservation::exact_turn(
                            project_interrupted_turn_state(&response, provider_turn_ref)?,
                            provider_turn_ref.clone(),
                            replay,
                            complete,
                        )
                    }
                    None => ProviderSessionReconciliationObservation::session_scoped(
                        match activity {
                            swallowtail_core::ProviderSessionActivityState::Active => {
                                InterruptedTurnState::Active
                            }
                            swallowtail_core::ProviderSessionActivityState::Inactive => {
                                InterruptedTurnState::InactiveUnresolved
                            }
                            swallowtail_core::ProviderSessionActivityState::Unknown => {
                                InterruptedTurnState::Unknown
                            }
                        },
                        replay,
                        complete,
                    ),
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
        let cleanup = settle(
            &connection,
            task,
            resource,
            interrupted,
            close_budget(&services, agreement.deadline()),
        )
        .await;
        let observation = finish_runtime(projected, &cleanup)?;
        ProviderSessionReconciliationOutcome::new(&plan, &request, observation, cleanup)
    }
}
