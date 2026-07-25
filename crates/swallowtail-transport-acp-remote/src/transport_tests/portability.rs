use super::{assert_initialize, initialize};
use crate::transport_tests::{host, http, websocket};
use crate::{RemoteAcpClient, RemoteAcpConnectRequest};
use swallowtail_core::RemoteAcpTransport;
use swallowtail_runtime::{AuthorizedEndpoint, CleanupOutcome, EndpointRef, NetworkGrant, ScopeId};
use swallowtail_testkit::{ExecutionTopologyFixture, RemoteAcpPreflightFixture};
use url::Url;

pub(super) async fn public_profile_runs_both_transports_and_topologies() {
    for (topology, label) in [
        (ExecutionTopologyFixture::local(), "local"),
        (
            ExecutionTopologyFixture::remote_authoritative(),
            "remote-authoritative",
        ),
    ] {
        tokio::time::timeout(
            std::time::Duration::from_secs(10),
            run_http(topology.execution_host_id().clone()),
        )
        .await
        .unwrap_or_else(|_| panic!("HTTP public portability cell timed out: {label}"));
        tokio::time::timeout(
            std::time::Duration::from_secs(10),
            run_websocket(topology.execution_host_id().clone()),
        )
        .await
        .unwrap_or_else(|_| panic!("WebSocket public portability cell timed out: {label}"));
    }
}

async fn run_http(execution_host_id: swallowtail_core::ExecutionHostId) {
    let (endpoint, server, evidence) = http::portability_server().await;
    let mut connection = connect(
        RemoteAcpTransport::StreamableHttpSse,
        execution_host_id,
        &endpoint,
    )
    .await;
    connection.send(initialize()).await.unwrap();
    assert_initialize(
        &connection
            .next_event()
            .await
            .expect("HTTP initialize event exists")
            .unwrap(),
    );
    assert_eq!(connection.close().await, CleanupOutcome::Clean);
    assert!(evidence.lock().unwrap().delete_seen);
    server.abort();
    let _ = server.await;
}

async fn run_websocket(execution_host_id: swallowtail_core::ExecutionHostId) {
    let (endpoint, server) = websocket::termination_server(false).await;
    let connection = connect(RemoteAcpTransport::WebSocket, execution_host_id, &endpoint).await;
    assert_eq!(connection.close().await, CleanupOutcome::Clean);
    assert!(server.await.unwrap(), "WebSocket peer observed close");
}

async fn connect(
    transport: RemoteAcpTransport,
    execution_host_id: swallowtail_core::ExecutionHostId,
    endpoint: &Url,
) -> crate::RemoteAcpConnection {
    let fixture = RemoteAcpPreflightFixture::new(transport, execution_host_id.clone());
    let plan = fixture.preflight().unwrap();
    let scope = ScopeId::new("fixture.remote-acp-portability").unwrap();
    let endpoint_ref = EndpointRef::new("fixture.remote-acp-endpoint").unwrap();
    let grant = NetworkGrant::new(
        scope.clone(),
        endpoint_ref.clone(),
        plan.endpoint_audience().clone(),
        AuthorizedEndpoint::new(endpoint.as_str()).unwrap(),
    );
    RemoteAcpClient
        .connect(
            &plan,
            RemoteAcpConnectRequest::new(scope, endpoint_ref),
            grant,
            host::services(execution_host_id),
        )
        .await
        .unwrap()
}
