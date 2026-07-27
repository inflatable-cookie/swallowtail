use crate::codex_app_server_lifecycle_claim;
use crate::selection::{CodexLifecycleBehavior, classify_lifecycle_version};
use serde_json::Value;
use swallowtail_core::{
    ProviderSessionManagementAction, ProviderSessionManagementEffect, SafeDiagnostic,
};
use swallowtail_runtime::{
    CleanupOutcome, ProviderSessionManagementAgreement, ProviderSessionManagementOutcome,
    RuntimeFailure,
};

pub(super) struct LifecycleAssessment {
    pub(super) behavior: CodexLifecycleBehavior,
}

pub(super) fn lifecycle_assessment(
    plan: &swallowtail_core::PreflightPlan,
) -> Result<LifecycleAssessment, RuntimeFailure> {
    let claim = codex_app_server_lifecycle_claim();
    let binding = plan
        .interface_versions()
        .find(|binding| binding.axis() == claim.axis())
        .ok_or_else(|| {
            failure(
                "swallowtail.codex.lifecycle.version_missing",
                "Codex management plan is missing its exact executable version",
            )
        })?;
    let classified = classify_lifecycle_version(binding.version()).ok_or_else(|| {
        failure(
            "swallowtail.codex.lifecycle.version_incompatible",
            "Codex executable version does not support thread lifecycle management",
        )
    })?;
    Ok(LifecycleAssessment {
        behavior: classified.behavior,
    })
}

pub(super) fn lifecycle_request(
    action: ProviderSessionManagementAction,
    target: &str,
) -> (&'static str, Value) {
    let method = match action {
        ProviderSessionManagementAction::Archive => "thread/archive",
        ProviderSessionManagementAction::Restore => "thread/unarchive",
        ProviderSessionManagementAction::Delete(_) => "thread/delete",
    };
    (method, serde_json::json!({ "threadId": target }))
}

pub(super) fn notification_method(action: ProviderSessionManagementAction) -> &'static str {
    match action {
        ProviderSessionManagementAction::Archive => "thread/archived",
        ProviderSessionManagementAction::Restore => "thread/unarchived",
        ProviderSessionManagementAction::Delete(_) => "thread/deleted",
    }
}

pub(super) fn validate_lifecycle_response(
    action: ProviderSessionManagementAction,
    target: &str,
    response: &Value,
) -> Result<(), RuntimeFailure> {
    let valid = match action {
        ProviderSessionManagementAction::Archive | ProviderSessionManagementAction::Delete(_) => {
            response.as_object().is_some_and(serde_json::Map::is_empty)
        }
        ProviderSessionManagementAction::Restore => {
            response
                .get("thread")
                .and_then(|thread| thread.get("id"))
                .and_then(Value::as_str)
                == Some(target)
        }
    };
    if valid {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.codex.lifecycle.malformed_response",
            "Codex app-server returned a malformed thread lifecycle response",
        ))
    }
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

pub(super) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
