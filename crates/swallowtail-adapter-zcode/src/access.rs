use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for host-owned ZCode settings configuration.
pub const ZCODE_CONFIG_AUDIENCE: &str = "zcode.host-config";

#[must_use]
/// Builds an access profile that never leases or extracts provider credentials.
pub fn zcode_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(ZCODE_CONFIG_AUDIENCE).expect("static ZCode audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{ZCODE_CONFIG_AUDIENCE, zcode_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn host_config_never_becomes_a_swallowtail_credential() {
        let profile = zcode_access_profile(
            AccessProfileId::new("zcode.fixture.host-config").expect("access id"),
        );
        assert_eq!(
            profile.credential_mechanism(),
            &CredentialMechanism::LocalUnauthenticated
        );
        assert_eq!(
            profile.entitlement_metering(),
            &EntitlementMetering::SubscriptionAllowance
        );
        assert_eq!(profile.endpoint_audience().as_str(), ZCODE_CONFIG_AUDIENCE);
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
