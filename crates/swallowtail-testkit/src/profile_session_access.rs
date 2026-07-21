use crate::SyntheticProfile;
use swallowtail_core::{ResourceAccess, SessionAccessPolicy};

pub(crate) fn policy(profile: SyntheticProfile) -> SessionAccessPolicy {
    match profile {
        SyntheticProfile::ConnectionScopedDirectSession => SessionAccessPolicy::resource_free(),
        SyntheticProfile::PersistentAcpHarness => {
            SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
        }
        _ => SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
    }
}
