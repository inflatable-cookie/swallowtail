use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for Deep Agents host-owned provider API keys.
pub const DEEPAGENTS_PROVIDER_API_KEY_AUDIENCE: &str = "deepagents.provider-api-key";

#[must_use]
/// Builds a host-owned API-key profile without reading or leasing credentials.
pub fn deepagents_provider_api_key_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new(DEEPAGENTS_PROVIDER_API_KEY_AUDIENCE)
            .expect("static Deep Agents audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{DEEPAGENTS_PROVIDER_API_KEY_AUDIENCE, deepagents_provider_api_key_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn provider_api_keys_never_become_a_swallowtail_credential() {
        let profile = deepagents_provider_api_key_access_profile(
            AccessProfileId::new("deepagents.fixture.provider-api-key").expect("access id"),
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
            DEEPAGENTS_PROVIDER_API_KEY_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
