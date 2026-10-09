use super::error::InvalidInterfaceCompatibilityClaim;
use super::ordering::{compare_versions, is_semantic_prerelease, validate_version};
use super::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaimId,
    InterfaceCompatibilityMatch, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceUnverifiedNewer, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionScheme,
};
use std::cmp::Ordering;
use std::collections::BTreeSet;

const MAX_OPAQUE_MEMBERS: usize = 32;
const MAX_OPAQUE_EXCLUSIONS: usize = 32;
const MAX_OPAQUE_TEXT_BYTES: usize = 256;

/// One inclusive compatibility segment. Segment starts are behavior milestones.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterfaceVersionSegment {
    minimum: InterfaceVersion,
    maximum: InterfaceVersion,
    behavior_revision: InterfaceBehaviorRevision,
    support_status: InterfaceSupportStatus,
}

impl InterfaceVersionSegment {
    /// Creates an inclusive compatibility segment.
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
    /// Creates a compatibility segment for one exact version.
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
    /// Returns the first included interface version.
    pub const fn minimum(&self) -> &InterfaceVersion {
        &self.minimum
    }

    #[must_use]
    /// Returns the last included interface version.
    pub const fn maximum(&self) -> &InterfaceVersion {
        &self.maximum
    }

    #[must_use]
    /// Returns the behavior revision shared by the segment.
    pub const fn behavior_revision(&self) -> &InterfaceBehaviorRevision {
        &self.behavior_revision
    }

    #[must_use]
    /// Returns the maintainer support status for the segment.
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
    /// Creates and validates a compatibility claim for one interface axis.
    ///
    /// Ordered-scheme segments must be ordered and non-overlapping. Opaque
    /// members are exact points and are stored in canonical text order.
    pub fn new(
        id: InterfaceCompatibilityClaimId,
        axis: InterfaceVersionAxis,
        scheme: InterfaceVersionScheme,
        newer_version_posture: InterfaceNewerVersionPosture,
        segments: impl IntoIterator<Item = InterfaceVersionSegment>,
        exclusions: impl IntoIterator<Item = InterfaceVersion>,
    ) -> Result<Self, InvalidInterfaceCompatibilityClaim> {
        let (segments, exclusions) = if scheme == InterfaceVersionScheme::Opaque {
            let segments = collect_bounded(
                segments,
                MAX_OPAQUE_MEMBERS,
                "Opaque compatibility claims permit at most 32 exact members",
            )?;
            let exclusions: BTreeSet<InterfaceVersion> = collect_bounded(
                exclusions,
                MAX_OPAQUE_EXCLUSIONS,
                "Opaque compatibility claims permit at most 32 exclusions",
            )?
            .into_iter()
            .collect();
            (segments, exclusions)
        } else {
            (
                segments.into_iter().collect(),
                exclusions.into_iter().collect(),
            )
        };
        let claim = Self {
            id,
            axis,
            scheme,
            newer_version_posture,
            segments,
            exclusions,
        };
        claim.validate()?;
        let mut claim = claim;
        if claim.scheme == InterfaceVersionScheme::Opaque {
            claim
                .segments
                .sort_by(|left, right| left.minimum().cmp(right.minimum()));
        }
        Ok(claim)
    }

    #[must_use]
    /// Returns the stable identity of this compatibility claim.
    pub const fn id(&self) -> &InterfaceCompatibilityClaimId {
        &self.id
    }

    #[must_use]
    /// Returns the interface axis governed by the claim.
    pub const fn axis(&self) -> &InterfaceVersionAxis {
        &self.axis
    }

    #[must_use]
    /// Returns the version-ordering scheme used by the claim.
    pub const fn scheme(&self) -> InterfaceVersionScheme {
        self.scheme
    }

    #[must_use]
    /// Returns the policy for stable versions above the qualified window.
    pub const fn newer_version_posture(&self) -> InterfaceNewerVersionPosture {
        self.newer_version_posture
    }

    #[must_use]
    /// Reports whether the scheme defines an interval; Opaque claims are sets.
    pub const fn has_version_interval(&self) -> bool {
        !matches!(self.scheme, InterfaceVersionScheme::Opaque)
    }

    #[must_use]
    /// Returns the first ordered boundary or canonical opaque member.
    pub fn baseline(&self) -> &InterfaceVersion {
        self.segments
            .first()
            .expect("validated claim has a segment")
            .minimum()
    }

    #[must_use]
    /// Returns the last ordered boundary or canonical opaque member.
    pub fn latest_qualified(&self) -> &InterfaceVersion {
        self.segments
            .last()
            .expect("validated claim has a segment")
            .maximum()
    }

