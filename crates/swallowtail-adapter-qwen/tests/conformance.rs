#[path = "conformance/fixture.rs"]
mod fixture_support;
mod support;

use fixture_support::{
    assert_status_code, cancelled, completed, fixture, isolation_rejected, timed_out,
};
use swallowtail_runtime::{ProcessExit, ProviderObservation, RuntimeEventKind, TerminalStatus};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    run_one_shot_structured_cli_profile, run_structured_harness_native_boundary_assertions,
};

#[test]
fn provider_neutral_one_shot_profile_and_native_pack_cover_contract_023() {
    let one_shot = run_one_shot_structured_cli_profile();
    assert_eq!(one_shot.profile(), SyntheticProfile::OneShotStructuredCli);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::ProcessLifecycle,
    ] {
        assert!(one_shot.covers(assertion), "missing {assertion:?}");
    }

    let native = run_structured_harness_native_boundary_assertions();
    assert_eq!(native.profile(), SyntheticProfile::OneShotStructuredCli);
    for assertion in [
        ConformanceAssertion::AmbientHarnessAuthority,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::NativeBudgetIndependent,
        ConformanceAssertion::NoTranscriptDeletionClaim,
    ] {
        assert!(native.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn qwen_production_driver_preserves_native_and_host_truth_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let success = completed(
            &topology,
            &fixture("success.jsonl"),
            ProcessExit::new(true, Some(0)),
            "success",
        );
        assert_eq!(success.outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            success.outcome.output().map(|value| value.as_str()),
            Some("fixture answer")
        );
        assert!(success.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                if usage.input_tokens() == Some(12) && usage.output_tokens() == Some(2)
        )));
        assert!(success.outcome.remote_resource_deletions().next().is_none());
        assert_redacted(&success.events, &success.outcome);

        for (exit, code) in [
            (53, "swallowtail.qwen.headless.native_turn_limit"),
            (55, "swallowtail.qwen.headless.native_budget"),
        ] {
            let evidence = completed(&topology, "", ProcessExit::new(false, Some(exit)), code);
            assert_status_code(&evidence.outcome, code, true);
        }

        let provider = completed(
            &topology,
            &fixture("provider-failure.jsonl"),
            ProcessExit::new(true, Some(0)),
            "provider-failure",
        );
        assert_status_code(
            &provider.outcome,
            "swallowtail.qwen.headless.provider_failed",
            true,
        );
        assert_redacted(&provider.events, &provider.outcome);

        let malformed = completed(
            &topology,
            &fixture("malformed.jsonl"),
            ProcessExit::new(true, Some(0)),
            "malformed",
        );
        assert_status_code(
            &malformed.outcome,
            "swallowtail.qwen.headless.malformed_stream",
            false,
        );
        assert_redacted(&malformed.events, &malformed.outcome);

        let disconnected = completed(&topology, "", ProcessExit::new(true, Some(0)), "disconnect");
        assert_status_code(
            &disconnected.outcome,
            "swallowtail.qwen.headless.incomplete_stream",
            false,
        );

        assert_eq!(cancelled(&topology).status(), &TerminalStatus::Cancelled);
        assert_eq!(timed_out(&topology).status(), &TerminalStatus::TimedOut);
        isolation_rejected(&topology);
    }
}

fn assert_redacted(
    events: &[swallowtail_runtime::RuntimeEvent],
    outcome: &swallowtail_runtime::TerminalOutcome,
) {
    let public = format!("{events:?}{outcome:?}");
    for private in [
        "fixture-private-prompt",
        "fixture-private-workspace",
        "fixture-provider-secret-never-diagnose",
        "fixture answer",
    ] {
        assert!(!public.contains(private));
    }
}
