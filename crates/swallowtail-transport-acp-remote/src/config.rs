use crate::error::{RemoteAcpError, binding_error, endpoint_error};
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{
    CredentialMechanism, PreflightPlan, RemoteAcpConnectionBounds, RemoteAcpTransport,
    SupportAuthority,
};
use swallowtail_runtime::{EndpointRef, NetworkGrant, ScopeId};
use url::Url;

pub(crate) struct TransportConfig {
    pub(crate) endpoint: Url,
    pub(crate) transport: RemoteAcpTransport,
    pub(crate) bounds: RemoteAcpConnectionBounds,
    pub(crate) maximum_cookie_count: NonZeroU32,
    pub(crate) maximum_cookie_bytes: NonZeroU64,
}

impl TransportConfig {
    pub(crate) fn from_bound_grant(
        plan: &PreflightPlan,
        scope: &ScopeId,
        endpoint_ref: &EndpointRef,
        grant: &NetworkGrant,
    ) -> Result<Self, RemoteAcpError> {
        let requirements = plan.requirements().remote_acp().ok_or_else(binding_error)?;
        if plan.credential_mechanism() != &CredentialMechanism::Unauthenticated
            || plan.credential_reference().is_some()
            || !plan
                .requirements()
                .access()
                .accepts_support_authority(SupportAuthority::ExperimentalObserved)
            || grant.scope() != scope
            || grant.endpoint() != endpoint_ref
            || grant.audience() != plan.endpoint_audience()
        {
            return Err(binding_error());
        }
        let endpoint =
            Url::parse(grant.authorized().as_driver_value()).map_err(|_| endpoint_error())?;
        if endpoint.has_authority()
            && endpoint.username().is_empty()
            && endpoint.password().is_none()
            && endpoint.query().is_none()
            && endpoint.fragment().is_none()
        {
            let transport = match endpoint.scheme() {
                "http" | "https" => RemoteAcpTransport::StreamableHttpSse,
                "ws" | "wss" => RemoteAcpTransport::WebSocket,
                _ => return Err(endpoint_error()),
            };
            if transport != requirements.transport() {
                return Err(endpoint_error());
            }
            return Ok(Self {
                endpoint,
                transport,
                bounds: requirements.bounds(),
                maximum_cookie_count: requirements
                    .affinity()
                    .maximum_cookie_count()
                    .expect("validated remote ACP affinity"),
                maximum_cookie_bytes: requirements
                    .affinity()
                    .maximum_cookie_bytes()
                    .expect("validated remote ACP affinity"),
            });
        }
        Err(endpoint_error())
    }
}
