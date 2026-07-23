use swallowtail_core::{InterfaceCompatibilityClaim, InterfaceVersion, InterfaceVersionScheme};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosedSemanticWindowCase {
    baseline: InterfaceVersion,
    latest_qualified: InterfaceVersion,
    accepted: Vec<InterfaceVersion>,
    rejected: Vec<InterfaceVersion>,
}

impl ClosedSemanticWindowCase {
    #[must_use]
    pub fn new(baseline: InterfaceVersion, latest_qualified: InterfaceVersion) -> Self {
        Self {
            baseline,
            latest_qualified,
            accepted: Vec::new(),
            rejected: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_accepted(mut self, versions: impl IntoIterator<Item = InterfaceVersion>) -> Self {
        self.accepted = versions.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_rejected(mut self, versions: impl IntoIterator<Item = InterfaceVersion>) -> Self {
        self.rejected = versions.into_iter().collect();
        self
    }
}

/// Asserts a bounded semantic claim without widening it through sampling.
pub fn assert_closed_semantic_compatibility_window(
    claim: &InterfaceCompatibilityClaim,
    case: &ClosedSemanticWindowCase,
) {
    assert_eq!(claim.scheme(), InterfaceVersionScheme::Semantic);
    assert_eq!(claim.baseline(), &case.baseline);
    assert_eq!(claim.latest_qualified(), &case.latest_qualified);
    assert!(claim.supports(&case.baseline));
    assert!(claim.supports(&case.latest_qualified));
    for version in &case.accepted {
        assert!(
            claim.supports(version),
            "expected compatible version {}",
            version.as_str()
        );
    }
    for version in &case.rejected {
        assert!(
            !claim.supports(version),
            "expected incompatible version {}",
            version.as_str()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{ClosedSemanticWindowCase, assert_closed_semantic_compatibility_window};
    use swallowtail_core::{
        InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
        InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionScheme,
        InterfaceVersionSegment,
    };

    #[test]
    fn closed_window_covers_boundaries_interior_and_prerelease() {
        let claim = InterfaceCompatibilityClaim::new(
            valid(InterfaceCompatibilityClaimId::new, "fixture.claim-1"),
            valid(InterfaceVersionAxis::new, "fixture.runtime"),
            InterfaceVersionScheme::Semantic,
            [InterfaceVersionSegment::new(
                version("0.14.0"),
                version("0.32.1"),
                valid(InterfaceBehaviorRevision::new, "fixture.behavior-1"),
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .expect("claim is valid");
        let case = ClosedSemanticWindowCase::new(version("0.14.0"), version("0.32.1"))
            .with_accepted([version("0.18.0"), version("0.30.0")])
            .with_rejected([version("0.13.5"), version("0.18.0-rc.1"), version("0.32.2")]);

        assert_closed_semantic_compatibility_window(&claim, &case);
    }

    fn version(value: &str) -> InterfaceVersion {
        valid(InterfaceVersion::new, value)
    }

    fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
    where
        E: std::fmt::Debug,
    {
        constructor(value.to_owned()).expect("fixture text is valid")
    }
}
