use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{CleanupOutcome, TerminalStatus};

/// Server-side methods observed for one disposable MCP run.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HttpMcpTranscript {
    initialize: bool,
    tools_list: bool,
    tools_call: bool,
    authenticated: bool,
    result: Option<String>,
}

impl HttpMcpTranscript {
    pub(crate) fn record_method(&mut self, method: &str) {
        match method {
            "initialize" => self.initialize = true,
            "tools/list" => self.tools_list = true,
            "tools/call" => self.tools_call = true,
            _ => {}
        }
    }

    pub(crate) fn mark_authenticated(&mut self) {
        self.authenticated = true;
    }

    pub(crate) fn record_result(&mut self, result: String) {
        self.result = Some(result);
    }

    /// MCP `initialize` reached this listener with a matching bearer.
    #[must_use]
    pub const fn connected(&self) -> bool {
        self.authenticated && self.initialize
    }

    /// MCP `tools/list` reached this listener after authenticate.
    #[must_use]
    pub const fn tools_listed(&self) -> bool {
        self.tools_list
    }

    /// MCP `tools/call` reached this listener.
    #[must_use]
    pub const fn tool_called(&self) -> bool {
        self.tools_call
    }

    /// Deterministic tool result observed by the listener.
    #[must_use]
    pub fn tool_result(&self) -> Option<&str> {
        self.result.as_deref()
    }
}

/// Cleanup classification kept for every attempt, independent of the stop name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpMcpCleanupClass {
    /// No session existed, so cleanup was never attempted.
    NotAttempted,
    /// Every applicable cleanup action completed.
    Clean,
    /// Cleanup completed with a non-fatal degradation.
    Degraded,
    /// A required cleanup action failed.
    Failed,
    /// No cleanup action applied to this session.
    NotApplicable,
}

impl HttpMcpCleanupClass {
    /// Stable short name for the live record line.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAttempted => "not_attempted",
            Self::Clean => "clean",
            Self::Degraded => "degraded",
            Self::Failed => "failed",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// One probe attempt's honouring record. Debug redacts secrets by construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpMcpLiveRecord {
    declaration_sent: bool,
    connected: bool,
    tools_listed: bool,
    tool_called: bool,
    tool_result: bool,
    terminal_status: &'static str,
    terminal_code: Option<String>,
    terminal_completed: bool,
    cleanup_clean: bool,
    cleanup_class: HttpMcpCleanupClass,
    cleanup_code: Option<String>,
    cleanup_stage: Option<String>,
    stop: Option<HttpMcpLiveStop>,
    model: Option<String>,
}

/// Named failure when the one authorized attempt did not honour the entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpMcpLiveStop {
    /// Host executable was not exact `0.81.2`.
    HostVersion,
    /// No already-configured model was usable without a login.
    NoUsableModel,
    /// Claude Agent ACP required host-owned login during the attempt.
    HostAuthRequired,
    /// The turn ended on a permission observation instead of a tool call.
    PermissionObserved,
    /// The MCP listener never saw an authenticated initialize.
    McpNotConnected,
    /// Tools were not listed after connect.
    ToolsNotListed,
    /// Connect and list happened, but the deterministic tool was not called.
    ToolNotCalled,
    /// The tool was called but the deterministic result did not reach the turn.
    ToolResultMissing,
    /// The turn finished without `Completed`.
    TurnNotCompleted,
    /// Session cleanup was not `Clean`.
    CleanupFailed,
}

