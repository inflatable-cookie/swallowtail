use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for Kiro's host-owned local account state.
pub const KIRO_LOCAL_ACCOUNT_AUDIENCE: &str = "kiro.local-account";

#[must_use]
/// Builds a local-account profile without reading or leasing credentials.
pub fn kiro_local_account_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new(KIRO_LOCAL_ACCOUNT_AUDIENCE).expect("static Kiro audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{KIRO_LOCAL_ACCOUNT_AUDIENCE, kiro_local_account_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn local_account_state_never_becomes_a_swallowtail_credential() {
        let profile = kiro_local_account_access_profile(
            AccessProfileId::new("kiro.fixture.local-account").expect("access id"),
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
            KIRO_LOCAL_ACCOUNT_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
