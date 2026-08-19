use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for Cline's provider-owned local account state.
pub const CLINE_LOCAL_ACCOUNT_AUDIENCE: &str = "cline.local-account";

#[must_use]
/// Builds a local-account profile without reading or leasing credentials.
pub fn cline_local_account_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(CLINE_LOCAL_ACCOUNT_AUDIENCE)
            .expect("static Cline audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{CLINE_LOCAL_ACCOUNT_AUDIENCE, cline_local_account_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn local_account_state_never_becomes_a_swallowtail_credential() {
        let profile = cline_local_account_access_profile(
            AccessProfileId::new("cline.fixture.local-account").expect("access id"),
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
            CLINE_LOCAL_ACCOUNT_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
