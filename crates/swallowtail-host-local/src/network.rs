use crate::host::LocalProcessHost;
use crate::output::failure;
use swallowtail_core::EndpointAudience;
use swallowtail_runtime::{
    AuthorizedEndpoint, BoxFuture, EndpointRef, NetworkGrant, NetworkPolicyService, RuntimeFailure,
    ScopeId,
};

impl NetworkPolicyService for LocalProcessHost {
    fn authorize(
        &self,
        scope: ScopeId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
        let result = if let Some(approved) = self.approvals.endpoints.get(&endpoint) {
            (|| {
                if approved.audience != audience {
                    Err(failure(
                        "swallowtail.local_network.audience_mismatch",
                        "Local endpoint is approved for a different audience",
                    ))
                } else {
                    let authorized =
                        AuthorizedEndpoint::new(approved.value.clone()).map_err(|_| {
                            failure(
                                "swallowtail.local_network.endpoint_invalid",
                                "Local endpoint approval is invalid",
                            )
                        })?;
                    Ok(NetworkGrant::new(scope, endpoint, audience, authorized))
                }
            })()
        } else {
            self.serving_endpoints
                .authorize(&scope, &endpoint, &audience)
                .and_then(|grant| grant.ok_or_else(endpoint_not_approved))
        };
        Box::pin(async move { result })
    }
}

fn endpoint_not_approved() -> RuntimeFailure {
    failure(
        "swallowtail.local_network.endpoint_not_approved",
        "Local endpoint reference is not approved",
    )
}
