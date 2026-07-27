#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FixtureEvent {
    TaskStarted,
    CredentialAcquired,
    ResourceAcquired,
    Dispatched,
    TaskJoined,
    ResourceReleased,
    CredentialReleased,
}

pub(super) fn record(events: &std::sync::Mutex<Vec<FixtureEvent>>, event: FixtureEvent) {
    events
        .lock()
        .expect("fixture event lock is not poisoned")
        .push(event);
}
