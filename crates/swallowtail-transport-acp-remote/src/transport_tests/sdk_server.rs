use agent_client_protocol::Channel;
use agent_client_protocol_http::AcpHttpServer;
use axum::body::Body;
use http::{Request, StatusCode};
use tower::ServiceExt;

#[test]
fn maintained_server_matches_raw_http_boundary_cases() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let router = AcpHttpServer::new(|| Channel::duplex().0).into_router();

            let health = router
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/health")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(health.status(), StatusCode::OK);

            let missing_connection = router
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/acp")
                        .header("accept", "text/event-stream")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(missing_connection.status(), StatusCode::BAD_REQUEST);

            let wrong_content_type = router
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/acp")
                        .header("content-type", "text/plain")
                        .body(Body::from(
                            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
                        ))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(
                wrong_content_type.status(),
                StatusCode::UNSUPPORTED_MEDIA_TYPE
            );
        });
}
