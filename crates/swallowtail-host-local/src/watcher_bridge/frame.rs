use super::protocol::{
    DecodedRequest, correlation_id, decode_request, decoded_request_id, dispatch,
    error_http_status, error_message, jsonrpc_error,
};
use super::state::LiveLease;
use crate::operation_bridge::{OperationBridgeFrame, OperationBridgeResponse};
use std::sync::Arc;

pub(super) fn handle_frame(
    live: &Arc<LiveLease>,
    frame: OperationBridgeFrame,
) -> OperationBridgeResponse {
    if live.admit_connection().is_err() {
        return OperationBridgeResponse::close(
            429,
            "Too Many Requests",
            jsonrpc_error(None, -32004, "Watcher bridge connection limit reached").into_bytes(),
        );
    }
    let _guard = ConnectionGuard(live);
    let decoded = match decode_request(&frame.body) {
        Ok(decoded) => decoded,
        Err(error) => {
            return watcher_failure(&error, super::protocol::recoverable_request_id(&frame.body));
        }
    };
    let request_id = decoded_request_id(&decoded);
    let admitted = match request_correlation(&decoded) {
        Ok(Some(id)) => match live.admit_request(&id) {
            Ok(()) => true,
            Err(error) => return watcher_failure(&error, request_id),
        },
        Ok(None) => false,
        Err(error) => return watcher_failure(&error, request_id),
    };
    let dispatched = dispatch(live, decoded);
    if admitted {
        live.release_request();
    }
    match dispatched {
        Ok(Some(body)) => OperationBridgeResponse::close(200, "OK", body.into_bytes()),
        Ok(None) => OperationBridgeResponse::close(202, "Accepted", b"{}".to_vec()),
        Err(error) => watcher_failure(&error, request_id),
    }
}

fn request_correlation(
    decoded: &DecodedRequest,
) -> Result<Option<String>, swallowtail_runtime::RuntimeFailure> {
    match decoded {
        DecodedRequest::Initialized => Ok(None),
        DecodedRequest::Initialize { id }
        | DecodedRequest::ToolsList { id }
        | DecodedRequest::ToolsCall { id, .. } => correlation_id(id).map(Some),
    }
}

fn watcher_failure(
    error: &swallowtail_runtime::RuntimeFailure,
    id: Option<serde_json::Value>,
) -> OperationBridgeResponse {
    let (status, reason, code) = error_http_status(error);
    OperationBridgeResponse::close(
        status,
        reason,
        jsonrpc_error(id, code, error_message(error)).into_bytes(),
    )
}

struct ConnectionGuard<'a>(&'a Arc<LiveLease>);

impl Drop for ConnectionGuard<'_> {
    fn drop(&mut self) {
        self.0.release_connection();
    }
}
