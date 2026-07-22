use super::{ProfilePreflightFixture, SyntheticProfile, restrictive_rpc_policy, valid};
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding,
    InterfaceVersionScheme, InterfaceVersionSegment, ProviderId,
};

impl ProfilePreflightFixture {
    pub(crate) fn harness_rpc_contract() -> Self {
        let mut fixture = Self::new(SyntheticProfile::LongLivedRpcHarness);
        let version = harness_rpc_version("1.0.0");
        let policy = restrictive_rpc_policy();
        fixture.driver = fixture.driver.clone().with_interface_compatibility(
            InterfaceCompatibilityClaim::new(
                valid(
                    InterfaceCompatibilityClaimId::new,
                    "fixture.harness-rpc.claim-1",
                ),
                version.axis().clone(),
                InterfaceVersionScheme::Semantic,
                [InterfaceVersionSegment::exact(
                    version.version().clone(),
                    valid(InterfaceBehaviorRevision::new, "fixture.rpc-v1"),
                    InterfaceSupportStatus::Maintained,
                )],
                [],
            )
            .expect("fixture compatibility window is valid"),
        );
        fixture.instance = fixture
            .instance
            .clone()
            .with_interface_versions([version.clone()])
            .with_harness_rpc_policy(policy.clone());
        fixture.route = fixture
            .route
            .clone()
            .with_provider_id(valid(ProviderId::new, "fixture.downstream-provider"));
        fixture.requirements = fixture
            .requirements
            .clone()
            .with_interface_versions([version])
            .with_harness_rpc_policy(policy);
        fixture
    }

    pub(crate) fn require_harness_rpc_version(&mut self, version: &str) {
        self.requirements = self
            .requirements
            .clone()
            .with_interface_versions([harness_rpc_version(version)]);
    }

    pub(crate) fn use_harness_rpc_compatibility_window(&mut self, version: &str) {
        let binding = harness_rpc_version(version);
        self.driver = self
            .driver
            .clone()
            .with_interface_compatibility(harness_rpc_window_claim());
        self.instance = self
            .instance
            .clone()
            .with_interface_versions([binding.clone()]);
        self.requirements = self.requirements.clone().with_interface_versions([binding]);
    }
}

fn harness_rpc_version(version: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        valid(InterfaceVersionAxis::new, "fixture.harness.package"),
        valid(InterfaceVersion::new, version),
    )
}

fn harness_rpc_window_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        valid(
            InterfaceCompatibilityClaimId::new,
            "fixture.harness-rpc.claim-window-2",
        ),
        valid(InterfaceVersionAxis::new, "fixture.harness.package"),
        InterfaceVersionScheme::Semantic,
        [
            InterfaceVersionSegment::new(
                valid(InterfaceVersion::new, "0.8.0"),
                valid(InterfaceVersion::new, "0.8.9"),
                valid(InterfaceBehaviorRevision::new, "fixture.rpc-v1"),
                InterfaceSupportStatus::Deprecated,
            ),
            InterfaceVersionSegment::new(
                valid(InterfaceVersion::new, "0.9.0"),
                valid(InterfaceVersion::new, "1.0.0"),
                valid(InterfaceBehaviorRevision::new, "fixture.rpc-v2"),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [valid(InterfaceVersion::new, "0.9.5")],
    )
    .expect("fixture compatibility window is valid")
}
