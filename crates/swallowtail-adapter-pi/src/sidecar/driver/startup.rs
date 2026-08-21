use crate::failure::failure;
use crate::sidecar::connection::SidecarConnection;
use crate::sidecar::selection::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_WIRE_AXIS,
};
use crate::sidecar::wire::PiSdkSidecarCommand;
use crate::sidecar::{PI_SDK_SIDECAR_BEHAVIOR, PI_SDK_SIDECAR_SDK_PACKAGE, PI_SDK_SIDECAR_WIRE};
use serde_json::{Value, json};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::RuntimeFailure;

const EXPECTED_TOOLS: [&str; 4] = ["read", "grep", "find", "ls"];

pub(super) async fn configure(
    connection: &SidecarConnection,
    plan: &PreflightPlan,
    leased_cwd: &str,
) -> Result<(), RuntimeFailure> {
    let provider = plan
        .provider_id()
        .expect("validated sidecar provider")
        .as_str();
    let model = plan.model_id().expect("validated sidecar model").as_str();
    let sdk_version = bound_version(plan, PI_SDK_SIDECAR_PACKAGE_AXIS);
    let node_version = bound_version(plan, PI_SDK_SIDECAR_NODE_AXIS);
    let wire_version = bound_version(plan, PI_SDK_SIDECAR_WIRE_AXIS);
    let response = connection
        .command(
            "bootstrap-1".to_owned(),
            PiSdkSidecarCommand::Bootstrap,
            json!({"cwd": leased_cwd, "provider": provider, "model": model}),
        )
        .await?;
    if !response.success {
        return Err(failure(
            "swallowtail.pi.sdk-sidecar.startup_rejected",
            "Pi SDK sidecar rejected its restrictive bootstrap",
        ));
    }
    if !bootstrap_matches(
        response.data.as_ref(),
        leased_cwd,
        provider,
        model,
        &sdk_version,
        &node_version,
        &wire_version,
    ) {
        return Err(failure(
            "swallowtail.pi.sdk-sidecar.bootstrap_mismatch",
            "Pi SDK sidecar bootstrap identity did not match the preflight-bound runtime, wire, resource, provider, or model",
        ));
    }
    let state = connection
        .command("state-1".to_owned(), PiSdkSidecarCommand::State, json!({}))
        .await?;
    if !state.success || !state_matches(state.data.as_ref(), leased_cwd, provider, model) {
        return Err(failure(
            "swallowtail.pi.sdk-sidecar.state_mismatch",
            "Pi SDK sidecar state did not match the preflight-bound provider, model, and resource",
        ));
    }
    Ok(())
}

fn bound_version(plan: &PreflightPlan, axis: &str) -> String {
    plan.interface_versions()
        .find(|binding| binding.axis().as_str() == axis)
        .expect("validated sidecar plan binds every axis")
        .version()
        .as_str()
        .to_owned()
}

fn bootstrap_matches(
    data: Option<&Value>,
    cwd: &str,
    provider: &str,
    model: &str,
    sdk_version: &str,
    node_version: &str,
    wire_version: &str,
) -> bool {
    let Some(data) = data else {
        return false;
    };
    wire_version == PI_SDK_SIDECAR_WIRE
        && data.get("wire").and_then(Value::as_str) == Some(PI_SDK_SIDECAR_WIRE)
        && data.get("behavior").and_then(Value::as_str) == Some(PI_SDK_SIDECAR_BEHAVIOR)
        && data.get("sdkPackage").and_then(Value::as_str) == Some(PI_SDK_SIDECAR_SDK_PACKAGE)
        && data.get("sdkVersion").and_then(Value::as_str) == Some(sdk_version)
        && data.get("nodeVersion").and_then(Value::as_str) == Some(node_version)
        && data.get("cwd").and_then(Value::as_str) == Some(cwd)
        && data.get("provider").and_then(Value::as_str) == Some(provider)
        && data.get("model").and_then(Value::as_str) == Some(model)
        && data.get("idle").and_then(Value::as_bool) == Some(true)
        && data.get("streaming").and_then(Value::as_bool) == Some(false)
        && tools_match(data)
}

fn state_matches(data: Option<&Value>, cwd: &str, provider: &str, model: &str) -> bool {
    let Some(data) = data else {
        return false;
    };
    data.get("cwd").and_then(Value::as_str) == Some(cwd)
        && data.get("provider").and_then(Value::as_str) == Some(provider)
        && data.get("model").and_then(Value::as_str) == Some(model)
        && data.get("idle").and_then(Value::as_bool) == Some(true)
        && data.get("streaming").and_then(Value::as_bool) == Some(false)
        && tools_match(data)
}

fn tools_match(data: &Value) -> bool {
    data.get("tools")
        .and_then(Value::as_array)
        .is_some_and(|tools| {
            tools.len() == EXPECTED_TOOLS.len()
                && tools
                    .iter()
                    .zip(EXPECTED_TOOLS)
                    .all(|(tool, expected)| tool.as_str() == Some(expected))
        })
}
