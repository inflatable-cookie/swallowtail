use super::{InterfaceBehaviorRevision, InterfaceSupportStatus, InterfaceVersion};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Qualified behavior and support evidence for one interface version.
pub struct InterfaceCompatibilityMatch {
    behavior_revision: InterfaceBehaviorRevision,
    support_status: InterfaceSupportStatus,
}

impl InterfaceCompatibilityMatch {
    pub(super) const fn new(
        behavior_revision: InterfaceBehaviorRevision,
        support_status: InterfaceSupportStatus,
    ) -> Self {
        Self {
            behavior_revision,
            support_status,
        }
    }

    #[must_use]
    /// Returns the behavior revision selected for the version.
    pub const fn behavior_revision(&self) -> &InterfaceBehaviorRevision {
        &self.behavior_revision
    }

    #[must_use]
    /// Returns the maintainer support status of the matched segment.
    pub const fn support_status(&self) -> InterfaceSupportStatus {
        self.support_status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Evidence for a permitted stable version newer than the qualified window.
pub struct InterfaceUnverifiedNewer {
    version: InterfaceVersion,
    latest_qualified: InterfaceVersion,
    behavior_revision: InterfaceBehaviorRevision,
}

impl InterfaceUnverifiedNewer {
    pub(super) const fn new(
        version: InterfaceVersion,
        latest_qualified: InterfaceVersion,
        behavior_revision: InterfaceBehaviorRevision,
    ) -> Self {
        Self {
            version,
            latest_qualified,
            behavior_revision,
        }
    }

    #[must_use]
    /// Returns the observed newer interface version.
    pub const fn version(&self) -> &InterfaceVersion {
        &self.version
    }

    #[must_use]
    /// Returns the last version with qualified evidence.
    pub const fn latest_qualified(&self) -> &InterfaceVersion {
        &self.latest_qualified
    }

    #[must_use]
    /// Returns the latest qualified behavior revision used as provisional evidence.
    pub const fn behavior_revision(&self) -> &InterfaceBehaviorRevision {
        &self.behavior_revision
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Result of assessing one version against an interface compatibility claim.
pub enum InterfaceCompatibilityAssessment {
    /// Version belongs to a qualified compatibility segment.
    Qualified(InterfaceCompatibilityMatch),
    /// Stable newer version is permitted without qualified behavior evidence.
    UnverifiedNewer(InterfaceUnverifiedNewer),
    /// Version is invalid, excluded, or outside the permitted window.
    Incompatible,
}

impl InterfaceCompatibilityAssessment {
    #[must_use]
    /// Reports whether execution is permitted by the claim.
    pub const fn is_permitted(&self) -> bool {
        matches!(self, Self::Qualified(_) | Self::UnverifiedNewer(_))
    }

    #[must_use]
    /// Returns the qualified or provisional behavior revision, when available.
    pub const fn behavior_revision(&self) -> Option<&InterfaceBehaviorRevision> {
        match self {
            Self::Qualified(matched) => Some(matched.behavior_revision()),
            Self::UnverifiedNewer(unverified) => Some(unverified.behavior_revision()),
            Self::Incompatible => None,
        }
    }
}
