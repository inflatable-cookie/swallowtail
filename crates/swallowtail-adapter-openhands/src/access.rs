use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for OpenHands owned-loopback Agent Server access.
pub const OPENHANDS_LOCAL_ACCOUNT_AUDIENCE: &str = "openhands.local-loopback";

#[must_use]
/// Builds a local-loopback profile without reading or leasing credentials.
pub fn openhands_local_config_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new(OPENHANDS_LOCAL_ACCOUNT_AUDIENCE)
            .expect("static OpenHands audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{OPENHANDS_LOCAL_ACCOUNT_AUDIENCE, openhands_local_config_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn local_loopback_never_becomes_a_swallowtail_credential() {
        let profile = openhands_local_config_access_profile(
            AccessProfileId::new("openhands.fixture.local-loopback").expect("access id"),
        );
        assert_eq!(
            profile.credential_mechanism(),
            &CredentialMechanism::LocalUnauthenticated
        );
        assert_eq!(
            profile.entitlement_metering(),
            &EntitlementMetering::Unknown
        );
        assert_eq!(
            profile.endpoint_audience().as_str(),
            OPENHANDS_LOCAL_ACCOUNT_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
