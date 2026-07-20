use crate::event::ExtensionNamespace;
use crate::runtime_identity::{AccessProfileId, EndpointAudience};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CredentialMechanism {
    InteractiveOauth,
    DeviceOauth,
    AutomationToken,
    ApiKey,
    WorkloadIdentity,
    CloudProviderIdentity,
    GatewayHelper,
    LocalUnauthenticated,
    ProviderSpecific(ExtensionNamespace),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EntitlementMetering {
    SubscriptionAllowance,
    PrepaidCredits,
    BundledCredits,
    PayAsYouGo,
    CloudAccountBilling,
    LocalCompute,
    Unknown,
    ProviderSpecific(ExtensionNamespace),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SupportAuthority {
    ProviderSupported,
    IntegrationMaintainerSupported,
    ExperimentalObserved,
    Prohibited,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessProfile {
    id: AccessProfileId,
    credential_mechanism: CredentialMechanism,
    entitlement_metering: EntitlementMetering,
    endpoint_audience: EndpointAudience,
    support_authority: SupportAuthority,
}

impl AccessProfile {
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
            support_authority,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &AccessProfileId {
        &self.id
    }

    #[must_use]
    pub const fn credential_mechanism(&self) -> &CredentialMechanism {
        &self.credential_mechanism
    }

    #[must_use]
    pub const fn entitlement_metering(&self) -> &EntitlementMetering {
        &self.entitlement_metering
    }

    #[must_use]
    pub const fn endpoint_audience(&self) -> &EndpointAudience {
        &self.endpoint_audience
    }

    #[must_use]
    pub const fn support_authority(&self) -> SupportAuthority {
        self.support_authority
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CredentialState {
    NotRequired,
    Unknown,
    Required,
    Ready,
    Expired,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EntitlementState {
    Unknown,
    Available,
    Unavailable,
    Exhausted,
    Restricted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EndpointAuthorization {
    Unknown,
    Allowed,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RuntimeReadiness {
    Unknown,
    Ready,
    Degraded,
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
    pub const fn profile_id(&self) -> &AccessProfileId {
        &self.profile_id
    }

    #[must_use]
    pub const fn credential(&self) -> CredentialState {
        self.credential
    }

    #[must_use]
    pub const fn entitlement(&self) -> EntitlementState {
        self.entitlement
    }

    #[must_use]
    pub const fn endpoint_authorization(&self) -> EndpointAuthorization {
        self.endpoint_authorization
    }

    #[must_use]
    pub const fn runtime_readiness(&self) -> RuntimeReadiness {
        self.runtime_readiness
    }

    #[must_use]
    pub const fn support_authority(&self) -> SupportAuthority {
        self.support_authority
    }
}