    /// Iterates behavior milestones in scheme order or opaque text order.
    pub fn milestones(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionSegment> {
        self.segments.iter()
    }

    /// Iterates explicitly excluded version points in stable order.
    pub fn exclusions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersion> {
        self.exclusions.iter()
    }

    #[must_use]
    /// Returns qualified behavior evidence for an exactly supported version.
    ///
    /// Unverified newer versions are deliberately not returned here.
    pub fn classify(&self, version: &InterfaceVersion) -> Option<InterfaceCompatibilityMatch> {
        if self.exclusions.contains(version) || validate_version(self.scheme, version).is_err() {
            return None;
        }
        if self.scheme == InterfaceVersionScheme::Opaque {
            return self
                .segments
                .iter()
                .find(|segment| segment.minimum() == version)
                .map(segment_match);
        }
        if self.scheme == InterfaceVersionScheme::Semantic && is_semantic_prerelease(version) {
            return self
                .segments
                .iter()
                .find(|segment| segment.minimum() == version && segment.maximum() == version)
                .map(segment_match);
        }
        self.segments
            .iter()
            .find(|segment| segment_contains(self.scheme, segment, version))
            .map(segment_match)
    }

    #[must_use]
    /// Reports whether `version` belongs to a qualified segment.
    pub fn supports(&self, version: &InterfaceVersion) -> bool {
        self.classify(version).is_some()
    }

    #[must_use]
    /// Assesses `version` as qualified, unverified newer, or incompatible.
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
    /// Reports whether the claim permits use of `version`.
    pub fn permits(&self, version: &InterfaceVersion) -> bool {
        self.assess(version).is_permitted()
    }

    fn validate(&self) -> Result<(), InvalidInterfaceCompatibilityClaim> {
        if self.scheme == InterfaceVersionScheme::Opaque {
            return self.validate_opaque();
        }
        self.validate_ordered_segments()?;
        for exclusion in &self.exclusions {
            validate_version(self.scheme, exclusion)?;
        }
        Ok(())
    }

    fn validate_opaque(&self) -> Result<(), InvalidInterfaceCompatibilityClaim> {
        if self.newer_version_posture != InterfaceNewerVersionPosture::QualifiedOnly {
            return Err(InvalidInterfaceCompatibilityClaim::new(
                "Opaque compatibility claims must remain qualified-only",
            ));
        }
        if self.segments.is_empty() {
            return Err(InvalidInterfaceCompatibilityClaim::new(
                "Compatibility window must contain at least one segment",
            ));
        }

        let mut member_versions = BTreeSet::<InterfaceVersion>::new();
        for segment in &self.segments {
            validate_version(InterfaceVersionScheme::Opaque, segment.minimum())?;
            validate_version(InterfaceVersionScheme::Opaque, segment.maximum())?;
            if segment.minimum() != segment.maximum() {
                return Err(InvalidInterfaceCompatibilityClaim::new(
                    "Compatibility segment boundaries are invalid",
                ));
            }
            if segment.minimum().as_str().len() > MAX_OPAQUE_TEXT_BYTES {
                return Err(InvalidInterfaceCompatibilityClaim::new(
                    "Opaque version text exceeds 256 bytes",
                ));
            }
            if segment.behavior_revision.as_str().len() > MAX_OPAQUE_TEXT_BYTES {
                return Err(InvalidInterfaceCompatibilityClaim::new(
                    "Opaque behavior revision text exceeds 256 bytes",
                ));
            }
            if !member_versions.insert(segment.minimum().clone()) {
                return Err(InvalidInterfaceCompatibilityClaim::new(
                    "Opaque compatibility members must be unique",
                ));
            }
        }

        for exclusion in &self.exclusions {
            if exclusion.as_str().len() > MAX_OPAQUE_TEXT_BYTES {
                return Err(InvalidInterfaceCompatibilityClaim::new(
                    "Opaque version text exceeds 256 bytes",
                ));
            }
            validate_version(InterfaceVersionScheme::Opaque, exclusion)?;
            if member_versions.contains(exclusion) {
                return Err(InvalidInterfaceCompatibilityClaim::new(
                    "Opaque exclusions cannot name a claimed member",
                ));
            }
        }

        if self.segments.len() >= 2 {
            let Some(maintained_revision) = self
                .segments
                .iter()
                .find(|segment| segment.support_status == InterfaceSupportStatus::Maintained)
                .map(|segment| &segment.behavior_revision)
            else {
                return Err(opaque_support_status_error());
            };
            if self.segments.iter().any(|segment| {
                (segment.support_status == InterfaceSupportStatus::Maintained
                    && &segment.behavior_revision != maintained_revision)
                    || (segment.support_status == InterfaceSupportStatus::Deprecated
                        && &segment.behavior_revision == maintained_revision)
            }) {
                return Err(opaque_support_status_error());
            }
        }

        Ok(())
    }

    fn validate_ordered_segments(&self) -> Result<(), InvalidInterfaceCompatibilityClaim> {
        if self.segments.is_empty() {
            return Err(InvalidInterfaceCompatibilityClaim::new(
                "Compatibility window must contain at least one segment",
            ));
        }
        for segment in &self.segments {
            validate_version(self.scheme, segment.minimum())?;
            validate_version(self.scheme, segment.maximum())?;
            if compare_versions(self.scheme, segment.minimum(), segment.maximum())?
                == Ordering::Greater
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

fn collect_bounded<T>(
    values: impl IntoIterator<Item = T>,
    maximum: usize,
    overflow_message: &'static str,
) -> Result<Vec<T>, InvalidInterfaceCompatibilityClaim> {
    let mut values = values.into_iter();
    let mut collected = Vec::with_capacity(maximum);
    while collected.len() < maximum {
        match values.next() {
            Some(value) => collected.push(value),
            None => return Ok(collected),
        }
    }
    if values.next().is_some() {
        return Err(InvalidInterfaceCompatibilityClaim::new(overflow_message));
    }
    Ok(collected)
}

fn segment_match(segment: &InterfaceVersionSegment) -> InterfaceCompatibilityMatch {
    InterfaceCompatibilityMatch::new(segment.behavior_revision.clone(), segment.support_status)
}

fn opaque_support_status_error() -> InvalidInterfaceCompatibilityClaim {
    InvalidInterfaceCompatibilityClaim::new(
        "Opaque support status must follow the claim's maintained behavior revision",
    )
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
