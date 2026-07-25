use super::error::InvalidInterfaceCompatibilityClaim;
use super::ordering::{compare_versions, is_semantic_prerelease, validate_version};
use super::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaimId,
    InterfaceCompatibilityMatch, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceUnverifiedNewer, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionScheme,
};
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// One inclusive compatibility segment. Segment starts are behavior milestones.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterfaceVersionSegment {
    minimum: InterfaceVersion,
    maximum: InterfaceVersion,
    behavior_revision: InterfaceBehaviorRevision,
    support_status: InterfaceSupportStatus,
}

impl InterfaceVersionSegment {
    #[must_use]
    pub const fn new(
        minimum: InterfaceVersion,
        maximum: InterfaceVersion,
        behavior_revision: InterfaceBehaviorRevision,
        support_status: InterfaceSupportStatus,
    ) -> Self {
        Self {
            minimum,
            maximum,
            behavior_revision,
            support_status,
        }
    }

    #[must_use]
    pub fn exact(
        version: InterfaceVersion,
        behavior_revision: InterfaceBehaviorRevision,
        support_status: InterfaceSupportStatus,
    ) -> Self {
        Self {
            minimum: version.clone(),
            maximum: version,
            behavior_revision,
            support_status,
        }
    }

    #[must_use]
    pub const fn minimum(&self) -> &InterfaceVersion {
        &self.minimum
    }

    #[must_use]
    pub const fn maximum(&self) -> &InterfaceVersion {
        &self.maximum
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

/// One qualified compatibility window for one interface axis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterfaceCompatibilityClaim {
    id: InterfaceCompatibilityClaimId,
    axis: InterfaceVersionAxis,
    scheme: InterfaceVersionScheme,
    newer_version_posture: InterfaceNewerVersionPosture,
    segments: Vec<InterfaceVersionSegment>,
    exclusions: BTreeSet<InterfaceVersion>,
}

impl InterfaceCompatibilityClaim {
    pub fn new(
        id: InterfaceCompatibilityClaimId,
        axis: InterfaceVersionAxis,
        scheme: InterfaceVersionScheme,
        newer_version_posture: InterfaceNewerVersionPosture,
        segments: impl IntoIterator<Item = InterfaceVersionSegment>,
        exclusions: impl IntoIterator<Item = InterfaceVersion>,
    ) -> Result<Self, InvalidInterfaceCompatibilityClaim> {
        let claim = Self {
            id,
            axis,
            scheme,
            newer_version_posture,
            segments: segments.into_iter().collect(),
            exclusions: exclusions.into_iter().collect(),
        };
        claim.validate()?;
        Ok(claim)
    }

    #[must_use]
    pub const fn id(&self) -> &InterfaceCompatibilityClaimId {
        &self.id
    }

    #[must_use]
    pub const fn axis(&self) -> &InterfaceVersionAxis {
        &self.axis
    }

    #[must_use]
    pub const fn scheme(&self) -> InterfaceVersionScheme {
        self.scheme
    }

    #[must_use]
    pub const fn newer_version_posture(&self) -> InterfaceNewerVersionPosture {
        self.newer_version_posture
    }

    #[must_use]
    pub fn baseline(&self) -> &InterfaceVersion {
        self.segments
            .first()
            .expect("validated claim has a segment")
            .minimum()
    }

    #[must_use]
    pub fn latest_qualified(&self) -> &InterfaceVersion {
        self.segments
            .last()
            .expect("validated claim has a segment")
            .maximum()
    }

