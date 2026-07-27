use super::super::validation::{validate_initialize, validate_plan};
use super::access::open_management_connection;
use super::control::{Controlled, cancelled_or_expired, deadline_wait, wait_controlled};
use super::*;
use crate::failure::failure;
use swallowtail_core::{
    ProviderSessionAffectedScope, ProviderSessionDeletionStrength, ProviderSessionManagementAction,
    ProviderSessionManagementEffect,
};
use swallowtail_runtime::{ImmediateCancellation, ProviderSessionManagementAgreement};

impl ClaudeAgentAcpDriver {
    pub(super) async fn manage_delete(
        &self,
        plan: ProviderSessionManagementPlan,
        agreement: &ProviderSessionManagementAgreement,
        cancellation: &ImmediateCancellation,
        request_id: &RequestId,
        services: HostServices,
    ) -> Result<ProviderSessionManagementOutcome, RuntimeFailure> {
        let action = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        );
        if agreement.action() != action {
            return Err(failure(
                "swallowtail.claude_agent.lifecycle.deletion_strength_mismatch",
                "Claude Agent management plan requests a different deletion strength",
            ));
        }
        if agreement.affected_scope() != ProviderSessionAffectedScope::ProviderDefinedDescendants {
            return Err(failure(
                "swallowtail.claude_agent.lifecycle.affected_scope_mismatch",
                "Claude Agent management plan requests a different affected scope",
            ));
        }
        let selected = validate_plan(plan.preflight(), &self.credential)?;
        if cancelled_or_expired(agreement, cancellation, &services)? {
            return Ok(outcome(
                agreement,
                ProviderSessionManagementEffect::failed_before_effect(action),
            ));
        }

        let mut pending =
            open_management_connection(self, plan.preflight(), agreement, request_id, &services)
                .await?;
        let initialized = wait_controlled(
            pending.connection.initialize(),
            cancellation,
            deadline_wait(agreement, &services)?,
        )
        .await;
        match initialized {
            Controlled::Completed(Ok(response)) => {
                let lifecycle = match validate_initialize(&response, &selected) {
                    Ok(lifecycle) => lifecycle,
                    Err(error) => {
                        let _ = pending.close(&services).await;
                        return Err(error);
                    }
                };
                if !lifecycle.delete {
                    let cleanup = pending.close(&services).await;
                    return Ok(with_cleanup(
                        outcome(
                            agreement,
                            ProviderSessionManagementEffect::failed_before_effect(action),
                        )
                        .with_diagnostic(
                            swallowtail_core::SafeDiagnostic::new(
                                "swallowtail.claude_agent.lifecycle.delete_unavailable",
                                "Claude Agent did not negotiate ACP session deletion",
                            ),
                        ),
                        cleanup,
                    ));
                }
            }
            Controlled::Completed(Err(error)) => {
                let _ = pending.close(&services).await;
                return Err(error);
            }
            Controlled::Cancelled | Controlled::Deadline => {
                let cleanup = pending.close(&services).await;
                return Ok(with_cleanup(
                    outcome(
                        agreement,
                        ProviderSessionManagementEffect::failed_before_effect(action),
                    ),
                    cleanup,
                ));
            }
        }
        if cancelled_or_expired(agreement, cancellation, &services)? {
            let cleanup = pending.close(&services).await;
            return Ok(with_cleanup(
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ),
                cleanup,
            ));
        }

        let response = match pending
            .connection
            .begin_request(
                "session/delete",
                json!({
                    "sessionId": agreement
                        .binding()
                        .provider_session_ref()
                        .as_provider_value()
                }),
            )
            .await
        {
            Ok(response) => response,
            Err(error) => {
                let cleanup = pending.close(&services).await;
                return Ok(with_cleanup(
                    outcome(
                        agreement,
                        ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                    )
                    .with_diagnostic(error.diagnostic().clone()),
                    cleanup,
                ));
            }
        };
        let response =
            wait_controlled(response, cancellation, deadline_wait(agreement, &services)?).await;
        let result = match response {
            Controlled::Completed(Ok(response))
                if response.as_object().is_some_and(serde_json::Map::is_empty) =>
            {
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::applied(
                        action,
                        ProviderSessionAffectedScope::ProviderDefinedDescendants,
                    ),
                )
            }
            Controlled::Completed(Ok(_)) => outcome(
                agreement,
                ProviderSessionManagementEffect::unconfirmed_after_effect(action),
            )
            .with_diagnostic(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.claude_agent.lifecycle.delete_malformed",
                "Claude Agent returned a malformed ACP session-delete response",
            )),
            Controlled::Completed(Err(error)) => outcome(
                agreement,
                ProviderSessionManagementEffect::unconfirmed_after_effect(action),
            )
            .with_diagnostic(error.diagnostic().clone()),
            Controlled::Cancelled | Controlled::Deadline => {
                let _ = pending.connection.cancel_session().await;
                let cleanup = pending.close(&services).await;
                return Ok(with_cleanup(
                    outcome(
                        agreement,
                        ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                    ),
                    cleanup,
                ));
            }
        };
        let cleanup = pending.close(&services).await;
        Ok(with_cleanup(result, cleanup))
    }
}

fn outcome(
    agreement: &ProviderSessionManagementAgreement,
    effect: ProviderSessionManagementEffect,
) -> ProviderSessionManagementOutcome {
    ProviderSessionManagementOutcome::new(agreement.binding().clone(), effect)
}

fn with_cleanup(
    outcome: ProviderSessionManagementOutcome,
    cleanup: CleanupOutcome,
) -> ProviderSessionManagementOutcome {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => outcome,
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            outcome.with_diagnostic(diagnostic)
        }
    }
}
