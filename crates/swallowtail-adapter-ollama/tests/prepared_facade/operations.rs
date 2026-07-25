use super::fixtures::{attempt_input, inventory_input, prepared};
use crate::support::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_core::{
    AttachedRuntimeResidency, ExecutionHostId, HostServiceKind, InstanceOwnership,
};
use swallowtail_runtime::{CleanupOutcome, TerminalStatus};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_inventory_and_inference_preserve_external_runtime_truth() {
    for host in [
        "ollama.prepared.local",
        "ollama.prepared.remote-authoritative",
    ] {
        let fixture = Fixture::with_host(host);
        let prepared = prepared(&fixture);
        assert_eq!(
            prepared.instance().ownership(),
            InstanceOwnership::ExternalAttached
        );
        assert!(matches!(
            prepared.runtime().compatibility(),
            swallowtail_core::InterfaceCompatibilityAssessment::Qualified(_)
        ));
        assert_eq!(prepared.runtime().installed().len(), 1);
        assert_eq!(prepared.runtime().running().len(), 1);
        assert_eq!(
            prepared.runtime().selected_detail().model_tag().as_str(),
            "fixture-model:8b"
        );
        for absent in [
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::ModelArtifact,
            HostServiceKind::ServingEndpoint,
        ] {
            assert!(
                !prepared
                    .available_host_services()
                    .any(|kind| kind == absent)
            );
        }

        let inventory = prepared
            .prepare_inventory(inventory_input("prepared-inventory"))
            .expect("inventory prepares");
        assert_prepared_operation_evidence_matches_plan(
            inventory.evidence().operation(),
            inventory.plan(),
        );
        assert_eq!(
            inventory.plan().model_id().unwrap().as_str(),
            "ollama.fixture.model"
        );
        let snapshot =
            block_on(inventory.observe_inventory(fixture.services())).expect("inventory observes");
        assert_eq!(snapshot.entries().len(), 1);
        assert_eq!(snapshot.installed().count(), 1);
        assert_eq!(snapshot.running().count(), 1);
        assert!(snapshot.selected_detail().is_some());
        assert_eq!(fixture.server.inference_attempts(), 0);

        let attempt = prepared
            .prepare_inference_attempt(attempt_input("prepared-inference"))
            .expect("inference prepares");
        assert_prepared_operation_evidence_matches_plan(
            attempt.evidence().operation(),
            attempt.plan(),
        );
        assert_eq!(
            attempt.request().policy().attached_runtime_residency(),
            Some(AttachedRuntimeResidency::RuntimeManaged)
        );
        let mut run =
            block_on(attempt.start_run(fixture.services())).expect("prepared inference starts");
        let mut events = run.take_events().expect("events exist");
        let terminal = run.take_terminal_outcome().expect("terminal exists");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert!(fixture.server.is_reachable());
        assert_eq!(
            attempt.plan().execution_host_id(),
            &ExecutionHostId::new(host).unwrap()
        );
    }
}
