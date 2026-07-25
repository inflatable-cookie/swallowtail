#[allow(dead_code)]
#[path = "direct_driver/fixture.rs"]
mod fixture;
#[allow(dead_code)]
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_kimi_platform::{
    KIMI_PLATFORM_ENDPOINT_AUDIENCE, KIMI_PLATFORM_MODEL_ID, KimiPlatformCatalogueProfileInput,
    KimiPlatformInferenceAttemptInput, KimiPlatformModelSelection, prepare_kimi_platform_direct,
};
use swallowtail_core::{
    Capability, DriverRole, EntitlementMetering, ModelId, ModelRouteId, ModelRouteRevision,
    ReasoningMode,
};
use swallowtail_runtime::{CleanupOutcome, OperationContent, RequestId, TerminalStatus};
use swallowtail_testkit::{
    ExecutionTopologyFixture, assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn catalogue_and_exact_k3_attempt_remain_separate_on_both_host_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = Fixture::for_topology(&topology);
        let prepared =
            prepare_kimi_platform_direct(fixture.preparation_input(), &fixture.services())
                .expect("Kimi Platform integration prepares");
        assert_eq!(
            prepared.access_profile().endpoint_audience().as_str(),
            KIMI_PLATFORM_ENDPOINT_AUDIENCE
        );

        let catalogue = prepared
            .prepare_catalogue(KimiPlatformCatalogueProfileInput::new(
                RequestId::new("prepared-catalogue").expect("request id"),
            ))
            .expect("catalogue prepares");
        assert_eq!(
            catalogue.plan().requirements().driver_role(),
            DriverRole::ModelCatalog
        );
        assert!(catalogue.plan().model_route_id().is_none());
        assert_prepared_operation_evidence_matches_plan(
            catalogue.evidence().operation(),
            catalogue.plan(),
        );
        let models =
            block_on(catalogue.list_models(fixture.services())).expect("catalogue succeeds");
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id().as_str(), KIMI_PLATFORM_MODEL_ID);
        assert_eq!(fixture.server.attempts(), 0);

        let attempt = prepared
            .prepare_inference_attempt(KimiPlatformInferenceAttemptInput::new(
                RequestId::new("prepared-attempt").expect("request id"),
                KimiPlatformModelSelection::new(
                    ModelRouteId::new("kimi-platform.prepared.k3").expect("route id"),
                    ModelRouteRevision::new("2026-07-21").expect("route revision"),
                    ModelId::new(KIMI_PLATFORM_MODEL_ID).expect("model id"),
                ),
                OperationContent::new("prepared fixture prompt").expect("content"),
                ReasoningMode::new("high").expect("reasoning"),
                NonZeroU64::new(128).expect("output bound"),
            ))
            .expect("attempt prepares");
        assert_eq!(
            attempt.plan().requirements().driver_role(),
            DriverRole::StructuredRun
        );
        assert_eq!(
            attempt.plan().model_id().expect("model").as_str(),
            KIMI_PLATFORM_MODEL_ID
        );
        assert!(!has_capability(&attempt, Capability::ToolCalls));
        assert!(!has_capability(
            &attempt,
            Capability::DirectToolContinuation
        ));
        assert_eq!(attempt.request().tools().len(), 0);
        assert_prepared_operation_evidence_matches_plan(
            attempt.evidence().operation(),
            attempt.plan(),
        );

        let mut run =
            block_on(attempt.start_run(fixture.services())).expect("prepared attempt starts");
        let mut events = run.take_events().expect("events");
        let terminal = run.take_terminal_outcome().expect("terminal");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(fixture.server.attempts(), 1);
        assert_eq!(fixture.releases(), 2);
        assert_eq!(fixture.release_after_blocking(), [1, 2]);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn membership_access_and_alternate_model_reject_before_provider_effects() {
    let fixture = Fixture::new();
    let failure = prepare_kimi_platform_direct(
        fixture.preparation_input_with_metering(EntitlementMetering::SubscriptionAllowance),
        &fixture.services(),
    )
    .expect_err("Kimi membership metering is not a Platform credential");
    assert_eq!(
        failure.stage(),
        swallowtail_runtime::PreparationStage::AccessEvidence
    );

    let prepared = prepare_kimi_platform_direct(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let failure = prepared
        .prepare_inference_attempt(KimiPlatformInferenceAttemptInput::new(
            RequestId::new("wrong-model").expect("request id"),
            KimiPlatformModelSelection::new(
                ModelRouteId::new("kimi-platform.other").expect("route id"),
                ModelRouteRevision::new("1").expect("route revision"),
                ModelId::new("compatible-looking-model").expect("model id"),
            ),
            OperationContent::new("must not run").expect("content"),
            ReasoningMode::new("high").expect("reasoning"),
            NonZeroU64::new(128).expect("bound"),
        ))
        .expect_err("model substitution rejects");
    assert_eq!(
        failure.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

fn has_capability(
    attempt: &swallowtail_adapter_kimi_platform::KimiPlatformPreparedInferenceAttempt,
    capability: Capability,
) -> bool {
    attempt
        .plan()
        .requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == capability)
}
