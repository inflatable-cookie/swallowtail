use crate::error::{
    binding_error, capacity_error, endpoint_error, protocol_error, transport_error,
};
use crate::{RemoteAcpConnection, RemoteAcpErrorKind};
use futures_channel::mpsc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{BoxFuture, CleanupOutcome, JoinedTask, RuntimeFailure};

struct FailingJoin;

impl JoinedTask for FailingJoin {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "fixture.join_failed",
                "Fixture join failed",
            )))
        })
    }
}

#[test]
fn cleanup_failure_is_explicit_and_redacted() {
    let (commands, _command_rx) = mpsc::channel(1);
    let (_event_tx, events) = mpsc::channel(1);
    let connection = RemoteAcpConnection {
        commands,
        events,
        connection_task: Some(Box::new(FailingJoin)),
        deadline_task: None,
        deadline_done: None,
    };

    let cleanup = futures_executor::block_on(connection.close());
    let CleanupOutcome::Failed(diagnostic) = cleanup else {
        panic!("failed join must produce failed cleanup");
    };
    assert_eq!(diagnostic.code(), "swallowtail.remote_acp.cleanup_failed");
    assert!(!diagnostic.message().contains("fixture.join_failed"));
}

#[test]
fn stable_errors_never_include_raw_transport_material() {
    let private = "private-endpoint-cookie-frame";
    for error in [
        binding_error(),
        endpoint_error(),
        capacity_error(),
        protocol_error(),
        transport_error(),
    ] {
        assert!(!format!("{error}").contains(private));
        assert!(!format!("{error:?}").contains(private));
        assert!(matches!(
            error.kind(),
            RemoteAcpErrorKind::BindingRejected
                | RemoteAcpErrorKind::EndpointRejected
                | RemoteAcpErrorKind::CapacityExceeded
                | RemoteAcpErrorKind::ProtocolRejected
                | RemoteAcpErrorKind::TransportFailed
        ));
    }
}
