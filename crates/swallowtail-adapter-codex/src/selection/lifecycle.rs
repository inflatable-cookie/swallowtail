use super::{CODEX_LATEST_QUALIFIED_VERSION, axis, segment};
use swallowtail_core::{
    Capability, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionScheme, ProviderSessionAffectedScope,
    ProviderSessionDeletionStrength, ProviderSessionManagementAction,
};

pub(super) const ARCHIVE_BEHAVIOR: &str = "codex.app-server.lifecycle.v1.archive-response";
pub(super) const RESTORE_BEHAVIOR: &str = "codex.app-server.lifecycle.v1.archive-restore-response";
pub(super) const NOTIFICATIONS_BEHAVIOR: &str =
    "codex.app-server.lifecycle.v1.archive-restore-notifications";
pub(super) const DESCENDANT_ARCHIVE_BEHAVIOR: &str =
    "codex.app-server.lifecycle.v1.best-effort-descendant-archive";
pub(super) const HARD_DELETE_BEHAVIOR: &str =
    "codex.app-server.lifecycle.v1.strict-descendant-hard-delete";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CodexLifecycleBehavior {
    ArchiveResponse,
    ArchiveRestoreResponse,
    ArchiveRestoreNotifications,
    BestEffortDescendantArchive,
    StrictDescendantHardDelete,
}

impl CodexLifecycleBehavior {
    pub(crate) const fn supports(self, action: ProviderSessionManagementAction) -> bool {
        match action {
            ProviderSessionManagementAction::Archive => true,
            ProviderSessionManagementAction::Restore => !matches!(self, Self::ArchiveResponse),
            ProviderSessionManagementAction::Delete(_) => {
                matches!(self, Self::StrictDescendantHardDelete)
            }
        }
    }

    pub(crate) const fn capabilities(self) -> &'static [Capability] {
        match self {
            Self::ArchiveResponse => &[Capability::ProviderSessionArchive],
            Self::ArchiveRestoreResponse => &[
                Capability::ProviderSessionArchive,
                Capability::ProviderSessionRestore,
            ],
            Self::ArchiveRestoreNotifications | Self::BestEffortDescendantArchive => &[
                Capability::ProviderSessionArchive,
                Capability::ProviderSessionRestore,
            ],
            Self::StrictDescendantHardDelete => &[
                Capability::ProviderSessionArchive,
                Capability::ProviderSessionRestore,
                Capability::ProviderSessionDelete,
            ],
        }
    }

    pub(crate) const fn expects_notification(
        self,
        action: ProviderSessionManagementAction,
    ) -> bool {
        match action {
            ProviderSessionManagementAction::Archive | ProviderSessionManagementAction::Restore => {
                !matches!(self, Self::ArchiveResponse | Self::ArchiveRestoreResponse)
            }
            ProviderSessionManagementAction::Delete(_) => {
                matches!(self, Self::StrictDescendantHardDelete)
            }
        }
    }

    pub(crate) const fn affected_scope(
        self,
        action: ProviderSessionManagementAction,
    ) -> ProviderSessionAffectedScope {
        match action {
            ProviderSessionManagementAction::Delete(_)
                if matches!(self, Self::StrictDescendantHardDelete) =>
            {
                ProviderSessionAffectedScope::ProviderDefinedDescendants
            }
            _ => ProviderSessionAffectedScope::TargetOnly,
        }
    }

    pub(crate) const fn delete_action(self) -> Option<ProviderSessionManagementAction> {
        if matches!(self, Self::StrictDescendantHardDelete) {
            Some(ProviderSessionManagementAction::Delete(
                ProviderSessionDeletionStrength::ProviderHardDeleted,
            ))
        } else {
            None
        }
    }
}

pub(crate) struct CodexLifecycleAssessment {
    pub(crate) behavior: CodexLifecycleBehavior,
    pub(crate) unverified_newer: bool,
}

pub(crate) fn classify_lifecycle_version(
    version: &InterfaceVersion,
) -> Option<CodexLifecycleAssessment> {
    let assessment = codex_app_server_lifecycle_claim().assess(version);
    let behavior = match assessment.behavior_revision()?.as_str() {
        ARCHIVE_BEHAVIOR => CodexLifecycleBehavior::ArchiveResponse,
        RESTORE_BEHAVIOR => CodexLifecycleBehavior::ArchiveRestoreResponse,
        NOTIFICATIONS_BEHAVIOR => CodexLifecycleBehavior::ArchiveRestoreNotifications,
        DESCENDANT_ARCHIVE_BEHAVIOR => CodexLifecycleBehavior::BestEffortDescendantArchive,
        HARD_DELETE_BEHAVIOR => CodexLifecycleBehavior::StrictDescendantHardDelete,
        _ => return None,
    };
    Some(CodexLifecycleAssessment {
        behavior,
        unverified_newer: matches!(
            assessment,
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ),
    })
}

#[must_use]
/// Returns the qualified compatibility claim for app-server thread lifecycle operations.
pub fn codex_app_server_lifecycle_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("codex.app-server.lifecycle-window-1")
            .expect("static claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            segment(
                "0.80.0",
                "0.81.0",
                ARCHIVE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.84.0",
                "0.91.0",
                ARCHIVE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.92.0",
                "0.103.0",
                RESTORE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.104.0",
                "0.107.0",
                NOTIFICATIONS_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.110.0",
                "0.122.0",
                NOTIFICATIONS_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
            segment(
                "0.123.0",
                "0.139.0",
                DESCENDANT_ARCHIVE_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
            segment(
                "0.140.0",
                CODEX_LATEST_QUALIFIED_VERSION,
                HARD_DELETE_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [
            super::version("0.149.2").expect("static Codex unpublished gap is valid"),
            super::version("0.150.2").expect("static Codex unpublished gap is valid"),
        ],
    )
    .expect("static Codex app-server lifecycle claim is valid")
}
