use crate::failure::failure;
use swallowtail_runtime::RuntimeFailure;

pub(super) fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.websocket_endpoint_invalid",
        "Kimi local-server WebSocket endpoint is invalid",
    )
}

pub(super) fn credential_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.websocket_credential_invalid",
        "Kimi local-server WebSocket credential is invalid",
    )
}

pub(super) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.websocket_protocol_failed",
        "Kimi local-server WebSocket protocol failed",
    )
}

pub(in crate::local_server::interactive) fn resync_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.websocket_resync_required",
        "Kimi local-server WebSocket requires explicit resynchronization",
    )
}

pub(super) fn disconnected() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.websocket_disconnected",
        "Kimi local-server WebSocket disconnected before terminal truth",
    )
}

pub(super) fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.websocket_backpressure",
        "Kimi local-server WebSocket delivery exceeded its bounded capacity",
    )
}

pub(super) fn turn_timeout() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.turn_timed_out",
        "Kimi local-server turn timed out",
    )
}
