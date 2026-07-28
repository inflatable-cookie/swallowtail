use crate::{
    ConformanceAssertion, ConformanceReport, PreflightFixtureCase, RuntimePreflightFixture,
    SyntheticProfile,
};
use swallowtail_core::{
    HarnessIsolation, OwnedRemoteResourceKind, PreflightDimension, SafeDiagnostic,
};
use swallowtail_runtime::{
    CleanupOutcome, OperationPolicy, ProviderExecutionPolicy, ProviderRecoveryPolicy,
    ProviderRetentionPolicy, RemoteResourceDeletionOutcome, StreamReattachmentPolicy,
    TerminalOutcome, TerminalStatus, validate_harness_isolation_policy,
};

pub(crate) fn run() -> ConformanceReport {
    let mut report = ConformanceReport::new(SyntheticProfile::OneShotStructuredCli);
    assert_isolation_binding();
    report.record(ConformanceAssertion::AmbientHarnessAuthority);

    assert_retention_and_deletion_truth();
    report.record(ConformanceAssertion::DurableRetentionExplicit);
    report.record(ConformanceAssertion::NoTranscriptDeletionClaim);
    report.record(ConformanceAssertion::OwnedRemoteDeletionTruth);

    assert_native_and_host_causes_are_distinct();
    report.record(ConformanceAssertion::NativeBudgetIndependent);
    report
}

fn assert_isolation_binding() {
    let fixture = RuntimePreflightFixture::for_case(PreflightFixtureCase::HarnessIsolationAmbient);
    let plan = fixture
        .preflight()
        .expect("ambient structured-harness preflight succeeds");
    let matching = OperationPolicy::offline().with_harness_isolation(HarnessIsolation::AmbientHost);
    validate_harness_isolation_policy(&plan, &matching)
        .expect("ambient runtime policy matches its preflight binding");

    for enforced in [
        HarnessIsolation::ProviderEnforced,
        HarnessIsolation::HostEnforced,
    ] {
        let mismatched = OperationPolicy::offline().with_harness_isolation(enforced);
        validate_harness_isolation_policy(&plan, &mismatched)
            .expect_err("enforced isolation cannot substitute for ambient authority");
    }
    assert_eq!(fixture.provider_side_effect_count(), 0);

    let direct =
        RuntimePreflightFixture::for_case(PreflightFixtureCase::DirectInferenceHarnessIsolation);
    let failure = direct
        .preflight()
        .expect_err("direct inference cannot claim harness isolation");
    assert_eq!(failure.dimension(), PreflightDimension::HarnessIsolation);
    assert_eq!(direct.provider_side_effect_count(), 0);
}

fn assert_retention_and_deletion_truth() {
    let durable = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
        .with_harness_isolation(HarnessIsolation::AmbientHost);
    assert_eq!(
        durable.provider_execution(),
        ProviderExecutionPolicy::Attached
    );
    assert_eq!(
        durable.provider_retention(),
        ProviderRetentionPolicy::DurableAllowed
    );
    assert_eq!(
        durable.provider_recovery(),
        ProviderRecoveryPolicy::Prohibited
    );
    assert_eq!(
        durable.stream_reattachment(),
        StreamReattachmentPolicy::Disabled
    );

    let preserved = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean);
    assert_eq!(preserved.remote_resource_deletions().count(), 0);

    let prohibited = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(HarnessIsolation::AmbientHost);
    assert_eq!(
        prohibited.provider_retention(),
        ProviderRetentionPolicy::Prohibited
    );
    let ephemeral = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean);
    assert_eq!(ephemeral.remote_resource_deletions().count(), 0);

    let temporary = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::TemporaryAllowed)
        .with_harness_isolation(HarnessIsolation::AmbientHost);
    assert_eq!(
        temporary.provider_retention(),
        ProviderRetentionPolicy::TemporaryAllowed
    );
    let deleted = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean)
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::Session,
            RemoteResourceDeletionOutcome::Confirmed,
        );
    assert_eq!(
        deleted.remote_resource_deletion(OwnedRemoteResourceKind::Session),
        Some(RemoteResourceDeletionOutcome::Confirmed)
    );
}

fn assert_native_and_host_causes_are_distinct() {
    let native = TerminalStatus::ProviderFailed(SafeDiagnostic::new(
        "fixture.harness.native_budget",
        "Harness native budget reached",
    ));
    let process = TerminalStatus::ProviderFailed(SafeDiagnostic::new(
        "fixture.harness.process_failed",
        "Harness process failed",
    ));
    assert_ne!(native, process);
    assert_ne!(native, TerminalStatus::Cancelled);
    assert_ne!(native, TerminalStatus::TimedOut);
}
