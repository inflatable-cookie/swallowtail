use super::*;

pub(super) fn work_failure(failure: WorkFailure) -> TurnFailure {
    match failure {
        WorkFailure::Runtime(error) => TurnFailure::Provider(error, CleanupOutcome::Clean),
        WorkFailure::Stopped(stop) => TurnFailure::Stopped(stop, CleanupOutcome::Clean),
    }
}

pub(super) fn runtime_failure(error: RuntimeFailure) -> TurnFailure {
    TurnFailure::Runtime(error, CleanupOutcome::Clean)
}
