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
    CleanupEvent, FixtureHost, FixtureSelection, Scenario, open_request, selection, turn_request,
};
use swallowtail_adapter_pi::{PiRpcDriver, pi_rpc_descriptor};
use swallowtail_core::{
    CredentialRef, DriverRole, ExecutionHostId, HarnessMessageClass, InterfaceVersion,
    InterfaceVersionAxis, InterfaceVersionBinding,
};
use swallowtail_runtime::{
    CallbackPayload, CallbackRequestKind, CallbackResponse, CallbackResult, CleanupOutcome,
    Deadline, EnvironmentRef, HarnessCommandAcknowledgement, HarnessCommandId,
    HarnessScheduledMessage, InteractiveSessionDriver, MonotonicInstant, OperationContent,
    RuntimeEventKind, TerminalStatus,
};

#[test]
fn descriptor_is_a_distinct_exact_pi_rpc_driver() {
    let descriptor = pi_rpc_descriptor();
    assert_eq!(descriptor.identity().id().as_str(), "swallowtail.pi.rpc");
    assert_eq!(descriptor.integration_family().as_str(), "pi");
    assert_eq!(
        descriptor.transport_family().as_str(),
        "strict-lf-jsonl-stdio"
    );
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));

    let axis = InterfaceVersionAxis::new("pi.package").expect("valid axis");
    assert!(
        descriptor.supports_interface_version(&InterfaceVersionBinding::new(
            axis.clone(),
            InterfaceVersion::new("0.80.10").expect("valid version"),
        ))
    );
    assert!(
        !descriptor.supports_interface_version(&InterfaceVersionBinding::new(
            axis,
            InterfaceVersion::new("0.80.11").expect("valid version"),
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
    .expect("Pi session opens");

    assert_eq!(
        fixture.process_arguments(),
        [
            "--mode",
            "rpc",
            "--no-session",
            "--offline",
            "--provider",
            "fixture-provider",
            "--model",
            "fixture-model",
            "--tools",
            "read,grep,find,ls",
            "--no-extensions",
            "--no-skills",
            "--no-prompt-templates",
            "--no-themes",
            "--no-context-files",
        ]
    );
    assert_eq!(
        fixture.process_environment(),
        ["pi.fixture.environment".to_owned()]
    );

    let mut turn =
        block_on(session.start_turn(turn_request("turn-success", deadline()), services.clone()))
            .expect("Pi turn starts");
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
        CallbackRequestKind::HarnessUiDialog(_)
    ));
    block_on(callbacks.responder().respond(CallbackResponse::new(
        callback.callback_id().clone(),
        callback.turn_id().expect("callback turn").clone(),
        CallbackResult::Success(
            CallbackPayload::new(b"Allow".to_vec(), 64).expect("bounded callback payload"),
        ),
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
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(
        fixture.cleanup_events(),
        [
            CleanupEvent::ProcessWait,
            CleanupEvent::ResourceRelease,
            CleanupEvent::CredentialRelease,
        ]
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

fn setup(scenario: Scenario) -> (ExecutionHostId, FixtureHost, FixtureSelection, PiRpcDriver) {
    let host_id = ExecutionHostId::new("pi.fixture.host").expect("valid host");
    let fixture = FixtureHost::new(scenario);
    let selected = selection(host_id.clone());
    let driver = driver(selected.credential.clone());
    (host_id, fixture, selected, driver)
}

fn driver(credential: CredentialRef) -> PiRpcDriver {
    PiRpcDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        credential,
    )
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
