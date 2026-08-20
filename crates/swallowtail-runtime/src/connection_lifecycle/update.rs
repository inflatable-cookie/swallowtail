use swallowtail_core::{
    InstalledExecutableObservation, InstanceUpdateObservation, InterfaceCompatibilityClaim,
    InvalidInstanceUpdateObservation,
};

/// Projects an instance update affordance from a Contract 029 claim and
/// optional Contract 032 observation.
///
/// This function does not install, upgrade, authenticate, admit an instance,
/// or start sign-in. It does not create a second currentness system.
pub fn observe_instance_update(
    claim: &InterfaceCompatibilityClaim,
    installed: Option<InstalledExecutableObservation>,
) -> Result<InstanceUpdateObservation, InvalidInstanceUpdateObservation> {
    InstanceUpdateObservation::from_claim(claim, installed)
}
