use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Audience for Command Code's provider-owned local account state.
pub const COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE: &str = "command-code.local-account";

#[must_use]
/// Builds a subscription-backed profile without reading or leasing credentials.
pub fn command_code_local_account_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE)
            .expect("static Command Code audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE, command_code_local_account_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn local_account_state_never_becomes_a_swallowtail_credential() {
        let profile = command_code_local_account_access_profile(
            AccessProfileId::new("command-code.fixture.local-account").expect("access id"),
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
            COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
