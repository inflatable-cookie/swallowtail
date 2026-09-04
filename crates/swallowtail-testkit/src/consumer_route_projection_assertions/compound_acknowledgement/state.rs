use swallowtail_runtime::{
    ConsumerRouteAcknowledgementState, ConsumerRouteCompoundAcknowledgement,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceKind,
};

use crate::{ConsumerRouteProjectionFixture, consumer_route_projection_source};

use super::super::support::{assert_kind, compose, contribution, observation_source};
use super::support::{
    acknowledgement_row, effective_reasoning_rejected_plan, state_value, terminal_plan, value,
};

/// Proves that each compound half retains its own effective or rejected state.
pub fn assert_compound_acknowledgement_associates_each_half_state() {
    let acknowledgement = effective_reasoning_rejected_plan();
    assert!(matches!(
        acknowledgement.reasoning(),
        ConsumerRouteAcknowledgementState::Effective(_)
    ));
    assert!(matches!(
        acknowledgement.plan(),
        ConsumerRouteAcknowledgementState::Rejected(_)
    ));

    let absent = ConsumerRouteCompoundAcknowledgement::new(
        ConsumerRouteAcknowledgementState::absent(),
        ConsumerRouteAcknowledgementState::absent(),
    )
    .expect("absent halves carry no provider value");
    assert_eq!(state_value(absent.reasoning()), None);
    assert_eq!(state_value(absent.plan()), None);
}

/// Proves that state association preserves exact provider values without normalization.
pub fn assert_compound_acknowledgement_preserves_exact_provider_values() {
    let acknowledgement = ConsumerRouteCompoundAcknowledgement::new(
        ConsumerRouteAcknowledgementState::effective(value("xhigh")),
        ConsumerRouteAcknowledgementState::rejected(value("yolo")),
    )
    .expect("the compound acknowledgement is valid");
    assert_eq!(state_value(acknowledgement.reasoning()), Some("xhigh"));
    assert_eq!(state_value(acknowledgement.plan()), Some("yolo"));
}

/// Proves that terminal non-dispatch is distinct from pending, absence, and rejection.
pub fn assert_compound_acknowledgement_terminal_not_dispatched_is_distinct() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();
    let acknowledgement = terminal_plan();
    assert_ne!(
        acknowledgement.plan(),
        &ConsumerRouteAcknowledgementState::absent()
    );
    assert!(matches!(
        acknowledgement.plan(),
        ConsumerRouteAcknowledgementState::RequestedNotDispatched
    ));

    let row = acknowledgement_row(&applicability, observation_source(), acknowledgement);
    assert!(!row.state_support().pending());
    let admitted = contribution(&applicability, Vec::new(), Vec::new(), vec![row])
        .expect("terminal non-dispatch is admitted as completed observation");
    let projection = compose(&fixture, &[&admitted]).expect("the projection composes");
    let row = projection
        .active_session_state()
        .rows()
        .next()
        .expect("the compound row survives composition");
    assert!(matches!(
        row.compound_acknowledgement()
            .expect("the compound value survives")
            .plan(),
        ConsumerRouteAcknowledgementState::RequestedNotDispatched
    ));
    assert!(!row.state_support().pending());
    assert_eq!(projection.active_session_state().rows().len(), 1);

    let replacement_source = consumer_route_projection_source(
        "fixture.source.active-session-replacement",
        ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
    );
    let replacement_row =
        acknowledgement_row(&applicability, replacement_source.clone(), terminal_plan());
    let replacement = ConsumerRouteProjectionContribution::new(
        applicability,
        [replacement_source],
        Vec::new(),
        Vec::new(),
        [replacement_row],
    )
    .expect("replacement observation is admitted");
    let replaced = compose(&fixture, &[&replacement]).expect("replacement projection composes");
    assert_ne!(projection.identity(), replaced.identity());
    assert!(matches!(
        replaced
            .active_session_state()
            .rows()
            .next()
            .and_then(ConsumerRouteProjectionRow::compound_acknowledgement)
            .map(ConsumerRouteCompoundAcknowledgement::plan),
        Some(ConsumerRouteAcknowledgementState::RequestedNotDispatched)
    ));
}

/// Proves that speculative or out-of-order terminal half states fail closed.
pub fn assert_compound_acknowledgement_rejects_impossible_half_combinations() {
    for result in [
        ConsumerRouteCompoundAcknowledgement::new(
            ConsumerRouteAcknowledgementState::requested_not_dispatched(),
            ConsumerRouteAcknowledgementState::absent(),
        ),
        ConsumerRouteCompoundAcknowledgement::new(
            ConsumerRouteAcknowledgementState::effective(value("on")),
            ConsumerRouteAcknowledgementState::requested_not_dispatched(),
        ),
        ConsumerRouteCompoundAcknowledgement::new(
            ConsumerRouteAcknowledgementState::absent(),
            ConsumerRouteAcknowledgementState::requested_not_dispatched(),
        ),
    ] {
        let failure = result.expect_err("an impossible half-state combination is rejected");
        assert_kind(
            &failure,
            ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
        );
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.consumer_route_projection.acknowledgement_state_invalid"
        );
    }
}

/// Proves that the public accessors preserve reasoning-first, Plan-second order.
pub fn assert_compound_acknowledgement_preserves_reasoning_first_order() {
    let acknowledgement = terminal_plan();
    assert_eq!(state_value(acknowledgement.reasoning()), Some("off"));
    assert!(matches!(
        acknowledgement.plan(),
        ConsumerRouteAcknowledgementState::RequestedNotDispatched
    ));
}
