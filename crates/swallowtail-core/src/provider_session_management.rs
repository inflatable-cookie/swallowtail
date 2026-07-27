use crate::{Capability, InterfaceCompatibilityAssessment, InterfaceVersionBinding};

/// How an opaque provider-session management binding was obtained.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionBindingOrigin {
    Created,
    Loaded,
    Resumed,
    ExplicitlyImported,
}

/// One exact provider-session management action.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionManagementAction {
    Archive,
    Restore,
    Delete(ProviderSessionDeletionStrength),
}

impl ProviderSessionManagementAction {
    #[must_use]
    pub const fn required_capability(self) -> Capability {
        match self {
            Self::Archive => Capability::ProviderSessionArchive,
            Self::Restore => Capability::ProviderSessionRestore,
            Self::Delete(_) => Capability::ProviderSessionDelete,
        }
    }

    #[must_use]
    pub const fn target_state(self) -> ProviderSessionLifecycleState {
        match self {
            Self::Archive => ProviderSessionLifecycleState::Archived,
            Self::Restore => ProviderSessionLifecycleState::Unarchived,
            Self::Delete(_) => ProviderSessionLifecycleState::Deleted,
        }
    }

    #[must_use]
    pub const fn deletion_strength(self) -> Option<ProviderSessionDeletionStrength> {
        match self {
            Self::Archive | Self::Restore => None,
            Self::Delete(strength) => Some(strength),
        }
    }
}

/// Provider state on the persistent-session lifecycle plane.
///
/// `Unarchived` does not mean that a runtime attachment is active.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionLifecycleState {
    Unarchived,
    Archived,
    Deleted,
}

/// Provider state accepted before one planned action.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionInitialStateRequirement {
    Unarchived,
    Archived,
    UnarchivedOrArchived,
}

/// Explicit evidence that the caller has closed its runtime attachment.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionActivityEvidence {
    CallerAssertedInactive,
}

/// Cancellation behavior promised by one exact management route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionCancellationPosture {
    BeforeDispatchOnly,
    ProviderNative,
}

/// Strongest deletion semantics promised by one exact provider route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionDeletionStrength {
    HistoryRemoved,
    ProviderDataDeleted,
    ProviderHardDeleted,
}

/// Provider resources whose state the action is documented to affect.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionAffectedScope {
    TargetOnly,
    ProviderDefinedDescendants,
}

/// Truth known after one provider-session management attempt.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionEffectTruth {
    Applied,
    AlreadyInTargetState,
    TargetAlreadyAbsent,
    FailedBeforeEffect,
    UnconfirmedAfterEffect,
}

/// Provider-neutral effect evidence, excluding target identity and diagnostics.
///
/// Constructors prevent a failed or uncertain attempt from carrying a
/// confirmed resulting state or affected scope.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderSessionManagementEffect {
    action: ProviderSessionManagementAction,
    truth: ProviderSessionEffectTruth,
    affected_scope: Option<ProviderSessionAffectedScope>,
}

impl ProviderSessionManagementEffect {
    #[must_use]
    pub const fn applied(
        action: ProviderSessionManagementAction,
        affected_scope: ProviderSessionAffectedScope,
    ) -> Self {
        Self {
            action,
            truth: ProviderSessionEffectTruth::Applied,
            affected_scope: Some(affected_scope),
        }
    }

    #[must_use]
    pub const fn already_archived(affected_scope: ProviderSessionAffectedScope) -> Self {
        Self {
            action: ProviderSessionManagementAction::Archive,
            truth: ProviderSessionEffectTruth::AlreadyInTargetState,
            affected_scope: Some(affected_scope),
        }
    }

    #[must_use]
    pub const fn already_unarchived(affected_scope: ProviderSessionAffectedScope) -> Self {
        Self {
            action: ProviderSessionManagementAction::Restore,
            truth: ProviderSessionEffectTruth::AlreadyInTargetState,
            affected_scope: Some(affected_scope),
        }
    }

    #[must_use]
    pub const fn target_already_absent(
        deletion_strength: ProviderSessionDeletionStrength,
        affected_scope: ProviderSessionAffectedScope,
    ) -> Self {
        Self {
            action: ProviderSessionManagementAction::Delete(deletion_strength),
            truth: ProviderSessionEffectTruth::TargetAlreadyAbsent,
            affected_scope: Some(affected_scope),
        }
    }

    #[must_use]
    pub const fn failed_before_effect(action: ProviderSessionManagementAction) -> Self {
        Self {
            action,
            truth: ProviderSessionEffectTruth::FailedBeforeEffect,
            affected_scope: None,
        }
    }

    #[must_use]
    pub const fn unconfirmed_after_effect(action: ProviderSessionManagementAction) -> Self {
        Self {
            action,
            truth: ProviderSessionEffectTruth::UnconfirmedAfterEffect,
            affected_scope: None,
        }
    }

