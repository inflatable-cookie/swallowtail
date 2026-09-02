#[path = "driver/conformance.rs"]
mod conformance;
#[path = "driver/failures.rs"]
mod failures;
#[path = "driver/lifecycle.rs"]
mod lifecycle;
#[path = "driver/profile.rs"]
mod profile;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{
    CleanupEvent, FixtureHost, FixtureSelection, Scenario, allow_user_input_result, close_session,
    open_request, selection, turn_request,
};
use swallowtail_adapter_oh_my_pi::{OhMyPiRpcDriver, oh_my_pi_rpc_descriptor};
use swallowtail_core::{
    DriverRole, ExecutionHostId, HarnessMessageClass, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding,
};
use swallowtail_runtime::{
    CallbackRequestKind, CallbackResponse, CleanupOutcome, Deadline, EnvironmentRef,
    HarnessCommandAcknowledgement, HarnessCommandId, HarnessScheduledMessage,
    InteractiveSessionDriver, MonotonicInstant, OperationContent, RuntimeEventKind, TerminalStatus,
};

#[test]
fn descriptor_is_a_distinct_exact_oh_my_pi_rpc_driver() {
    let descriptor = oh_my_pi_rpc_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.oh-my-pi.rpc"
    );
    assert_eq!(descriptor.integration_family().as_str(), "oh-my-pi");
    assert_eq!(
        descriptor.transport_family().as_str(),
        "oh-my-pi-rpc-v2-jsonl-stdio"
    );
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
    assert!(descriptor.supports_role(DriverRole::ModelCatalog));

    let axis = InterfaceVersionAxis::new("oh-my-pi.package").expect("valid axis");
    assert!(
        descriptor.supports_interface_version(&InterfaceVersionBinding::new(
            axis.clone(),
            InterfaceVersion::new("17.2.9").expect("valid version"),
        ))
    );
    assert!(
        !descriptor.supports_interface_version(&InterfaceVersionBinding::new(
            axis,
            InterfaceVersion::new("17.4.1").expect("valid version"),
        ))
    );
}

#[test]
fn restrictive_session_relays_scheduling_ui_and_joined_cleanup() {
    let (host_id, fixture, selected, driver) = setup(Scenario::Hold);
    let services = fixture.services(host_id);
    let mut session = block_on(driver.open_session(
        selected.plan,
        open_request("session-success", selected.resource),
        services.clone(),
    ))
    .expect("OhMyPi session opens");

    assert_eq!(
        fixture.process_arguments(),
        [
            "--mode",
            "rpc",
            "--no-session",
            "--provider",
            "fixture-provider",
            "--model",
            "fixture-model",
            "--tools",
            "read,grep,glob,todo,ask",
            "--no-extensions",
            "--no-skills",
            "--no-rules",
            "--no-prewalk",
            "--approval-mode",
            "always-ask",
        ]
    );
    assert_eq!(
        fixture.process_environment(),
        ["pi.fixture.environment".to_owned()]
    );

    let mut turn =
        block_on(session.start_turn(turn_request("turn-success", deadline()), services.clone()))
            .expect("OhMyPi turn starts");
    let steering = block_on(turn.schedule_harness_message(scheduled(
        "steer-1",
        HarnessMessageClass::Steering,
        "private steering",
    )))
    .expect("steering acknowledgement arrives");
    assert_eq!(
        steering.acknowledgement(),
        HarnessCommandAcknowledgement::Accepted
    );
    let duplicate = block_on(turn.schedule_harness_message(scheduled(
        "steer-2",
        HarnessMessageClass::Steering,
        "duplicate steering",
    )))
    .expect("duplicate steering is classified");
    assert_eq!(
        duplicate.acknowledgement(),
        HarnessCommandAcknowledgement::Rejected
    );
    let follow_up = block_on(turn.schedule_harness_message(scheduled(
        "follow-up-1",
        HarnessMessageClass::FollowUp,
        "private follow up",
    )))
    .expect("follow-up acknowledgement arrives");
    assert_eq!(
        follow_up.acknowledgement(),
        HarnessCommandAcknowledgement::Accepted
    );

    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut callback_requests = callbacks.take_requests().expect("callback stream exists");
    let callback = block_on(callback_requests.next())
        .expect("UI callback arrives")
        .expect("UI callback is valid");
    assert!(matches!(
        callback.kind(),
        CallbackRequestKind::HarnessUserInput(_)
    ));
    block_on(callbacks.responder().respond(CallbackResponse::new(
        callback.callback_id().clone(),
        callback.turn_id().expect("callback turn").clone(),
        allow_user_input_result(&callback),
    )))
    .expect("UI callback response is relayed");

    let events = block_on(
        turn.take_events()
            .expect("event stream exists")
            .collect::<Vec<_>>(),
    );
    assert!(events.iter().all(Result::is_ok));
    assert!(events.iter().any(|event| {
        event
            .as_ref()
            .is_ok_and(|event| event.kind() == &RuntimeEventKind::OutputDelta)
    }));
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(OperationContent::as_str),
        Some("fixture answer")
    );
    assert!(!format!("{terminal:?}").contains("fixture answer"));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert_eq!(
        fixture.cleanup_events(),
        [CleanupEvent::ProcessWait, CleanupEvent::ResourceRelease,]
    );

    let inputs = fixture.inputs();
    assert!(inputs.iter().any(|value| value["type"] == "prompt"));
    assert!(inputs.iter().any(|value| value["type"] == "steer"));
    assert!(inputs.iter().any(|value| value["type"] == "follow_up"));
    assert!(
        inputs
            .iter()
            .any(|value| value["type"] == "extension_ui_response")
    );
    assert!(!format!("{inputs:?}").contains("credential"));
}

fn setup(
    scenario: Scenario,
) -> (
    ExecutionHostId,
    FixtureHost,
    FixtureSelection,
    OhMyPiRpcDriver,
) {
    let host_id = ExecutionHostId::new("pi.fixture.host").expect("valid host");
    let fixture = FixtureHost::new(scenario);
    let selected = selection(host_id.clone());
    let driver = driver();
    (host_id, fixture, selected, driver)
}

fn driver() -> OhMyPiRpcDriver {
    OhMyPiRpcDriver::new(EnvironmentRef::new("pi.fixture.environment").expect("valid environment"))
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}

fn scheduled(id: &str, class: HarnessMessageClass, content: &str) -> HarnessScheduledMessage {
    HarnessScheduledMessage::new(
        HarnessCommandId::new(id).expect("valid command id"),
        class,
        OperationContent::new(content).expect("valid scheduled content"),
    )
}
