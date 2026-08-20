use super::observe_instance_update;
use swallowtail_core::{
    ExecutionHostId, InstalledExecutableCompatibility, InstalledExecutableObservation,
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment,
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

fn binding(version: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        InterfaceVersionAxis::new("fixture.harness").expect("axis is valid"),
        InterfaceVersion::new(version).expect("version is valid"),
    )
}

#[test]
fn update_observation_reuses_029_and_032_without_side_effects() {
    let claim = claim();
    let unobserved = observe_instance_update(&claim, None).expect("unobserved is valid");
    let installed = InstalledExecutableObservation::classify(
        ExecutionHostId::new("fixture.host.local").expect("host id is valid"),
        binding("1.2.0"),
        &claim,
    )
    .expect("matching axis is valid");
    let observed =
        observe_instance_update(&claim, Some(installed.clone())).expect("observed is valid");

    assert!(unobserved.is_unobserved());
    assert_eq!(observed.installed(), Some(&installed));
    assert!(matches!(
        observed.compatibility(),
        Some(InstalledExecutableCompatibility::Qualified(_))
    ));
}

#[test]
fn update_observation_cannot_admit_or_start_sign_in() {
    let claim = claim();
    let observation = observe_instance_update(&claim, None).expect("unobserved is valid");

    assert_eq!(observation.claim_id(), claim.id());
    assert_eq!(observation.installed(), None);
}
