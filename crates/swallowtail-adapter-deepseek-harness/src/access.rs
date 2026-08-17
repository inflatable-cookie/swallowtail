use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for host-owned DeepSeek Harness Cordis configuration.
pub const DEEPSEEK_HARNESS_CONFIG_AUDIENCE: &str = "deepseek-harness.host-config";

#[must_use]
/// Builds an access profile that never leases or extracts provider credentials.
pub fn deepseek_harness_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(DEEPSEEK_HARNESS_CONFIG_AUDIENCE)
            .expect("static DeepSeek Harness audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{DEEPSEEK_HARNESS_CONFIG_AUDIENCE, deepseek_harness_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn host_config_never_becomes_a_swallowtail_credential() {
        let profile = deepseek_harness_access_profile(
            AccessProfileId::new("deepseek-harness.fixture.host-config").expect("access id"),
        );
        assert_eq!(
            profile.credential_mechanism(),
            &CredentialMechanism::LocalUnauthenticated
        );
        assert_eq!(
            profile.entitlement_metering(),
            &EntitlementMetering::SubscriptionAllowance
        );
        assert_eq!(
            profile.endpoint_audience().as_str(),
            DEEPSEEK_HARNESS_CONFIG_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
