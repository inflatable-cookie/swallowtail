use super::evidence;
use super::trace;
use crate::{ObservableActivityFixtureCase, ObservableActivityTraceFixture};
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, ObservableActivityProfile,
};
use swallowtail_runtime::{
    ActivityKind, EventBufferFailureKind, OrderedEventBuffer, RuntimeEvent, RuntimeEventKind,
};

pub(super) fn assert_details() {
    assert_lifecycle_fidelity_is_exact();
    assert_labels_refine_without_becoming_identity();
    assert_available_profile_does_not_require_occurrence();
    assert_unknown_activity_is_semantic_or_rejected();
    assert_output_and_final_assistant_are_distinct();
    assert_correlations_remain_separate_exchanges();
    evidence::assert_unverified_newer_profile_is_not_widened();
    evidence::assert_bounds_and_redaction();
    assert_ordering_failures();
}

fn assert_labels_refine_without_becoming_identity() {
    let fixture = ObservableActivityTraceFixture::for_case(
        ObservableActivityFixtureCase::UpdateAndCompletion,
    );
    let labels = fixture
        .events()
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => activity.label().map(|label| label.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(labels, ["Inspect", "Inspect portable contract"]);
    trace::validate(fixture.profile(), fixture.events())
        .expect("label refinement is not an activity identity conflict");
}

fn assert_available_profile_does_not_require_occurrence() {
    let available = ObservableActivityTraceFixture::for_case(
        ObservableActivityFixtureCase::IntermediateAssistant,
    );
    let no_activity =
        ObservableActivityTraceFixture::for_case(ObservableActivityFixtureCase::Unavailable);

    trace::validate(available.profile(), no_activity.events())
        .expect("available profile is a maximum, not a required event count");
}

fn assert_lifecycle_fidelity_is_exact() {
    for case in [
        ObservableActivityFixtureCase::CompleteLifecycle,
        ObservableActivityFixtureCase::UpdateAndCompletion,
        ObservableActivityFixtureCase::CompletionOnly,
    ] {
        let fixture = ObservableActivityTraceFixture::for_case(case);
        let activity = first_activity(&fixture);
        assert_ne!(
            fixture.profile().lifecycle(activity.kind().class()),
            ActivityLifecycleFidelity::Unavailable
        );
    }
}

fn assert_unknown_activity_is_semantic_or_rejected() {
    let fixture =
        ObservableActivityTraceFixture::for_case(ObservableActivityFixtureCase::UnknownSemantic);
    assert!(fixture.events().iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(activity)
            if matches!(activity.kind(), ActivityKind::Unknown(_))
    )));
    assert!(
        !fixture
            .events()
            .iter()
            .any(|event| matches!(event.kind(), RuntimeEventKind::Progress))
    );
    let fail_closed = ObservableActivityProfile::available(
        [],
        [ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::FinalAnswerText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("assistant profile is valid")],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("fail-closed profile is valid");
    assert!(trace::validate(&fail_closed, fixture.events()).is_err());
}

fn assert_output_and_final_assistant_are_distinct() {
    let fixture =
        ObservableActivityTraceFixture::for_case(ObservableActivityFixtureCase::FinalAssistant);
    let assistant = fixture
        .events()
        .iter()
        .find_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity)
                if activity.kind().class() == ActivityKindClass::AssistantMessage =>
            {
                Some((event.sequence(), activity))
            }
            _ => None,
        })
        .expect("final assistant activity is present");
    let output = fixture
        .events()
        .iter()
        .find(|event| matches!(event.kind(), RuntimeEventKind::OutputAvailable))
        .expect("final output is present");

    assert_ne!(assistant.0, output.sequence());
    assert_eq!(
        assistant
            .1
            .content()
            .expect("assistant content is present")
            .content()
            .as_str(),
        output
            .content()
            .expect("output content is present")
            .as_str()
    );
}

fn assert_correlations_remain_separate_exchanges() {
    for case in [
        ObservableActivityFixtureCase::CallbackCorrelation,
        ObservableActivityFixtureCase::DirectToolCorrelation,
    ] {
        let fixture = ObservableActivityTraceFixture::for_case(case);
        let exchange_sequence = fixture
            .events()
            .iter()
            .find(|event| {
                matches!(
                    event.kind(),
                    RuntimeEventKind::CallbackRequested(_)
                        | RuntimeEventKind::DirectToolCallAvailable(_)
                )
            })
            .expect("exchange event is present")
            .sequence();
        let activity_sequence = fixture
            .events()
            .iter()
            .find(|event| matches!(event.kind(), RuntimeEventKind::Activity(_)))
            .expect("correlated activity is present")
            .sequence();
        assert_ne!(exchange_sequence, activity_sequence);
    }
}

fn assert_ordering_failures() {
    let fixture =
        ObservableActivityTraceFixture::for_case(ObservableActivityFixtureCase::CompleteLifecycle);
    let events = fixture.events();
    let mut missing_start = OrderedEventBuffer::new(events.len()).expect("capacity is valid");
    let failure = missing_start
        .push(events[1].clone())
        .expect_err("activity before operation start must fail");
    assert_eq!(failure.kind(), EventBufferFailureKind::MissingStart);

    let mut duplicate_completion =
        OrderedEventBuffer::new(events.len() + 1).expect("capacity is valid");
    for event in events {
        duplicate_completion
            .push(event.clone())
            .expect("canonical event is accepted");
    }
    let completion = events.last().expect("completion is present").clone();
    let RuntimeEventKind::Activity(activity) = completion.kind() else {
        panic!("last canonical event must be activity");
    };
    let failure = duplicate_completion
        .push(RuntimeEvent::new(
            completion.sequence() + 1,
            RuntimeEventKind::Activity(activity.clone()),
        ))
        .expect_err("duplicate activity completion must fail");
    assert_eq!(
        failure.kind(),
        EventBufferFailureKind::DuplicateActivityCompletion
    );
}

fn first_activity(
    fixture: &ObservableActivityTraceFixture,
) -> &swallowtail_runtime::ActivityObservation {
    fixture
        .events()
        .iter()
        .find_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .expect("fixture contains activity")
}
