use crate::failure::unsupported;
use swallowtail_core::{
    ProviderSessionAffectedScope, ProviderSessionManagementAction, ProviderSessionManagementEffect,
    SafeDiagnostic,
};
use swallowtail_runtime::{
    CleanupOutcome, ProviderSessionManagementAgreement, ProviderSessionManagementOutcome,
    RuntimeFailure,
};

use crate::local_server::protocol::{
    RestFailureKind, RestReply, decode_archive, decode_rest, decode_session,
};
use crate::local_server::transport::{Response, session_path};

pub(super) fn action_path(
    provider_session_id: &str,
    action: ProviderSessionManagementAction,
) -> Result<String, RuntimeFailure> {
    let base = session_path(provider_session_id)?;
    let suffix = match action {
        ProviderSessionManagementAction::Archive => "archive",
        ProviderSessionManagementAction::Restore => "restore",
        ProviderSessionManagementAction::Delete(_) => {
            return Err(unsupported("Kimi local-server session deletion"));
        }
    };
    Ok(format!("{base}:{suffix}"))
}

pub(super) fn classify_response(
    agreement: &ProviderSessionManagementAgreement,
    response: Result<Response, RuntimeFailure>,
    expected_session_id: &str,
) -> ProviderSessionManagementOutcome {
    let action = agreement.action();
    let Ok(response) = response else {
        return unconfirmed(
            agreement,
            "Kimi local-server transport did not confirm the lifecycle effect",
        );
    };
    match decode_rest(response.status, &response.body) {
        Ok(RestReply::Failure(RestFailureKind::Server)) | Err(_) => unconfirmed(
            agreement,
            "Kimi local-server response did not confirm the lifecycle effect",
        ),
        Ok(RestReply::Failure(_)) => outcome(
            agreement,
            ProviderSessionManagementEffect::failed_before_effect(action),
        )
        .with_diagnostic(SafeDiagnostic::new(
            "swallowtail.kimi.local_server.lifecycle_rejected",
            "Kimi local server rejected the lifecycle request",
        )),
        Ok(RestReply::Success(_)) => {
            let confirmed = match action {
                ProviderSessionManagementAction::Archive => decode_archive(&response.body).is_ok(),
                ProviderSessionManagementAction::Restore => decode_session(&response.body)
                    .is_ok_and(|session| !session.archived && session.id == expected_session_id),
                ProviderSessionManagementAction::Delete(_) => false,
            };
            if confirmed {
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::applied(
                        action,
                        ProviderSessionAffectedScope::TargetOnly,
                    ),
                )
            } else {
                unconfirmed(
                    agreement,
                    "Kimi local-server response did not confirm the lifecycle effect",
                )
            }
        }
    }
}

fn unconfirmed(
    agreement: &ProviderSessionManagementAgreement,
    message: &'static str,
) -> ProviderSessionManagementOutcome {
    outcome(
        agreement,
        ProviderSessionManagementEffect::unconfirmed_after_effect(agreement.action()),
    )
    .with_diagnostic(SafeDiagnostic::new(
        "swallowtail.kimi.local_server.lifecycle_unconfirmed",
        message,
    ))
}

pub(super) fn outcome(
    agreement: &ProviderSessionManagementAgreement,
    effect: ProviderSessionManagementEffect,
) -> ProviderSessionManagementOutcome {
    ProviderSessionManagementOutcome::new(agreement.binding().clone(), effect)
}

pub(super) fn with_cleanup(
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
