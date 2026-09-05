use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::{command_rejected, failure};
use crate::sdk::profile::{ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile};
use crate::sdk::selection::{
    CLAUDE_AGENT_SDK_NATIVE_AXIS, CLAUDE_AGENT_SDK_NODE_AXIS, CLAUDE_AGENT_SDK_PACKAGE_AXIS,
    CLAUDE_AGENT_SDK_WIRE_AXIS,
};
use crate::sdk::wire::ClaudeAgentSdkCommand;
use crate::sdk::{CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_PACKAGE, CLAUDE_AGENT_SDK_WIRE};
use serde_json::{Value, json};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::RuntimeFailure;

const MAXIMUM_CAPABILITIES: usize = 64;
const MAXIMUM_CAPABILITY_BYTES: usize = 96;
const READINESS_REQUESTED: &str = "requested-with-supported-list";
const READINESS_CONFIRMED: &str = "confirmed";

/// Runtime-advertised readiness observed at open. Capabilities are the only
/// axis that is runtime behavior rather than declaration, so nothing here is
/// inferred from the shipped SDK declarations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionReadiness {
    capabilities: Vec<String>,
    cwd: String,
    requested_model: String,
    effective_model: String,
    supported_models: Vec<String>,
    readiness: ReadinessState,
    node_version: String,
    node_version_posture: NodeVersionPosture,
    profile: ClaudeAgentSdkSessionProfile,
    permission_mode: ClaudeAgentSdkPermissionMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NodeVersionPosture {
    Qualified,
    UnverifiedNewer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReadinessState {
    RequestedWithSupportedList,
    Confirmed,
}

impl ReadinessState {
    const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedWithSupportedList => READINESS_REQUESTED,
            Self::Confirmed => READINESS_CONFIRMED,
        }
    }
}

impl NodeVersionPosture {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "Qualified",
            Self::UnverifiedNewer => "UnverifiedNewer",
        }
    }
}

impl SessionReadiness {
    pub(crate) fn advertises(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|value| value == capability)
    }

    /// The admitted tool set the sidecar echoed at open.
    pub(crate) const fn profile(&self) -> ClaudeAgentSdkSessionProfile {
        self.profile
    }

    /// The effective permission mode the sidecar confirmed at open.
    pub(crate) const fn permission_mode(&self) -> ClaudeAgentSdkPermissionMode {
        self.permission_mode
    }

    pub(crate) fn requested_model(&self) -> &str {
        &self.requested_model
    }

    pub(crate) fn effective_model(&self) -> &str {
        &self.effective_model
    }

    pub(crate) fn readiness_state(&self) -> &'static str {
        self.readiness.as_str()
    }

    /// Confirms the first-turn `system/init` evidence carried by the sidecar's
    /// first query response. Open intentionally leaves the effective model and
    /// runtime capabilities unconfirmed until this exchange completes.
    pub(crate) fn confirm_first_turn(
        &mut self,
        data: Option<&Value>,
    ) -> Result<(), RuntimeFailure> {
        let data = data.ok_or_else(init_missing)?;
        if text(data, "readiness") != Some(READINESS_CONFIRMED) {
            return Err(init_missing());
        }
        if text(data, "cwd") != Some(self.cwd.as_str()) {
            return Err(failure(
                "swallowtail.claude-agent.sdk.cwd_mismatch",
                "Claude Agent SDK sidecar first-turn init did not report the leased working directory",
            ));
        }
        if text(data, "requestedModel") != Some(self.requested_model.as_str()) {
            return Err(failure(
                "swallowtail.claude-agent.sdk.open_mismatch",
                "Claude Agent SDK sidecar first-turn init changed the requested model",
            ));
        }
        let effective_model = text(data, "model")
            .filter(|model| !model.is_empty())
            .ok_or_else(|| {
                failure(
                    "swallowtail.claude-agent.sdk.model_missing",
                    "Claude Agent SDK sidecar first-turn init did not report an effective model",
                )
            })?;
        if !self.supported_models.is_empty()
            && !self
                .supported_models
                .iter()
                .any(|model| model == effective_model)
        {
            return Err(failure(
                "swallowtail.claude-agent.sdk.supported_model_rejected",
                "Claude Agent SDK sidecar first-turn init reported an effective model outside its supported model list",
            ));
        }
        self.effective_model = effective_model.to_owned();
        self.capabilities = capabilities(data)?;
        self.readiness = ReadinessState::Confirmed;
        Ok(())
    }

    pub(crate) fn node_version(&self) -> &str {
        &self.node_version
    }

    pub(crate) const fn node_version_posture(&self) -> &'static str {
        self.node_version_posture.as_str()
    }
}

