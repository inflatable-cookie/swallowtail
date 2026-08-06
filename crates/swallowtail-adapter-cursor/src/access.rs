use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Endpoint audience for Cursor's provider-owned local subscription login.
pub const CURSOR_SUBSCRIPTION_AUDIENCE: &str = "cursor-agent.subscription";

/// Builds an access profile for Cursor's locally authenticated subscription.
#[must_use]
pub fn cursor_subscription_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(CURSOR_SUBSCRIPTION_AUDIENCE)
            .expect("static Cursor endpoint audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::cursor_subscription_access_profile;
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn delegated_local_login_never_becomes_a_swallowtail_credential() {
        let profile = cursor_subscription_access_profile(
            AccessProfileId::new("cursor.fixture.subscription").expect("valid access id"),
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
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
