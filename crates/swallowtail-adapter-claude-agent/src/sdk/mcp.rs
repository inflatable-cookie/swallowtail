//! Consumer-declared stdio MCP servers for one SDK sidecar session.
//!
//! This is not the Claude Code watcher family and not an in-process SDK
//! callback server. Declared servers are stdio processes with an explicit env
//! object built from the same deny-by-default allowlist as the native child.
//! SSE/HTTP configs carry URLs and optional headers, and in-process `sdk`
//! servers execute inside the sidecar, so none of those shapes are
//! representable here.

use super::prepared::preparation_failure;
use super::profile::ClaudeAgentSdkSessionProfile;
use super::registered_tool::CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER;
use swallowtail_runtime::{PreparationFailure, PreparationStage};

/// Exact env names the sidecar copies into the native child. MCP server env
/// uses the same deny-by-default set: a named key is forwarded only when it
/// is in this list or starts with `LC_`.
pub(crate) const MCP_CHILD_ENV_EXACT_KEYS: &[&str] = &[
    "HOME",
    "PATH",
    "TMPDIR",
    "LANG",
    "USER",
    "SHELL",
    "TERM",
    "COLORTERM",
    "__CF_USER_TEXT_ENCODING",
    "XPC_FLAGS",
    "XPC_SERVICE_NAME",
    "MallocNanoZone",
    "COMMAND_MODE",
];

const WATCHER_SERVER_NAME: &str = "swallowtail-watchers";
const MAXIMUM_NAME_BYTES: usize = 64;
const MAXIMUM_COMMAND_BYTES: usize = 512;
const MAXIMUM_ARG_BYTES: usize = 256;
const MAXIMUM_SERVERS: usize = 16;
const MAXIMUM_ARGS: usize = 32;
const MAXIMUM_ENV_KEYS: usize = 32;
const MAXIMUM_TOOLS: usize = 32;

/// One consumer-declared stdio MCP server and the tools it may expose.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkMcpServer {
    name: String,
    command: String,
    args: Vec<String>,
    env_allowlist_keys: Vec<String>,
    tools: Vec<String>,
    optional: bool,
}

impl ClaudeAgentSdkMcpServer {
    /// Declares one required stdio MCP server.
    ///
    /// `tools` are the server-local tool names that join the admitted set as
    /// `mcp__<name>__<tool>`. Env keys must already be in the sidecar child
    /// allowlist; values are never taken from inherited `process.env` as a
    /// whole.
    pub fn stdio(
        name: impl Into<String>,
        command: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<String>>,
        env_allowlist_keys: impl IntoIterator<Item = impl Into<String>>,
        tools: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, PreparationFailure> {
        let server = Self {
            name: name.into(),
            command: command.into(),
            args: args.into_iter().map(Into::into).collect(),
            env_allowlist_keys: env_allowlist_keys.into_iter().map(Into::into).collect(),
            tools: tools.into_iter().map(Into::into).collect(),
            optional: false,
        };
        server.validate()?;
        Ok(server)
    }

    /// Marks this declared server optional: a connect failure is recorded in
    /// open evidence instead of failing the open.
    #[must_use]
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Returns the configured server name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stdio command.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Returns the stdio arguments.
    #[must_use]
    pub fn args(&self) -> &[String] {
        &self.args
    }

    /// Returns the env keys this server may receive from the child allowlist.
    #[must_use]
    pub fn env_allowlist_keys(&self) -> &[String] {
        &self.env_allowlist_keys
    }

    /// Returns the server-local tool names admitted for this server.
    #[must_use]
    pub fn tools(&self) -> &[String] {
        &self.tools
    }

    /// Reports whether a connect failure may be recorded without failing open.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        self.optional
    }

    /// Returns the exact upstream MCP tool names admitted for this server.
    #[must_use]
    pub fn admitted_tool_names(&self) -> Vec<String> {
        self.tools
            .iter()
            .map(|tool| mcp_tool_name(&self.name, tool))
            .collect()
    }