impl HttpMcpLiveRecord {
    #[must_use]
    pub fn from_attempt(
        declaration_sent: bool,
        transcript: &HttpMcpTranscript,
        terminal: &TerminalStatus,
        cleanup: CleanupOutcome,
        model: Option<String>,
    ) -> Self {
        let connected = transcript.connected();
        let tools_listed = transcript.tools_listed();
        let tool_called = transcript.tool_called();
        let tool_result = transcript.tool_result() == Some(super::HTTP_MCP_LIVE_TOOL_RESULT);
        let terminal_completed = matches!(terminal, TerminalStatus::Completed);
        let terminal_status = terminal_status(terminal);
        let terminal_code = terminal
            .failure()
            .map(|failure| failure.diagnostic().code().to_owned());
        let cleanup_clean = matches!(cleanup, CleanupOutcome::Clean);
        let kept_cleanup = KeptCleanup::from_outcome(&cleanup);
        let stop = if !connected {
            Some(HttpMcpLiveStop::McpNotConnected)
        } else if !tools_listed {
            Some(HttpMcpLiveStop::ToolsNotListed)
        } else if matches!(terminal, TerminalStatus::ProviderRequestObserved(_)) {
            Some(HttpMcpLiveStop::PermissionObserved)
        } else if terminal.failure().is_some_and(|failure| {
            failure.diagnostic().code() == "swallowtail.claude_agent.acp.terminal_auth_rejected"
        }) {
            Some(HttpMcpLiveStop::HostAuthRequired)
        } else if !tool_called {
            Some(HttpMcpLiveStop::ToolNotCalled)
        } else if !tool_result {
            Some(HttpMcpLiveStop::ToolResultMissing)
        } else if !terminal_completed {
            Some(HttpMcpLiveStop::TurnNotCompleted)
        } else if !cleanup_clean {
            Some(HttpMcpLiveStop::CleanupFailed)
        } else {
            None
        };
        Self {
            declaration_sent,
            connected,
            tools_listed,
            tool_called,
            tool_result,
            terminal_status,
            terminal_code,
            terminal_completed,
            cleanup_clean,
            cleanup_class: kept_cleanup.class,
            cleanup_code: kept_cleanup.code,
            cleanup_stage: kept_cleanup.stage,
            stop,
            model,
        }
    }

    #[must_use]
    pub const fn pre_attempt_stop(stop: HttpMcpLiveStop, model: Option<String>) -> Self {
        Self {
            declaration_sent: false,
            connected: false,
            tools_listed: false,
            tool_called: false,
            tool_result: false,
            terminal_status: "not_started",
            terminal_code: None,
            terminal_completed: false,
            cleanup_clean: false,
            cleanup_class: HttpMcpCleanupClass::NotAttempted,
            cleanup_code: None,
            cleanup_stage: None,
            stop: Some(stop),
            model,
        }
    }

    /// Declaration was present on production `session/new`.
    #[must_use]
    pub const fn declaration_sent(&self) -> bool {
        self.declaration_sent
    }

    /// The disposable server logged an authenticated connect.
    #[must_use]
    pub const fn connected(&self) -> bool {
        self.connected
    }

    /// The disposable server listed its tool.
    #[must_use]
    pub const fn tools_listed(&self) -> bool {
        self.tools_listed
    }

    /// The disposable server received the deterministic tool call.
    #[must_use]
    pub const fn tool_called(&self) -> bool {
        self.tool_called
    }

    /// The deterministic result was recorded server-side.
    #[must_use]
    pub const fn tool_result(&self) -> bool {
        self.tool_result
    }

    /// Turn completed.
    #[must_use]
    pub const fn terminal_completed(&self) -> bool {
        self.terminal_completed
    }

