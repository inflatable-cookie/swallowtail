use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for OpenCode ACP's host-owned `opencode auth login` state.
pub const OPENCODE_ACP_HOST_ACCOUNT_AUDIENCE: &str = "opencode.acp.host-account";

#[must_use]
/// Builds a host-account profile without reading or leasing credentials.
pub fn opencode_acp_host_account_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new(OPENCODE_ACP_HOST_ACCOUNT_AUDIENCE)
            .expect("static OpenCode ACP audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{OPENCODE_ACP_HOST_ACCOUNT_AUDIENCE, opencode_acp_host_account_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn host_account_state_never_becomes_a_swallowtail_credential() {
        let profile = opencode_acp_host_account_access_profile(
            AccessProfileId::new("opencode.acp.fixture.host-account").expect("access id"),
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
            OPENCODE_ACP_HOST_ACCOUNT_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