/// Opens the session, verifying the bound runtime, wire, package, native
/// binary, resource, admitted tool set, permission mode, and initialize-time
/// first-party provenance before any provider work. Subscription fields are
/// labelled observations, not gates. The effective model and runtime
/// capabilities remain unconfirmed until the first query's `system/init`
/// evidence.
pub(crate) async fn open(
    connection: &SdkConnection,
    plan: &PreflightPlan,
    leased_cwd: &str,
    profile: ClaudeAgentSdkSessionProfile,
) -> Result<SessionReadiness, RuntimeFailure> {
    let model = plan
        .model_id()
        .expect("validated sidecar model route")
        .as_str()
        .to_owned();
    let tools: Vec<&str> = profile
        .tools()
        .map(crate::sdk::profile::ClaudeAgentSdkTool::as_str)
        .collect();
    let response = connection
        .command(
            "open-1".to_owned(),
            ClaudeAgentSdkCommand::Open,
            json!({
                "cwd": leased_cwd,
                "model": model,
                "tools": tools,
                "permissionMode": profile.permission_mode().as_str(),
            }),
        )
        .await?;
    if !response.success {
        return Err(command_rejected(
            "swallowtail.claude-agent.sdk.open_rejected",
            "Claude Agent SDK sidecar rejected its restrictive open",
            response
                .failure_code
                .expect("a rejected response carries its fixed sidecar code"),
        ));
    }
    let expected = Expectation {
        cwd: leased_cwd,
        requested_model: &model,
        profile,
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
    requested_model: &'a str,
    profile: ClaudeAgentSdkSessionProfile,
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
            && text(data, "nodeVersion").is_some()
            && text(data, "cwd") == Some(expected.cwd)
            && text(data, "requestedModel") == Some(expected.requested_model)
            && tools_match(data, expected.profile)
            && text(data, "permissionMode") == Some(expected.profile.permission_mode().as_str())
    });
    if !identity_matches {
        return Err(failure(
            "swallowtail.claude-agent.sdk.open_mismatch",
            "Claude Agent SDK sidecar identity did not match the preflight-bound runtime, wire, package, native binary, resource, model, tool set, or permission mode",
        ));
    }
    let data = data.expect("validated sidecar open identity carries data");
    if text(data, "readiness") != Some(READINESS_REQUESTED) {
        return Err(failure(
            "swallowtail.claude-agent.sdk.open_mismatch",
            "Claude Agent SDK sidecar did not report requested-with-supported-list readiness",
        ));
    }
    let supported_models = supported_models(data);
    let node_version = text(data, "nodeVersion")
        .filter(|version| !version.is_empty())
        .ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.open_mismatch",
                "Claude Agent SDK sidecar did not report its Node runtime version",
            )
        })?;
    let node_version_posture = node_version_posture(node_version, expected.node_version)
        .ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.open_mismatch",
                "Claude Agent SDK sidecar Node runtime was older than the qualified point",
            )
        })?;
    account_ready(data)?;
    Ok(SessionReadiness {
        // Capabilities are runtime evidence from first-turn system/init, not
        // an initialize-response claim.
        capabilities: Vec::new(),
        cwd: expected.cwd.to_owned(),
        requested_model: expected.requested_model.to_owned(),
        effective_model: String::new(),
        supported_models,
        readiness: ReadinessState::RequestedWithSupportedList,
        node_version: node_version.to_owned(),
        node_version_posture,
        profile: expected.profile,
        permission_mode: expected.profile.permission_mode(),
    })
}

/// Accepts only a first-party session. An API-key or delegated cloud
/// provenance label fails closed rather than silently running the route on a
/// different access profile. Subscription evidence remains observational.
fn account_ready(data: &Value) -> Result<(), RuntimeFailure> {
    let account = data.get("account").ok_or_else(account_mismatch)?;
    if text(account, "apiProvider") != Some("firstParty") {
        return Err(failure(
            "swallowtail.claude-agent.sdk.account_not_first_party",
            "Claude Agent SDK sidecar did not report a first-party account",
        ));
    }
    // Readiness is provenance labels only; no email, organization, or token
    // material is admitted even if a future sidecar offered it.
    if account.get("email").is_some() || account.get("organization").is_some() {
        return Err(account_mismatch());
    }
    Ok(())
}

fn supported_models(data: &Value) -> Vec<String> {
    data.get("supportedModels")
        .and_then(Value::as_array)
        .map(|supported| {
            supported
                .iter()
                .filter_map(Value::as_str)
                .filter(|model| !model.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn init_missing() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.init_missing",
        "Claude Agent SDK sidecar did not yield system/init as the first query message",
    )
}

fn node_version_posture(observed: &str, qualified: &str) -> Option<NodeVersionPosture> {
    let observed = version_parts(observed)?;
    let qualified = version_parts(qualified)?;
    match observed.cmp(&qualified) {
        std::cmp::Ordering::Less => None,
        std::cmp::Ordering::Equal => Some(NodeVersionPosture::Qualified),
        std::cmp::Ordering::Greater => Some(NodeVersionPosture::UnverifiedNewer),
    }
}

fn version_parts(value: &str) -> Option<[u32; 3]> {
    let mut parts = value.split('.');
    let result = [
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    ];
    if parts.next().is_some() {
        return None;
    }
    Some(result)
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

/// The sidecar's echo must be the admitted set exactly: same tools, same
/// order, no additions. A widened echo is a substitution, not a convenience.
fn tools_match(data: &Value, profile: ClaudeAgentSdkSessionProfile) -> bool {
    data.get("tools")
        .and_then(Value::as_array)
        .is_some_and(|tools| {
            tools.len() == profile.tools().count()
                && tools
                    .iter()
                    .zip(profile.tools())
                    .all(|(tool, expected)| tool.as_str() == Some(expected.as_str()))
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
