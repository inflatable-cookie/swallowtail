use crate::{
    ExecutionHostId, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceCompatibilityMatch, InterfaceVersionBinding, SafeDiagnostic,
};
use std::error::Error;
use std::fmt;

/// Classification of one exact installed executable against one driver claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstalledExecutableCompatibility {
    Compatible(InterfaceCompatibilityMatch),
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
    pub fn classify(
        execution_host_id: ExecutionHostId,
        version: InterfaceVersionBinding,
        claim: &InterfaceCompatibilityClaim,
    ) -> Result<Self, InvalidInstalledExecutableObservation> {
        if version.axis() != claim.axis() {
            return Err(InvalidInstalledExecutableObservation::axis_mismatch());
        }
        let compatibility = claim
            .classify(version.version())
            .map_or(InstalledExecutableCompatibility::Incompatible, |matched| {
                InstalledExecutableCompatibility::Compatible(matched)
            });
        Ok(Self {
            execution_host_id,
            version,
            claim_id: claim.id().clone(),
            compatibility,
        })
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn version(&self) -> &InterfaceVersionBinding {
        &self.version
    }

    #[must_use]
    pub const fn claim_id(&self) -> &InterfaceCompatibilityClaimId {
        &self.claim_id
    }

    #[must_use]
    pub const fn compatibility(&self) -> &InstalledExecutableCompatibility {
        &self.compatibility
    }

    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(
            self.compatibility,
            InstalledExecutableCompatibility::Compatible(_)
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
        InterfaceCompatibilityClaimId, InterfaceSupportStatus, InterfaceVersion,
        InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
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
        let InstalledExecutableCompatibility::Compatible(matched) = observation.compatibility()
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
