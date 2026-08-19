use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for Goose's provider-owned local config state.
pub const GOOSE_LOCAL_ACCOUNT_AUDIENCE: &str = "goose.local-config";

#[must_use]
/// Builds a local-config profile without reading or leasing credentials.
pub fn goose_local_config_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new(GOOSE_LOCAL_ACCOUNT_AUDIENCE)
            .expect("static Goose audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{GOOSE_LOCAL_ACCOUNT_AUDIENCE, goose_local_config_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn local_config_state_never_becomes_a_swallowtail_credential() {
        let profile = goose_local_config_access_profile(
            AccessProfileId::new("goose.fixture.local-config").expect("access id"),
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
            GOOSE_LOCAL_ACCOUNT_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
