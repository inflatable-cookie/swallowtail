//! Bounded joined-teardown helper shared by both bridge profiles.

use std::thread::sleep;
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(1);

/// Waits for `ready` within one bounded budget.
///
/// The budget bounds the normal join attempt. A timeout is reported to the
/// caller as failed cleanup; it is never a licence to detach the work.
pub(crate) fn join_within(budget: Duration, mut ready: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + budget;
    loop {
        if ready() {
            return true;
        }
        if Instant::now() >= deadline {
            return ready();
        }
        sleep(POLL_INTERVAL);
    }
}
