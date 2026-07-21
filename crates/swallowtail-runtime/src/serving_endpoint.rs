use crate::input::required_text;
use crate::{BoxFuture, CleanupOutcome, EndpointRef, InputValueRequired, RuntimeFailure, ScopeId};
use std::fmt;
use swallowtail_core::{EndpointAudience, ExecutionHostId};

/// Endpoint value observed from an owned child. It is not authorized by parsing alone.
pub struct ObservedServingEndpoint(String);

impl ObservedServingEndpoint {
    pub fn new(value: impl Into<String>) -> Result<Self, InputValueRequired> {
        required_text("observed serving endpoint", value).map(Self)
    }

    /// Passes the observed value only to the execution-host publication port.
    #[must_use]
    pub fn as_driver_value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ObservedServingEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ObservedServingEndpoint(<opaque>)")
    }
}

/// Safe host-scoped handoff for an endpoint owned by one serving handle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServingEndpointBinding {
    scope: ScopeId,
    execution_host_id: ExecutionHostId,
    endpoint: EndpointRef,
    audience: EndpointAudience,
}

impl ServingEndpointBinding {
    #[must_use]
    pub const fn new(
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
    ) -> Self {
        Self {
            scope,
            execution_host_id,
            endpoint,
            audience,
        }
    }

    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn endpoint(&self) -> &EndpointRef {
        &self.endpoint
    }

    #[must_use]
    pub const fn audience(&self) -> &EndpointAudience {
        &self.audience
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ServingEndpointLease {
    binding: ServingEndpointBinding,
}

impl ServingEndpointLease {
    #[must_use]
    pub const fn new(binding: ServingEndpointBinding) -> Self {
        Self { binding }
    }

    #[must_use]
    pub const fn binding(&self) -> &ServingEndpointBinding {
        &self.binding
    }
}

pub trait ServingEndpointService: Send + Sync {
    fn publish(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        audience: EndpointAudience,
        observed: ObservedServingEndpoint,
    ) -> BoxFuture<'static, Result<ServingEndpointLease, RuntimeFailure>>;

    fn release(&self, lease: ServingEndpointLease) -> BoxFuture<'static, CleanupOutcome>;
}

#[cfg(test)]
mod tests {
    use super::{ObservedServingEndpoint, ServingEndpointBinding};
    use crate::{EndpointRef, ScopeId};
    use swallowtail_core::{EndpointAudience, ExecutionHostId};

    #[test]
    fn binding_is_host_scoped_and_observation_is_redacted() {
        let raw = "http://127.0.0.1:49152";
        let observed = ObservedServingEndpoint::new(raw).expect("observation is valid");
        let binding = ServingEndpointBinding::new(
            ScopeId::new("serving-scope").expect("scope is valid"),
            ExecutionHostId::new("host.local").expect("host is valid"),
            EndpointRef::new("serving-endpoint-1").expect("endpoint is valid"),
            EndpointAudience::new("llama-local").expect("audience is valid"),
        );

        assert_eq!(observed.as_driver_value(), raw);
        assert!(!format!("{observed:?}").contains(raw));
        assert_eq!(binding.execution_host_id().as_str(), "host.local");
    }
}
