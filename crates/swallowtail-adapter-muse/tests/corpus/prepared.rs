use super::common::{SUCCESS, exact_model, host_id, model, prepare, run_input};
use super::support;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_muse::{MUSE_CODE_PAYLOAD_BASENAME, MUSE_LOCAL_META_ACCOUNT_AUDIENCE};
use swallowtail_core::{
    Capability, CapabilityConstraint, DriverRole, HarnessIsolation, ObservableActivityAvailability,
    ReasoningMode, ResourceAccess,
};
use swallowtail_runtime::{CleanupOutcome, TerminalStatus};
use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, assert_prepared_operation_evidence_matches_plan,
    run_one_shot_structured_cli_profile,
};

#[test]
fn prepared_facade_advertises_and_accepts_exactly_seven_efforts() {
    let prepared = prepare(host_id());
    let advertised = prepared
        .instance()
        .capabilities()
        .iter()
        .find(|(capability, _)| *capability == Capability::ReasoningSelection)
        .expect("reasoning is advertised")
        .1
        .iter()
        .filter_map(|constraint| match constraint {
            CapabilityConstraint::ReasoningMode(mode) => Some(mode.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        advertised,
        ["high", "low", "medium", "minimal", "none", "ultra", "xhigh"]
    );

    for effort in ["none", "minimal", "low", "medium", "high", "xhigh", "ultra"] {
        let run = prepared
            .prepare_run(run_input(exact_model(effort), effort))
            .expect("qualified effort prepares");
        assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());
        assert_eq!(
            run.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );
        assert_eq!(
            run.request()
                .policy()
                .reasoning_mode()
                .map(ReasoningMode::as_str),
            Some(effort)
        );
    }
}

#[test]
fn prepared_run_uses_local_account_and_exact_read_only_cli_binding() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "low"))
        .expect("run prepares");
    assert_eq!(
        prepared.access_profile().endpoint_audience().as_str(),
        MUSE_LOCAL_META_ACCOUNT_AUDIENCE
    );
    assert_eq!(
        run.plan().requirements().harness_isolation(),
        Some(HarnessIsolation::ProviderEnforced)
    );
    assert_eq!(
        run.plan()
            .requirements()
            .capabilities()
            .find(|requirement| requirement.capability() == Capability::WorkingResource)
            .expect("working resource")
            .constraints()
            .find_map(|constraint| match constraint {
                CapabilityConstraint::ResourceAccess(access) => Some(*access),
                _ => None,
            }),
        Some(ResourceAccess::Read)
    );
    let host = support::FixtureHost::scripted([SUCCESS]);
    let mut handle = block_on(run.start_run(host.services(host_id))).expect("run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>());
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let observed = host.observations();
    assert_eq!(observed.len(), 1);
    let process = &observed[0];
    assert!(process.executable.ends_with(MUSE_CODE_PAYLOAD_BASENAME));
    assert_eq!(process.environments, ["muse.fixture.environment"]);
    assert_eq!(
        process.working_resource.as_deref(),
        Some("muse.fixture.workspace")
    );
    for pair in [
        ["--provider", "meta"],
        ["--model", "muse-spark-1.2"],
        ["--reasoning-effort", "low"],
    ] {
        assert!(
            process
                .arguments
                .windows(2)
                .any(|arguments| arguments == pair)
        );
    }
    for exact in [
        "--disable-web-tools",
        "--no-foreign-personal-context",
        "--no-session-log",
        "--disable-write",
        "--disable-shell",
    ] {
        assert!(process.arguments.iter().any(|argument| argument == exact));
    }
    assert!(!process.arguments.iter().any(|argument| {
        argument.contains("token") || argument.contains("credential") || argument.contains("auth")
    }));
}

#[test]
fn descriptor_and_common_profile_keep_unsupported_surfaces_unavailable() {
    let descriptor = swallowtail_adapter_muse::muse_headless_descriptor();
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
    for role in [
        DriverRole::ModelCatalog,
        DriverRole::InteractiveSession,
        DriverRole::ProviderSessionCatalogue,
    ] {
        assert!(!descriptor.supports_role(role));
    }
    let prepared = prepare(host_id());
    for capability in [
        Capability::ModelCatalog,
        Capability::InteractiveSession,
        Capability::ToolCalls,
        Capability::ProviderManagedRecovery,
        Capability::ProviderSessionCatalogue,
        Capability::ProviderSessionReconciliation,
    ] {
        assert!(
            prepared
                .instance()
                .capabilities()
                .iter()
                .all(|(advertised, _)| advertised != capability),
            "unexpected {capability:?}"
        );
    }
    let report = run_one_shot_structured_cli_profile();
    assert_eq!(report.profile(), SyntheticProfile::OneShotStructuredCli);
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
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}
