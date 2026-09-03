use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use crate::sdk::selection::{
    CLAUDE_AGENT_SDK_NATIVE_AXIS, CLAUDE_AGENT_SDK_NODE_AXIS, CLAUDE_AGENT_SDK_PACKAGE_AXIS,
    CLAUDE_AGENT_SDK_WIRE_AXIS,
};
use crate::sdk::wire::ClaudeAgentSdkCommand;
use crate::sdk::{CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_PACKAGE, CLAUDE_AGENT_SDK_WIRE};
use serde_json::{Value, json};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::RuntimeFailure;

/// Read-only tool set. Bash, terminal, and every write tool stay outside this
/// route until Contract 023 process authority and Contract 041 mediation
/// evidence admit them; a capability advertisement is not admission.
pub(crate) const EXPECTED_TOOLS: [&str; 3] = ["Read", "Glob", "Grep"];
const MAXIMUM_CAPABILITIES: usize = 64;
const MAXIMUM_CAPABILITY_BYTES: usize = 96;

/// Runtime-advertised readiness observed at open. Capabilities are the only
/// axis that is runtime behavior rather than declaration, so nothing here is
/// inferred from the shipped SDK declarations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionReadiness {
    capabilities: Vec<String>,
    cwd: String,
}

impl SessionReadiness {
    pub(crate) fn advertises(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|value| value == capability)
    }
}

/// Opens the session, verifying the bound runtime, wire, package, native
/// binary, resource, read-only tool set, and first-party subscription
/// readiness before any provider work.
pub(crate) async fn open(
    connection: &SdkConnection,
    plan: &PreflightPlan,
    leased_cwd: &str,
) -> Result<SessionReadiness, RuntimeFailure> {
    let model = plan
        .model_id()
        .expect("validated sidecar model route")
        .as_str()
        .to_owned();
    let response = connection
        .command(
            "open-1".to_owned(),
            ClaudeAgentSdkCommand::Open,
            json!({"cwd": leased_cwd, "model": model}),
        )
        .await?;
    if !response.success {
        return Err(failure(
            "swallowtail.claude-agent.sdk.open_rejected",
            "Claude Agent SDK sidecar rejected its restrictive open",
        ));
    }
    let expected = Expectation {
        cwd: leased_cwd,
        model: &model,
        sdk_version: &bound_version(plan, CLAUDE_AGENT_SDK_PACKAGE_AXIS),
        native_version: &bound_version(plan, CLAUDE_AGENT_SDK_NATIVE_AXIS),
        node_version: &bound_version(plan, CLAUDE_AGENT_SDK_NODE_AXIS),
        wire_version: &bound_version(plan, CLAUDE_AGENT_SDK_WIRE_AXIS),
    };
    readiness(response.data.as_ref(), &expected)
}

fn bound_version(plan: &PreflightPlan, axis: &str) -> String {
    plan.interface_versions()
        .find(|binding| binding.axis().as_str() == axis)
        .expect("validated sidecar plan binds every axis")
        .version()
        .as_str()
        .to_owned()
}

struct Expectation<'a> {
    cwd: &'a str,
    model: &'a str,
    sdk_version: &'a str,
    native_version: &'a str,
    node_version: &'a str,
    wire_version: &'a str,
}

fn readiness(
    data: Option<&Value>,
    expected: &Expectation<'_>,
) -> Result<SessionReadiness, RuntimeFailure> {
    let identity_matches = data.is_some_and(|data| {
        expected.wire_version == CLAUDE_AGENT_SDK_WIRE
            && text(data, "wire") == Some(CLAUDE_AGENT_SDK_WIRE)
            && text(data, "behavior") == Some(CLAUDE_AGENT_SDK_BEHAVIOR)
            && text(data, "sdkPackage") == Some(CLAUDE_AGENT_SDK_PACKAGE)
            && text(data, "sdkVersion") == Some(expected.sdk_version)
            && text(data, "nativeVersion") == Some(expected.native_version)
            && text(data, "nodeVersion") == Some(expected.node_version)
            && text(data, "cwd") == Some(expected.cwd)
            // The effective model is confirmed from the runtime's own init
            // evidence; a session that silently ran an ambient default fails.
            && text(data, "model") == Some(expected.model)
            && tools_match(data)
    });
    if !identity_matches {
        return Err(failure(
            "swallowtail.claude-agent.sdk.open_mismatch",
            "Claude Agent SDK sidecar identity did not match the preflight-bound runtime, wire, package, native binary, resource, model, or tool set",
        ));
    }
    let data = data.expect("validated sidecar open identity carries data");
    account_ready(data)?;
    Ok(SessionReadiness {
        capabilities: capabilities(data)?,
        cwd: expected.cwd.to_owned(),
    })
}

/// Accepts only a first-party subscription session. An API-key or delegated
/// cloud provenance label fails closed rather than silently running the route
/// on a different access profile.
fn account_ready(data: &Value) -> Result<(), RuntimeFailure> {
    let account = data.get("account").ok_or_else(account_mismatch)?;
    if text(account, "apiProvider") != Some("firstParty")
        || text(account, "apiKeySource") != Some("oauth")
        || account.get("subscriptionPresent").and_then(Value::as_bool) != Some(true)
    {
        return Err(account_mismatch());
    }
    // Readiness is provenance labels only; no email, organization, or token
    // material is admitted even if a future sidecar offered it.
    if account.get("email").is_some() || account.get("organization").is_some() {
        return Err(account_mismatch());
    }
    Ok(())
}

fn capabilities(data: &Value) -> Result<Vec<String>, RuntimeFailure> {
    let advertised = data
        .get("capabilities")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.capabilities_invalid",
                "Claude Agent SDK sidecar advertised an invalid capability set",
            )
        })?;
    if advertised.len() > MAXIMUM_CAPABILITIES {
        return Err(failure(
            "swallowtail.claude-agent.sdk.capabilities_invalid",
            "Claude Agent SDK sidecar advertised an invalid capability set",
        ));
    }
    let mut capabilities = Vec::with_capacity(advertised.len());
    for value in advertised {
        let value = value
            .as_str()
            .filter(|value| {
                !value.is_empty()
                    && value.len() <= MAXIMUM_CAPABILITY_BYTES
                    && !value.chars().any(char::is_control)
            })
            .ok_or_else(|| {
                failure(
                    "swallowtail.claude-agent.sdk.capabilities_invalid",
                    "Claude Agent SDK sidecar advertised an invalid capability set",
                )
            })?;
        if capabilities.iter().any(|existing| existing == value) {
            return Err(failure(
                "swallowtail.claude-agent.sdk.capabilities_invalid",
                "Claude Agent SDK sidecar advertised an invalid capability set",
            ));
        }
        capabilities.push(value.to_owned());
    }
    Ok(capabilities)
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

fn text<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
    value.get(field).and_then(Value::as_str)
}

fn account_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.account_not_ready",
        "Claude Agent SDK sidecar did not report a first-party subscription session",
    )
}