    #[must_use]
    pub const fn action(self) -> ProviderSessionManagementAction {
        self.action
    }

    #[must_use]
    pub const fn truth(self) -> ProviderSessionEffectTruth {
        self.truth
    }

    #[must_use]
    pub const fn resulting_state(self) -> Option<ProviderSessionLifecycleState> {
        match self.truth {
            ProviderSessionEffectTruth::Applied
            | ProviderSessionEffectTruth::AlreadyInTargetState
            | ProviderSessionEffectTruth::TargetAlreadyAbsent => Some(self.action.target_state()),
            ProviderSessionEffectTruth::FailedBeforeEffect
            | ProviderSessionEffectTruth::UnconfirmedAfterEffect => None,
        }
    }

    #[must_use]
    pub const fn affected_scope(self) -> Option<ProviderSessionAffectedScope> {
        self.affected_scope
    }

    /// Returns deletion strength only when this attempt confirmed the effect.
    #[must_use]
    pub const fn confirmed_deletion_strength(self) -> Option<ProviderSessionDeletionStrength> {
        match self.truth {
            ProviderSessionEffectTruth::Applied => self.action.deletion_strength(),
            ProviderSessionEffectTruth::AlreadyInTargetState
            | ProviderSessionEffectTruth::TargetAlreadyAbsent
            | ProviderSessionEffectTruth::FailedBeforeEffect
            | ProviderSessionEffectTruth::UnconfirmedAfterEffect => None,
        }
    }
}

/// One exact interface point and its visible compatibility status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionInterfaceCompatibility {
    binding: InterfaceVersionBinding,
    assessment: InterfaceCompatibilityAssessment,
}

impl ProviderSessionInterfaceCompatibility {
    #[must_use]
    pub const fn new(
        binding: InterfaceVersionBinding,
        assessment: InterfaceCompatibilityAssessment,
    ) -> Self {
        Self {
            binding,
            assessment,
        }
    }

    #[must_use]
    pub const fn binding(&self) -> &InterfaceVersionBinding {
        &self.binding
    }

    #[must_use]
    pub const fn assessment(&self) -> &InterfaceCompatibilityAssessment {
        &self.assessment
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ProviderSessionAffectedScope, ProviderSessionDeletionStrength, ProviderSessionEffectTruth,
        ProviderSessionLifecycleState, ProviderSessionManagementAction,
        ProviderSessionManagementEffect,
    };
    use crate::Capability;

    #[test]
    fn actions_require_independent_capabilities_and_states() {
        let archive = ProviderSessionManagementAction::Archive;
        let restore = ProviderSessionManagementAction::Restore;
        let delete = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        );

        assert_eq!(
            archive.required_capability(),
            Capability::ProviderSessionArchive
        );
        assert_eq!(
            restore.required_capability(),
            Capability::ProviderSessionRestore
        );
        assert_eq!(
            delete.required_capability(),
            Capability::ProviderSessionDelete
        );
        assert_eq!(
            archive.target_state(),
            ProviderSessionLifecycleState::Archived
        );
        assert_eq!(
            restore.target_state(),
            ProviderSessionLifecycleState::Unarchived
        );
        assert_eq!(
            delete.target_state(),
            ProviderSessionLifecycleState::Deleted
        );
    }

    #[test]
    fn deletion_strengths_and_effect_truth_do_not_substitute() {
        assert_ne!(
            ProviderSessionDeletionStrength::HistoryRemoved,
            ProviderSessionDeletionStrength::ProviderDataDeleted
        );
        assert_ne!(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
            ProviderSessionDeletionStrength::ProviderHardDeleted
        );

        let action = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderHardDeleted,
        );
        let applied = ProviderSessionManagementEffect::applied(
            action,
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
        );
        let absent = ProviderSessionManagementEffect::target_already_absent(
            ProviderSessionDeletionStrength::ProviderHardDeleted,
            ProviderSessionAffectedScope::TargetOnly,
        );
        let before = ProviderSessionManagementEffect::failed_before_effect(action);
        let uncertain = ProviderSessionManagementEffect::unconfirmed_after_effect(action);

        assert_eq!(applied.truth(), ProviderSessionEffectTruth::Applied);
        assert_eq!(
            applied.confirmed_deletion_strength(),
            Some(ProviderSessionDeletionStrength::ProviderHardDeleted)
        );
        assert_eq!(
            absent.truth(),
            ProviderSessionEffectTruth::TargetAlreadyAbsent
        );
        assert_eq!(absent.confirmed_deletion_strength(), None);
        assert_eq!(before.resulting_state(), None);
        assert_eq!(uncertain.resulting_state(), None);
        assert_ne!(before, uncertain);
    }
}
