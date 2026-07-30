use super::fixture::PreparedFixture;
use crate::http_support::StreamFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::atomic::Ordering;
use swallowtail_adapter_opencode::{OpenCodeRunProfileInput, OpenCodeSessionProfileInput};
use swallowtail_core::{Capability, ResourceAccess};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRole, CallbackFailureKind, CallbackPayload, CallbackRequest,
    CallbackRequestKind, CallbackResponse, CallbackResult, CleanupOutcome, HarnessQuestionId,
    HarnessQuestionOptionId, HarnessUserInputAnswer, HarnessUserInputResponse, OperationContent,
    RequestId, RuntimeTurnId, TerminalStatus, TurnRequest,
};

#[test]
fn prepared_session_dispatches_one_image_and_correlated_permission_and_question_responses() {
    let fixture = PreparedFixture::new_with_fixture(
        "opencode.input-callback.session",
        "1.18.10",
        StreamFixture::InputCallbacks,
    );
    let prepared = fixture.prepared();
    let profile = prepared
        .prepare_session(
            OpenCodeSessionProfileInput::new(
                RequestId::new("opencode-input-callback-session").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            )
            .with_image_attachments()
            .with_provider_callbacks(),
        )
        .expect("input and callback session prepares");
    assert!(
        profile
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::Attachments)
    );
    assert_eq!(
        profile
            .plan()
            .requirements()
            .session_access_policy()
            .and_then(|policy| policy.resource_access()),
        Some(ResourceAccess::ReadWrite)
    );
    assert_eq!(
        profile
            .plan()
            .requirements()
            .extension_namespaces()
            .map(swallowtail_core::ExtensionNamespace::as_str)
            .collect::<Vec<_>>(),
        vec!["opencode/permission", "opencode/question"]
    );

    let services = fixture.services();
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    let turn_id = RuntimeTurnId::new("opencode-input-callback-turn").unwrap();
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                turn_id.clone(),
                OperationContent::new("inspect the approved image").unwrap(),
            )
            .with_attachments([fixture.attachment()]),
            services,
        ),
    )
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("callback stream exists");
    let responder = callbacks.responder();

    let permission = next_callback(&mut requests);
    assert_extension(&permission, "opencode/permission", "permission");
    let persistent = block_on(responder.respond(CallbackResponse::new(
        permission.callback_id().clone(),
        turn_id.clone(),
        CallbackResult::Success(
            CallbackPayload::new(br#"{"reply":"always"}"#, 256).expect("reply is bounded"),
        ),
    )))
    .expect_err("persistent permission is rejected");
    assert_eq!(
        persistent.diagnostic().code(),
        "swallowtail.opencode.callback_malformed"
    );
    let permission_response = CallbackResponse::new(
        permission.callback_id().clone(),
        turn_id.clone(),
        CallbackResult::Success(
            CallbackPayload::new(br#"{"reply":"once"}"#, 256).expect("reply is bounded"),
        ),
    );
    block_on(responder.respond(permission_response.clone())).expect("one-shot reply is accepted");
    let duplicate =
        block_on(responder.respond(permission_response)).expect_err("duplicate reply is rejected");
    assert_eq!(
        duplicate.diagnostic().code(),
        "swallowtail.opencode.callback_unknown_or_duplicate"
    );

    let question = next_callback(&mut requests);
    let CallbackRequestKind::HarnessUserInput(user_input) = question.kind() else {
        panic!("question is typed user input");
    };
    assert_eq!(user_input.questions().len(), 1);
    block_on(
        responder.respond(CallbackResponse::new(
            question.callback_id().clone(),
            turn_id,
            CallbackResult::UserInput(
                HarnessUserInputResponse::new(
                    [HarnessUserInputAnswer::selected(
                        HarnessQuestionId::new("question-0").unwrap(),
                        [HarnessQuestionOptionId::new("Safe").unwrap()],
                        None,
                    )],
                    1,
                    256,
                )
                .unwrap(),
            ),
        )),
    )
    .expect("ordered answer is accepted");

    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.attachment_releases.load(Ordering::SeqCst), 1);

    let requests = fixture.server.requests();
    assert!(requests.iter().any(|request| {
        request.contains("/session/ses_fixture/prompt_async?")
            && request.contains(r#""type":"file""#)
            && request.contains(r#""mime":"image/png""#)
            && request.contains(r#""filename":"approved-image.png""#)
            && request.contains("data:image/png;base64,iVBORw0KGgo=")
    }));
    assert!(requests.iter().any(|request| {
        request.contains("/permission/per_fixture/reply?")
            && request.contains(r#"{"reply":"once"}"#)
    }));
    assert!(requests.iter().any(|request| {
        request.contains("/question/que_fixture/reply?")
            && request.contains(r#"{"answers":[["Safe"]]}"#)
    }));
    assert!(requests.iter().any(|request| {
        request.starts_with("POST /session?directory=")
            && request.contains(r#""permission":"*""#)
            && request.contains(r#""action":"ask""#)
    }));
}

#[test]
fn prepared_run_uses_run_correlation_and_reject_paths_without_persistent_authority() {
    let fixture = PreparedFixture::new_with_fixture(
        "opencode.input-callback.run",
        "1.18.10",
        StreamFixture::InputCallbacks,
    );
    let prepared = fixture.prepared();
    let profile = prepared
        .prepare_run(
            OpenCodeRunProfileInput::new(
                RequestId::new("opencode-input-callback-run").unwrap(),
                fixture.model(),
                OperationContent::new("inspect the approved image").unwrap(),
                fixture.resource.clone(),
            )
            .with_attachments([fixture.attachment()])
            .with_provider_callbacks(),
        )
        .expect("input and callback run prepares");
    let services = fixture.services();
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    let run_id = run.run_id().clone();
    let mut callbacks = run.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("callback stream exists");
    let responder = callbacks.responder();

    let permission = next_callback(&mut requests);
    assert_eq!(permission.run_id(), Some(&run_id));
    assert!(permission.turn_id().is_none());
    let mismatch = block_on(responder.respond(CallbackResponse::new(
        permission.callback_id().clone(),
        RuntimeTurnId::new("wrong-operation").unwrap(),
        CallbackResult::Success(
            CallbackPayload::new(br#"{"reply":"reject"}"#, 256).expect("reply is bounded"),
        ),
    )))
    .expect_err("turn response cannot answer a run callback");
    assert_eq!(
        mismatch.diagnostic().code(),
        "swallowtail.opencode.callback_operation_mismatch"
    );
    block_on(responder.respond(CallbackResponse::for_run(
        permission.callback_id().clone(),
        run_id.clone(),
        CallbackResult::Success(
            CallbackPayload::new(br#"{"reply":"reject"}"#, 256).expect("reply is bounded"),
        ),
    )))
    .expect("explicit rejection is accepted");

    let question = next_callback(&mut requests);
    assert_eq!(question.run_id(), Some(&run_id));
    block_on(responder.respond(CallbackResponse::for_run(
        question.callback_id().clone(),
        run_id,
        CallbackResult::Failure {
            kind: CallbackFailureKind::ConsumerFailed,
            detail: None,
        },
    )))
    .expect("question rejection is accepted");

    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.attachment_releases.load(Ordering::SeqCst), 1);

    let requests = fixture.server.requests();
    assert!(requests.iter().any(|request| {
        request.contains("/permission/per_fixture/reply?")
            && request.contains(r#""reply":"reject""#)
            && request.contains("Consumer rejected the one-shot request.")
    }));
    assert!(
        requests
            .iter()
            .any(|request| { request.contains("/question/que_fixture/reject?") })
    );
    assert!(
        !requests
            .iter()
            .any(|request| request.contains(r#""reply":"always""#))
    );
}

#[test]
fn cancellation_abandons_pending_callbacks_before_attachment_cleanup() {
    let fixture = PreparedFixture::new_with_fixture(
        "opencode.input-callback.cancel",
        "1.18.10",
        StreamFixture::InputCallbacks,
    );
    let prepared = fixture.prepared();
    let profile = prepared
        .prepare_session(
            OpenCodeSessionProfileInput::new(
                RequestId::new("opencode-input-callback-cancel").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            )
            .with_image_attachments()
            .with_provider_callbacks(),
        )
        .expect("session prepares");
    let services = fixture.services();
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    let turn_id = RuntimeTurnId::new("opencode-input-callback-cancel-turn").unwrap();
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                turn_id.clone(),
                OperationContent::new("wait for permission").unwrap(),
            )
            .with_attachments([fixture.attachment()]),
            services,
        ),
    )
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("callback stream exists");
    let pending = next_callback(&mut requests);
    let responder = callbacks.responder();

    block_on(turn.cancellation().request()).expect("turn cancellation requests");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    let late = block_on(responder.respond(CallbackResponse::new(
        pending.callback_id().clone(),
        turn_id,
        CallbackResult::Success(
            CallbackPayload::new(br#"{"reply":"once"}"#, 256).expect("reply is bounded"),
        ),
    )))
    .expect_err("late callback is rejected");
    assert_eq!(
        late.diagnostic().code(),
        "swallowtail.opencode.callback_closed"
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.attachment_releases.load(Ordering::SeqCst), 1);
    assert!(
        !fixture
            .server
            .requests()
            .iter()
            .any(|request| request.contains("/permission/per_fixture/reply?"))
    );
}

#[test]
fn unsupported_attachment_shape_rejects_during_preparation() {
    let fixture = PreparedFixture::new("opencode.input.invalid", "1.18.10");
    let prepared = fixture.prepared();
    let attachment = fixture.attachment();
    let context = AttachmentDescriptor::new(
        attachment.reference().clone(),
        "image/png",
        AttachmentRole::Context,
    )
    .unwrap()
    .with_known_length(8);
    let requests_before = fixture.server.requests().len();
    let error = prepared
        .prepare_run(
            OpenCodeRunProfileInput::new(
                RequestId::new("opencode-input-invalid").unwrap(),
                fixture.model(),
                OperationContent::new("invalid attachment shape").unwrap(),
                fixture.resource.clone(),
            )
            .with_attachments([context]),
        )
        .expect_err("non-input attachment rejects");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.opencode.preparation.attachments_unsupported"
    );
    assert_eq!(fixture.server.requests().len(), requests_before);
}

#[test]
fn cancelled_session_rejects_before_attachment_materialization() {
    let fixture = PreparedFixture::new("opencode.input.cancelled-session", "1.18.10");
    let prepared = fixture.prepared();
    let profile = prepared
        .prepare_session(
            OpenCodeSessionProfileInput::new(
                RequestId::new("opencode-input-cancelled-session").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            )
            .with_image_attachments(),
        )
        .expect("session prepares");
    let services = fixture.services();
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    block_on(session.cancellation().request()).expect("session cancellation requests");
    let error = match block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("opencode-input-cancelled-turn").unwrap(),
                OperationContent::new("must not materialize").unwrap(),
            )
            .with_attachments([fixture.attachment()]),
            services,
        ),
    ) {
        Ok(_) => panic!("cancelled session must reject the turn"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.session_cancelled"
    );
    assert_eq!(fixture.attachment_releases.load(Ordering::SeqCst), 0);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(
        !fixture
            .server
            .requests()
            .iter()
            .any(|request| request.contains("/prompt_async?"))
    );
}

fn next_callback(requests: &mut swallowtail_runtime::BoxCallbackStream) -> CallbackRequest {
    block_on(requests.next())
        .expect("callback arrives")
        .expect("callback is valid")
}

fn assert_extension(request: &CallbackRequest, namespace: &str, payload_field: &str) {
    let CallbackRequestKind::Extension(extension) = request.kind() else {
        panic!("provider request is an extension");
    };
    assert_eq!(extension.namespace().as_str(), namespace);
    let payload: serde_json::Value =
        serde_json::from_slice(extension.payload()).expect("callback payload is JSON");
    assert!(payload.get(payload_field).is_some());
    let rendered = format!("{request:?}");
    assert!(!rendered.contains("src/approved.rs"));
    assert!(!rendered.contains("Choose a bounded mode."));
}
