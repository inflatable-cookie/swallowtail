use crate::{
    ExecutionHostId, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceCompatibilityMatch, InterfaceUnverifiedNewer,
    InterfaceVersionBinding, SafeDiagnostic,
};
use std::error::Error;
use std::fmt;

/// Classification of one exact installed executable against one driver claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstalledExecutableCompatibility {
    /// Version belongs to a qualified behavior segment.
    Qualified(InterfaceCompatibilityMatch),
    /// Stable newer version is permitted without qualified evidence.
    UnverifiedNewer(InterfaceUnverifiedNewer),
    /// Version is invalid, excluded, or unsupported.
    Incompatible,
}

/// Safe evidence from one explicit host-approved installed executable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstalledExecutableObservation {
    execution_host_id: ExecutionHostId,
    version: InterfaceVersionBinding,
    claim_id: InterfaceCompatibilityClaimId,
    compatibility: InstalledExecutableCompatibility,
}

impl InstalledExecutableObservation {
    /// Classifies one exact host observation against a matching claim axis.
    pub fn classify(
        execution_host_id: ExecutionHostId,
        version: InterfaceVersionBinding,
        claim: &InterfaceCompatibilityClaim,
    ) -> Result<Self, InvalidInstalledExecutableObservation> {
        if version.axis() != claim.axis() {
            return Err(InvalidInstalledExecutableObservation::axis_mismatch());
        }
        let compatibility = match claim.assess(version.version()) {
            InterfaceCompatibilityAssessment::Qualified(matched) => {
                InstalledExecutableCompatibility::Qualified(matched)
            }
            InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) => {
                InstalledExecutableCompatibility::UnverifiedNewer(unverified)
            }
            InterfaceCompatibilityAssessment::Incompatible => {
                InstalledExecutableCompatibility::Incompatible
            }
        };
        Ok(Self {
            execution_host_id,
            version,
            claim_id: claim.id().clone(),
            compatibility,
        })
    }

    #[must_use]
    /// Returns the host on which the executable was observed.
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    /// Returns exact observed interface version.
    pub const fn version(&self) -> &InterfaceVersionBinding {
        &self.version
    }

    #[must_use]
    /// Returns compatibility claim used for classification.
    pub const fn claim_id(&self) -> &InterfaceCompatibilityClaimId {
        &self.claim_id
    }

    #[must_use]
    /// Returns qualified, unverified, or incompatible evidence.
    pub const fn compatibility(&self) -> &InstalledExecutableCompatibility {
        &self.compatibility
    }

    #[must_use]
    /// Reports whether the version has qualified behavior evidence.
    pub const fn is_qualified(&self) -> bool {
        matches!(
            self.compatibility,
            InstalledExecutableCompatibility::Qualified(_)
        )
    }

    #[must_use]
    /// Reports whether policy permits this exact executable.
    pub const fn is_permitted(&self) -> bool {
        matches!(
            self.compatibility,
            InstalledExecutableCompatibility::Qualified(_)
                | InstalledExecutableCompatibility::UnverifiedNewer(_)
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Rejection raised when executable evidence and claim axes differ.
pub struct InvalidInstalledExecutableObservation {
    diagnostic: SafeDiagnostic,
}

impl InvalidInstalledExecutableObservation {
    fn axis_mismatch() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.installed_executable.axis_mismatch",
                "Installed executable version axis does not match the compatibility claim",
            ),
        }
    }

    #[must_use]
    /// Returns the redacted observation diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidInstalledExecutableObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidInstalledExecutableObservation {}

#[cfg(test)]
mod tests {
    use super::{InstalledExecutableCompatibility, InstalledExecutableObservation};
    use crate::{
        ExecutionHostId, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
        InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
        InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
        InterfaceVersionSegment,
    };

    #[test]
    fn exact_observation_is_classified_without_host_material() {
        let claim = claim();
        let host = ExecutionHostId::new("fixture.host.remote").expect("host id is valid");
        let observation = InstalledExecutableObservation::classify(
            host.clone(),
            binding("1.2.0", "fixture.harness"),
            &claim,
        )
        .expect("matching axis is valid");

        assert_eq!(observation.execution_host_id(), &host);
        assert_eq!(observation.claim_id(), claim.id());
        let InstalledExecutableCompatibility::Qualified(matched) = observation.compatibility()
        else {
            panic!("qualified version must be compatible");
        };
        assert_eq!(
            matched.behavior_revision(),
            &InterfaceBehaviorRevision::new("fixture.behavior.v1")
                .expect("behavior revision is valid")
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        let debug = format!("{observation:?}");
        assert!(!debug.contains("/private/"));
        assert!(!debug.contains("stdout"));
    }

    #[test]
    fn incompatible_and_mismatched_axes_remain_distinct() {
        let claim = claim();
        let host = ExecutionHostId::new("fixture.host.local").expect("host id is valid");
        let incompatible = InstalledExecutableObservation::classify(
            host.clone(),
            binding("2.0.0", "fixture.harness"),
            &claim,
        )
        .expect("matching axis is valid");
        assert_eq!(
            incompatible.compatibility(),
            &InstalledExecutableCompatibility::Incompatible
        );

        let failure = InstalledExecutableObservation::classify(
            host,
            binding("1.2.0", "fixture.other"),
            &claim,
        )
        .expect_err("axis substitution must fail");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.installed_executable.axis_mismatch"
        );
    }

    fn claim() -> InterfaceCompatibilityClaim {
        InterfaceCompatibilityClaim::new(
            InterfaceCompatibilityClaimId::new("fixture.claim.v1").expect("claim id is valid"),
            InterfaceVersionAxis::new("fixture.harness").expect("axis is valid"),
            InterfaceVersionScheme::Semantic,
            InterfaceNewerVersionPosture::QualifiedOnly,
            [InterfaceVersionSegment::new(
                InterfaceVersion::new("1.0.0").expect("version is valid"),
                InterfaceVersion::new("1.5.0").expect("version is valid"),
                InterfaceBehaviorRevision::new("fixture.behavior.v1")
                    .expect("behavior revision is valid"),
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .expect("claim is valid")
    }

    fn binding(version: &str, axis: &str) -> InterfaceVersionBinding {
        InterfaceVersionBinding::new(
            InterfaceVersionAxis::new(axis).expect("axis is valid"),
            InterfaceVersion::new(version).expect("version is valid"),
        )
    }
}
