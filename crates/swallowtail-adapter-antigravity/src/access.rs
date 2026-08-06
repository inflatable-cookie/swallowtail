use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for provider-owned personal Google authentication.
pub const ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE: &str = "antigravity.personal-google";

#[must_use]
/// Builds a subscription-backed profile without exposing provider credentials.
pub fn antigravity_personal_google_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE)
            .expect("static Antigravity audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE, antigravity_personal_google_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn provider_owned_google_sign_in_never_becomes_a_swallowtail_credential() {
        let profile = antigravity_personal_google_access_profile(
            AccessProfileId::new("antigravity.fixture.personal").expect("valid access id"),
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
            ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
