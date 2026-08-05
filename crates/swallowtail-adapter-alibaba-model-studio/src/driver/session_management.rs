use super::AlibabaModelStudioDriver;
use super::access::AccessLeases;
use super::session::cleanup::{CleanupAccess, ManagedDeletion, delete_retained_conversation};
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::ConversationRef;
use swallowtail_core::{
    ProviderSessionAffectedScope, ProviderSessionDeletionStrength, ProviderSessionManagementAction,
    ProviderSessionManagementEffect,
};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxFuture, CleanupOutcome, DeleteProviderSessionRequest,
    HostServices, ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RestoreProviderSessionRequest, RuntimeFailure, ScopeId,
    validate_provider_session_management_request,
};

impl ProviderSessionManagementDriver for AlibabaModelStudioDriver {
    fn archive_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: ArchiveProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("provider-session archive")) })
    }

    fn restore_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: RestoreProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("provider-session restore")) })
    }

    fn delete_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: DeleteProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            Self::validate_plan(plan.preflight())?;
            let agreement = request.agreement();
            let action = ProviderSessionManagementAction::Delete(
                ProviderSessionDeletionStrength::ProviderDataDeleted,
            );
            if agreement.action() != action
                || agreement.affected_scope()
                    != ProviderSessionAffectedScope::ProviderDefinedDescendants
            {
                return Err(failure(
                    "swallowtail.alibaba_model_studio.lifecycle_agreement_mismatch",
                    "Alibaba Model Studio deletion agreement did not match the retained route",
                ));
            }
            if request.cancellation().is_requested() || deadline_elapsed(&plan, &services)? {
                return Ok(ProviderSessionManagementOutcome::new(
                    agreement.binding().clone(),
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ));
            }
            let scope = ScopeId::new(format!(
                "alibaba-model-studio:management:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.alibaba_model_studio.scope_invalid",
                    "Alibaba Model Studio management scope was invalid",
                )
            })?;
            let conversation = ConversationRef::new(
                agreement
                    .binding()
                    .provider_session_ref()
                    .as_provider_value()
                    .to_owned(),
            )
            .map_err(protocol)?;
            let mut access =
                AccessLeases::acquire(plan.preflight(), scope.clone(), &services).await?;
            let cleanup_access = CleanupAccess::acquire(&access)?;
            let deletion = delete_retained_conversation(
                &self.transport,
                &scope,
                &services,
                &cleanup_access,
                &conversation,
            )
            .await;
            let release = access.release(&services).await;
            let release_diagnostic = cleanup_diagnostic(release);
            let (effect, operation_diagnostic) = match deletion {
                ManagedDeletion::Applied => (
                    ProviderSessionManagementEffect::applied(
                        action,
                        ProviderSessionAffectedScope::ProviderDefinedDescendants,
                    ),
                    None,
                ),
                ManagedDeletion::FailedBeforeEffect(error) => (
                    ProviderSessionManagementEffect::failed_before_effect(action),
                    Some(error.diagnostic().clone()),
                ),
                ManagedDeletion::UnconfirmedAfterEffect(error) => (
                    ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                    Some(error.diagnostic().clone()),
                ),
            };
            let diagnostic = release_diagnostic.or(operation_diagnostic);
            let outcome =
                ProviderSessionManagementOutcome::new(agreement.binding().clone(), effect);
            Ok(match diagnostic {
                Some(diagnostic) => outcome.with_diagnostic(diagnostic),
                None => outcome,
            })
        })
    }
}

fn deadline_elapsed(
    plan: &ProviderSessionManagementPlan,
    services: &HostServices,
) -> Result<bool, RuntimeFailure> {
    let Some(deadline) = plan.agreement().deadline() else {
        return Ok(false);
    };
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.alibaba_model_studio.time_service_missing",
            "Deadline-bound Alibaba Model Studio deletion requires time service",
        )
    })?;
    Ok(time.now() >= deadline.instant())
}

fn cleanup_diagnostic(outcome: CleanupOutcome) -> Option<swallowtail_core::SafeDiagnostic> {
    match outcome {
        CleanupOutcome::Failed(diagnostic) | CleanupOutcome::Degraded(diagnostic) => {
            Some(diagnostic)
        }
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => None,
    }
}
