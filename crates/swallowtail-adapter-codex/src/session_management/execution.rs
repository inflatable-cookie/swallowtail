use super::protocol::{
    failure, lifecycle_assessment, lifecycle_request, notification_method, outcome,
    validate_lifecycle_response, with_cleanup,
};
use crate::CodexAppServerDriver;
use crate::app_server::{close_connection, scope};
use std::future::Future;
use std::task::Poll;
use swallowtail_core::{
    ProviderSessionEffectTruth, ProviderSessionManagementAction, ProviderSessionManagementEffect,
    SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ImmediateCancellation, ProviderSessionManagementAgreement,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RuntimeFailure,
};

impl CodexAppServerDriver {
    pub(super) async fn manage(
        &self,
        plan: ProviderSessionManagementPlan,
        agreement: &ProviderSessionManagementAgreement,
        cancellation: &ImmediateCancellation,
        request_id: &str,
        services: HostServices,
    ) -> Result<ProviderSessionManagementOutcome, RuntimeFailure> {
        let action = agreement.action();
        let preflight = plan.preflight();
        let app_behavior = self.validate_plan(preflight)?;
        let assessment = lifecycle_assessment(preflight)?;
        if !assessment.behavior.supports(action) {
            return Err(failure(
                "swallowtail.codex.lifecycle.action_unsupported",
                "Prepared Codex version does not support this thread lifecycle action",
            ));
        }
        if matches!(action, ProviderSessionManagementAction::Delete(_))
            && assessment.behavior.delete_action() != Some(action)
        {
            return Err(failure(
                "swallowtail.codex.lifecycle.deletion_strength_mismatch",
                "Codex management plan requests a different deletion strength",
            ));
        }
        if agreement.affected_scope() != assessment.behavior.affected_scope(action) {
            return Err(failure(
                "swallowtail.codex.lifecycle.affected_scope_mismatch",
                "Codex management plan requests a different affected scope",
            ));
        }
        if cancelled_or_expired(agreement, cancellation, &services)? {
            return Ok(outcome(
                agreement,
                ProviderSessionManagementEffect::failed_before_effect(action),
            ));
        }

        let request_id = swallowtail_runtime::RequestId::new(request_id)
            .expect("validated request id remains valid");
        let (connection, task) = self
            .spawn_connection(
                preflight,
                app_behavior,
                scope("management", &request_id),
                agreement.binding().working_resource().cloned(),
                &services,
            )
            .await?;
        let initialization = wait_controlled(
            connection.initialize(false),
            cancellation,
            deadline_wait(agreement, &services)?,
        )
        .await;
        match initialization {
            Controlled::Completed(Ok(())) => {}
            Controlled::Completed(Err(error)) => {
                let _ = connection.cancel_session().await;
                let _ = task.join().await;
                return Err(error);
            }
            Controlled::Cancelled | Controlled::Deadline => {
                let _ = connection.cancel_session().await;
                let _ = task.join().await;
                return Ok(outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ));
            }
        }
        if cancelled_or_expired(agreement, cancellation, &services)? {
            let cleanup = close_connection(&connection, task).await;
            return Ok(with_cleanup(
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ),
                cleanup,
            ));
        }

        let target = agreement
            .binding()
            .provider_session_ref()
            .as_provider_value();
        let (method, params) = lifecycle_request(action, target);
        let response = match connection.dispatch_request(method, params).await {
            Ok(response) => response,
            Err(error) => {
                let cleanup = close_connection(&connection, task).await;
                return Ok(with_cleanup(
                    outcome(
                        agreement,
                        ProviderSessionManagementEffect::failed_before_effect(action),
                    )
                    .with_diagnostic(error.diagnostic().clone()),
                    cleanup,
                ));
            }
        };

        let response =
            wait_controlled(response, cancellation, deadline_wait(agreement, &services)?).await;
        let result = match response {
            Controlled::Cancelled | Controlled::Deadline => {
                let _ = connection.cancel_session().await;
                let _ = task.join().await;
                return Ok(outcome(
                    agreement,
                    ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                ));
            }
            Controlled::Completed(result) => result,
        };

        let cleanup = close_connection(&connection, task).await;
        let mut result = match result {
            Ok(response) => match validate_lifecycle_response(action, target, &response) {
                Ok(()) => outcome(
                    agreement,
                    ProviderSessionManagementEffect::applied(
                        action,
                        assessment.behavior.affected_scope(action),
                    ),
                ),
                Err(error) => outcome(
                    agreement,
                    ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                )
                .with_diagnostic(error.diagnostic().clone()),
            },
            Err(error)
                if error.diagnostic().code() == "swallowtail.codex.app_server.request_failed" =>
            {
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                )
                .with_diagnostic(error.diagnostic().clone())
            }
            Err(error) => outcome(
                agreement,
                ProviderSessionManagementEffect::unconfirmed_after_effect(action),
            )
            .with_diagnostic(error.diagnostic().clone()),
        };
        if result.effect().truth() == ProviderSessionEffectTruth::Applied
            && assessment.behavior.expects_notification(action)
            && !connection
                .lifecycle_notifications()
                .iter()
                .any(|notification| notification.matches(notification_method(action), target))
        {
            result = result.with_diagnostic(SafeDiagnostic::new(
                "swallowtail.codex.lifecycle.notification_disagreement",
                "Codex lifecycle response succeeded without the expected matching notification",
            ));
        }
        Ok(with_cleanup(result, cleanup))
    }
}

fn cancelled_or_expired(
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
) -> Result<bool, RuntimeFailure> {
    if cancellation.is_requested() {
        return Ok(true);
    }
    let Some(deadline) = agreement.deadline() else {
        return Ok(false);
    };
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.codex.lifecycle.time_service_missing",
            "Deadline-bound Codex lifecycle management requires a time service",
        )
    })?;
    Ok(time.now() >= deadline.instant())
}

fn deadline_wait(
    agreement: &ProviderSessionManagementAgreement,
    services: &HostServices,
) -> Result<Option<BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>, RuntimeFailure> {
    agreement
        .deadline()
        .map(|deadline| {
            services
                .time()
                .ok_or_else(|| {
                    failure(
                        "swallowtail.codex.lifecycle.time_service_missing",
                        "Deadline-bound Codex lifecycle management requires a time service",
                    )
                })
                .map(|time| time.wait_until(deadline))
        })
        .transpose()
}

enum Controlled<T> {
    Completed(T),
    Cancelled,
    Deadline,
}

async fn wait_controlled<F, T>(
    operation: F,
    cancellation: &ImmediateCancellation,
    deadline: Option<BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>,
) -> Controlled<T>
where
    F: Future<Output = T>,
{
    let mut operation = Box::pin(operation);
    let mut cancelled = cancellation.wait_requested();
    let mut deadline = deadline;
    std::future::poll_fn(|context| {
        if let Poll::Ready(result) = operation.as_mut().poll(context) {
            return Poll::Ready(Controlled::Completed(result));
        }
        if cancelled.as_mut().poll(context).is_ready() {
            return Poll::Ready(Controlled::Cancelled);
        }
        if deadline
            .as_mut()
            .is_some_and(|deadline| deadline.as_mut().poll(context).is_ready())
        {
            return Poll::Ready(Controlled::Deadline);
        }
        Poll::Pending
    })
    .await
}
