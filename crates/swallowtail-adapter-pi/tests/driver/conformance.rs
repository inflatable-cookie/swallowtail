use crate::support::{
    CleanupEvent, FixtureHost, Scenario, open_request, selection_for_topology, turn_request,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_pi::PiRpcDriver;
use swallowtail_core::{HarnessIsolation, HarnessMessageClass};
use swallowtail_runtime::{
    CallbackPayload, CallbackRequestKind, CallbackResponse, CallbackResult,
    CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef,
    HarnessCommandAcknowledgement, HarnessCommandId, HarnessScheduledMessage,
    InteractiveSessionDriver, MonotonicInstant, OperationContent, RuntimeEventKind, TerminalStatus,
};
use swallowtail_testkit::ExecutionTopologyFixture;

#[test]
fn production_scheduling_and_ui_preserve_both_host_topologies() {
    for (index, topology) in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = FixtureHost::new(Scenario::Hold);
        let selected = selection_for_topology(&topology);
        assert_eq!(
            selected.plan.execution_host_id(),
            topology.execution_host_id()
        );
        assert_eq!(
            selected.plan.instance_target_ref(),
            topology.instance_target()
        );
        assert_eq!(
            selected.plan.requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            selected.plan.provider_id().expect("provider").as_str(),
            "fixture-provider"
        );
        assert_eq!(
            selected.plan.model_id().expect("model").as_str(),
            "fixture-model"
        );
        let services = fixture.services(topology.execution_host_id().clone());
        let mut session = block_on(driver(selected.credential.clone()).open_session(
            selected.plan,
            open_request(&format!("topology-session-{index}"), selected.resource),
            services.clone(),
        ))
        .expect("Pi topology session opens");
        let mut turn = block_on(session.start_turn(
            turn_request(&format!("topology-turn-{index}"), deadline(100_000)),
            services,
        ))
        .expect("Pi topology turn starts");

        assert_eq!(
            block_on(turn.schedule_harness_message(scheduled(
                &format!("topology-steer-{index}"),
                HarnessMessageClass::Steering,
            )))
            .expect("steering response")
            .acknowledgement(),
            HarnessCommandAcknowledgement::Accepted
        );
        assert_eq!(
            block_on(turn.schedule_harness_message(scheduled(
                &format!("topology-follow-{index}"),
                HarnessMessageClass::FollowUp,
            )))
            .expect("follow-up response")
            .acknowledgement(),
            HarnessCommandAcknowledgement::Accepted
        );

        let mut callbacks = turn.take_callbacks().expect("callback exchange");
        let mut requests = callbacks.take_requests().expect("callback stream");
        let callback = block_on(requests.next())
            .expect("callback arrives")
            .expect("callback is valid");
        assert!(matches!(
            callback.kind(),
            CallbackRequestKind::HarnessUiDialog(_)
        ));
        assert_eq!(callback.deadline(), Some(deadline(10_000)));
        block_on(callbacks.responder().respond(CallbackResponse::new(
            callback.callback_id().clone(),
            callback.turn_id().expect("callback turn").clone(),
            CallbackResult::Success(
                CallbackPayload::new(b"Allow".to_vec(), 64).expect("bounded result"),
            ),
        )))
        .expect("callback response relays");

        let events = block_on(
            turn.take_events()
                .expect("event stream")
                .collect::<Vec<_>>(),
        );
        let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
        assert!(events.iter().all(Result::is_ok));
        assert!(events.iter().any(|event| {
            event
                .as_ref()
                .is_ok_and(|event| matches!(event.kind(), RuntimeEventKind::HarnessUiDisplay(_)))
        }));
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

        assert_eq!(
            fixture.process_executable(),
            topology.instance_target().as_host_value()
        );
        assert_eq!(
            fixture.process_working_resource(),
            topology.working_resource().as_host_value()
        );
        assert_eq!(
            fixture.cleanup_events(),
            [
                CleanupEvent::ProcessWait,
                CleanupEvent::ResourceRelease,
                CleanupEvent::CredentialRelease,
            ]
        );
        let inputs = fixture.inputs();
        let kinds: Vec<_> = inputs
            .iter()
            .filter_map(|value| value.get("type").and_then(serde_json::Value::as_str))
            .collect();
        assert!(position(&kinds, "prompt") < position(&kinds, "steer"));
        assert!(position(&kinds, "steer") < position(&kinds, "follow_up"));
        let public = format!("{events:?}{terminal:?}");
        for private in [
            "fixture private prompt",
            "private scheduled message",
            "fixture answer",
            topology.instance_target().as_host_value(),
        ] {
            assert!(!public.contains(private));
        }
    }
}

#[test]
fn callback_timeout_cancels_once_and_rejects_the_late_answer() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::Hold);
    let selected = selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let mut session = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        open_request("callback-timeout-session", selected.resource),
        services.clone(),
    ))
    .expect("Pi session opens");
    let mut turn = block_on(session.start_turn(
        turn_request("callback-timeout-turn", deadline(100_000)),
        services,
    ))
    .expect("Pi turn starts");
    block_on(turn.schedule_harness_message(scheduled(
        "callback-timeout-follow",
        HarnessMessageClass::FollowUp,
    )))
    .expect("follow-up response");
    let mut callbacks = turn.take_callbacks().expect("callback exchange");
    let mut requests = callbacks.take_requests().expect("callback stream");
    let callback = block_on(requests.next())
        .expect("callback arrives")
        .expect("callback is valid");

    fixture.advance_time(10_000);
    fixture.wait_for_input("extension_ui_response");
    let timeout_response = fixture
        .inputs()
        .into_iter()
        .find(|value| value["type"] == "extension_ui_response")
        .expect("timeout response was sent");
    assert_eq!(timeout_response["id"], "ui-dialog-1");
    assert_eq!(timeout_response["cancelled"], true);

    let error = block_on(callbacks.responder().respond(CallbackResponse::new(
        callback.callback_id().clone(),
        callback.turn_id().expect("callback turn").clone(),
        CallbackResult::Success(
            CallbackPayload::new(b"Allow".to_vec(), 64).expect("bounded result"),
        ),
    )))
    .expect_err("late callback response fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.callback_expired"
    );
    assert_eq!(
        block_on(turn.cancellation().request()).expect("turn aborts"),
        CancellationAcknowledgement::Requested
    );
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal outcome"));
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

fn driver(credential: swallowtail_core::CredentialRef) -> PiRpcDriver {
    PiRpcDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        credential,
    )
}

fn deadline(ticks: u64) -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(ticks))
}

fn scheduled(id: &str, class: HarnessMessageClass) -> HarnessScheduledMessage {
    HarnessScheduledMessage::new(
        HarnessCommandId::new(id).expect("valid command id"),
        class,
        OperationContent::new("private scheduled message").expect("valid content"),
    )
}

fn position(kinds: &[&str], expected: &str) -> usize {
    kinds
        .iter()
        .position(|kind| *kind == expected)
        .expect("command kind is present")
}
