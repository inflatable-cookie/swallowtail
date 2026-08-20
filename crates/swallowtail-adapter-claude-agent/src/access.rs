use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, EndpointAudience, EntitlementMetering,
    SupportAuthority,
};

/// Endpoint audience for Claude Agent ACP's maintainer-supported Anthropic access.
pub const CLAUDE_AGENT_ACP_SUBSCRIPTION_AUDIENCE: &str = "api.anthropic.com";

/// Describes maintainer-supported Claude Agent ACP access through local subscription state.
///
/// The caller still supplies separate observed or caller-asserted access
/// status. This constructor discovers no credential, account, entitlement, or
/// readiness. It does not extract keychain bytes.
#[must_use]
pub fn claude_agent_acp_subscription_access_profile(id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        id,
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(CLAUDE_AGENT_ACP_SUBSCRIPTION_AUDIENCE)
            .expect("static Claude Agent audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        CLAUDE_AGENT_ACP_SUBSCRIPTION_AUDIENCE, claude_agent_acp_subscription_access_profile,
    };
    use swallowtail_core::{
        AccessProfileId, CredentialMechanism, EntitlementMetering, SupportAuthority,
    };

    #[test]
    fn subscription_profile_encodes_no_credential_or_readiness() {
        let profile = claude_agent_acp_subscription_access_profile(
            AccessProfileId::new("fixture.claude-agent.subscription").expect("valid access id"),
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
            CLAUDE_AGENT_ACP_SUBSCRIPTION_AUDIENCE
        );
        assert_eq!(
            profile.support_authority(),
            SupportAuthority::IntegrationMaintainerSupported
        );
        assert!(profile.credential_reference().is_none());
    }
}