    fn validate(&self) -> Result<(), PreparationFailure> {
        if !is_identifier(&self.name) {
            return Err(mcp_failure(
                "swallowtail.claude-agent.sdk.profile.mcp_server_name_invalid",
                "Claude Agent SDK preparation rejects an MCP server name outside the stdio identifier set",
            ));
        }
        if self.name == WATCHER_SERVER_NAME {
            return Err(mcp_failure(
                "swallowtail.claude-agent.sdk.profile.mcp_server_forbidden",
                "Claude Agent SDK preparation rejects the reserved watcher MCP server name",
            ));
        }
        // The Swallowtail-owned Contract 063 carrier is a separate path with a
        // separate owner. A consumer-declared server may not present its name.
        if self.name == CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER {
            return Err(mcp_failure(
                "swallowtail.claude-agent.sdk.profile.mcp_server_forbidden",
                "Claude Agent SDK preparation rejects the reserved registered-tool carrier name",
            ));
        }
        if !is_bounded_text(&self.command, MAXIMUM_COMMAND_BYTES) {
            return Err(mcp_failure(
                "swallowtail.claude-agent.sdk.profile.mcp_command_invalid",
                "Claude Agent SDK preparation requires a non-empty stdio MCP command",
            ));
        }
        if self.args.len() > MAXIMUM_ARGS
            || self
                .args
                .iter()
                .any(|argument| !is_bounded_text(argument, MAXIMUM_ARG_BYTES))
        {
            return Err(mcp_failure(
                "swallowtail.claude-agent.sdk.profile.mcp_command_invalid",
                "Claude Agent SDK preparation rejects an MCP stdio argument outside its bound",
            ));
        }
        if self.env_allowlist_keys.len() > MAXIMUM_ENV_KEYS {
            return Err(mcp_failure(
                "swallowtail.claude-agent.sdk.profile.mcp_env_key_rejected",
                "Claude Agent SDK preparation rejects an MCP env allowlist outside its bound",
            ));
        }
        let mut env_keys = Vec::new();
        for key in &self.env_allowlist_keys {
            if !env_key_allowed(key) || env_keys.iter().any(|existing| existing == key) {
                return Err(mcp_failure(
                    "swallowtail.claude-agent.sdk.profile.mcp_env_key_rejected",
                    "Claude Agent SDK preparation rejects an MCP env key outside the sidecar child allowlist",
                ));
            }
            env_keys.push(key.clone());
        }
        if self.tools.is_empty() || self.tools.len() > MAXIMUM_TOOLS {
            return Err(mcp_failure(
                "swallowtail.claude-agent.sdk.profile.mcp_tool_invalid",
                "Claude Agent SDK preparation requires at least one admitted MCP tool per server",
            ));
        }
        let mut tools = Vec::new();
        for tool in &self.tools {
            if !is_identifier(tool) || tools.iter().any(|existing| existing == tool) {
                return Err(mcp_failure(
                    "swallowtail.claude-agent.sdk.profile.mcp_tool_invalid",
                    "Claude Agent SDK preparation rejects a repeated or invalid MCP tool name",
                ));
            }
            tools.push(tool.clone());
        }
        Ok(())
    }
}

/// Copy session profile plus the owned MCP server set it admits.
///
/// MCP servers own strings, so they cannot live inside the Copy profile that
/// `v0.4.2` exposes through `const fn` accessors. This binding is the additive
/// prepared input that carries both.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkMcpBinding {
    profile: ClaudeAgentSdkSessionProfile,
    servers: Vec<ClaudeAgentSdkMcpServer>,
}

impl ClaudeAgentSdkMcpBinding {
    pub(crate) fn from_parts(
        profile: ClaudeAgentSdkSessionProfile,
        servers: Vec<ClaudeAgentSdkMcpServer>,
    ) -> Result<Self, PreparationFailure> {
        validate_servers(&servers)?;
        Ok(Self { profile, servers })
    }

    /// Returns the Copy session profile this binding carries.
    #[must_use]
    pub const fn session_profile(&self) -> ClaudeAgentSdkSessionProfile {
        self.profile
    }

    /// Returns the declared servers in construction order.
    #[must_use]
    pub fn servers(&self) -> &[ClaudeAgentSdkMcpServer] {
        &self.servers
    }

    /// Returns native tool names followed by admitted MCP tool names.
    #[must_use]
    pub fn admitted_tool_names(&self) -> Vec<String> {
        admitted_tool_names(&self.profile, &self.servers)
    }
}

/// Connection status for one declared MCP server at open.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaudeAgentSdkMcpServerStatusKind {
    /// The server connected before open returned.
    Connected,
    /// The server had not finished connecting when open returned.
    Pending,
    /// The server failed to connect, or required auth this route does not admit.
    Failed,
}

impl ClaudeAgentSdkMcpServerStatusKind {
    /// Returns the exact open-evidence status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Pending => "pending",
            Self::Failed => "failed",
        }
    }
}

/// Bounded per-server status carried in open evidence.
///
/// Provider error text, URLs, headers, and config objects never appear here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkMcpServerStatus {
    name: String,
    kind: ClaudeAgentSdkMcpServerStatusKind,
    failure_code: Option<&'static str>,
}

