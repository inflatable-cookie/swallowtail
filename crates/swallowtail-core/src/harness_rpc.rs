use std::collections::BTreeSet;
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Class of message scheduled through a harness RPC session.
pub enum HarnessMessageClass {
    /// Ordinary user prompt.
    Prompt,
    /// Input intended to steer active work.
    Steering,
    /// Follow-up queued after current work.
    FollowUp,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Harness-owned configuration source admitted for an operation.
pub enum HarnessConfigurationSource {
    /// Installed extension packages.
    Extensions,
    /// Installed skill definitions.
    Skills,
    /// Harness prompt templates.
    PromptTemplates,
    /// Ambient context files.
    ContextFiles,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Ambient harness background action requiring explicit policy.
pub enum HarnessBackgroundAction {
    /// Check for harness updates.
    UpdateCheck,
    /// Emit harness telemetry.
    Telemetry,
    /// Install or mutate packages.
    PackageMutation,
    /// Retry an operation automatically.
    AutomaticRetry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Positive bounds for concurrent and queued harness messages.
pub struct HarnessSchedulingBounds {
    maximum_active_operations: NonZeroU32,
    maximum_completed_prompts: NonZeroU32,
    maximum_pending_steering: NonZeroU32,
    maximum_pending_follow_up: NonZeroU32,
}

impl HarnessSchedulingBounds {
    #[must_use]
    /// Creates exact positive scheduling bounds.
    pub const fn new(
        maximum_active_operations: NonZeroU32,
        maximum_completed_prompts: NonZeroU32,
        maximum_pending_steering: NonZeroU32,
        maximum_pending_follow_up: NonZeroU32,
    ) -> Self {
        Self {
            maximum_active_operations,
            maximum_completed_prompts,
            maximum_pending_steering,
            maximum_pending_follow_up,
        }
    }

    #[must_use]
    /// Returns maximum simultaneously active operations.
    pub const fn maximum_active_operations(self) -> NonZeroU32 {
        self.maximum_active_operations
    }

    #[must_use]
    /// Returns maximum completed prompts retained by the session.
    pub const fn maximum_completed_prompts(self) -> NonZeroU32 {
        self.maximum_completed_prompts
    }

    #[must_use]
    /// Returns maximum queued steering messages.
    pub const fn maximum_pending_steering(self) -> NonZeroU32 {
        self.maximum_pending_steering
    }

    #[must_use]
    /// Returns maximum queued follow-up messages.
    pub const fn maximum_pending_follow_up(self) -> NonZeroU32 {
        self.maximum_pending_follow_up
    }
}

/// Operation-visible RPC policy. Empty allow-lists mean disabled.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessRpcPolicy {
    scheduling: HarnessSchedulingBounds,
    configuration_sources: BTreeSet<HarnessConfigurationSource>,
    background_actions: BTreeSet<HarnessBackgroundAction>,
}

impl HarnessRpcPolicy {
    #[must_use]
    /// Creates scheduling policy with all ambient sources and actions disabled.
    pub fn restrictive(scheduling: HarnessSchedulingBounds) -> Self {
        Self {
            scheduling,
            configuration_sources: BTreeSet::new(),
            background_actions: BTreeSet::new(),
        }
    }

    #[must_use]
    /// Replaces admitted harness configuration sources.
    pub fn with_configuration_sources(
        mut self,
        sources: impl IntoIterator<Item = HarnessConfigurationSource>,
    ) -> Self {
        self.configuration_sources = sources.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces admitted harness background actions.
    pub fn with_background_actions(
        mut self,
        actions: impl IntoIterator<Item = HarnessBackgroundAction>,
    ) -> Self {
        self.background_actions = actions.into_iter().collect();
        self
    }

    #[must_use]
    /// Returns exact scheduling bounds.
    pub const fn scheduling(&self) -> HarnessSchedulingBounds {
        self.scheduling
    }

    #[must_use]
    /// Reports whether a configuration source is admitted.
    pub fn permits_configuration_source(&self, source: HarnessConfigurationSource) -> bool {
        self.configuration_sources.contains(&source)
    }

    #[must_use]
    /// Reports whether a background action is admitted.
    pub fn permits_background_action(&self, action: HarnessBackgroundAction) -> bool {
        self.background_actions.contains(&action)
    }
}

#[cfg(test)]
mod tests {
    use super::{HarnessBackgroundAction, HarnessRpcPolicy, HarnessSchedulingBounds};
    use std::num::NonZeroU32;

    #[test]
    fn restrictive_policy_disables_ambient_background_actions() {
        let one = NonZeroU32::new(1).unwrap();
        let policy =
            HarnessRpcPolicy::restrictive(HarnessSchedulingBounds::new(one, one, one, one));

        assert!(!policy.permits_background_action(HarnessBackgroundAction::AutomaticRetry));
    }
}
