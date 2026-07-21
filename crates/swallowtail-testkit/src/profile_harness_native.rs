use crate::{
    ConformanceAssertion, ConformanceReport, PreflightFixtureCase, RuntimePreflightFixture,
    SyntheticProfile,
};
use swallowtail_core::{HarnessIsolation, PreflightDimension, SafeDiagnostic};
use swallowtail_runtime::{
    CleanupOutcome, OperationPolicy, ProviderExecutionPolicy, ProviderRecoveryPolicy,
    ProviderRetentionPolicy, StreamReattachmentPolicy, TerminalOutcome, TerminalStatus,
    validate_harness_isolation_policy,
};

pub(crate) fn run() -> ConformanceReport {
    let mut report = ConformanceReport::new(SyntheticProfile::OneShotStructuredCli);
    assert_isolation_binding();
    report.record(ConformanceAssertion::AmbientHarnessAuthority);

    assert_retention_without_deletion();
    report.record(ConformanceAssertion::DurableRetentionExplicit);
    report.record(ConformanceAssertion::NoTranscriptDeletionClaim);

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

fn assert_retention_without_deletion() {
    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
        .with_harness_isolation(HarnessIsolation::AmbientHost);
    assert_eq!(
        policy.provider_execution(),
        ProviderExecutionPolicy::Attached
    );
    assert_eq!(
        policy.provider_retention(),
        ProviderRetentionPolicy::DurableAllowed
    );
    assert_eq!(
        policy.provider_recovery(),
        ProviderRecoveryPolicy::Prohibited
    );
    assert_eq!(
        policy.stream_reattachment(),
        StreamReattachmentPolicy::Disabled
    );

    let exited = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean);
    assert_eq!(exited.remote_resource_deletions().count(), 0);
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
