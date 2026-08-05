use swallowtail_core::{
    ActivityDisclosure, FailureClassification, FailureKind, FailureOrigin, FailureRecovery,
    SafeDiagnostic,
};
use swallowtail_runtime::{
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, CleanupOutcome, RuntimeRunId, TerminalFailureSource, TerminalOutcome,
    TerminalStatus,
};

/// Runs provider-free assertions for Contract 051.
pub fn assert_portable_failure_classification_contract() {
    let unknown = SafeDiagnostic::new("fixture.unknown", "Fixture failed");
    assert!(unknown.failure_classification().is_unknown());

    let provider = classified(
        "fixture.provider.rate_limited",
        FailureOrigin::Provider,
        FailureKind::RateLimited,
        FailureRecovery::RetryMaySucceed,
    );
    let harness = classified(
        "fixture.harness.rate_limited",
        FailureOrigin::Harness,
        FailureKind::RateLimited,
        FailureRecovery::RetryMaySucceed,
    );
    assert_eq!(
        provider.failure_classification().kind(),
        harness.failure_classification().kind()
    );
    assert_ne!(provider.code(), harness.code());

    let cleanup = SafeDiagnostic::new("fixture.cleanup", "Cleanup failed");
    let outcome = TerminalOutcome::new(
        TerminalStatus::ProviderFailed(provider.clone()),
        CleanupOutcome::Failed(cleanup.clone()),
    );
    let failure = outcome.failure().expect("provider failure is visible");
    assert_eq!(failure.source(), TerminalFailureSource::Provider);
    assert_eq!(failure.diagnostic(), &provider);
    assert_eq!(outcome.cleanup().diagnostic(), Some(&cleanup));

    let activity = ActivityObservation::new(
        ActivityId::new("fixture-warning").expect("activity id is valid"),
        ActivityOperationId::Run(RuntimeRunId::new("fixture-run").expect("run id is valid")),
        ActivityKind::WarningOrError,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Failed,
        None,
        ActivityDisclosure::AdapterNormalizedSummary,
    )
    .expect("warning activity is valid")
    .with_diagnostic(harness.clone())
    .expect("warning activity admits failure evidence");
    assert_eq!(activity.diagnostic(), Some(&harness));

    let task = ActivityObservation::new(
        ActivityId::new("fixture-task").expect("activity id is valid"),
        ActivityOperationId::Run(RuntimeRunId::new("fixture-run").expect("run id is valid")),
        ActivityKind::Task,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Completed,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .expect("task activity is valid");
    assert!(task.with_diagnostic(harness).is_err());
}

fn classified(
    code: &'static str,
    origin: FailureOrigin,
    kind: FailureKind,
    recovery: FailureRecovery,
) -> SafeDiagnostic {
    SafeDiagnostic::new(code, "Fixture failed")
        .with_failure_classification(FailureClassification::new(origin, kind, recovery))
}

#[cfg(test)]
mod tests {
    #[test]
    fn portable_failure_contract_passes() {
        super::assert_portable_failure_classification_contract();
    }
}
