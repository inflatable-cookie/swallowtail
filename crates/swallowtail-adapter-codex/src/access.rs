use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

pub const CODEX_CHATGPT_SUBSCRIPTION_AUDIENCE: &str = "codex";

/// Describes provider-supported Codex access through a cached ChatGPT login.
///
/// The caller still supplies separate observed or caller-asserted access
/// status. This constructor discovers no credential, account, entitlement, or
/// readiness.
#[must_use]
pub fn codex_chatgpt_subscription_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(CODEX_CHATGPT_SUBSCRIPTION_AUDIENCE)
            .expect("static Codex audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{CODEX_CHATGPT_SUBSCRIPTION_AUDIENCE, codex_chatgpt_subscription_access_profile};
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn chatgpt_subscription_profile_encodes_no_credential_or_readiness() {
        let profile = codex_chatgpt_subscription_access_profile(
            AccessProfileId::new("fixture.codex.chatgpt").expect("valid access id"),
        );

        assert_eq!(
            profile.credential_mechanism(),
            &CredentialMechanism::InteractiveOauth
        );
        assert_eq!(
            profile.entitlement_metering(),
            &EntitlementMetering::SubscriptionAllowance
        );
        assert_eq!(
            profile.endpoint_audience().as_str(),
            CODEX_CHATGPT_SUBSCRIPTION_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::ProviderSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
