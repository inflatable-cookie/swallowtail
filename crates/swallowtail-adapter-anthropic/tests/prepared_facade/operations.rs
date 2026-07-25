use super::fixtures::PreparedFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_anthropic::AnthropicCatalogueProfileInput;
use swallowtail_core::{Capability, DriverRole, ExecutionHostId};
use swallowtail_runtime::{CleanupOutcome, RequestId, TerminalStatus};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn catalogue_and_one_attempt_remain_separate_on_both_host_topologies() {
    for host_id in [
        ExecutionHostId::new("anthropic.prepared.local").unwrap(),
        ExecutionHostId::new("anthropic.prepared.remote-authoritative").unwrap(),
    ] {
        let fixture = PreparedFixture::new(host_id);
        let prepared = fixture.prepared();
        let catalogue = prepared
            .prepare_catalogue(AnthropicCatalogueProfileInput::new(
                RequestId::new("prepared-catalogue").unwrap(),
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
        assert_eq!(models.len(), 3);
        assert_eq!(fixture.server.inference_attempts(), 0);
        assert_eq!(fixture.releases(), 1);

        let attempt = prepared
            .prepare_inference_attempt(fixture.attempt_input("prepared-attempt"))
            .expect("inference attempt prepares");
        assert_eq!(
            attempt.plan().requirements().driver_role(),
            DriverRole::StructuredRun
        );
        assert_eq!(
            attempt.plan().model_id().unwrap().as_str(),
            "claude-fixture-primary"
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
        let mut events = run.take_events().expect("event stream exists");
        let terminal = run.take_terminal_outcome().expect("terminal exists");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert_eq!(fixture.releases(), 2);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

fn has_capability(
    attempt: &swallowtail_adapter_anthropic::AnthropicPreparedInferenceAttempt,
    capability: Capability,
) -> bool {
    attempt
        .plan()
        .requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == capability)
}
