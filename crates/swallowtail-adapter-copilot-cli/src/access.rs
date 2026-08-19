use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for Copilot CLI's host-owned GitHub login or BYOK state.
pub const COPILOT_CLI_HOST_ACCOUNT_AUDIENCE: &str = "copilot-cli.host-account";

#[must_use]
/// Builds a host-account profile without reading or leasing credentials.
pub fn copilot_cli_host_account_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new(COPILOT_CLI_HOST_ACCOUNT_AUDIENCE)
            .expect("static Copilot CLI audience is valid"),
        SupportAuthority::ExperimentalObserved,
    )
}

#[cfg(test)]
mod tests {
    use super::{COPILOT_CLI_HOST_ACCOUNT_AUDIENCE, copilot_cli_host_account_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn host_account_state_never_becomes_a_swallowtail_credential() {
        let profile = copilot_cli_host_account_access_profile(
            AccessProfileId::new("copilot-cli.fixture.host-account").expect("access id"),
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
            COPILOT_CLI_HOST_ACCOUNT_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ExperimentalObserved
        );
        assert!(profile.credential_reference().is_none());
    }
}