impl ClaudeAgentSdkMcpServerStatus {
    pub(crate) fn connected(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ClaudeAgentSdkMcpServerStatusKind::Connected,
            failure_code: None,
        }
    }

    pub(crate) fn pending(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ClaudeAgentSdkMcpServerStatusKind::Pending,
            failure_code: None,
        }
    }

    pub(crate) fn failed(name: impl Into<String>, failure_code: &'static str) -> Self {
        Self {
            name: name.into(),
            kind: ClaudeAgentSdkMcpServerStatusKind::Failed,
            failure_code: Some(failure_code),
        }
    }

    /// Returns the declared server name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the bounded connection status.
    #[must_use]
    pub const fn kind(&self) -> ClaudeAgentSdkMcpServerStatusKind {
        self.kind
    }

    /// Returns the typed failure code when status is failed.
    #[must_use]
    pub const fn failure_code(&self) -> Option<&'static str> {
        self.failure_code
    }
}

pub(crate) fn validate_servers(
    servers: &[ClaudeAgentSdkMcpServer],
) -> Result<(), PreparationFailure> {
    if servers.len() > MAXIMUM_SERVERS {
        return Err(mcp_failure(
            "swallowtail.claude-agent.sdk.profile.mcp_server_name_invalid",
            "Claude Agent SDK preparation rejects an MCP server set outside its bound",
        ));
    }
    let mut names = Vec::new();
    for server in servers {
        server.validate()?;
        if names.iter().any(|existing| existing == &server.name) {
            return Err(mcp_failure(
                "swallowtail.claude-agent.sdk.profile.mcp_server_repeated",
                "Claude Agent SDK preparation admits each MCP server name at most once",
            ));
        }
        names.push(server.name.clone());
    }
    Ok(())
}

pub(crate) fn admitted_tool_names(
    profile: &ClaudeAgentSdkSessionProfile,
    servers: &[ClaudeAgentSdkMcpServer],
) -> Vec<String> {
    let mut names: Vec<String> = profile
        .tools()
        .map(|tool| tool.as_str().to_owned())
        .collect();
    names.extend(admitted_mcp_tool_names(servers));
    names
}

pub(crate) fn admitted_mcp_tool_names(servers: &[ClaudeAgentSdkMcpServer]) -> Vec<String> {
    servers
        .iter()
        .flat_map(ClaudeAgentSdkMcpServer::admitted_tool_names)
        .collect()
}

/// Driver-owned stdio MCP declaration used at open.
///
/// Consumer-declared servers convert into this type. The Swallowtail-owned
/// registered-tool courier is constructed here so it never passes through
/// [`ClaudeAgentSdkMcpServer`], which rejects the reserved carrier name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenStdioMcpServer {
    name: String,
    command: String,
    args: Vec<String>,
    env_allowlist_keys: Vec<String>,
    tools: Vec<String>,
    optional: bool,
}

impl OpenStdioMcpServer {
    pub(crate) fn from_consumer(server: &ClaudeAgentSdkMcpServer) -> Self {
        Self {
            name: server.name().to_owned(),
            command: server.command().to_owned(),
            args: server.args().to_vec(),
            env_allowlist_keys: server.env_allowlist_keys().to_vec(),
            tools: server.tools().to_vec(),
            optional: server.is_optional(),
        }
    }

    pub(crate) fn registered_tool_courier(
        command: String,
        args: Vec<String>,
        tools: Vec<String>,
    ) -> Self {
        Self {
            name: CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER.to_owned(),
            command,
            args,
            env_allowlist_keys: vec!["PATH".to_owned()],
            tools,
            optional: false,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn command(&self) -> &str {
        &self.command
    }

    pub(crate) fn args(&self) -> &[String] {
        &self.args
    }

    pub(crate) fn env_allowlist_keys(&self) -> &[String] {
        &self.env_allowlist_keys
    }

    pub(crate) fn tools(&self) -> &[String] {
        &self.tools
    }

    pub(crate) const fn is_optional(&self) -> bool {
        self.optional
    }

    pub(crate) fn admitted_tool_names(&self) -> Vec<String> {
        self.tools
            .iter()
            .map(|tool| mcp_tool_name(&self.name, tool))
            .collect()
    }
}

pub(crate) fn combine_open_servers(
    servers: &[ClaudeAgentSdkMcpServer],
    registered: Option<OpenStdioMcpServer>,
) -> Vec<OpenStdioMcpServer> {
    let mut open: Vec<OpenStdioMcpServer> = servers
        .iter()
        .map(OpenStdioMcpServer::from_consumer)
        .collect();
    if let Some(registered) = registered {
        open.push(registered);
    }
    open
}

pub(crate) fn admitted_open_tool_names(
    profile: &ClaudeAgentSdkSessionProfile,
    servers: &[OpenStdioMcpServer],
) -> Vec<String> {
    let mut names: Vec<String> = profile
        .tools()
        .map(|tool| tool.as_str().to_owned())
        .collect();
    names.extend(
        servers
            .iter()
            .flat_map(OpenStdioMcpServer::admitted_tool_names),
    );
    names
}

pub(crate) fn admitted_open_mcp_tool_names(servers: &[OpenStdioMcpServer]) -> Vec<String> {
    servers
        .iter()
        .flat_map(OpenStdioMcpServer::admitted_tool_names)
        .collect()
}

pub(crate) fn mcp_tool_name(server: &str, tool: &str) -> String {
    format!("mcp__{server}__{tool}")
}

fn env_key_allowed(key: &str) -> bool {
    is_bounded_text(key, MAXIMUM_NAME_BYTES)
        && (MCP_CHILD_ENV_EXACT_KEYS.contains(&key) || key.starts_with("LC_"))
}

fn is_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    value.len() <= MAXIMUM_NAME_BYTES
        && !value.chars().any(char::is_control)
        && first.is_ascii_alphanumeric()
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        })
}

