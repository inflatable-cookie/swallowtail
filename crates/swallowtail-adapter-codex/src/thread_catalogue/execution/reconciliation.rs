use super::super::protocol::project_reconciliation_activity;
use super::pipeline::{Controls, Operation, run_controlled_rpc};
use super::resource::ScopedResource;
use super::{control_before_dispatch, finish, require_catalogue_version, runtime_from_operation};
use crate::CodexAppServerDriver;
use crate::app_server::scope;
use crate::session_replay::{project_interrupted_turn_state, project_thread_history};
use serde_json::json;
use swallowtail_runtime::{
    HostServices, InterruptedTurnState, ProviderSessionOperationFailureStage,
    ProviderSessionReconciliationObservation, ProviderSessionReconciliationOutcome,
    ProviderSessionReconciliationPlan, ProviderSessionReconciliationRequest, RuntimeFailure,
    bound_provider_session_replay_tail,
};

/// Reconciliation reports runtime failures, so its dispatch stage is only a
/// label: the stage is dropped when the failure crosses back to the runtime.
const OPERATION: Operation = Operation::new(
    (
        "swallowtail.codex.thread_reconciliation.cancelled",
        "Codex thread reconciliation was cancelled",
    ),
    (
        "swallowtail.codex.thread_reconciliation.timed_out",
        "Codex thread reconciliation timed out",
    ),
    ProviderSessionOperationFailureStage::CatalogueDispatch,
);

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
            OPERATION,
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
                let activity = project_reconciliation_activity(
                    response,
                    agreement.binding().provider_session_ref(),
                    &cwd,
                )?;
                let replay =
                    project_thread_history(response, agreement.binding().provider_session_ref())
                        .map_err(|error| OPERATION.runtime(error))?;
                let (replay, complete) =
                    bound_provider_session_replay_tail(replay, agreement.bounds());
                Ok(match agreement.provider_turn_ref() {
                    Some(provider_turn_ref) => {
                        ProviderSessionReconciliationObservation::exact_turn(
                            project_interrupted_turn_state(response, provider_turn_ref)
                                .map_err(|error| OPERATION.runtime(error))?,
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
            },
        )
        .await
        .map_err(runtime_from_operation)?;
        let observation =
            finish(settled.projected, settled.cleanup.clone()).map_err(runtime_from_operation)?;
        ProviderSessionReconciliationOutcome::new(&plan, &request, observation, settled.cleanup)
    }
}
