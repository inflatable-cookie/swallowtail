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

/// Secret-free code and class for one stop diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpMcpLiveStopDiagnostic {
    code: String,
    class: String,
}

impl HttpMcpLiveStopDiagnostic {
    fn from_safe(diagnostic: &SafeDiagnostic) -> Self {
        let classification = diagnostic.failure_classification();
        Self {
            code: diagnostic.code().to_owned(),
            class: format!(
                "{:?}/{:?}/{:?}",
                classification.origin(),
                classification.kind(),
                classification.recovery()
            ),
        }
    }

    /// Stable diagnostic code. Never a secret.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Portable classification axes. Never a secret.
    #[must_use]
    pub fn class(&self) -> &str {
        &self.class
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
    terminal_completed: bool,
    cleanup_clean: bool,
    stop: Option<HttpMcpLiveStop>,
    terminal_diagnostic: Option<HttpMcpLiveStopDiagnostic>,
    cleanup_diagnostic: Option<HttpMcpLiveStopDiagnostic>,
    model: Option<String>,
}

/// Named failure when the one authorized attempt did not honour the entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpMcpLiveStop {
    /// Isolated executable was not exact `0.59.0`.
    HostVersion,
    /// No already-configured model was usable without a login.
    NoUsableModel,
    /// Gemini CLI required host-owned login or auth change during the attempt.
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
        let cleanup_clean = matches!(cleanup, CleanupOutcome::Clean);
        let terminal_diagnostic = terminal
            .failure()
            .map(|failure| HttpMcpLiveStopDiagnostic::from_safe(failure.diagnostic()));
        let cleanup_diagnostic = cleanup
            .diagnostic()
            .map(HttpMcpLiveStopDiagnostic::from_safe);
        let stop = if !connected {
            Some(HttpMcpLiveStop::McpNotConnected)
        } else if !tools_listed {
            Some(HttpMcpLiveStop::ToolsNotListed)
        } else if matches!(terminal, TerminalStatus::ProviderRequestObserved(_)) {
            Some(HttpMcpLiveStop::PermissionObserved)
        } else if terminal.failure().is_some_and(|failure| {
            failure.diagnostic().code() == "swallowtail.gemini.acp.auth_required"
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
            terminal_completed,
            cleanup_clean,
            stop,
            terminal_diagnostic,
            cleanup_diagnostic,
            model,
        }
    }

    #[must_use]
    pub fn pre_attempt_stop(
        stop: HttpMcpLiveStop,
        diagnostic: SafeDiagnostic,
        model: Option<String>,
    ) -> Self {
        Self {
            declaration_sent: false,
            connected: false,
            tools_listed: false,
            tool_called: false,
            tool_result: false,
            terminal_completed: false,
            cleanup_clean: false,
            stop: Some(stop),
            terminal_diagnostic: Some(HttpMcpLiveStopDiagnostic::from_safe(&diagnostic)),
            cleanup_diagnostic: None,
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

    /// Session cleanup joined clean.
    #[must_use]
    pub const fn cleanup_clean(&self) -> bool {
        self.cleanup_clean
    }

    /// `None` when the tuple is accepted.
    #[must_use]
    pub const fn stop(&self) -> Option<HttpMcpLiveStop> {
        self.stop
    }

    /// Terminal failure diagnostic when one was present. Secret-free.
    #[must_use]
    pub const fn terminal_diagnostic(&self) -> Option<&HttpMcpLiveStopDiagnostic> {
        self.terminal_diagnostic.as_ref()
    }

    /// Cleanup diagnostic when cleanup was not `Clean`. Secret-free.
    #[must_use]
    pub const fn cleanup_diagnostic(&self) -> Option<&HttpMcpLiveStopDiagnostic> {
        self.cleanup_diagnostic.as_ref()
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
