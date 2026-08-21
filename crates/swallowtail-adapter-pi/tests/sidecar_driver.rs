mod support;

#[path = "sidecar_driver/catalogue.rs"]
mod catalogue;
#[path = "sidecar_driver/failures.rs"]
mod failures;
#[path = "sidecar_driver/lifecycle.rs"]
mod lifecycle;
#[path = "sidecar_driver/versions.rs"]
mod versions;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{
    CleanupEvent, SidecarFixtureHost, SidecarScenario, open_request, sidecar_selection,
    turn_request,
};
use swallowtail_adapter_pi::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
    PI_SDK_SIDECAR_WIRE_AXIS, PiSdkSidecarDriver, pi_sdk_sidecar_descriptor,
};
use swallowtail_core::{
    CredentialRef, DriverRole, ExecutionHostId, HarnessMessageClass, InterfaceVersion,
    InterfaceVersionAxis, InterfaceVersionBinding,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, EnvironmentRef, HarnessCommandAcknowledgement, HarnessCommandId,
    HarnessScheduledMessage, InteractiveSessionDriver, MonotonicInstant, OperationContent,
    RuntimeEventKind, TerminalStatus,
};

#[test]
fn descriptor_is_a_distinct_exact_pi_sdk_sidecar_driver() {
    let descriptor = pi_sdk_sidecar_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.pi.sdk-sidecar"
    );
    assert_eq!(descriptor.integration_family().as_str(), "pi");
    assert_eq!(
        descriptor.transport_family().as_str(),
        "swallowtail-pi-sdk-jsonl-v1"
    );
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    assert!(descriptor.supports_role(DriverRole::ModelCatalog));
    assert!(!descriptor.supports_role(DriverRole::StructuredRun));
    assert!(!descriptor.supports_role(DriverRole::Discovery));

    for (axis, qualified, rejected) in [
        (PI_SDK_SIDECAR_PACKAGE_AXIS, "0.84.2", "0.84.1"),
        (PI_SDK_SIDECAR_NODE_AXIS, "22.23.2", "22.23.3"),
        (
            PI_SDK_SIDECAR_WIRE_AXIS,
            "swallowtail-pi-sdk-jsonl-v1",
            "swallowtail-pi-sdk-jsonl-v2",
        ),
        (
            PI_SDK_SIDECAR_SIDECAR_AXIS,
            swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SOURCE_TAG,
            "swallowtail-pi-sdk-sidecar@0.0.0",
        ),
    ] {
        let axis = InterfaceVersionAxis::new(axis).expect("valid axis");
        assert!(
            descriptor.supports_interface_version(&InterfaceVersionBinding::new(
                axis.clone(),
                InterfaceVersion::new(qualified).expect("valid version"),
            ))
        );
        assert!(
            !descriptor.supports_interface_version(&InterfaceVersionBinding::new(
                axis,
                InterfaceVersion::new(rejected).expect("valid version"),
            ))
        );
    }
    // The RPC package axis is not part of this descriptor.
    assert!(
        !descriptor.supports_interface_version(&InterfaceVersionBinding::new(
            InterfaceVersionAxis::new("pi.package").expect("valid axis"),
            InterfaceVersion::new("0.84.2").expect("valid version"),
        ))
    );
}

#[test]
fn fresh_session_relays_scheduling_and_joined_cleanup() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.host");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Hold);
    let selected = sidecar_selection(host_id.clone());
    let services = fixture.services(host_id);
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("sidecar-session-success", selected.resource),
        services.clone(),
    ))
    .expect("sidecar session opens");

    assert_eq!(fixture.process_arguments(), Vec::<String>::new());
    assert_eq!(
        fixture.process_environment(),
        ["pi.fixture.environment".to_owned()]
    );

    let mut turn = block_on(session.start_turn(
        turn_request("sidecar-turn-success", deadline()),
        services.clone(),
    ))
    .expect("sidecar turn starts");
    let steering = block_on(
        turn.schedule_harness_message(scheduled("sidecar-steer-1", HarnessMessageClass::Steering)),
    )
    .expect("steering acknowledgement arrives");
    assert_eq!(
        steering.acknowledgement(),
        HarnessCommandAcknowledgement::Accepted
    );
    let duplicate = block_on(
        turn.schedule_harness_message(scheduled("sidecar-steer-2", HarnessMessageClass::Steering)),
    )
    .expect("duplicate steering is classified");
    assert_eq!(
        duplicate.acknowledgement(),
        HarnessCommandAcknowledgement::Rejected
    );
    let follow_up = block_on(turn.schedule_harness_message(scheduled(
        "sidecar-follow-up-1",
        HarnessMessageClass::FollowUp,
    )))
    .expect("follow-up acknowledgement arrives");
    assert_eq!(
        follow_up.acknowledgement(),
        HarnessCommandAcknowledgement::Accepted
    );

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
    let bootstrap = &inputs[0];
    assert_eq!(bootstrap["command"], "bootstrap");
    assert_eq!(bootstrap["params"]["cwd"], "/fixture/pi-sidecar-workspace");
    assert_eq!(bootstrap["params"]["provider"], "fixture-provider");
    assert_eq!(bootstrap["params"]["model"], "fixture-model");
    for key in ["agentDir", "sessionDir", "sdkModule"] {
        assert!(
            bootstrap["params"].get(key).is_none(),
            "application-provisioned {key} must not flow through the driver"
        );
    }
    for command in ["state", "prompt", "steer", "follow_up", "close"] {
        assert!(
            inputs.iter().any(|value| value["command"] == command),
            "missing {command} command"
        );
    }
    assert!(!format!("{inputs:?}").contains("pi.fixture.delegated-auth"));
}

fn driver(credential: CredentialRef) -> PiSdkSidecarDriver {
    PiSdkSidecarDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        credential,
    )
}

fn make_host_id(value: &str) -> ExecutionHostId {
    ExecutionHostId::new(value).expect("valid host")
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(100_000))
}

fn scheduled(id: &str, class: HarnessMessageClass) -> HarnessScheduledMessage {
    HarnessScheduledMessage::new(
        HarnessCommandId::new(id).expect("valid command id"),
        class,
        OperationContent::new("private scheduled message").expect("valid scheduled content"),
    )
}
