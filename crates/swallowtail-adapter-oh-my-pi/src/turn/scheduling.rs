use super::ActiveTurn;
use std::sync::atomic::Ordering;
use swallowtail_core::HarnessMessageClass;

impl ActiveTurn {
    pub(crate) fn reserve_scheduling(&self, class: HarnessMessageClass) -> bool {
        match class {
            HarnessMessageClass::Steering => !self.steering_scheduled.swap(true, Ordering::SeqCst),
            HarnessMessageClass::FollowUp => !self.follow_up_scheduled.swap(true, Ordering::SeqCst),
            HarnessMessageClass::Prompt => false,
        }
    }

    pub(crate) fn release_scheduling(&self, class: HarnessMessageClass) {
        match class {
            HarnessMessageClass::Steering => {
                self.steering_scheduled.store(false, Ordering::SeqCst);
            }
            HarnessMessageClass::FollowUp => {
                self.follow_up_scheduled.store(false, Ordering::SeqCst);
            }
            HarnessMessageClass::Prompt => {}
        }
    }
}
