use crate::host::LocalProcessHost;
use crate::hosted::LocalEndpointApproval;
use crate::output::failure;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use swallowtail_core::{EndpointAudience, ExecutionHostId};
use swallowtail_runtime::{
    AuthorizedEndpoint, BoxFuture, CleanupOutcome, EndpointRef, NetworkGrant,
    ObservedServingEndpoint, RuntimeFailure, ScopeId, ServingEndpointBinding, ServingEndpointLease,
    ServingEndpointService,
};

struct PublishedEndpoint {
    scope: ScopeId,
    execution_host_id: ExecutionHostId,
    audience: EndpointAudience,
    value: String,
}

#[derive(Default)]
pub(crate) struct LocalServingEndpointState {
    sequence: AtomicU64,
    published: Mutex<HashMap<EndpointRef, PublishedEndpoint>>,
}

impl LocalServingEndpointState {
    fn publish(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        audience: EndpointAudience,
        value: String,
        reserved: &HashMap<EndpointRef, LocalEndpointApproval>,
    ) -> ServingEndpointLease {
        let mut published = self
            .published
            .lock()
            .expect("local serving-endpoint lock poisoned");
        let reference = loop {
            let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
            let candidate = EndpointRef::new(format!(
                "local.owned-serving-endpoint.{}.{}",
                std::process::id(),
                sequence
            ))
            .expect("generated endpoint reference is valid");
            if !reserved.contains_key(&candidate) && !published.contains_key(&candidate) {
                break candidate;
            }
        };
        let binding = ServingEndpointBinding::new(
            scope.clone(),
            execution_host_id.clone(),
            reference.clone(),
            audience.clone(),
        );
        published.insert(
            reference,
            PublishedEndpoint {
                scope,
                execution_host_id,
                audience,
                value,
            },
        );
        ServingEndpointLease::new(binding)
    }

    pub(crate) fn authorize(
        &self,
        scope: &ScopeId,
        endpoint: &EndpointRef,
        audience: &EndpointAudience,
    ) -> Result<Option<NetworkGrant>, RuntimeFailure> {
        let published = self
            .published
            .lock()
            .expect("local serving-endpoint lock poisoned");
        let Some(entry) = published.get(endpoint) else {
            return Ok(None);
        };
        if entry.scope != *scope || entry.audience != *audience {
            return Err(failure(
                "swallowtail.local_serving_endpoint.binding_mismatch",
                "Published serving endpoint has a different scope or audience",
            ));
        }
        let authorized = AuthorizedEndpoint::new(entry.value.clone()).map_err(|_| {
            failure(
                "swallowtail.local_serving_endpoint.endpoint_invalid",
                "Published serving endpoint is invalid",
            )
        })?;
        Ok(Some(NetworkGrant::new(
            scope.clone(),
            endpoint.clone(),
            audience.clone(),
            authorized,
        )))
    }

    fn release(&self, binding: &ServingEndpointBinding) -> bool {
        let mut published = self
            .published
            .lock()
            .expect("local serving-endpoint lock poisoned");
        let owned = published.get(binding.endpoint()).is_some_and(|entry| {
            entry.scope == *binding.scope()
                && entry.execution_host_id == *binding.execution_host_id()
                && entry.audience == *binding.audience()
        });
        if owned {
            published.remove(binding.endpoint());
        }
        owned
    }
}

impl ServingEndpointService for LocalProcessHost {
    fn publish(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        audience: EndpointAudience,
        observed: ObservedServingEndpoint,
    ) -> BoxFuture<'static, Result<ServingEndpointLease, RuntimeFailure>> {
        let result = self
            .require_bound_execution_host(&execution_host_id)
            .and_then(|()| validate_loopback_http(observed.as_driver_value()))
            .map(|value| {
                self.serving_endpoints.publish(
                    scope,
                    execution_host_id,
                    audience,
                    value,
                    &self.approvals.endpoints,
                )
            });
        Box::pin(async move { result })
    }

    fn release(&self, lease: ServingEndpointLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = if self.serving_endpoints.release(lease.binding()) {
            CleanupOutcome::Clean
        } else {
            CleanupOutcome::Failed(
                failure(
                    "swallowtail.local_serving_endpoint.lease_not_owned",
                    "Serving endpoint lease is not owned by this local host",
                )
                .diagnostic()
                .clone(),
            )
        };
        Box::pin(async move { outcome })
    }
}

fn validate_loopback_http(value: &str) -> Result<String, RuntimeFailure> {
    let authority = value.strip_prefix("http://").ok_or_else(|| {
        failure(
            "swallowtail.local_serving_endpoint.scheme_not_approved",
            "Owned serving endpoint must use plain HTTP on the execution host",
        )
    })?;
    let address = authority.parse::<SocketAddr>().map_err(|_| {
        failure(
            "swallowtail.local_serving_endpoint.address_invalid",
            "Owned serving endpoint must be an exact socket address",
        )
    })?;
    if !address.ip().is_loopback() || address.port() == 0 {
        return Err(failure(
            "swallowtail.local_serving_endpoint.not_loopback",
            "Owned serving endpoint must use a nonzero loopback socket",
        ));
    }
    Ok(value.to_owned())
}