    pub fn milestones(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionSegment> {
        self.segments.iter()
    }

    pub fn exclusions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersion> {
        self.exclusions.iter()
    }

    #[must_use]
    pub fn classify(&self, version: &InterfaceVersion) -> Option<InterfaceCompatibilityMatch> {
        if self.exclusions.contains(version) || validate_version(self.scheme, version).is_err() {
            return None;
        }
        if self.scheme == InterfaceVersionScheme::Semantic && is_semantic_prerelease(version) {
            return self
                .segments
                .iter()
                .find(|segment| segment.minimum() == version && segment.maximum() == version)
                .map(|segment| {
                    InterfaceCompatibilityMatch::new(
                        segment.behavior_revision.clone(),
                        segment.support_status,
                    )
                });
        }
        self.segments
            .iter()
            .find(|segment| segment_contains(self.scheme, segment, version))
            .map(|segment| {
                InterfaceCompatibilityMatch::new(
                    segment.behavior_revision.clone(),
                    segment.support_status,
                )
            })
    }

    #[must_use]
    pub fn supports(&self, version: &InterfaceVersion) -> bool {
        self.classify(version).is_some()
    }

    #[must_use]
    pub fn assess(&self, version: &InterfaceVersion) -> InterfaceCompatibilityAssessment {
        if self.exclusions.contains(version) || validate_version(self.scheme, version).is_err() {
            return InterfaceCompatibilityAssessment::Incompatible;
        }
        if let Some(matched) = self.classify(version) {
            return InterfaceCompatibilityAssessment::Qualified(matched);
        }
        if self.newer_version_posture != InterfaceNewerVersionPosture::AllowUnverified
            || self.scheme == InterfaceVersionScheme::Opaque
            || (self.scheme == InterfaceVersionScheme::Semantic && is_semantic_prerelease(version))
            || !compare_versions(self.scheme, self.latest_qualified(), version)
                .is_ok_and(|ordering| ordering == Ordering::Less)
        {
            return InterfaceCompatibilityAssessment::Incompatible;
        }
        let latest = self.segments.last().expect("validated claim has a segment");
        InterfaceCompatibilityAssessment::UnverifiedNewer(InterfaceUnverifiedNewer::new(
            version.clone(),
            latest.maximum().clone(),
            latest.behavior_revision().clone(),
        ))
    }

    #[must_use]
    pub fn permits(&self, version: &InterfaceVersion) -> bool {
        self.assess(version).is_permitted()
    }

    fn validate(&self) -> Result<(), InvalidInterfaceCompatibilityClaim> {
        self.validate_segments()?;
        if self.scheme == InterfaceVersionScheme::Opaque
            && self.newer_version_posture != InterfaceNewerVersionPosture::QualifiedOnly
        {
            return Err(InvalidInterfaceCompatibilityClaim::new(
                "Opaque compatibility claims must remain qualified-only",
            ));
        }
        for exclusion in &self.exclusions {
            validate_version(self.scheme, exclusion)?;
        }
        Ok(())
    }

    fn validate_segments(&self) -> Result<(), InvalidInterfaceCompatibilityClaim> {
        if self.segments.is_empty() {
            return Err(InvalidInterfaceCompatibilityClaim::new(
                "Compatibility window must contain at least one segment",
            ));
        }
        if self.scheme == InterfaceVersionScheme::Opaque && self.segments.len() != 1 {
            return Err(InvalidInterfaceCompatibilityClaim::new(
                "Opaque version windows permit one exact segment only",
            ));
        }
        for segment in &self.segments {
            validate_version(self.scheme, segment.minimum())?;
            validate_version(self.scheme, segment.maximum())?;
            let ordering = compare_versions(self.scheme, segment.minimum(), segment.maximum())?;
            if ordering == Ordering::Greater
                || (self.scheme == InterfaceVersionScheme::Opaque && ordering != Ordering::Equal)
            {
                return Err(InvalidInterfaceCompatibilityClaim::new(
                    "Compatibility segment boundaries are invalid",
                ));
            }
        }
        for pair in self.segments.windows(2) {
            if compare_versions(self.scheme, pair[0].maximum(), pair[1].minimum())?
                != Ordering::Less
            {
                return Err(InvalidInterfaceCompatibilityClaim::new(
                    "Compatibility segments must be ordered and non-overlapping",
                ));
            }
        }
        Ok(())
    }
}

fn segment_contains(
    scheme: InterfaceVersionScheme,
    segment: &InterfaceVersionSegment,
    version: &InterfaceVersion,
) -> bool {
    compare_versions(scheme, segment.minimum(), version)
        .is_ok_and(|order| order != Ordering::Greater)
        && compare_versions(scheme, version, segment.maximum())
            .is_ok_and(|order| order != Ordering::Greater)
}
