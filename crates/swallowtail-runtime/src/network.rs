use crate::input::required_text;
use crate::{BoxFuture, EndpointRef, InputValueRequired, RuntimeFailure, ScopeId};
use std::fmt;
use swallowtail_core::EndpointAudience;

/// Redacted endpoint value approved by the execution host for one transport.
pub struct AuthorizedEndpoint(String);

impl AuthorizedEndpoint {
    /// Creates a nonempty authorized endpoint value.
    pub fn new(value: impl Into<String>) -> Result<Self, InputValueRequired> {
        required_text("authorized endpoint", value).map(Self)
    }

    /// Exposes the exact host-approved endpoint only to the transport driver.
    #[must_use]
    pub fn as_driver_value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AuthorizedEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AuthorizedEndpoint(<opaque>)")
    }
}

impl fmt::Display for AuthorizedEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<authorized endpoint>")
    }
}

#[derive(Debug)]
/// Operation-scoped transport authority for one endpoint and audience.
pub struct NetworkGrant {
    scope: ScopeId,
    endpoint: EndpointRef,
    audience: EndpointAudience,
    authorized: AuthorizedEndpoint,
}

impl NetworkGrant {
    #[must_use]
    /// Creates a grant from exact scope, endpoint, audience, and resolved value.
    pub const fn new(
        scope: ScopeId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
        authorized: AuthorizedEndpoint,
    ) -> Self {
        Self {
            scope,
            endpoint,
            audience,
            authorized,
        }
    }

    #[must_use]
    /// Returns the operation scope that owns the grant.
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    /// Returns the opaque endpoint reference approved by the host.
    pub const fn endpoint(&self) -> &EndpointRef {
        &self.endpoint
    }

    #[must_use]
    /// Returns the exact endpoint audience.
    pub const fn audience(&self) -> &EndpointAudience {
        &self.audience
    }

    #[must_use]
    /// Returns the redacted, driver-usable endpoint value.
    pub const fn authorized(&self) -> &AuthorizedEndpoint {
        &self.authorized
    }
}

/// Execution-host boundary for operation-scoped transport authorization.
///
/// A network grant does not enable provider-side tools or external search.
pub trait NetworkPolicyService: Send + Sync {
    /// Authorizes one opaque endpoint reference for an exact scope and audience.
    fn authorize(
        &self,
        scope: ScopeId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>>;
}

#[cfg(test)]
mod tests {
    use super::{AuthorizedEndpoint, NetworkGrant};
    use crate::{EndpointRef, ScopeId};
    use swallowtail_core::EndpointAudience;

    #[test]
    fn grant_binds_scope_audience_and_redacts_endpoint() {
        let raw = "https://private.example.test/v1";
        let grant = NetworkGrant::new(
            ScopeId::new("scope-1").expect("scope is valid"),
            EndpointRef::new("endpoint-1").expect("endpoint is valid"),
            EndpointAudience::new("api-1").expect("audience is valid"),
            AuthorizedEndpoint::new(raw).expect("endpoint is valid"),
        );

        assert_eq!(grant.authorized().as_driver_value(), raw);
        assert!(!format!("{grant:?}").contains(raw));
    }

    #[test]
    fn authorized_endpoint_rejects_blank_values() {
        let error = AuthorizedEndpoint::new("  ").expect_err("blank endpoint must fail");
        assert_eq!(error.field(), "authorized endpoint");
    }
}
