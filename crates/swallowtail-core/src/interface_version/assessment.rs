use super::{InterfaceBehaviorRevision, InterfaceSupportStatus, InterfaceVersion};

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub const fn behavior_revision(&self) -> &InterfaceBehaviorRevision {
        &self.behavior_revision
    }

    #[must_use]
    pub const fn support_status(&self) -> InterfaceSupportStatus {
        self.support_status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub const fn version(&self) -> &InterfaceVersion {
        &self.version
    }

    #[must_use]
    pub const fn latest_qualified(&self) -> &InterfaceVersion {
        &self.latest_qualified
    }

    #[must_use]
    pub const fn behavior_revision(&self) -> &InterfaceBehaviorRevision {
        &self.behavior_revision
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InterfaceCompatibilityAssessment {
    Qualified(InterfaceCompatibilityMatch),
    UnverifiedNewer(InterfaceUnverifiedNewer),
    Incompatible,
}

impl InterfaceCompatibilityAssessment {
    #[must_use]
    pub const fn is_permitted(&self) -> bool {
        matches!(self, Self::Qualified(_) | Self::UnverifiedNewer(_))
    }

    #[must_use]
    pub const fn behavior_revision(&self) -> Option<&InterfaceBehaviorRevision> {
        match self {
            Self::Qualified(matched) => Some(matched.behavior_revision()),
            Self::UnverifiedNewer(unverified) => Some(unverified.behavior_revision()),
            Self::Incompatible => None,
        }
    }
}
