use crate::connection::OhMyPiConnection;
use crate::failure::failure;
use serde_json::{Value, json};

pub(super) async fn configure(
    connection: &OhMyPiConnection,
    provider: &str,
    model: &str,
    reasoning: Option<&swallowtail_core::ReasoningMode>,
) -> Result<(), swallowtail_runtime::RuntimeFailure> {
    negotiate(connection).await?;
    select_model(connection, provider, model).await?;
    if let Some(reasoning) = reasoning {
        let response = connection
            .command(
                "setup-thinking".to_owned(),
                "set_thinking_level",
                json!({"id": "setup-thinking", "type": "set_thinking_level", "level": reasoning.as_str()}),
            )
            .await?;
        if !response.success {
            return Err(startup_rejected());
        }
    }
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
        (
            "setup-interrupt",
            "set_interrupt_mode",
            json!({"id": "setup-interrupt", "type": "set_interrupt_mode", "mode": "wait"}),
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
    if !state.success || !state_matches(state.data.as_ref(), provider, model, reasoning) {
        return Err(failure(
            "swallowtail.oh_my_pi.rpc.state_mismatch",
            "OhMyPi RPC state did not match the preflight-bound provider, model, and policy",
        ));
    }
    Ok(())
}

pub(super) async fn negotiate(
    connection: &OhMyPiConnection,
) -> Result<(), swallowtail_runtime::RuntimeFailure> {
    let response = connection
        .command(
            "protocol-2".to_owned(),
            "negotiate_protocol",
            json!({"id": "protocol-2", "type": "negotiate_protocol", "protocolVersion": 2}),
        )
        .await?;
    if response.success
        && response
            .data
            .as_ref()
            .and_then(|data| data.get("protocolVersion"))
            .and_then(Value::as_u64)
            == Some(2)
    {
        Ok(())
    } else {
        Err(startup_rejected())
    }
}

async fn select_model(
    connection: &OhMyPiConnection,
    provider: &str,
    model: &str,
) -> Result<(), swallowtail_runtime::RuntimeFailure> {
    let response = connection
        .command(
            "setup-model".to_owned(),
            "set_model",
            json!({"id": "setup-model", "type": "set_model", "provider": provider, "modelId": model}),
        )
        .await?;
    if response.success
        && response
            .data
            .as_ref()
            .and_then(|data| data.get("provider"))
            .and_then(Value::as_str)
            == Some(provider)
        && response
            .data
            .as_ref()
            .and_then(|data| data.get("id"))
            .and_then(Value::as_str)
            == Some(model)
    {
        Ok(())
    } else {
        Err(startup_rejected())
    }
}

fn state_matches(
    data: Option<&Value>,
    provider: &str,
    model: &str,
    reasoning: Option<&swallowtail_core::ReasoningMode>,
) -> bool {
    let Some(data) = data else {
        return false;
    };
    data.pointer("/model/provider").and_then(Value::as_str) == Some(provider)
        && data.pointer("/model/id").and_then(Value::as_str) == Some(model)
        && reasoning.is_none_or(|reasoning| {
            data.get("thinkingLevel").and_then(Value::as_str) == Some(reasoning.as_str())
        })
        && data.get("isStreaming").and_then(Value::as_bool) == Some(false)
        && data.get("isCompacting").and_then(Value::as_bool) == Some(false)
        && data.get("steeringMode").and_then(Value::as_str) == Some("one-at-a-time")
        && data.get("followUpMode").and_then(Value::as_str) == Some("one-at-a-time")
        && data.get("interruptMode").and_then(Value::as_str) == Some("wait")
        && data.get("autoCompactionEnabled").and_then(Value::as_bool) == Some(false)
        && data.get("queuedMessageCount").and_then(Value::as_u64) == Some(0)
}

fn startup_rejected() -> swallowtail_runtime::RuntimeFailure {
    failure(
        "swallowtail.oh_my_pi.rpc.startup_rejected",
        "OhMyPi RPC rejected its restrictive startup configuration",
    )
}
