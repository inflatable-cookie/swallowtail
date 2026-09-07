use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::{command_rejected, failure};
use crate::sdk::mcp::{
    ClaudeAgentSdkMcpServer, ClaudeAgentSdkMcpServerStatus, ClaudeAgentSdkMcpServerStatusKind,
    OpenStdioMcpServer, admitted_open_mcp_tool_names, admitted_open_tool_names,
    combine_open_servers,
};
use crate::sdk::profile::{
    ClaudeAgentSdkEffort, ClaudeAgentSdkEffortOutcome, ClaudeAgentSdkPermissionMode,
    ClaudeAgentSdkSessionProfile,
};
use crate::sdk::selection::{
    CLAUDE_AGENT_SDK_NATIVE_AXIS, CLAUDE_AGENT_SDK_NODE_AXIS, CLAUDE_AGENT_SDK_PACKAGE_AXIS,
    CLAUDE_AGENT_SDK_WIRE_AXIS,
};
use crate::sdk::wire::ClaudeAgentSdkCommand;
use crate::sdk::{CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_PACKAGE, CLAUDE_AGENT_SDK_WIRE};
use serde_json::{Value, json};
use swallowtail_core::{PreflightPlan, SessionRef};
use swallowtail_runtime::RuntimeFailure;

const MAXIMUM_CAPABILITIES: usize = 64;
const MAXIMUM_CAPABILITY_BYTES: usize = 96;
const READINESS_REQUESTED: &str = "requested-with-supported-list";
const READINESS_CONFIRMED: &str = "confirmed";
const MAXIMUM_LISTING_PAGE: usize = 1_000;
const MAXIMUM_LISTING_TEXT_BYTES: usize = 16 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionListing {
    pub(crate) provider_session_ref: SessionRef,
    pub(crate) cwd: String,
    pub(crate) created_at_unix_milliseconds: Option<u64>,
    pub(crate) last_modified_unix_milliseconds: u64,
    pub(crate) title: Option<String>,
}

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
    effort: ClaudeAgentSdkEffortOutcome,
    readiness: ReadinessState,
    node_version: String,
    node_version_posture: NodeVersionPosture,
    profile: ClaudeAgentSdkSessionProfile,
    permission_mode: ClaudeAgentSdkPermissionMode,
    resuming: bool,
    expected_provider_session_ref: Option<SessionRef>,
    provider_session_ref: Option<SessionRef>,
    mcp_server_status: Vec<ClaudeAgentSdkMcpServerStatus>,
    admitted_mcp_tools: Vec<String>,
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

    pub(crate) fn mcp_server_status(&self) -> &[ClaudeAgentSdkMcpServerStatus] {
        &self.mcp_server_status
    }

    pub(crate) fn admitted_mcp_tools(&self) -> &[String] {
        &self.admitted_mcp_tools
    }

    pub(crate) fn provider_session_ref(&self) -> Option<&SessionRef> {
        self.provider_session_ref.as_ref()
    }

    pub(crate) fn requested_model(&self) -> &str {
        &self.requested_model
    }

    pub(crate) fn effective_model(&self) -> &str {
        &self.effective_model
    }

    pub(crate) fn confirm_model_change(&mut self, model: &str) {
        self.effective_model = model.to_owned();
    }

    pub(crate) fn supported_models(&self) -> &[String] {
        &self.supported_models
    }

    pub(crate) const fn effort(&self) -> ClaudeAgentSdkEffortOutcome {
        self.effort
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
        let provider_session_ref = if self.profile.persist_session() || self.resuming {
            text(data, "sessionId")
                .filter(|session_id| !session_id.is_empty())
                .map(SessionRef::new)
                .transpose()
                .map_err(|_| {
                    if self.resuming {
                        resume_session_unknown()
                    } else {
                        init_missing()
                    }
                })?
        } else {
            None
        };
        if self.profile.persist_session() && provider_session_ref.is_none() {
            return Err(resume_session_unknown());
        }
        if self.resuming
            && provider_session_ref.as_ref() != self.expected_provider_session_ref.as_ref()
        {
            return Err(resume_session_unknown());
        }
        if text(data, "cwd") != Some(self.cwd.as_str()) {
            return Err(if self.resuming {
                failure(
                    "swallowtail.claude-agent.sdk.resume_cwd_mismatch",
                    "Claude Agent SDK resume init did not report the leased working directory",
                )
            } else {
                failure(
                    "swallowtail.claude-agent.sdk.cwd_mismatch",
                    "Claude Agent SDK sidecar first-turn init did not report the leased working directory",
                )
            });
        }
        if text(data, "requestedModel") != Some(self.requested_model.as_str()) {
            return Err(failure(
                "swallowtail.claude-agent.sdk.open_mismatch",
                "Claude Agent SDK sidecar first-turn init changed the requested model",
            ));
        }
        if self.resuming {
            if data.get("accountVerified").and_then(Value::as_bool) != Some(true) {
                return Err(resume_account_mismatch());
            }
            if data
                .get("account")
                .and_then(|account| text(account, "apiProvider"))
                != Some("firstParty")
            {
                return Err(resume_account_mismatch());
            }
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
        let effort = match (self.profile.effort(), text(data, "effort")) {
            (Some(requested), Some(reported)) => {
                let reported = parse_effort(reported).ok_or_else(effort_unconfirmed)?;
                if reported != requested {
                    return Err(effort_unconfirmed());
                }
                ClaudeAgentSdkEffortOutcome::Confirmed(reported)
            }
            (Some(requested), None) => ClaudeAgentSdkEffortOutcome::RequestedOnly(requested),
            (None, Some(_)) => return Err(effort_unconfirmed()),
            (None, None) => ClaudeAgentSdkEffortOutcome::NotRequested,
        };
        self.effective_model = effective_model.to_owned();
        self.capabilities = capabilities(data)?;
        self.effort = effort;
        self.readiness = ReadinessState::Confirmed;
        self.provider_session_ref = provider_session_ref;
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
    mcp_servers: &[ClaudeAgentSdkMcpServer],
    registered_courier: Option<OpenStdioMcpServer>,
) -> Result<SessionReadiness, RuntimeFailure> {
    start(
        connection,
        plan,
        leased_cwd,
        profile,
        mcp_servers,
        registered_courier,
        None,
        None,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn resume(
    connection: &SdkConnection,
    plan: &PreflightPlan,
    leased_cwd: &str,
    profile: ClaudeAgentSdkSessionProfile,
    mcp_servers: &[ClaudeAgentSdkMcpServer],
    registered_courier: Option<OpenStdioMcpServer>,
    provider_session_ref: &SessionRef,
    resume_session_at: Option<&str>,
) -> Result<SessionReadiness, RuntimeFailure> {
    start(
        connection,
        plan,
        leased_cwd,
        profile,
        mcp_servers,
        registered_courier,
        Some(provider_session_ref),
        resume_session_at,
    )
    .await
}

pub(crate) async fn list(
    connection: &SdkConnection,
    request_id: &str,
    leased_cwd: &str,
    limit: usize,
    offset: usize,
) -> Result<Vec<SessionListing>, RuntimeFailure> {
    let response = connection
        .command(
            format!("list-sessions:{request_id}"),
            ClaudeAgentSdkCommand::ListSessions,
            json!({"cwd": leased_cwd, "limit": limit, "offset": offset}),
        )
        .await?;
    if !response.success {
        return Err(command_rejected(
            "swallowtail.claude-agent.sdk.listing_failed",
            "Claude Agent SDK sidecar rejected session listing",
            response
                .failure_code
                .expect("a rejected response carries its fixed sidecar code"),
        ));
    }
    let data = response.data.as_ref().ok_or_else(listing_invalid)?;
    if text(data, "cwd") != Some(leased_cwd) {
        return Err(listing_invalid());
    }
    let entries = data
        .get("sessions")
        .and_then(Value::as_array)
        .ok_or_else(listing_invalid)?;
    if entries.len() > limit || entries.len() > MAXIMUM_LISTING_PAGE {
        return Err(listing_invalid());
    }
    entries
        .iter()
        .map(|entry| project_listing(entry, leased_cwd))
        .collect()
}

fn project_listing(value: &Value, leased_cwd: &str) -> Result<SessionListing, RuntimeFailure> {
    let provider_session_ref =
        SessionRef::new(text(value, "sessionId").ok_or_else(listing_invalid)?)
            .map_err(|_| listing_invalid())?;
    let last_modified = value
        .get("lastModified")
        .and_then(Value::as_u64)
        .ok_or_else(listing_invalid)?;
    let created_at = match value.get("createdAt") {
        None | Some(Value::Null) => None,
        Some(value) => Some(value.as_u64().ok_or_else(listing_invalid)?),
    };
    let title = match value.get("title") {
        None | Some(Value::Null) => None,
        Some(Value::String(title))
            if !title.is_empty()
                && title.len() <= MAXIMUM_LISTING_TEXT_BYTES
                && !title.chars().any(char::is_control) =>
        {
            Some(title.clone())
        }
        Some(_) => return Err(listing_invalid()),
    };
    Ok(SessionListing {
        provider_session_ref,
        cwd: leased_cwd.to_owned(),
        created_at_unix_milliseconds: created_at,
        last_modified_unix_milliseconds: last_modified,
        title,
    })
}

#[allow(clippy::too_many_arguments)]
async fn start(
    connection: &SdkConnection,
    plan: &PreflightPlan,
    leased_cwd: &str,
    profile: ClaudeAgentSdkSessionProfile,
    mcp_servers: &[ClaudeAgentSdkMcpServer],
    registered_courier: Option<OpenStdioMcpServer>,
    provider_session_ref: Option<&SessionRef>,
    resume_session_at: Option<&str>,
) -> Result<SessionReadiness, RuntimeFailure> {
    let model = plan
        .model_id()
        .expect("validated sidecar model route")
        .as_str()
        .to_owned();
    let open_servers = combine_open_servers(mcp_servers, registered_courier);
    let tools = admitted_open_tool_names(&profile, &open_servers);
    let mut params = json!({
        "cwd": leased_cwd,
        "model": model,
        "tools": tools,
        "permissionMode": profile.permission_mode().as_str(),
    });
    if profile.persist_session() {
        params["persistSession"] = json!(true);
    }
    if let Some(provider_session_ref) = provider_session_ref {
        params["resume"] = json!(provider_session_ref.as_provider_value());
    }
    if let Some(resume_session_at) = resume_session_at {
        params["resumeSessionAt"] = json!(resume_session_at);
    }
    if let Some(effort) = profile.effort() {
        params["effort"] = json!(effort.as_str());
    }
    if !open_servers.is_empty() {
        params["mcpServers"] = mcp_servers_params(&open_servers);
    }
    let response = connection
        .command("open-1".to_owned(), ClaudeAgentSdkCommand::Open, params)
        .await?;
    if !response.success {
        let code = response
            .failure_code
            .expect("a rejected response carries its fixed sidecar code");
        return Err(if provider_session_ref.is_some() {
            resume_rejected(code)
        } else {
            command_rejected(
                "swallowtail.claude-agent.sdk.open_rejected",
                "Claude Agent SDK sidecar rejected its restrictive open",
                code,
            )
        });
    }
    let expected = Expectation {
        cwd: leased_cwd,
        requested_model: &model,
        profile,
        admitted_tools: tools,
        mcp_servers: &open_servers,
        sdk_version: &bound_version(plan, CLAUDE_AGENT_SDK_PACKAGE_AXIS),
        native_version: &bound_version(plan, CLAUDE_AGENT_SDK_NATIVE_AXIS),
        node_version: &bound_version(plan, CLAUDE_AGENT_SDK_NODE_AXIS),
        wire_version: &bound_version(plan, CLAUDE_AGENT_SDK_WIRE_AXIS),
        resuming: provider_session_ref.is_some(),
        expected_provider_session_ref: provider_session_ref,
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
    admitted_tools: Vec<String>,
    mcp_servers: &'a [OpenStdioMcpServer],
    sdk_version: &'a str,
    native_version: &'a str,
    node_version: &'a str,
    wire_version: &'a str,
    resuming: bool,
    expected_provider_session_ref: Option<&'a SessionRef>,
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
            && tools_match(data, &expected.admitted_tools)
            && text(data, "permissionMode") == Some(expected.profile.permission_mode().as_str())
            && effort_matches(data, expected.profile)
            && data.get("persistSession").and_then(Value::as_bool)
                == Some(expected.profile.persist_session())
            && mcp_status_present(data, expected.mcp_servers)
            && data.get("resuming").and_then(Value::as_bool) == Some(expected.resuming)
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
    let supported_models = supported_models(data)?;
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
    account_ready(data, expected.resuming)?;
    let mcp_server_status = mcp_server_status(data, expected.mcp_servers)?;
    Ok(SessionReadiness {
        // Capabilities are runtime evidence from first-turn system/init, not
        // an initialize-response claim.
        capabilities: Vec::new(),
        cwd: expected.cwd.to_owned(),
        requested_model: expected.requested_model.to_owned(),
        effective_model: String::new(),
        supported_models,
        effort: expected.profile.effort().map_or(
            ClaudeAgentSdkEffortOutcome::NotRequested,
            ClaudeAgentSdkEffortOutcome::RequestedOnly,
        ),
        readiness: ReadinessState::RequestedWithSupportedList,
        node_version: node_version.to_owned(),
        node_version_posture,
        profile: expected.profile,
        permission_mode: expected.profile.permission_mode(),
        resuming: expected.resuming,
        expected_provider_session_ref: expected.expected_provider_session_ref.cloned(),
        provider_session_ref: None,
        mcp_server_status,
        admitted_mcp_tools: admitted_open_mcp_tool_names(expected.mcp_servers),
    })
}

/// Accepts only a first-party session. An API-key or delegated cloud
/// provenance label fails closed rather than silently running the route on a
/// different access profile. Subscription evidence remains observational.
fn account_ready(data: &Value, resuming: bool) -> Result<(), RuntimeFailure> {
    let account = data.get("account").ok_or_else(|| {
        if resuming {
            resume_account_mismatch()
        } else {
            account_mismatch()
        }
    })?;
    if text(account, "apiProvider") != Some("firstParty") {
        return Err(if resuming {
            resume_account_mismatch()
        } else {
            failure(
                "swallowtail.claude-agent.sdk.account_not_first_party",
                "Claude Agent SDK sidecar did not report a first-party account",
            )
        });
    }
    // Readiness is provenance labels only; no email, organization, or token
    // material is admitted even if a future sidecar offered it.
    if account.get("email").is_some() || account.get("organization").is_some() {
        return Err(if resuming {
            resume_account_mismatch()
        } else {
            account_mismatch()
        });
    }
    Ok(())
}

fn supported_models(data: &Value) -> Result<Vec<String>, RuntimeFailure> {
    let Some(supported) = data.get("supportedModels").and_then(Value::as_array) else {
        return Ok(Vec::new());
    };
    if supported.len() > 64 {
        return Err(failure(
            "swallowtail.claude-agent.sdk.supported_models_invalid",
            "Claude Agent SDK sidecar advertised too many supported models",
        ));
    }
    let mut models = Vec::with_capacity(supported.len());
    for model in supported {
        let Some(model) = model.as_str().filter(|model| {
            !model.is_empty() && model.len() <= 128 && !model.chars().any(char::is_control)
        }) else {
            return Err(failure(
                "swallowtail.claude-agent.sdk.supported_models_invalid",
                "Claude Agent SDK sidecar advertised an invalid supported model",
            ));
        };
        if !models.iter().any(|existing| existing == model) {
            models.push(model.to_owned());
        }
    }
    Ok(models)
}

fn parse_effort(value: &str) -> Option<ClaudeAgentSdkEffort> {
    Some(match value {
        "low" => ClaudeAgentSdkEffort::Low,
        "medium" => ClaudeAgentSdkEffort::Medium,
        "high" => ClaudeAgentSdkEffort::High,
        "xhigh" => ClaudeAgentSdkEffort::XHigh,
        "max" => ClaudeAgentSdkEffort::Max,
        _ => return None,
    })
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
fn tools_match(data: &Value, admitted: &[String]) -> bool {
    data.get("tools")
        .and_then(Value::as_array)
        .is_some_and(|tools| {
            tools.len() == admitted.len()
                && tools
                    .iter()
                    .zip(admitted)
                    .all(|(tool, expected)| tool.as_str() == Some(expected.as_str()))
        })
}

fn mcp_status_present(data: &Value, servers: &[OpenStdioMcpServer]) -> bool {
    match data.get("mcpServerStatus") {
        None => servers.is_empty(),
        Some(value) => value
            .as_array()
            .is_some_and(|statuses| !servers.is_empty() && statuses.len() == servers.len()),
    }
}

fn mcp_servers_params(servers: &[OpenStdioMcpServer]) -> Value {
    Value::Array(
        servers
            .iter()
            .map(|server| {
                let mut value = json!({
                    "name": server.name(),
                    "command": server.command(),
                    "args": server.args(),
                    "envAllowlistKeys": server.env_allowlist_keys(),
                    "tools": server.tools(),
                    "optional": server.is_optional(),
                });
                if !server.env().is_empty() {
                    value["env"] = json!(
                        server
                            .env()
                            .iter()
                            .cloned()
                            .collect::<std::collections::BTreeMap<_, _>>()
                    );
                }
                value
            })
            .collect(),
    )
}

fn mcp_server_status(
    data: &Value,
    servers: &[OpenStdioMcpServer],
) -> Result<Vec<ClaudeAgentSdkMcpServerStatus>, RuntimeFailure> {
    if servers.is_empty() {
        return Ok(Vec::new());
    }
    let statuses = data
        .get("mcpServerStatus")
        .and_then(Value::as_array)
        .ok_or_else(mcp_status_invalid)?;
    if statuses.len() != servers.len() {
        return Err(mcp_status_invalid());
    }
    let mut projected = Vec::with_capacity(servers.len());
    for (server, value) in servers.iter().zip(statuses) {
        if text(value, "name") != Some(server.name()) {
            return Err(mcp_status_invalid());
        }
        if value.get("error").is_some()
            || value.get("url").is_some()
            || value.get("config").is_some()
        {
            return Err(mcp_status_invalid());
        }
        let kind = text(value, "status").ok_or_else(mcp_status_invalid)?;
        projected.push(match kind {
            "connected" => ClaudeAgentSdkMcpServerStatus::connected(server.name()),
            "pending" => ClaudeAgentSdkMcpServerStatus::pending(server.name()),
            "failed" => ClaudeAgentSdkMcpServerStatus::failed(
                server.name(),
                "swallowtail.claude-agent.sdk.mcp_server_failed",
            ),
            "needs-auth" => ClaudeAgentSdkMcpServerStatus::failed(
                server.name(),
                "swallowtail.claude-agent.sdk.mcp_server_needs_auth",
            ),
            _ => return Err(mcp_status_invalid()),
        });
        if !server.is_optional()
            && projected
                .last()
                .is_some_and(|status| status.kind() != ClaudeAgentSdkMcpServerStatusKind::Connected)
        {
            return Err(failure(
                projected
                    .last()
                    .and_then(ClaudeAgentSdkMcpServerStatus::failure_code)
                    .unwrap_or("swallowtail.claude-agent.sdk.mcp_server_failed"),
                "Claude Agent SDK required MCP server did not connect",
            ));
        }
    }
    Ok(projected)
}

fn mcp_status_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.mcp_status_invalid",
        "Claude Agent SDK sidecar returned invalid MCP server status evidence",
    )
}

fn effort_matches(data: &Value, profile: ClaudeAgentSdkSessionProfile) -> bool {
    match profile.effort() {
        Some(effort) => text(data, "requestedEffort") == Some(effort.as_str()),
        None => data.get("requestedEffort").is_none(),
    }
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

fn resume_rejected(code: crate::sdk::wire::ClaudeAgentSdkFailureCode) -> RuntimeFailure {
    match code {
        crate::sdk::wire::ClaudeAgentSdkFailureCode::ResumeCwdMismatch => failure(
            "swallowtail.claude-agent.sdk.resume_cwd_mismatch",
            "Claude Agent SDK resume did not use the leased working directory",
        ),
        crate::sdk::wire::ClaudeAgentSdkFailureCode::ResumeAccountMismatch => failure(
            "swallowtail.claude-agent.sdk.resume_account_mismatch",
            "Claude Agent SDK resume did not use the verified first-party account",
        ),
        crate::sdk::wire::ClaudeAgentSdkFailureCode::ResumeSessionUnknown => failure(
            "swallowtail.claude-agent.sdk.resume_session_unknown",
            "Claude Agent SDK could not identify the bound provider session",
        ),
        crate::sdk::wire::ClaudeAgentSdkFailureCode::ResumeBoundaryInvalid => failure(
            "swallowtail.claude-agent.sdk.resume_boundary_invalid",
            "Claude Agent SDK rejected the resume message boundary",
        ),
        other => command_rejected(
            "swallowtail.claude-agent.sdk.resume_rejected",
            "Claude Agent SDK sidecar rejected resume",
            other,
        ),
    }
}

fn resume_session_unknown() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.resume_session_unknown",
        "Claude Agent SDK did not report the bound provider session",
    )
}

fn resume_account_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.resume_account_mismatch",
        "Claude Agent SDK resume did not report the verified first-party account",
    )
}

fn listing_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.listing_invalid",
        "Claude Agent SDK sidecar returned invalid session listing metadata",
    )
}

fn effort_unconfirmed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.effort_unconfirmed",
        "Claude Agent SDK sidecar did not confirm the requested opening effort",
    )
}
