use super::{
    ActivityAssistantPhase, ActivityCorrelation, ActivityDisclosure, ActivityId, ActivityKind,
    ActivityLifecyclePhase, ActivityObservation, ActivityOperationId, ActivityStatus,
};
use std::collections::HashMap;
use swallowtail_core::ProviderActivityRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActivityTransitionFailure {
    IdentityConflict,
    PhaseRegression,
    StatusRegression,
    DuplicateCompletion,
    AfterCompletion,
}

#[derive(Debug, Default)]
pub(crate) struct ActivityLifecycleTracker {
    activities: HashMap<ActivityId, ObservedActivityState>,
}

impl ActivityLifecycleTracker {
    pub(crate) fn observe(
        &mut self,
        observation: &ActivityObservation,
    ) -> Result<(), ActivityTransitionFailure> {
        let Some(state) = self.activities.get_mut(observation.activity_id()) else {
            self.activities.insert(
                observation.activity_id().clone(),
                ObservedActivityState::from_observation(observation),
            );
            return Ok(());
        };
        if !state.matches_identity(observation) {
            return Err(ActivityTransitionFailure::IdentityConflict);
        }
        if state.completed {
            return Err(
                if observation.phase() == ActivityLifecyclePhase::Completed {
                    ActivityTransitionFailure::DuplicateCompletion
                } else {
                    ActivityTransitionFailure::AfterCompletion
                },
            );
        }
        if observation.phase() == ActivityLifecyclePhase::Started {
            return Err(ActivityTransitionFailure::PhaseRegression);
        }
        if status_regresses(state.status, observation.status()) {
            return Err(ActivityTransitionFailure::StatusRegression);
        }

        state.status = observation.status();
        state.completed = observation.phase() == ActivityLifecyclePhase::Completed;
        Ok(())
    }
}

#[derive(Debug)]
struct ObservedActivityState {
    operation_id: ActivityOperationId,
    provider_activity_ref: Option<ProviderActivityRef>,
    kind: ActivityKind,
    status: ActivityStatus,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    correlation: Option<ActivityCorrelation>,
    completed: bool,
}

impl ObservedActivityState {
    fn from_observation(observation: &ActivityObservation) -> Self {
        Self {
            operation_id: observation.operation_id().clone(),
            provider_activity_ref: observation.provider_activity_ref().cloned(),
            kind: observation.kind().clone(),
            status: observation.status(),
            assistant_phase: observation.assistant_phase(),
            disclosure: observation.disclosure(),
            correlation: observation.correlation().cloned(),
            completed: observation.phase() == ActivityLifecyclePhase::Completed,
        }
    }

    fn matches_identity(&self, observation: &ActivityObservation) -> bool {
        self.operation_id == *observation.operation_id()
            && self.provider_activity_ref.as_ref() == observation.provider_activity_ref()
            && self.kind == *observation.kind()
            && self.assistant_phase == observation.assistant_phase()
            && self.disclosure == observation.disclosure()
            && self.correlation.as_ref() == observation.correlation()
    }
}

fn status_regresses(previous: ActivityStatus, next: ActivityStatus) -> bool {
    matches!(
        (previous, next),
        (ActivityStatus::InProgress, ActivityStatus::Pending)
    )
}