    /// Stable terminal status name, without provider request or message data.
    #[must_use]
    pub(crate) const fn terminal_status(&self) -> &'static str {
        self.terminal_status
    }

    /// Typed terminal diagnostic code, when terminal status is a failure.
    #[must_use]
    pub(crate) fn terminal_diagnostic_code(&self) -> Option<&str> {
        self.terminal_code.as_deref()
    }

    /// Session cleanup joined clean.
    #[must_use]
    pub const fn cleanup_clean(&self) -> bool {
        self.cleanup_clean
    }

    /// Typed cleanup classification for this attempt.
    #[must_use]
    pub const fn cleanup_class(&self) -> HttpMcpCleanupClass {
        self.cleanup_class
    }

    /// Exact cleanup diagnostic code when cleanup degraded or failed.
    ///
    /// `None` for clean, not-applicable, and not-attempted cleanup.
    #[must_use]
    pub fn cleanup_diagnostic_code(&self) -> Option<&str> {
        self.cleanup_code.as_deref()
    }

    /// Adapter-owned cleanup stage tag when the diagnostic carried one.
    ///
    /// The tag is a fixed lowercase identifier the adapter appends to a
    /// `cleanup_failed` message; the message body itself is never kept.
    #[must_use]
    pub fn cleanup_diagnostic_stage(&self) -> Option<&str> {
        self.cleanup_stage.as_deref()
    }

    /// `None` when the tuple is accepted.
    #[must_use]
    pub const fn stop(&self) -> Option<HttpMcpLiveStop> {
        self.stop
    }

    /// Exact model string when one was known. Never a credential.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    pub(crate) fn force_stop(&mut self, stop: HttpMcpLiveStop) {
        self.stop = Some(stop);
    }

    /// Honouring succeeded: declaration, connect, list, call, result, completed, clean.
    #[must_use]
    pub const fn accepted(&self) -> bool {
        self.declaration_sent
            && self.connected
            && self.tools_listed
            && self.tool_called
            && self.tool_result
            && self.terminal_completed
            && self.cleanup_clean
            && self.stop.is_none()
    }

    /// Serializes only the secret-free evidence required to classify the attempt.
    #[must_use]
    pub(crate) fn to_json_line(&self) -> String {
        serde_json::json!({
            "schema_version": 1,
            "accepted": self.accepted(),
            "stop": self.stop().map(HttpMcpLiveStop::as_str),
            "model": self.model(),
            "declaration_sent": self.declaration_sent(),
            "connected": self.connected(),
            "tools_listed": self.tools_listed(),
            "tool_called": self.tool_called(),
            "tool_result": self.tool_result(),
            "terminal": {
                "status": self.terminal_status(),
                "code": self.terminal_diagnostic_code(),
            },
            "cleanup": {
                "class": self.cleanup_class().as_str(),
                "code": self.cleanup_diagnostic_code(),
                "stage": self.cleanup_diagnostic_stage(),
            },
        })
        .to_string()
    }
}

fn terminal_status(status: &TerminalStatus) -> &'static str {
    match status {
        TerminalStatus::Completed => "completed",
        TerminalStatus::Detached => "detached",
        TerminalStatus::Cancelled => "cancelled",
        TerminalStatus::TimedOut => "timed_out",
        TerminalStatus::ProviderRequestObserved(_) => "provider_request_observed",
        TerminalStatus::ProviderFailed(_) => "provider_failed",
        TerminalStatus::HostFailed(_) => "host_failed",
        TerminalStatus::RuntimeFailed(_) => "runtime_failed",
    }
}

/// Persists a record and writes the same JSON line to the selected output.
///
/// Any persistence or output error is returned so the live gate can fail
/// closed. The output includes only redacted typed evidence, never a raw
/// provider stream, bearer, account, session id, or host path.
pub(crate) fn persist_and_print_record(
    record: &HttpMcpLiveRecord,
    record_path: &Path,
    output: &mut impl Write,
) -> io::Result<()> {
    if let Some(parent) = record_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = record.to_json_line();
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(record_path)?;
    file.write_all(json.as_bytes())?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    drop(file);
    if !record_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "live gate record file was not written",
        ));
    }
    writeln!(output, "CLAUDE_AGENT_ACP_HTTP_MCP_RECORD={json}")?;
    Ok(())
}

/// Secret-free cleanup evidence kept from one [`CleanupOutcome`].
#[derive(Debug)]
struct KeptCleanup {
    class: HttpMcpCleanupClass,
    code: Option<String>,
    stage: Option<String>,
}

