use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for Muse Code's provider-owned local Meta account state.
pub const MUSE_LOCAL_META_ACCOUNT_AUDIENCE: &str = "muse-code.local-meta-account";

#[must_use]
/// Builds a subscription-backed profile without reading or leasing Meta credentials.
pub fn muse_local_meta_account_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(MUSE_LOCAL_META_ACCOUNT_AUDIENCE)
            .expect("static Muse Code audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{MUSE_LOCAL_META_ACCOUNT_AUDIENCE, muse_local_meta_account_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn local_meta_account_state_never_becomes_a_swallowtail_credential() {
        let profile = muse_local_meta_account_access_profile(
            AccessProfileId::new("muse.fixture.local-meta").expect("access id"),
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
            MUSE_LOCAL_META_ACCOUNT_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
