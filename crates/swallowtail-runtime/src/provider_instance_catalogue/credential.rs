use crate::{AccessEvidenceProvenance, PreparedAccessEvidence};
use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, CredentialState, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, RuntimeReadiness,
    SupportAuthority,
};

/// Safe credential, entitlement, endpoint, runtime, and support posture.
///
/// This projection contains no credential reference, lease, token, account
/// identity, or provider response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderCredentialPosture {
    profile_id: AccessProfileId,
    credential_mechanism: CredentialMechanism,
    credential_state: CredentialState,
    entitlement_metering: EntitlementMetering,
    entitlement_state: EntitlementState,
    endpoint_audience: EndpointAudience,
    endpoint_authorization: EndpointAuthorization,
    runtime_readiness: RuntimeReadiness,
    support_authority: SupportAuthority,
    provenance: AccessEvidenceProvenance,
}

impl ConfiguredProviderCredentialPosture {
    pub(super) fn from_evidence(
        profile: &AccessProfile,
        evidence: &PreparedAccessEvidence,
    ) -> Self {
        let status = evidence.status();
        Self {
            profile_id: profile.id().clone(),
            credential_mechanism: profile.credential_mechanism().clone(),
            credential_state: status.credential(),
            entitlement_metering: profile.entitlement_metering().clone(),
            entitlement_state: status.entitlement(),
            endpoint_audience: profile.endpoint_audience().clone(),
            endpoint_authorization: status.endpoint_authorization(),
            runtime_readiness: status.runtime_readiness(),
            support_authority: status.support_authority(),
            provenance: evidence.provenance().clone(),
        }
    }

    #[must_use]
    /// Returns the exact access-profile identity.
    pub const fn profile_id(&self) -> &AccessProfileId {
        &self.profile_id
    }

    #[must_use]
    /// Returns the configured credential mechanism without credential material.
    pub const fn credential_mechanism(&self) -> &CredentialMechanism {
        &self.credential_mechanism
    }

    #[must_use]
    /// Returns the independently observed credential readiness.
    pub const fn credential_state(&self) -> CredentialState {
        self.credential_state
    }

    #[must_use]
    /// Returns the configured entitlement metering posture.
    pub const fn entitlement_metering(&self) -> &EntitlementMetering {
        &self.entitlement_metering
    }

    #[must_use]
    /// Returns the independently observed entitlement state.
    pub const fn entitlement_state(&self) -> EntitlementState {
        self.entitlement_state
    }

    #[must_use]
    /// Returns the safe endpoint audience.
    pub const fn endpoint_audience(&self) -> &EndpointAudience {
        &self.endpoint_audience
    }

    #[must_use]
    /// Returns the independently observed endpoint authorization state.
    pub const fn endpoint_authorization(&self) -> EndpointAuthorization {
        self.endpoint_authorization
    }

    #[must_use]
    /// Returns the independently observed runtime readiness.
    pub const fn runtime_readiness(&self) -> RuntimeReadiness {
        self.runtime_readiness
    }

    #[must_use]
    /// Returns the support authority for the configured route.
    pub const fn support_authority(&self) -> SupportAuthority {
        self.support_authority
    }

    #[must_use]
    /// Returns the safe provenance of the access evidence.
    pub const fn provenance(&self) -> &AccessEvidenceProvenance {
        &self.provenance
    }

    pub(super) fn permits_selection(&self) -> bool {
        matches!(
            self.credential_state,
            CredentialState::Ready | CredentialState::NotRequired
        ) && self.entitlement_state == EntitlementState::Available
            && self.endpoint_authorization == EndpointAuthorization::Allowed
            && self.runtime_readiness == RuntimeReadiness::Ready
            && self.support_authority != SupportAuthority::Prohibited
    }
}
