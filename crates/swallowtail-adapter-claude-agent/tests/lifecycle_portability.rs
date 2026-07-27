#[path = "lifecycle_portability_support/mod.rs"]
mod lifecycle_portability;

use lifecycle_portability::{host, server};
use serde_json::json;
use swallowtail_core::{ExecutionHostId, RemoteAcpTransport};
use swallowtail_protocol_acp::Message;
use swallowtail_runtime::{AuthorizedEndpoint, CleanupOutcome, EndpointRef, NetworkGrant, ScopeId};
use swallowtail_testkit::RemoteAcpPreflightFixture;
use swallowtail_transport_acp_remote::{
    RemoteAcpClient, RemoteAcpConnectRequest, RemoteAcpErrorKind,
};

#[tokio::test(flavor = "current_thread")]
async fn qualified_delete_record_crosses_explicit_remote_transport_under_both_topologies() {
    for host_value in ["fixture.host.local", "fixture.host.remote-authoritative"] {
        let (endpoint, server, evidence) = server::spawn(false).await;
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let mut connection = connect(host_id, &endpoint).await;
        initialize(&mut connection).await;
        connection.send(delete()).await.expect("delete dispatches");
        assert!(matches!(
            connection.next_event().await,
            Some(Ok(Message::Response {
                id,
                result: Ok(result),
            })) if id == json!(2) && result.as_object().is_some_and(serde_json::Map::is_empty)
        ));
        assert_eq!(connection.close().await, CleanupOutcome::Clean);
        server.await.expect("server joins");
        assert_eq!(
            evidence.methods(),
            vec!["initialize".to_owned(), "session/delete".to_owned()]
        );
        assert_eq!(evidence.connections(), 1);
    }
}

#[tokio::test(flavor = "current_thread")]
async fn remote_disconnect_after_delete_has_no_stdio_retry_or_transport_fallback() {
    let (endpoint, server, evidence) = server::spawn(true).await;
    let host_id = ExecutionHostId::new("fixture.host.remote-only").expect("valid host");
    let services = host::services(host_id.clone());
    assert!(services.process().is_none());
    let mut connection = connect_with_services(host_id, &endpoint, services).await;
    initialize(&mut connection).await;
    connection.send(delete()).await.expect("delete dispatches");
    let failure = connection
        .next_event()
        .await
        .expect("disconnect is reported")
        .expect_err("disconnect invalidates the remote connection");
    assert_eq!(failure.kind(), RemoteAcpErrorKind::TransportFailed);
    let _ = connection.close().await;
    server.await.expect("server joins");
    assert_eq!(evidence.connections(), 1);
    assert_eq!(
        evidence.methods(),
        vec!["initialize".to_owned(), "session/delete".to_owned()]
    );
}

async fn connect(
    host_id: ExecutionHostId,
    endpoint: &str,
) -> swallowtail_transport_acp_remote::RemoteAcpConnection {
    let services = host::services(host_id.clone());
    connect_with_services(host_id, endpoint, services).await
}

async fn connect_with_services(
    host_id: ExecutionHostId,
    endpoint: &str,
    services: swallowtail_runtime::HostServices,
) -> swallowtail_transport_acp_remote::RemoteAcpConnection {
    let fixture = RemoteAcpPreflightFixture::new(RemoteAcpTransport::WebSocket, host_id);
    let plan = fixture.preflight().expect("remote ACP preflight is valid");
    let remote = plan
        .requirements()
        .remote_acp()
        .expect("remote ACP requirements remain explicit");
    assert_eq!(remote.transport(), RemoteAcpTransport::WebSocket);
    assert!(!remote.permits_retry());
    assert!(!remote.permits_reconnect());
    assert!(!remote.permits_transport_fallback());
    let scope = ScopeId::new("fixture.claude-lifecycle-remote").expect("valid scope");
    let endpoint_ref = EndpointRef::new("fixture.claude-lifecycle-endpoint").expect("valid ref");
    let grant = NetworkGrant::new(
        scope.clone(),
        endpoint_ref.clone(),
        plan.endpoint_audience().clone(),
        AuthorizedEndpoint::new(endpoint).expect("valid endpoint"),
    );
    RemoteAcpClient
        .connect(
            &plan,
            RemoteAcpConnectRequest::new(scope, endpoint_ref),
            grant,
            services,
        )
        .await
        .expect("remote ACP connects")
}

async fn initialize(connection: &mut swallowtail_transport_acp_remote::RemoteAcpConnection) {
    connection
        .send(Message::Request {
            id: json!(1),
            method: "initialize".to_owned(),
            params: json!({"protocolVersion": 1}),
        })
        .await
        .expect("initialize dispatches");
    assert!(matches!(
        connection.next_event().await,
        Some(Ok(Message::Response {
            id,
            result: Ok(result),
        })) if id == json!(1)
            && result["protocolVersion"] == json!(1)
            && result["agentInfo"]["version"] == json!("0.61.0")
            && result["agentCapabilities"]["sessionCapabilities"]["close"].is_object()
            && result["agentCapabilities"]["sessionCapabilities"]["delete"].is_object()
    ));
}

fn delete() -> Message {
    Message::Request {
        id: json!(2),
        method: "session/delete".to_owned(),
        params: json!({"sessionId": "opaque-claude-session"}),
    }
}
