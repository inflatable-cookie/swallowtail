#[test]
fn lifecycle_tracker_evicts_terminal_activities_before_refusing_new_ones() {
    use super::lifecycle::{ActivityLifecycleTracker, ActivityTransitionFailure, MAX_TRACKED_ACTIVITIES};

    let observation = |index: usize, phase: ActivityLifecyclePhase, status: ActivityStatus| {
        ActivityObservation::new(
            ActivityId::new(format!("activity-{index}")).unwrap(),
            run_id(),
            ActivityKind::AssistantMessage,
            phase,
            status,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .unwrap()
    };
    let mut tracker = ActivityLifecycleTracker::default();
    for index in 0..MAX_TRACKED_ACTIVITIES {
        tracker
            .observe(&observation(
                index,
                ActivityLifecyclePhase::Updated,
                ActivityStatus::InProgress,
            ))
            .expect("in-flight activities below the bound are admitted");
    }
    assert_eq!(
        tracker.observe(&observation(
            MAX_TRACKED_ACTIVITIES,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
        )),
        Err(ActivityTransitionFailure::TrackerFull),
        "a full tracker of in-flight activities refuses a new one"
    );

    tracker
        .observe(&observation(
            0,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
        ))
        .expect("the first activity completes");
    tracker
        .observe(&observation(
            MAX_TRACKED_ACTIVITIES,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
        ))
        .expect("a completed activity is evicted to admit a new one");
    assert!(
        tracker.established(observation(0, ActivityLifecyclePhase::Updated, ActivityStatus::InProgress).activity_id()).is_none(),
        "the evicted terminal activity is no longer tracked"
    );
    assert!(
        tracker.established(observation(1, ActivityLifecyclePhase::Updated, ActivityStatus::InProgress).activity_id()).is_some(),
        "in-flight activities survive eviction"
    );
}
