use crate::{ExtensionNamespace, SafeDiagnostic};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceAccess {
    Read,
    ReadWrite,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceRepresentation {
    Stream,
    BoundedBytes,
    TemporaryFile,
    Filesystem,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FilesystemBoundary {
    WorkingResource,
}

/// How a local harness process is isolated from execution-host resources.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HarnessIsolation {
    AmbientHost,
    ProviderEnforced,
    HostEnforced,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderApprovalPolicy {
    Never,
}

/// Permission for provider-side access beyond the selected provider route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExternalNetworkPolicy {
    Denied,
    HostApproved,
    AmbientHost,
}

/// Whether the operation may ask the provider or harness to search externally.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExternalSearchPolicy {
    Disabled,
    Enabled,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderRequestHandling {
    Reject,
    ObserveAndStop,
}

/// Declares the provider extensions which may be observed without granting authority.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderRequestPolicy {
    observe_and_stop: BTreeSet<ExtensionNamespace>,
}

impl ProviderRequestPolicy {
    #[must_use]
    pub fn reject_all() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn observe_and_stop(namespaces: impl IntoIterator<Item = ExtensionNamespace>) -> Self {
        Self {
            observe_and_stop: namespaces.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn handling_for(&self, namespace: &ExtensionNamespace) -> ProviderRequestHandling {
        if self.observe_and_stop.contains(namespace) {
            ProviderRequestHandling::ObserveAndStop
        } else {
            ProviderRequestHandling::Reject
        }
    }

    pub fn observed_extensions(&self) -> impl ExactSizeIterator<Item = &ExtensionNamespace> {
        self.observe_and_stop.iter()
    }
}

/// Expanded access policy for one interactive session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionAccessPolicy {
    resource_access: Option<ResourceAccess>,
    filesystem_boundary: Option<FilesystemBoundary>,
    harness_isolation: Option<HarnessIsolation>,
    approval_policy: ProviderApprovalPolicy,
    external_network: ExternalNetworkPolicy,
    external_search: ExternalSearchPolicy,
    provider_requests: ProviderRequestPolicy,
}

impl SessionAccessPolicy {
    pub fn new(
        resource_access: ResourceAccess,
        filesystem_boundary: FilesystemBoundary,
        harness_isolation: HarnessIsolation,
        approval_policy: ProviderApprovalPolicy,
        external_network: ExternalNetworkPolicy,
        external_search: ExternalSearchPolicy,
        provider_requests: ProviderRequestPolicy,
    ) -> Result<Self, IncompatibleSessionAccessPolicy> {
        if external_search == ExternalSearchPolicy::Enabled
            && external_network == ExternalNetworkPolicy::Denied
        {
            return Err(IncompatibleSessionAccessPolicy::search_requires_network());
        }
        if harness_isolation == HarnessIsolation::AmbientHost {
            return Err(IncompatibleSessionAccessPolicy::ambient_boundary());
        }
        Ok(Self {
            resource_access: Some(resource_access),
            filesystem_boundary: Some(filesystem_boundary),
            harness_isolation: Some(harness_isolation),
            approval_policy,
            external_network,
            external_search,
            provider_requests,
        })
    }

    #[must_use]
    pub fn read_only() -> Self {
        Self::new(
            ResourceAccess::Read,
            FilesystemBoundary::WorkingResource,
            HarnessIsolation::ProviderEnforced,
            ProviderApprovalPolicy::Never,
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
            ProviderRequestPolicy::reject_all(),
        )
        .expect("read-only session access policy is internally valid")
    }

    /// A session with no filesystem or working-resource authority.
    #[must_use]
    pub fn resource_free() -> Self {
        Self {
            resource_access: None,
            filesystem_boundary: None,
            harness_isolation: None,
            approval_policy: ProviderApprovalPolicy::Never,
            external_network: ExternalNetworkPolicy::Denied,
            external_search: ExternalSearchPolicy::Disabled,
            provider_requests: ProviderRequestPolicy::reject_all(),
        }
    }

    /// A local harness with an intended working resource but no filesystem or
    /// process-isolation claim.
    #[must_use]
    pub fn ambient_harness(resource_access: ResourceAccess) -> Self {
        Self {
            resource_access: Some(resource_access),
            filesystem_boundary: None,
            harness_isolation: Some(HarnessIsolation::AmbientHost),
            approval_policy: ProviderApprovalPolicy::Never,
            external_network: ExternalNetworkPolicy::AmbientHost,
            external_search: ExternalSearchPolicy::Disabled,
            provider_requests: ProviderRequestPolicy::reject_all(),
        }
    }

    #[must_use]
    pub fn bounded_workspace(
        observed_provider_requests: impl IntoIterator<Item = ExtensionNamespace>,
    ) -> Self {
        Self::new(
            ResourceAccess::ReadWrite,
            FilesystemBoundary::WorkingResource,
            HarnessIsolation::ProviderEnforced,
            ProviderApprovalPolicy::Never,
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
            ProviderRequestPolicy::observe_and_stop(observed_provider_requests),
        )
        .expect("bounded workspace session access policy is internally valid")
    }

    #[must_use]
    pub const fn resource_access(&self) -> Option<ResourceAccess> {
        self.resource_access
    }

    #[must_use]
    pub const fn filesystem_boundary(&self) -> Option<FilesystemBoundary> {
        self.filesystem_boundary
    }

    #[must_use]
    pub const fn harness_isolation(&self) -> Option<HarnessIsolation> {
        self.harness_isolation
    }

    #[must_use]
    pub const fn approval_policy(&self) -> ProviderApprovalPolicy {
        self.approval_policy
    }

    #[must_use]
    pub const fn external_network(&self) -> ExternalNetworkPolicy {
        self.external_network
    }

    #[must_use]
    pub const fn external_search(&self) -> ExternalSearchPolicy {
        self.external_search
    }

    #[must_use]
    pub const fn provider_requests(&self) -> &ProviderRequestPolicy {
        &self.provider_requests
    }
}

impl Default for SessionAccessPolicy {
    fn default() -> Self {
        Self::ambient_harness(ResourceAccess::Read)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncompatibleSessionAccessPolicy {
    diagnostic: SafeDiagnostic,
}

impl IncompatibleSessionAccessPolicy {
    fn search_requires_network() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.session_access_policy_rejected",
                "External search requires host-approved external network access",
            ),
        }
    }

    fn ambient_boundary() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.session_access_policy_rejected",
                "Ambient harness execution cannot claim a bounded filesystem",
            ),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for IncompatibleSessionAccessPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for IncompatibleSessionAccessPolicy {}

#[cfg(test)]
#[path = "session_access/tests.rs"]
mod tests;
