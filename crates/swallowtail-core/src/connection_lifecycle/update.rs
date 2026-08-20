use crate::SafeDiagnostic;
use crate::installed_executable::{
    InstalledExecutableCompatibility, InstalledExecutableObservation,
};
use crate::interface_version::{InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId};
use std::error::Error;
use std::fmt;

/// Derived instance update affordance from a Contract 029 claim and optional
/// Contract 032 installed-executable observation.
///
/// This reuses existing classification. It does not install, upgrade,
/// authenticate, admit an instance, or start sign-in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstanceUpdateObservation {
    claim_id: InterfaceCompatibilityClaimId,
    installed: Option<InstalledExecutableObservation>,
}

impl InstanceUpdateObservation {
    /// Projects an update observation from one claim and optional 032 evidence.
    pub fn from_claim(
        claim: &InterfaceCompatibilityClaim,
        installed: Option<InstalledExecutableObservation>,
    ) -> Result<Self, InvalidInstanceUpdateObservation> {
        if let Some(observation) = &installed {
            if observation.claim_id() != claim.id() {
                return Err(InvalidInstanceUpdateObservation::claim_mismatch());
            }
            if observation.version().axis() != claim.axis() {
                return Err(InvalidInstanceUpdateObservation::axis_mismatch());
            }
        }
        Ok(Self {
            claim_id: claim.id().clone(),
            installed,
        })
    }

    #[must_use]
    /// Returns the Contract 029 claim used for this observation.
    pub const fn claim_id(&self) -> &InterfaceCompatibilityClaimId {
        &self.claim_id
    }

    #[must_use]
    /// Returns the installed-executable observation, when one was supplied.
    pub const fn installed(&self) -> Option<&InstalledExecutableObservation> {
        self.installed.as_ref()
    }

    #[must_use]
    /// Reports whether no 032 evidence was supplied.
    pub const fn is_unobserved(&self) -> bool {
        self.installed.is_none()
    }

    #[must_use]
    /// Returns the reused 032 compatibility classification, when observed.
    pub const fn compatibility(&self) -> Option<&InstalledExecutableCompatibility> {
        match &self.installed {
            Some(observation) => Some(observation.compatibility()),
            None => None,
        }
    }
}

/// Rejection raised when update observation evidence does not match the claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidInstanceUpdateObservation {
    diagnostic: SafeDiagnostic,
}

impl InvalidInstanceUpdateObservation {
    fn claim_mismatch() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.connection_lifecycle.update_claim_mismatch",
                "Installed-executable observation does not match the compatibility claim",
            ),
        }
    }

    fn axis_mismatch() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.connection_lifecycle.update_axis_mismatch",
                "Installed-executable observation axis does not match the compatibility claim",
            ),
        }
    }

    #[must_use]
    /// Returns the redacted update-observation diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidInstanceUpdateObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidInstanceUpdateObservation {}

#[cfg(test)]
mod tests {
    use super::InstanceUpdateObservation;
    use crate::{
        ExecutionHostId, InstalledExecutableCompatibility, InstalledExecutableObservation,
        InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
        InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion,
        InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
        InterfaceVersionSegment,
    };

    fn claim() -> InterfaceCompatibilityClaim {
        InterfaceCompatibilityClaim::new(
            InterfaceCompatibilityClaimId::new("fixture.claim.v1").expect("claim id is valid"),
            InterfaceVersionAxis::new("fixture.harness").expect("axis is valid"),
            InterfaceVersionScheme::Semantic,
            InterfaceNewerVersionPosture::AllowUnverified,
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

    fn other_claim() -> InterfaceCompatibilityClaim {
        InterfaceCompatibilityClaim::new(
            InterfaceCompatibilityClaimId::new("fixture.claim.other").expect("claim id is valid"),
            InterfaceVersionAxis::new("fixture.other").expect("axis is valid"),
            InterfaceVersionScheme::Semantic,
            InterfaceNewerVersionPosture::QualifiedOnly,
            [InterfaceVersionSegment::new(
                InterfaceVersion::new("1.0.0").expect("version is valid"),
                InterfaceVersion::new("1.5.0").expect("version is valid"),
                InterfaceBehaviorRevision::new("fixture.behavior.other")
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

    fn host() -> ExecutionHostId {
        ExecutionHostId::new("fixture.host.local").expect("host id is valid")
    }

    #[test]
    fn unobserved_claim_projects_no_install_side_effect() {
        let claim = claim();
        let observation = InstanceUpdateObservation::from_claim(&claim, None)
            .expect("claim without observation is valid");

        assert_eq!(observation.claim_id(), claim.id());
        assert!(observation.is_unobserved());
        assert_eq!(observation.installed(), None);
        assert_eq!(observation.compatibility(), None);
    }

    #[test]
    fn observed_classification_is_reused_from_032() {
        let claim = claim();
        let installed = InstalledExecutableObservation::classify(
            host(),
            binding("1.2.0", "fixture.harness"),
            &claim,
        )
        .expect("matching axis is valid");
        let observation = InstanceUpdateObservation::from_claim(&claim, Some(installed.clone()))
            .expect("matching claim is valid");

        assert_eq!(observation.installed(), Some(&installed));
        assert!(matches!(
            observation.compatibility(),
            Some(InstalledExecutableCompatibility::Qualified(_))
        ));
        let debug = format!("{observation:?}");
        assert!(!debug.contains("/usr/bin/"));
        assert!(!debug.contains("stdout"));
    }

    #[test]
    fn unverified_newer_and_incompatible_stay_distinct() {
        let claim = claim();
        let unverified = InstalledExecutableObservation::classify(
            host(),
            binding("1.6.0", "fixture.harness"),
            &claim,
        )
        .expect("matching axis is valid");
        let incompatible = InstalledExecutableObservation::classify(
            host(),
            binding("0.9.0", "fixture.harness"),
            &claim,
        )
        .expect("matching axis is valid");

        assert!(matches!(
            InstanceUpdateObservation::from_claim(&claim, Some(unverified))
                .expect("unverified newer is valid")
                .compatibility(),
            Some(InstalledExecutableCompatibility::UnverifiedNewer(_))
        ));
        assert_eq!(
            InstanceUpdateObservation::from_claim(&claim, Some(incompatible))
                .expect("incompatible is valid")
                .compatibility(),
            Some(&InstalledExecutableCompatibility::Incompatible)
        );
    }

    #[test]
    fn mismatched_claim_is_rejected() {
        let claim = claim();
        let other = other_claim();
        let installed = InstalledExecutableObservation::classify(
            host(),
            binding("1.2.0", "fixture.other"),
            &other,
        )
        .expect("matching axis is valid");
        let failure = InstanceUpdateObservation::from_claim(&claim, Some(installed))
            .expect_err("claim substitution must fail");

        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.connection_lifecycle.update_claim_mismatch"
        );
    }
}
