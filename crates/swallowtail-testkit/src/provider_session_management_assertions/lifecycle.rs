use super::{
    driver::{FixtureBehavior, execute},
    trace::FixtureEvent,
};
use crate::{
    ProviderSessionManagementFixture, ProviderSessionManagementFixtureCase, RecordingHostServices,
    RecordingOutcome,
};
use swallowtail_core::{
    ProviderSessionDeletionStrength, ProviderSessionEffectTruth, ProviderSessionManagementAction,
};

pub(super) fn assert_cancellation_and_deadline_truth() {
    let fixture = ProviderSessionManagementFixture::local(
        ProviderSessionManagementFixtureCase::Qualified,
        ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderHardDeleted,
        ),
    );

    let before_plan = fixture.plan(None).expect("cancellation plan is valid");
    let before_host = RecordingHostServices::for_host(
        before_plan.preflight().execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let (before, before_events) = execute(
        &fixture,
        before_plan,
        before_host.services().clone(),
        FixtureBehavior::Apply,
        true,
    );
    assert_eq!(
        before.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );
    assert!(before_events.is_empty());
    assert!(before_host.calls().is_empty());

    let expired = fixture
        .plan(Some(swallowtail_runtime::Deadline::at(
            swallowtail_runtime::MonotonicInstant::from_ticks(10),
        )))
        .expect("expired-deadline plan is valid");
    let expired_host = RecordingHostServices::for_host(
        expired.preflight().execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let (expired, expired_events) = execute(
        &fixture,
        expired,
        expired_host.services().clone(),
        FixtureBehavior::Apply,
        false,
    );
    assert_eq!(
        expired.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );
    assert!(expired_events.is_empty());
    assert_eq!(expired_host.calls(), vec![crate::RecordedHostCall::TimeNow]);

    for behavior in [
        FixtureBehavior::CancelAfterDispatch,
        FixtureBehavior::DeadlineAfterDispatch,
    ] {
        let deadline = (behavior == FixtureBehavior::DeadlineAfterDispatch).then(|| {
            swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                100,
            ))
        });
        let plan = fixture
            .plan(deadline)
            .expect("after-dispatch plan is valid");
        let host = RecordingHostServices::for_host(
            plan.preflight().execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let (outcome, events) = execute(&fixture, plan, host.services().clone(), behavior, false);
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::UnconfirmedAfterEffect
        );
        assert_eq!(outcome.effect().confirmed_deletion_strength(), None);
        assert!(events.contains(&FixtureEvent::Dispatched));
        assert_eq!(events.last(), Some(&FixtureEvent::CredentialReleased));
    }

    let absent_plan = fixture.plan(None).expect("absent plan is valid");
    let absent_host = RecordingHostServices::for_host(
        absent_plan.preflight().execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let (absent, _) = execute(
        &fixture,
        absent_plan,
        absent_host.services().clone(),
        FixtureBehavior::AlreadyAbsent,
        false,
    );
    assert_eq!(
        absent.effect().truth(),
        ProviderSessionEffectTruth::TargetAlreadyAbsent
    );
    assert_eq!(absent.effect().confirmed_deletion_strength(), None);
}
