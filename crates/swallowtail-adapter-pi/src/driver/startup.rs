use crate::connection::PiConnection;
use crate::failure::failure;
use serde_json::{Value, json};

pub(super) async fn configure(
    connection: &PiConnection,
    provider: &str,
    model: &str,
) -> Result<(), swallowtail_runtime::RuntimeFailure> {
    for (id, command, value) in [
        (
            "setup-retry",
            "set_auto_retry",
            json!({"id": "setup-retry", "type": "set_auto_retry", "enabled": false}),
        ),
        (
            "setup-compaction",
            "set_auto_compaction",
            json!({"id": "setup-compaction", "type": "set_auto_compaction", "enabled": false}),
        ),
        (
            "setup-steering",
            "set_steering_mode",
            json!({"id": "setup-steering", "type": "set_steering_mode", "mode": "one-at-a-time"}),
        ),
        (
            "setup-follow-up",
            "set_follow_up_mode",
            json!({"id": "setup-follow-up", "type": "set_follow_up_mode", "mode": "one-at-a-time"}),
        ),
    ] {
        let response = connection.command(id.to_owned(), command, value).await?;
        if !response.success {
            return Err(startup_rejected());
        }
    }
    let state = connection
        .command(
            "state-1".to_owned(),
            "get_state",
            json!({"id": "state-1", "type": "get_state"}),
        )
        .await?;
    if !state.success || !state_matches(state.data.as_ref(), provider, model) {
        return Err(failure(
            "swallowtail.pi.rpc.state_mismatch",
            "Pi RPC state did not match the preflight-bound provider, model, and policy",
        ));
    }
    Ok(())
}

fn state_matches(data: Option<&Value>, provider: &str, model: &str) -> bool {
    let Some(data) = data else {
        return false;
    };
    data.pointer("/model/provider").and_then(Value::as_str) == Some(provider)
        && data.pointer("/model/id").and_then(Value::as_str) == Some(model)
        && data.get("isStreaming").and_then(Value::as_bool) == Some(false)
        && data.get("isCompacting").and_then(Value::as_bool) == Some(false)
        && data.get("steeringMode").and_then(Value::as_str) == Some("one-at-a-time")
        && data.get("followUpMode").and_then(Value::as_str) == Some("one-at-a-time")
        && data.get("autoCompactionEnabled").and_then(Value::as_bool) == Some(false)
        && data.get("pendingMessageCount").and_then(Value::as_u64) == Some(0)
}

fn startup_rejected() -> swallowtail_runtime::RuntimeFailure {
    failure(
        "swallowtail.pi.rpc.startup_rejected",
        "Pi RPC rejected its restrictive startup configuration",
    )
}
