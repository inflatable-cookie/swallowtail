#[test]
fn malformed_phase_disclosure_and_assistant_claims_fail() {
    assert!(
        ActivityObservation::new(
            ActivityId::new("bad-status").unwrap(),
            run_id(),
            ActivityKind::Task,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .is_err()
    );
    assert!(
        ActivityObservation::new(
            ActivityId::new("bad-assistant").unwrap(),
            run_id(),
            ActivityKind::AssistantMessage,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
            ActivityDisclosure::ProviderDisplayContent,
        )
        .is_err()
    );
    assert!(
        ActivityObservation::new(
            ActivityId::new("unavailable").unwrap(),
            run_id(),
            ActivityKind::Task,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::Unavailable,
        )
        .is_err()
    );
    assert!(
        ActivityObservation::new(
            ActivityId::new("identity-only-label").unwrap(),
            run_id(),
            ActivityKind::Task,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .unwrap()
        .with_label(ActivityLabel::new("Not disclosed").unwrap())
        .is_err()
    );
}

#[test]
fn activity_events_are_always_semantic() {
    let observation = ActivityObservation::new(
        ActivityId::new("task-1").unwrap(),
        run_id(),
        ActivityKind::Task,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Completed,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .unwrap();

    assert_eq!(
        RuntimeEventKind::Activity(observation).delivery(),
        EventDelivery::Semantic
    );
}

#[test]
fn warning_activity_may_carry_portable_safe_failure_evidence() {
    let diagnostic = SafeDiagnostic::new("fixture.warning", "Harness reported a warning")
        .with_failure_classification(FailureClassification::new(
            FailureOrigin::Harness,
            FailureKind::Unknown,
            FailureRecovery::Unknown,
        ));
    let observation = ActivityObservation::new(
        ActivityId::new("warning-1").unwrap(),
        run_id(),
        ActivityKind::WarningOrError,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Failed,
        None,
        ActivityDisclosure::AdapterNormalizedSummary,
    )
    .unwrap()
    .with_diagnostic(diagnostic.clone())
    .expect("warning activity admits safe diagnostic");

    assert_eq!(observation.diagnostic(), Some(&diagnostic));
}

#[test]
fn ordinary_activity_cannot_be_relabelled_as_failure_evidence() {
    let observation = ActivityObservation::new(
        ActivityId::new("task-diagnostic").unwrap(),
        run_id(),
        ActivityKind::Task,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Completed,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .unwrap();

    assert!(
        observation
            .with_diagnostic(SafeDiagnostic::new("fixture.failure", "Failed"))
            .is_err()
    );
}
