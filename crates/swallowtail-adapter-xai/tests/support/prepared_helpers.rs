use super::{DriverFixture, turn_request};
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use std::num::NonZeroU64;
use swallowtail_adapter_xai::XaiModelSelection;
use swallowtail_core::{
    Capability, CapabilityConstraint, ModelId, ModelRouteId, ModelRouteRevision,
};
use swallowtail_runtime::{InteractiveSessionHandle, RuntimeEvent, TerminalStatus};

pub(crate) fn model() -> XaiModelSelection {
    qualified_model("grok-fixture-exact")
}

pub(crate) fn qualified_model(model_id: &str) -> XaiModelSelection {
    XaiModelSelection::new(
        ModelRouteId::new("xai-grok-fixture").expect("route id is valid"),
        ModelRouteRevision::new("prepared-1").expect("revision is valid"),
        ModelId::new(model_id).expect("model id is valid"),
    )
}

pub(crate) fn assert_generation_requirement(
    plan: &swallowtail_core::PreflightPlan,
    capability: Capability,
    expected: Option<CapabilityConstraint>,
) {
    let requirement = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == capability);
    match expected {
        Some(expected) => assert_eq!(
            requirement
                .expect("generation capability exists")
                .constraints()
                .collect::<Vec<_>>(),
            vec![&expected]
        ),
        None => assert!(requirement.is_none()),
    }
}

pub(crate) fn assert_wire_controls(
    frame: &str,
    reasoning: Option<&str>,
    maximum: Option<u64>,
    chained: bool,
) {
    let value: Value = serde_json::from_str(frame).expect("wire frame parses");
    match reasoning {
        Some(reasoning) => assert_eq!(value["reasoning"]["effort"], reasoning),
        None => assert!(value.get("reasoning").is_none()),
    }
    match maximum {
        Some(maximum) => assert_eq!(value["max_output_tokens"], maximum),
        None => assert!(value.get("max_output_tokens").is_none()),
    }
    if chained {
        assert_eq!(value["previous_response_id"], "resp_fixture_first");
    } else {
        assert!(value.get("previous_response_id").is_none());
    }
}

pub(crate) fn complete_turn(
    session: &mut Box<dyn InteractiveSessionHandle>,
    fixture: &DriverFixture,
    turn: &str,
) -> Vec<RuntimeEvent> {
    let mut handle =
        block_on(session.start_turn(turn_request(turn), fixture.services())).expect("turn starts");
    let mut stream = handle.take_events().expect("events exist");
    let terminal = handle.take_terminal_outcome().expect("terminal exists");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event succeeds"));
        }
        (events, terminal.await)
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        block_on(handle.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    events
}

pub(crate) fn assert_output_edge(
    operation: &swallowtail_adapter_xai::XaiPreparedResponsesRun,
    maximum: NonZeroU64,
) {
    assert_eq!(
        operation
            .evidence()
            .maximum_output_tokens()
            .map(NonZeroU64::get),
        Some(maximum.get())
    );
    assert_generation_requirement(
        operation.plan(),
        Capability::OutputTokenLimit,
        Some(CapabilityConstraint::OutputTokenMaximum(maximum.get())),
    );
}
