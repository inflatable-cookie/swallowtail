#![deny(missing_docs)]

use crate::event::ExtensionNamespace;
use crate::runtime_identity::{AccessProfileId, CredentialRef, EndpointAudience};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Mechanism by which an integration obtains provider credentials.
pub enum CredentialMechanism {
    /// User completes an interactive OAuth flow.
    InteractiveOauth,
    /// User completes an OAuth device-code flow.
    DeviceOauth,
    /// Host supplies a non-interactive automation token.
    AutomationToken,
    /// Host supplies a provider API key.
    ApiKey,
    /// Workload identity supplies credentials without a static secret.
    WorkloadIdentity,
    /// Cloud-provider identity supplies credentials through its native chain.
    CloudProviderIdentity,
    /// A host-approved helper mediates credential access.
    GatewayHelper,
    /// Route requires no authentication and is not necessarily local.
    Unauthenticated,
    /// Local route requires no authentication.
    LocalUnauthenticated,
    /// Provider-specific mechanism outside the common vocabulary.
    ProviderSpecific(ExtensionNamespace),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Source against which provider usage is metered.
pub enum EntitlementMetering {
    /// Usage consumes an interactive subscription allowance.
    SubscriptionAllowance,
    /// Usage consumes credits purchased in advance.
    PrepaidCredits,
    /// Usage consumes credits bundled with another entitlement.
    BundledCredits,
    /// Usage is billed per request or unit consumed.
    PayAsYouGo,
    /// Usage is billed through a cloud-provider account.
    CloudAccountBilling,
    /// Usage consumes only caller-owned local compute.
    LocalCompute,
    /// Metering source could not be established.
    Unknown,
    /// Provider-specific metering outside the common vocabulary.
    ProviderSpecific(ExtensionNamespace),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Authority supporting or prohibiting an integration route.
pub enum SupportAuthority {
    /// Provider explicitly supports the route.
    ProviderSupported,
    /// Integration maintainer supports the route from observed evidence.
    IntegrationMaintainerSupported,
    /// Route is experimental and supported only by current observation.
    ExperimentalObserved,
    /// Route is explicitly prohibited.
    Prohibited,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Static access configuration bound to one configured provider instance.
pub struct AccessProfile {
    id: AccessProfileId,
    credential_mechanism: CredentialMechanism,
    entitlement_metering: EntitlementMetering,
    endpoint_audience: EndpointAudience,
    credential_reference: Option<CredentialRef>,
    support_authority: SupportAuthority,
}

impl AccessProfile {
    /// Creates an access profile without a host-private credential reference.
    #[must_use]
    pub const fn new(
        id: AccessProfileId,
        credential_mechanism: CredentialMechanism,
        entitlement_metering: EntitlementMetering,
        endpoint_audience: EndpointAudience,
        support_authority: SupportAuthority,
    ) -> Self {
        Self {
            id,
            credential_mechanism,
            entitlement_metering,
            endpoint_audience,
            credential_reference: None,
            support_authority,
        }
    }

    #[must_use]
    /// Returns the stable access-profile identity.
    pub const fn id(&self) -> &AccessProfileId {
        &self.id
    }

    #[must_use]
    /// Returns the configured credential mechanism.
    pub const fn credential_mechanism(&self) -> &CredentialMechanism {
        &self.credential_mechanism
    }

    #[must_use]
    /// Returns the configured entitlement metering source.
    pub const fn entitlement_metering(&self) -> &EntitlementMetering {
        &self.entitlement_metering
    }

    #[must_use]
    /// Returns the endpoint audience credentials must be valid for.
    pub const fn endpoint_audience(&self) -> &EndpointAudience {
        &self.endpoint_audience
    }

    #[must_use]
    /// Binds an opaque host-private credential reference.
    pub fn with_credential_reference(mut self, reference: CredentialRef) -> Self {
        self.credential_reference = Some(reference);
        self
    }

    #[must_use]
    /// Returns the opaque credential reference, when one is required.
    pub const fn credential_reference(&self) -> Option<&CredentialRef> {
        self.credential_reference.as_ref()
    }

