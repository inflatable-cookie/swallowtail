use super::*;

#[test]
fn loopback_publication_is_scope_bound_and_release_invalidates_it() {
    let host_id = host_id();
    let scope = scope();
    let audience = audience();
    let raw = "http://127.0.0.1:49152";
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .bind_execution_host(host_id.clone())
        .build();
    let lease = block_on(host.publish(
        scope.clone(),
        host_id.clone(),
        audience.clone(),
        ObservedServingEndpoint::new(raw).expect("observation is valid"),
    ))
    .expect("loopback endpoint publishes");
    assert_eq!(lease.binding().scope(), &scope);
    assert_eq!(lease.binding().execution_host_id(), &host_id);
    assert_eq!(lease.binding().audience(), &audience);

    let grant = block_on(host.authorize(
        scope.clone(),
        lease.binding().endpoint().clone(),
        audience.clone(),
    ))
    .expect("published endpoint authorizes");
    assert_eq!(grant.authorized().as_driver_value(), raw);
    assert!(!format!("{grant:?}").contains(raw));

    assert_failure_code(
        block_on(host.authorize(
            ScopeId::new("foreign-scope").expect("scope is valid"),
            lease.binding().endpoint().clone(),
            audience.clone(),
        )),
        "swallowtail.local_serving_endpoint.binding_mismatch",
    );
    let endpoint = lease.binding().endpoint().clone();
    assert_eq!(
        block_on(ServingEndpointService::release(&host, lease)),
        CleanupOutcome::Clean
    );
    assert_failure_code(
        block_on(host.authorize(scope, endpoint, audience)),
        "swallowtail.local_network.endpoint_not_approved",
    );
}

#[test]
fn serving_endpoint_rejects_unbound_non_loopback_and_foreign_release() {
    let unbound = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    assert_failure_code(
        block_on(unbound.publish(
            scope(),
            host_id(),
            audience(),
            ObservedServingEndpoint::new("http://127.0.0.1:49152").expect("observation is valid"),
        )),
        "swallowtail.local_host.execution_host_unbound",
    );

    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .bind_execution_host(host_id())
        .build();
    for raw in [
        "https://127.0.0.1:49152",
        "http://192.0.2.1:49152",
        "http://127.0.0.1:49152/v1",
        "http://127.0.0.1:0",
    ] {
        assert!(
            block_on(host.publish(
                scope(),
                host_id(),
                audience(),
                ObservedServingEndpoint::new(raw).expect("observation is valid"),
            ))
            .is_err(),
            "{raw} must fail"
        );
    }

    let foreign = ServingEndpointLease::new(ServingEndpointBinding::new(
        scope(),
        host_id(),
        swallowtail_runtime::EndpointRef::new("foreign-endpoint").expect("endpoint is valid"),
        audience(),
    ));
    assert_failed_cleanup(
        block_on(ServingEndpointService::release(&host, foreign)),
        "swallowtail.local_serving_endpoint.lease_not_owned",
    );
}