fn is_bounded_text(value: &str, maximum: usize) -> bool {
    !value.is_empty() && value.len() <= maximum && !value.chars().any(char::is_control)
}

fn mcp_failure(code: &'static str, message: &'static str) -> PreparationFailure {
    preparation_failure(PreparationStage::Preflight, code, message)
}

#[cfg(test)]
mod mcp_tests {
    use super::{
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER, ClaudeAgentSdkMcpServer, WATCHER_SERVER_NAME,
    };
    use crate::sdk::profile::ClaudeAgentSdkSessionProfile;

    #[test]
    fn a_stdio_server_round_trips_its_admitted_mcp_tool_names() {
        let server = ClaudeAgentSdkMcpServer::stdio(
            "docs",
            "/usr/bin/node",
            ["server.mjs"],
            ["PATH", "HOME"],
            ["search"],
        )
        .expect("a stdio declaration is admissible");
        assert_eq!(server.name(), "docs");
        assert_eq!(server.command(), "/usr/bin/node");
        assert_eq!(server.args(), ["server.mjs"]);
        assert_eq!(server.env_allowlist_keys(), ["PATH", "HOME"]);
        assert_eq!(server.tools(), ["search"]);
        assert!(!server.is_optional());
        assert_eq!(server.admitted_tool_names(), ["mcp__docs__search"]);
        assert!(server.optional().is_optional());
    }

    #[test]
    fn the_reserved_watcher_server_name_is_rejected_before_construction() {
        let failure = ClaudeAgentSdkMcpServer::stdio(
            WATCHER_SERVER_NAME,
            "/usr/bin/node",
            std::iter::empty::<String>(),
            std::iter::empty::<String>(),
            ["start"],
        )
        .expect_err("the watcher MCP family is a different route");
        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.claude-agent.sdk.profile.mcp_server_forbidden"
        );
    }

    #[test]
    fn the_reserved_registered_tool_carrier_name_is_rejected_before_construction() {
        let failure = ClaudeAgentSdkMcpServer::stdio(
            CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER,
            "/usr/bin/node",
            std::iter::empty::<String>(),
            std::iter::empty::<String>(),
            ["start"],
        )
        .expect_err("the Swallowtail-owned carrier is a different path");
        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.claude-agent.sdk.profile.mcp_server_forbidden"
        );
    }

    #[test]
    fn an_env_key_outside_the_child_allowlist_is_rejected() {
        let failure = ClaudeAgentSdkMcpServer::stdio(
            "docs",
            "/usr/bin/node",
            std::iter::empty::<String>(),
            ["ANTHROPIC_API_KEY"],
            ["search"],
        )
        .expect_err("credentials cannot be allowlisted");
        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.claude-agent.sdk.profile.mcp_env_key_rejected"
        );
    }

    #[test]
    fn a_repeated_server_name_is_rejected_on_the_profile_binding() {
        let docs = ClaudeAgentSdkMcpServer::stdio(
            "docs",
            "/usr/bin/node",
            std::iter::empty::<String>(),
            std::iter::empty::<String>(),
            ["search"],
        )
        .expect("first declaration is admissible");
        let again = ClaudeAgentSdkMcpServer::stdio(
            "docs",
            "/usr/bin/node",
            std::iter::empty::<String>(),
            std::iter::empty::<String>(),
            ["fetch"],
        )
        .expect("the type itself allows the name");
        let failure = ClaudeAgentSdkSessionProfile::read_only()
            .with_mcp_servers([docs, again])
            .expect_err("repeated names fail on the binding");
        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.claude-agent.sdk.profile.mcp_server_repeated"
        );
    }
}