impl KeptCleanup {
    fn from_outcome(outcome: &CleanupOutcome) -> Self {
        match outcome {
            CleanupOutcome::Clean => Self {
                class: HttpMcpCleanupClass::Clean,
                code: None,
                stage: None,
            },
            CleanupOutcome::NotApplicable => Self {
                class: HttpMcpCleanupClass::NotApplicable,
                code: None,
                stage: None,
            },
            CleanupOutcome::Degraded(diagnostic) => {
                Self::from_diagnostic(HttpMcpCleanupClass::Degraded, diagnostic)
            }
            CleanupOutcome::Failed(diagnostic) => {
                Self::from_diagnostic(HttpMcpCleanupClass::Failed, diagnostic)
            }
        }
    }

    fn from_diagnostic(class: HttpMcpCleanupClass, diagnostic: &SafeDiagnostic) -> Self {
        Self {
            class,
            code: Some(diagnostic.code().to_owned()),
            stage: cleanup_stage(diagnostic.message()),
        }
    }
}

/// Keeps only the adapter's trailing stage tag from a `cleanup_failed` message
/// (`... (task_join_failed)`). A stage is an adapter-owned identifier; a message
/// body may carry host text, so it is never retained.
fn cleanup_stage(message: &str) -> Option<String> {
    let (_, tail) = message.rsplit_once(" (")?;
    let stage = tail.strip_suffix(')')?;
    (!stage.is_empty()
        && stage.len() <= 40
        && stage
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'_'))
    .then(|| stage.to_owned())
}

impl HttpMcpLiveStop {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostVersion => "host_version",
            Self::NoUsableModel => "no_usable_model",
            Self::HostAuthRequired => "host_auth_required",
            Self::PermissionObserved => "permission_observed",
            Self::McpNotConnected => "mcp_not_connected",
            Self::ToolsNotListed => "tools_not_listed",
            Self::ToolNotCalled => "tool_not_called",
            Self::ToolResultMissing => "tool_result_missing",
            Self::TurnNotCompleted => "turn_not_completed",
            Self::CleanupFailed => "cleanup_failed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kept_cleanup_classifies_every_outcome() {
        let cases = [
            (CleanupOutcome::Clean, HttpMcpCleanupClass::Clean),
            (
                CleanupOutcome::NotApplicable,
                HttpMcpCleanupClass::NotApplicable,
            ),
            (
                CleanupOutcome::Degraded(SafeDiagnostic::new("fixture.degraded", "degraded")),
                HttpMcpCleanupClass::Degraded,
            ),
            (
                CleanupOutcome::Failed(SafeDiagnostic::new("fixture.failed", "failed")),
                HttpMcpCleanupClass::Failed,
            ),
        ];
        for (outcome, expected) in cases {
            assert_eq!(KeptCleanup::from_outcome(&outcome).class, expected);
        }
    }

    #[test]
    fn kept_cleanup_keeps_code_and_stage_but_never_the_message_body() {
        let diagnostic = SafeDiagnostic::new(
            "swallowtail.claude_agent.acp.cleanup_failed",
            "Claude Agent ACP protocol task did not join (task_join_failed)",
        );
        let kept = KeptCleanup::from_outcome(&CleanupOutcome::Failed(diagnostic));
        assert_eq!(kept.class, HttpMcpCleanupClass::Failed);
        assert_eq!(
            kept.code.as_deref(),
            Some("swallowtail.claude_agent.acp.cleanup_failed")
        );
        assert_eq!(kept.stage.as_deref(), Some("task_join_failed"));
        assert!(!format!("{kept:?}").contains("did not join"));
    }

    #[test]
    fn cleanup_stage_rejects_provider_or_host_text() {
        assert_eq!(cleanup_stage("plain message without a tag"), None);
        assert_eq!(cleanup_stage("failed at /private/dir ()"), None);
        assert_eq!(cleanup_stage("failed on /private/dir (Not A Tag)"), None);
        assert_eq!(cleanup_stage("failed while reading /private/dir"), None);
        assert_eq!(cleanup_stage("failed on a private path"), None);
    }
}