    #[must_use]
    /// Returns the authority supporting this access posture.
    pub const fn support_authority(&self) -> SupportAuthority {
        self.support_authority
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Observed readiness of the configured credential mechanism.
pub enum CredentialState {
    /// Route does not require credentials.
    NotRequired,
    /// Credential state has not been established.
    Unknown,
    /// Credentials are required but not available.
    Required,
    /// Credentials are present and ready for use.
    Ready,
    /// Previously available credentials have expired.
    Expired,
    /// Provider or credential authority rejected the credentials.
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Observed state of the account entitlement used by the route.
pub enum EntitlementState {
    /// Entitlement state has not been established.
    Unknown,
    /// Required entitlement is available.
    Available,
    /// Required entitlement is unavailable.
    Unavailable,
    /// Metered allowance or credits are exhausted.
    Exhausted,
    /// Entitlement exists but policy restricts the requested route.
    Restricted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Observed authorization for the configured endpoint audience.
pub enum EndpointAuthorization {
    /// Endpoint authorization has not been established.
    Unknown,
    /// Current access may target the configured endpoint.
    Allowed,
    /// Current access may not target the configured endpoint.
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Observed readiness of the provider or local runtime.
pub enum RuntimeReadiness {
    /// Runtime readiness has not been established.
    Unknown,
    /// Runtime is ready for ordinary use.
    Ready,
    /// Runtime is usable with an explicitly degraded posture.
    Degraded,
    /// Runtime is unavailable.
    Unavailable,
}

/// An observed access snapshot. No aggregate readiness boolean is provided.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessStatus {
    profile_id: AccessProfileId,
    credential: CredentialState,
    entitlement: EntitlementState,
    endpoint_authorization: EndpointAuthorization,
    runtime_readiness: RuntimeReadiness,
    support_authority: SupportAuthority,
}

impl AccessStatus {
    /// Creates an access snapshot without collapsing its independent dimensions.
    #[must_use]
    pub const fn new(
        profile_id: AccessProfileId,
        credential: CredentialState,
        entitlement: EntitlementState,
        endpoint_authorization: EndpointAuthorization,
        runtime_readiness: RuntimeReadiness,
        support_authority: SupportAuthority,
    ) -> Self {
        Self {
            profile_id,
            credential,
            entitlement,
            endpoint_authorization,
            runtime_readiness,
            support_authority,
        }
    }

    #[must_use]
    /// Returns the access profile this snapshot observes.
    pub const fn profile_id(&self) -> &AccessProfileId {
        &self.profile_id
    }

    #[must_use]
    /// Returns observed credential readiness.
    pub const fn credential(&self) -> CredentialState {
        self.credential
    }

    #[must_use]
    /// Returns observed entitlement state.
    pub const fn entitlement(&self) -> EntitlementState {
        self.entitlement
    }

    #[must_use]
    /// Returns observed endpoint authorization.
    pub const fn endpoint_authorization(&self) -> EndpointAuthorization {
        self.endpoint_authorization
    }

    #[must_use]
    /// Returns observed provider or runtime readiness.
    pub const fn runtime_readiness(&self) -> RuntimeReadiness {
        self.runtime_readiness
    }

    #[must_use]
    /// Returns the authority supporting this observation.
    pub const fn support_authority(&self) -> SupportAuthority {
        self.support_authority
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessProfile, CredentialMechanism, EntitlementMetering, SupportAuthority};
    use crate::{AccessProfileId, CredentialRef, EndpointAudience};

    #[test]
    fn access_profile_binds_an_opaque_credential_reference() {
        let reference = CredentialRef::new("private-credential-ref").expect("reference is valid");
        let profile = AccessProfile::new(
            AccessProfileId::new("fixture-access").expect("access id is valid"),
            CredentialMechanism::GatewayHelper,
            EntitlementMetering::Unknown,
            EndpointAudience::new("fixture-audience").expect("audience is valid"),
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(reference.clone());

        assert_eq!(profile.credential_reference(), Some(&reference));
        assert!(!format!("{profile:?}").contains(reference.as_host_value()));
    }

    #[test]
    fn unauthenticated_access_does_not_imply_local_topology_or_metering() {
        assert_ne!(
            CredentialMechanism::Unauthenticated,
            CredentialMechanism::LocalUnauthenticated
        );
        let profile = AccessProfile::new(
            AccessProfileId::new("remote-unauthenticated").expect("access id is valid"),
            CredentialMechanism::Unauthenticated,
            EntitlementMetering::Unknown,
            EndpointAudience::new("remote-acp-endpoint").expect("audience is valid"),
            SupportAuthority::ExperimentalObserved,
        );

        assert_eq!(
            profile.credential_mechanism(),
            &CredentialMechanism::Unauthenticated
        );
        assert_eq!(
            profile.entitlement_metering(),
            &EntitlementMetering::Unknown
        );
    }
}
