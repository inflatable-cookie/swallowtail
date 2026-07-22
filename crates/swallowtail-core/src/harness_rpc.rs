use std::collections::BTreeSet;
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HarnessMessageClass {
    Prompt,
    Steering,
    FollowUp,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HarnessConfigurationSource {
    Extensions,
    Skills,
    PromptTemplates,
    ContextFiles,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HarnessBackgroundAction {
    UpdateCheck,
    Telemetry,
    PackageMutation,
    AutomaticRetry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HarnessSchedulingBounds {
    maximum_active_operations: NonZeroU32,
    maximum_completed_prompts: NonZeroU32,
    maximum_pending_steering: NonZeroU32,
    maximum_pending_follow_up: NonZeroU32,
}

impl HarnessSchedulingBounds {
    #[must_use]
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
    pub const fn maximum_active_operations(self) -> NonZeroU32 {
        self.maximum_active_operations
    }

    #[must_use]
    pub const fn maximum_completed_prompts(self) -> NonZeroU32 {
        self.maximum_completed_prompts
    }

    #[must_use]
    pub const fn maximum_pending_steering(self) -> NonZeroU32 {
        self.maximum_pending_steering
    }

    #[must_use]
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
    pub fn restrictive(scheduling: HarnessSchedulingBounds) -> Self {
        Self {
            scheduling,
            configuration_sources: BTreeSet::new(),
            background_actions: BTreeSet::new(),
        }
    }

    #[must_use]
    pub fn with_configuration_sources(
        mut self,
        sources: impl IntoIterator<Item = HarnessConfigurationSource>,
    ) -> Self {
        self.configuration_sources = sources.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_background_actions(
        mut self,
        actions: impl IntoIterator<Item = HarnessBackgroundAction>,
    ) -> Self {
        self.background_actions = actions.into_iter().collect();
        self
    }

    #[must_use]
    pub const fn scheduling(&self) -> HarnessSchedulingBounds {
        self.scheduling
    }

    #[must_use]
    pub fn permits_configuration_source(&self, source: HarnessConfigurationSource) -> bool {
        self.configuration_sources.contains(&source)
    }

    #[must_use]
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
