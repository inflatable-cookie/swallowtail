//! Provider-free Grok ACP client-MCP probe harness.
//!
//! Verdicts are decided from captured frames only. Crate tests drive the fake
//! ACP fixture. The live installed-Grok entrypoint refuses to spawn unless
//! Desktop sets [`DESKTOP_GROK_ACP_CLIENT_MCP_PROBE_GATE`].

use serde_json::{Value, json};
use std::collections::VecDeque;
use std::fmt;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;
use swallowtail_protocol_acp::{ACP_PROTOCOL_VERSION, encode_error, encode_request};

/// Environment variable Desktop must set to `1` before the live Grok path.
pub const DESKTOP_GROK_ACP_CLIENT_MCP_PROBE_GATE: &str =
    "SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE";
/// Disposable stdio MCP server name sent on `session/new`.
pub const ECHO_MCP_SERVER_NAME: &str = "swallowtail-echo";
/// Sole tool exported by the disposable MCP server.
pub const ECHO_MCP_TOOL: &str = "echo";

const MAXIMUM_FRAMES: usize = 48;
const ECHO_PROMPT: &str = "Call the echo tool with text ping and return its result.";
const FIXTURE_SESSION: &str = "grok-fixture-session";
const FIXTURE_CWD: &str = "<host-approved-resource>";
const FIXTURE_COMMAND: &str = "<redacted-command>";
const STALE_CALLBACK_ID: u64 = 9001;
const LIVE_IDLE: Duration = Duration::from_millis(200);
const LIVE_WAIT: Duration = Duration::from_secs(8);
const LIVE_JOIN: Duration = Duration::from_secs(2);

/// Frame-decided client-MCP verdict for one exact Grok version segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientMcpVerdict {
    /// Session accepted a non-empty `mcpServers` list, discovered echo, and called it.
    AcceptsClientMcp,
    /// Session accepted a non-empty `mcpServers` list and never called echo.
    IgnoresClientMcp,
    /// Session setup rejected the non-empty `mcpServers` list.
    RejectsClientMcp,
    /// Frames are missing, incomplete, or contradictory.
    Inconclusive,
}

impl ClientMcpVerdict {
    /// Returns the capsule spelling for this verdict.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcceptsClientMcp => "accepts_client_mcp",
            Self::IgnoresClientMcp => "ignores_client_mcp",
            Self::RejectsClientMcp => "rejects_client_mcp",
            Self::Inconclusive => "inconclusive",
        }
    }

    /// Parses the capsule spelling.
    pub fn from_capsule(value: &str) -> Result<Self, GrokAcpClientMcpError> {
        match value {
            "accepts_client_mcp" => Ok(Self::AcceptsClientMcp),
            "ignores_client_mcp" => Ok(Self::IgnoresClientMcp),
            "rejects_client_mcp" => Ok(Self::RejectsClientMcp),
            "inconclusive" => Ok(Self::Inconclusive),
            _ => Err(GrokAcpClientMcpError {
                kind: GrokAcpClientMcpErrorKind::InvalidVerdict,
            }),
        }
    }
}

/// One captured ACP JSON-RPC message after redaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokAcpClientMcpFrame {
    direction: FrameDirection,
    message: Value,
}

impl GrokAcpClientMcpFrame {
    /// Returns whether the frame was sent by the harness.
    #[must_use]
    pub const fn is_outbound(&self) -> bool {
        matches!(self.direction, FrameDirection::Outbound)
    }

    /// Returns the redacted JSON-RPC object.
    #[must_use]
    pub const fn message(&self) -> &Value {
        &self.message
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FrameDirection {
    Outbound,
    Inbound,
}

/// Joined-cleanup observation recorded after disconnect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GrokAcpClientMcpCleanup {
    joined: bool,
}

impl GrokAcpClientMcpCleanup {
    /// Returns whether the peer wait completed.
    #[must_use]
    pub const fn joined(self) -> bool {
        self.joined
    }
}

/// Redacted probe capsule for one exact version segment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokAcpClientMcpCapsule {
    version: String,
    verdict: ClientMcpVerdict,
    frames: Vec<GrokAcpClientMcpFrame>,
    stale_callback_rejected: bool,
    cleanup: GrokAcpClientMcpCleanup,
    truncated: bool,
}

impl GrokAcpClientMcpCapsule {
    /// Exact version segment recorded on the capsule.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Frame-decided verdict.
    #[must_use]
    pub const fn verdict(&self) -> ClientMcpVerdict {
        self.verdict
    }

    /// Redacted captured frames, including the `session/new` request when sent.
    #[must_use]
    pub fn frames(&self) -> &[GrokAcpClientMcpFrame] {
        &self.frames
    }

    /// Returns whether a post-close callback was rejected.
    #[must_use]
    pub const fn stale_callback_rejected(&self) -> bool {
        self.stale_callback_rejected
    }

    /// Returns joined-cleanup evidence.
    #[must_use]
    pub const fn cleanup(&self) -> GrokAcpClientMcpCleanup {
        self.cleanup
    }

    /// Returns whether capture stopped at the frame bound.
    #[must_use]
    pub const fn truncated(&self) -> bool {
        self.truncated
    }

    /// Bounded redacted JSON object with exact frames and the verdict.
    #[must_use]
    pub fn to_json(&self) -> Value {
        json!({
            "route": "grok-build.acp",
            "version": self.version,
            "verdict": self.verdict.as_str(),
            "stale_callback_rejected": self.stale_callback_rejected,
            "cleanup_joined": self.cleanup.joined,
            "truncated": self.truncated,
            "frames": self.frames.iter().map(|frame| {
                json!({
                    "direction": match frame.direction {
                        FrameDirection::Outbound => "outbound",
                        FrameDirection::Inbound => "inbound",
                    },
                    "message": frame.message,
                })
            }).collect::<Vec<_>>(),
        })
    }
}

/// Stable classification for a probe failure before a capsule can be written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrokAcpClientMcpErrorKind {
    /// Live Grok spawn was requested without the Desktop gate.
    LiveProbeGated,
    /// Live executable or echo-server path was missing.
    LivePathMissing,
    /// ACP framing or JSON-RPC codec failed.
    Transport,
    /// Captured frame count exceeded the harness bound.
    FrameLimit,
    /// Capsule spelling was not one of the four verdicts.
    InvalidVerdict,
}

/// Bounded probe error without provider payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GrokAcpClientMcpError {
    kind: GrokAcpClientMcpErrorKind,
}

impl GrokAcpClientMcpError {
    /// Returns the stable failure classification.
    #[must_use]
    pub const fn kind(self) -> GrokAcpClientMcpErrorKind {
        self.kind
    }
}

impl fmt::Display for GrokAcpClientMcpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            GrokAcpClientMcpErrorKind::LiveProbeGated => {
                "live Grok ACP client-MCP probe requires the Desktop gate"
            }
            GrokAcpClientMcpErrorKind::LivePathMissing => {
                "live Grok ACP client-MCP probe is missing an executable path"
            }
            GrokAcpClientMcpErrorKind::Transport => "ACP client-MCP probe transport failed",
            GrokAcpClientMcpErrorKind::FrameLimit => {
                "ACP client-MCP probe exceeded the frame capture bound"
            }
            GrokAcpClientMcpErrorKind::InvalidVerdict => {
                "ACP client-MCP probe verdict spelling is invalid"
            }
        })
    }
}

impl std::error::Error for GrokAcpClientMcpError {}

/// ACP peer used by the probe. Fake fixtures never start a process.
pub trait GrokAcpClientMcpPeer {
    /// Writes one JSON-RPC object to the agent.
    fn push_outbound(&mut self, message: Value) -> Result<(), GrokAcpClientMcpError>;
    /// Returns every inbound object now available.
    fn take_inbound(&mut self) -> Result<Vec<Value>, GrokAcpClientMcpError>;
    /// Disconnects and reports whether the wait joined.
    fn close(&mut self) -> GrokAcpClientMcpCleanup;
    /// Optional post-close callback the harness must reject.
    fn stale_callback_request(&mut self) -> Option<Value>;
}

/// Returns whether Desktop has opened the live installed-Grok gate.
#[must_use]
pub fn desktop_grok_acp_client_mcp_probe_is_gated_open() -> bool {
    std::env::var(DESKTOP_GROK_ACP_CLIENT_MCP_PROBE_GATE).as_deref() == Ok("1")
}

/// Runs the fake ACP fixture that proves `expected` for one version segment.
pub fn grok_acp_client_mcp_fixture_probe(
    version: &str,
    expected: ClientMcpVerdict,
) -> Result<GrokAcpClientMcpCapsule, GrokAcpClientMcpError> {
    let mut peer = FakeAcpPeer::new(expected);
    let capsule = run_grok_acp_client_mcp_probe(&mut peer, version, FIXTURE_COMMAND, FIXTURE_CWD)?;
    if capsule.verdict != expected {
        return Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::InvalidVerdict,
        });
    }
    Ok(capsule)
}

/// Drives ACP initialize, non-empty `session/new`, one echo prompt, cleanup, and
/// stale-callback rejection. The verdict is computed from the captured frames.
pub fn run_grok_acp_client_mcp_probe(
    peer: &mut dyn GrokAcpClientMcpPeer,
    version: &str,
    echo_command: &str,
    cwd: &str,
) -> Result<GrokAcpClientMcpCapsule, GrokAcpClientMcpError> {
    let mut capture = FrameCapture::new();
    let mut next_id = 1_u64;

    let initialize = json!({
        "protocolVersion": ACP_PROTOCOL_VERSION,
        "clientCapabilities": {
            "fs": {"readTextFile": false, "writeTextFile": false},
            "terminal": false
        },
        "clientInfo": {
            "name": "swallowtail-grok-acp-client-mcp-probe",
            "title": "Swallowtail Grok ACP Client-MCP Probe",
            "version": "0.0.0"
        }
    });
    let initialize_id = next_id;
    exchange(peer, &mut capture, initialize_id, "initialize", initialize)?;
    next_id += 1;

    if request_succeeded(&capture.frames, initialize_id) {
        let authenticate_id = next_id;
        exchange(
            peer,
            &mut capture,
            authenticate_id,
            "authenticate",
            json!({"methodId": "cached_token", "_meta": {"headless": true}}),
        )?;
        next_id += 1;
        if request_succeeded(&capture.frames, authenticate_id) {
            let session_new_id = next_id;
            exchange(
                peer,
                &mut capture,
                session_new_id,
                "session/new",
                json!({
                    "cwd": cwd,
                    "mcpServers": [{
                        "name": ECHO_MCP_SERVER_NAME,
                        "command": echo_command,
                        "args": [],
                        "env": []
                    }]
                }),
            )?;
            next_id += 1;
            if let Some(session) = session_id_from(&capture.frames, session_new_id) {
                exchange(
                    peer,
                    &mut capture,
                    next_id,
                    "session/prompt",
                    json!({
                        "sessionId": session,
                        "prompt": [{"type": "text", "text": ECHO_PROMPT}]
                    }),
                )?;
                answer_callbacks(peer, &mut capture, &session)?;
            }
        }
    }

    let stale_callback_rejected = reject_stale_callback(peer, &mut capture)?;
    let cleanup = peer.close();
    let truncated = capture.truncated;
    let redacted: Vec<GrokAcpClientMcpFrame> = capture
        .frames
        .into_iter()
        .map(|frame| GrokAcpClientMcpFrame {
            direction: frame.direction,
            message: redact_value(frame.message),
        })
        .collect();
    let verdict = if truncated {
        ClientMcpVerdict::Inconclusive
    } else {
        grok_acp_client_mcp_verdict_from_frames(&redacted)
    };
    Ok(GrokAcpClientMcpCapsule {
        version: version.to_owned(),
        verdict,
        frames: redacted,
        stale_callback_rejected,
        cleanup,
        truncated,
    })
}

/// Decides the verdict from redacted frames. Missing `session/new` is inconclusive.
#[must_use]
pub fn grok_acp_client_mcp_verdict_from_frames(
    frames: &[GrokAcpClientMcpFrame],
) -> ClientMcpVerdict {
    let Some(session_new) = frames
        .iter()
        .find(|frame| frame.is_outbound() && method_of(&frame.message) == Some("session/new"))
    else {
        return ClientMcpVerdict::Inconclusive;
    };
    let servers = session_new
        .message
        .get("params")
        .and_then(|params| params.get("mcpServers"))
        .and_then(Value::as_array);
    if !servers.is_some_and(|list| !list.is_empty()) {
        return ClientMcpVerdict::Inconclusive;
    }
    let Some(id) = session_new.message.get("id").cloned() else {
        return ClientMcpVerdict::Inconclusive;
    };
    let response = frames
        .iter()
        .find(|frame| !frame.is_outbound() && frame.message.get("id") == Some(&id));
    match response {
        None => ClientMcpVerdict::Inconclusive,
        Some(frame) if frame.message.get("error").is_some() => {
            if error_rejects_client_mcp(frame.message.get("error").expect("error present")) {
                ClientMcpVerdict::RejectsClientMcp
            } else {
                ClientMcpVerdict::Inconclusive
            }
        }
        Some(frame)
            if frame
                .message
                .get("result")
                .and_then(|result| result.get("sessionId"))
                .and_then(Value::as_str)
                .is_some() =>
        {
            if echo_tool_called(frames) && echo_tool_completed(frames) {
                ClientMcpVerdict::AcceptsClientMcp
            } else if echo_tool_called(frames)
                || echo_named_tool_called(frames)
                || permission_was_rejected(frames)
            {
                ClientMcpVerdict::Inconclusive
            } else if prompt_turn_completed(frames) {
                ClientMcpVerdict::IgnoresClientMcp
            } else {
                ClientMcpVerdict::Inconclusive
            }
        }
        Some(_) => ClientMcpVerdict::Inconclusive,
    }
}

/// Answers one JSON-RPC MCP request for the disposable echo server.
///
/// Notifications return `None`. The handler has no filesystem or network path.
#[must_use]
pub fn grok_acp_echo_mcp_reply(request: &Value) -> Option<Value> {
    if request.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        return None;
    }
    let id = request.get("id").cloned()?;
    let method = request.get("method").and_then(Value::as_str)?;
    Some(match method {
        "initialize" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {"tools": {"listChanged": false}},
                "serverInfo": {"name": ECHO_MCP_SERVER_NAME, "version": "0.0.0"}
            }
        }),
        "tools/list" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [{
                    "name": ECHO_MCP_TOOL,
                    "description": "Echo the provided text. Non-mutating.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {"text": {"type": "string"}},
                        "required": ["text"],
                        "additionalProperties": false
                    }
                }]
            }
        }),
        "tools/call" => {
            let name = request
                .pointer("/params/name")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if name != ECHO_MCP_TOOL {
                return Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {"code": -32601, "message": "Method not found"}
                }));
            }
            let text = request
                .pointer("/params/arguments/text")
                .and_then(Value::as_str)
                .unwrap_or_default();
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{"type": "text", "text": text}],
                    "isError": false
                }
            })
        }
        "ping" => json!({"jsonrpc": "2.0", "id": id, "result": {}}),
        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32601, "message": "Method not found"}
        }),
    })
}

/// Encodes one MCP stdio frame as newline-delimited JSON.
///
/// MCP stdio forbids embedded newlines and does not use LSP Content-Length.
pub fn grok_acp_echo_mcp_stdio_frame(value: &Value) -> Result<Vec<u8>, GrokAcpClientMcpError> {
    let mut bytes = serde_json::to_vec(value).map_err(|_| GrokAcpClientMcpError {
        kind: GrokAcpClientMcpErrorKind::Transport,
    })?;
    if bytes.contains(&b'\n') {
        return Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        });
    }
    bytes.push(b'\n');
    Ok(bytes)
}

/// Opens a live installed-Grok ACP stdio peer. Refuses unless the Desktop gate is set.
pub fn open_desktop_live_grok_acp_peer(
    grok_executable: &Path,
    echo_mcp: &Path,
) -> Result<LiveGrokAcpPeer, GrokAcpClientMcpError> {
    if !desktop_grok_acp_client_mcp_probe_is_gated_open() {
        return Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::LiveProbeGated,
        });
    }
    if !grok_executable.is_file() || !echo_mcp.is_file() {
        return Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::LivePathMissing,
        });
    }
    LiveGrokAcpPeer::spawn(grok_executable)
}

/// Live ACP stdio child. Constructed only after the Desktop gate succeeds.
pub struct LiveGrokAcpPeer {
    child: Child,
    stdin: Option<ChildStdin>,
    incoming: Receiver<Result<Value, GrokAcpClientMcpError>>,
}

impl LiveGrokAcpPeer {
    fn spawn(executable: &Path) -> Result<Self, GrokAcpClientMcpError> {
        let mut command = Command::new(executable);
        command
            .args(["--no-auto-update", "agent", "stdio"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Ok(grok_home) = std::env::var("GROK_HOME") {
            command.env("HOME", &grok_home).env("GROK_HOME", grok_home);
        }
        let mut child = command.spawn().map_err(|_| GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
        let stdin = child.stdin.take().ok_or(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
        let stdout = child.stdout.take().ok_or(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
        if let Some(stderr) = child.stderr.take() {
            thread::spawn(move || {
                let mut reader = BufReader::new(stderr);
                let mut discarded = 0_usize;
                let mut line = String::new();
                while discarded < 64 * 1024 && reader.read_line(&mut line).unwrap_or(0) > 0 {
                    discarded = discarded.saturating_add(line.len());
                    line.clear();
                }
            });
        }
        let (sender, incoming) = mpsc::channel();
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut buffer = Vec::new();
            loop {
                buffer.clear();
                match reader.read_until(b'\n', &mut buffer) {
                    Ok(0) => break,
                    Ok(_) if buffer.len() > 64 * 1024 => {
                        let _ = sender.send(Err(GrokAcpClientMcpError {
                            kind: GrokAcpClientMcpErrorKind::FrameLimit,
                        }));
                        return;
                    }
                    Ok(_) => {
                        let line = buffer.strip_suffix(b"\n").unwrap_or(&buffer);
                        let line = line.strip_suffix(b"\r").unwrap_or(line);
                        if line.is_empty() {
                            continue;
                        }
                        let value =
                            serde_json::from_slice(line).map_err(|_| GrokAcpClientMcpError {
                                kind: GrokAcpClientMcpErrorKind::Transport,
                            });
                        if sender.send(value).is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        let _ = sender.send(Err(GrokAcpClientMcpError {
                            kind: GrokAcpClientMcpErrorKind::Transport,
                        }));
                        return;
                    }
                }
            }
        });
        Ok(Self {
            child,
            stdin: Some(stdin),
            incoming,
        })
    }
}

impl GrokAcpClientMcpPeer for LiveGrokAcpPeer {
    fn push_outbound(&mut self, message: Value) -> Result<(), GrokAcpClientMcpError> {
        let stdin = self.stdin.as_mut().ok_or(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
        let mut bytes = serde_json::to_vec(&message).map_err(|_| GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
        bytes.push(b'\n');
        stdin.write_all(&bytes).map_err(|_| GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
        stdin.flush().map_err(|_| GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })
    }

    fn take_inbound(&mut self) -> Result<Vec<Value>, GrokAcpClientMcpError> {
        let mut messages = Vec::new();
        match self.incoming.recv_timeout(LIVE_WAIT) {
            Ok(Ok(value)) => messages.push(value),
            Ok(Err(error)) => return Err(error),
            Err(RecvTimeoutError::Timeout | RecvTimeoutError::Disconnected) => return Ok(messages),
        }
        loop {
            match self.incoming.recv_timeout(LIVE_IDLE) {
                Ok(Ok(value)) => messages.push(value),
                Ok(Err(error)) => return Err(error),
                Err(RecvTimeoutError::Timeout | RecvTimeoutError::Disconnected) => break,
            }
        }
        Ok(messages)
    }

    fn close(&mut self) -> GrokAcpClientMcpCleanup {
        self.stdin.take();
        let started = std::time::Instant::now();
        while started.elapsed() < LIVE_JOIN {
            match self.child.try_wait() {
                Ok(Some(_)) => return GrokAcpClientMcpCleanup { joined: true },
                Ok(None) => thread::sleep(Duration::from_millis(50)),
                Err(_) => break,
            }
        }
        let _ = self.child.kill();
        GrokAcpClientMcpCleanup {
            joined: self.child.wait().is_ok(),
        }
    }

    fn stale_callback_request(&mut self) -> Option<Value> {
        None
    }
}

#[allow(dead_code)]
enum FakeBehavior {
    Verdict(ClientMcpVerdict),
    AuthenticateFailed,
    SessionUnauthorized,
    Chatty,
    PromptSilent,
    AsksPermission,
}

struct FakeAcpPeer {
    behavior: FakeBehavior,
    inbound: VecDeque<Value>,
    closed: bool,
    prompt_id: Option<Value>,
}

impl FakeAcpPeer {
    fn new(scenario: ClientMcpVerdict) -> Self {
        Self {
            behavior: FakeBehavior::Verdict(scenario),
            inbound: VecDeque::new(),
            closed: false,
            prompt_id: None,
        }
    }

    #[cfg(test)]
    fn authenticate_failed() -> Self {
        Self {
            behavior: FakeBehavior::AuthenticateFailed,
            inbound: VecDeque::new(),
            closed: false,
            prompt_id: None,
        }
    }

    #[cfg(test)]
    fn session_unauthorized() -> Self {
        Self {
            behavior: FakeBehavior::SessionUnauthorized,
            inbound: VecDeque::new(),
            closed: false,
            prompt_id: None,
        }
    }

    #[cfg(test)]
    fn chatty() -> Self {
        Self {
            behavior: FakeBehavior::Chatty,
            inbound: VecDeque::new(),
            closed: false,
            prompt_id: None,
        }
    }

    #[cfg(test)]
    fn prompt_silent() -> Self {
        Self {
            behavior: FakeBehavior::PromptSilent,
            inbound: VecDeque::new(),
            closed: false,
            prompt_id: None,
        }
    }

    #[cfg(test)]
    fn asks_permission() -> Self {
        Self {
            behavior: FakeBehavior::AsksPermission,
            inbound: VecDeque::new(),
            closed: false,
            prompt_id: None,
        }
    }

    fn push(&mut self, message: Value) {
        self.inbound.push_back(message);
    }

    fn emit_echo_accept(&mut self) {
        let prompt_id = self.prompt_id.clone();
        self.push(json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {
                "sessionId": FIXTURE_SESSION,
                "update": {
                    "sessionUpdate": "tool_call",
                    "toolCallId": "echo-1",
                    "title": ECHO_MCP_TOOL,
                    "kind": "other",
                    "status": "in_progress",
                    "content": [],
                    "_meta": {"mcpServerName": ECHO_MCP_SERVER_NAME}
                }
            }
        }));
        self.push(json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {
                "sessionId": FIXTURE_SESSION,
                "update": {
                    "sessionUpdate": "tool_call_update",
                    "toolCallId": "echo-1",
                    "status": "completed",
                    "content": [{
                        "type": "content",
                        "content": {"type": "text", "text": "ping"}
                    }]
                }
            }
        }));
        if let Some(id) = prompt_id {
            self.push(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {"stopReason": "end_turn"}
            }));
        }
    }
}

impl GrokAcpClientMcpPeer for FakeAcpPeer {
    fn push_outbound(&mut self, message: Value) -> Result<(), GrokAcpClientMcpError> {
        let id = message.get("id").cloned();
        match method_of(&message) {
            Some("initialize") => self.push(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": ACP_PROTOCOL_VERSION,
                    "agentCapabilities": {"loadSession": false},
                    "authMethods": [{"id": "cached_token", "name": "cached_token"}]
                }
            })),
            Some("authenticate") => match self.behavior {
                FakeBehavior::AuthenticateFailed => self.push(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {"code": -32000, "message": "not authenticated"}
                })),
                _ => self.push(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {}
                })),
            },
            Some("session/new") => match self.behavior {
                FakeBehavior::Verdict(ClientMcpVerdict::RejectsClientMcp) => self.push(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {"code": -32602, "message": "mcpServers are not supported"}
                })),
                FakeBehavior::SessionUnauthorized => self.push(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {"code": -32000, "message": "not authenticated"}
                })),
                FakeBehavior::Verdict(ClientMcpVerdict::Inconclusive)
                | FakeBehavior::AuthenticateFailed => {}
                FakeBehavior::Verdict(
                    ClientMcpVerdict::AcceptsClientMcp | ClientMcpVerdict::IgnoresClientMcp,
                )
                | FakeBehavior::Chatty
                | FakeBehavior::PromptSilent
                | FakeBehavior::AsksPermission => {
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"sessionId": FIXTURE_SESSION}
                    }));
                }
            },
            Some("session/prompt") => match self.behavior {
                FakeBehavior::Verdict(ClientMcpVerdict::AcceptsClientMcp) => {
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": FIXTURE_SESSION,
                            "update": {
                                "sessionUpdate": "available_commands_update",
                                "availableCommands": [{
                                    "name": ECHO_MCP_TOOL,
                                    "description": "echo"
                                }]
                            }
                        }
                    }));
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": FIXTURE_SESSION,
                            "update": {
                                "sessionUpdate": "tool_call",
                                "toolCallId": "echo-1",
                                "title": ECHO_MCP_TOOL,
                                "kind": "other",
                                "status": "in_progress",
                                "content": [],
                                "_meta": {"mcpServerName": ECHO_MCP_SERVER_NAME}
                            }
                        }
                    }));
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": FIXTURE_SESSION,
                            "update": {
                                "sessionUpdate": "tool_call_update",
                                "toolCallId": "echo-1",
                                "status": "completed",
                                "content": [{
                                    "type": "content",
                                    "content": {"type": "text", "text": "ping"}
                                }]
                            }
                        }
                    }));
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"stopReason": "end_turn"}
                    }));
                }
                FakeBehavior::Verdict(ClientMcpVerdict::IgnoresClientMcp) => {
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": FIXTURE_SESSION,
                            "update": {
                                "sessionUpdate": "agent_message_chunk",
                                "content": {"type": "text", "text": "no client mcp"}
                            }
                        }
                    }));
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"stopReason": "end_turn"}
                    }));
                }
                FakeBehavior::Chatty => {
                    for index in 0..MAXIMUM_FRAMES {
                        self.push(json!({
                            "jsonrpc": "2.0",
                            "method": "session/update",
                            "params": {
                                "sessionId": FIXTURE_SESSION,
                                "update": {
                                    "sessionUpdate": "agent_message_chunk",
                                    "content": {"type": "text", "text": index.to_string()}
                                }
                            }
                        }));
                    }
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {"stopReason": "end_turn"}
                    }));
                }
                FakeBehavior::PromptSilent => {}
                FakeBehavior::AsksPermission => {
                    self.prompt_id = id.clone();
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "id": 900,
                        "method": "session/request_permission",
                        "params": {
                            "sessionId": FIXTURE_SESSION,
                            "toolCall": {
                                "toolCallId": "echo-1",
                                "title": ECHO_MCP_TOOL
                            },
                            "options": [
                                {"optionId": "allow_once", "name": "Allow once", "kind": "allow_once"},
                                {"optionId": "reject_once", "name": "Reject once", "kind": "reject_once"}
                            ]
                        }
                    }));
                }
                FakeBehavior::Verdict(
                    ClientMcpVerdict::RejectsClientMcp | ClientMcpVerdict::Inconclusive,
                )
                | FakeBehavior::AuthenticateFailed
                | FakeBehavior::SessionUnauthorized => {}
            },
            Some("session/request_permission") | Some("fs/read_text_file") => {}
            None if matches!(self.behavior, FakeBehavior::AsksPermission) => {
                let selected = message
                    .pointer("/result/outcome/optionId")
                    .and_then(Value::as_str);
                if selected == Some("allow_once") {
                    self.emit_echo_accept();
                } else if let Some(prompt_id) = self.prompt_id.clone() {
                    self.push(json!({
                        "jsonrpc": "2.0",
                        "id": prompt_id,
                        "result": {"stopReason": "end_turn"}
                    }));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn take_inbound(&mut self) -> Result<Vec<Value>, GrokAcpClientMcpError> {
        let mut messages = Vec::new();
        while let Some(message) = self.inbound.pop_front() {
            messages.push(message);
        }
        Ok(messages)
    }

    fn close(&mut self) -> GrokAcpClientMcpCleanup {
        self.closed = true;
        GrokAcpClientMcpCleanup { joined: true }
    }

    fn stale_callback_request(&mut self) -> Option<Value> {
        Some(json!({
            "jsonrpc": "2.0",
            "id": STALE_CALLBACK_ID,
            "method": "fs/read_text_file",
            "params": {"sessionId": FIXTURE_SESSION, "path": "/private/secret.txt"}
        }))
    }
}

fn exchange(
    peer: &mut dyn GrokAcpClientMcpPeer,
    capture: &mut FrameCapture,
    id: u64,
    method: &str,
    params: Value,
) -> Result<(), GrokAcpClientMcpError> {
    let bytes = encode_request(id, method, params).map_err(|_| GrokAcpClientMcpError {
        kind: GrokAcpClientMcpErrorKind::Transport,
    })?;
    let outbound: Value =
        serde_json::from_slice(&bytes[..bytes.len() - 1]).map_err(|_| GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
    capture.push(FrameDirection::Outbound, outbound.clone());
    peer.push_outbound(outbound)?;
    for message in peer.take_inbound()? {
        capture.push(FrameDirection::Inbound, message);
    }
    Ok(())
}

fn answer_callbacks(
    peer: &mut dyn GrokAcpClientMcpPeer,
    capture: &mut FrameCapture,
    session: &str,
) -> Result<(), GrokAcpClientMcpError> {
    let pending: Vec<Value> = capture
        .frames
        .iter()
        .filter(|frame| !frame.is_outbound())
        .map(|frame| frame.message.clone())
        .collect();
    for message in pending {
        if method_of(&message) == Some("session/request_permission") {
            let id = message.get("id").cloned().unwrap_or(Value::Null);
            let response = permission_response(id, &message);
            capture.push(FrameDirection::Outbound, response.clone());
            peer.push_outbound(response)?;
            for inbound in peer.take_inbound()? {
                capture.push(FrameDirection::Inbound, inbound);
            }
        }
        if method_of(&message) == Some("fs/read_text_file") {
            reject_request(
                peer,
                capture,
                message.get("id").cloned().unwrap_or(Value::Null),
            )?;
        }
        let _ = session;
    }
    Ok(())
}

fn reject_stale_callback(
    peer: &mut dyn GrokAcpClientMcpPeer,
    capture: &mut FrameCapture,
) -> Result<bool, GrokAcpClientMcpError> {
    let request = peer
        .stale_callback_request()
        .unwrap_or_else(synthetic_stale_callback);
    capture.push(FrameDirection::Inbound, request.clone());
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    reject_request(peer, capture, id)?;
    Ok(true)
}

fn synthetic_stale_callback() -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": STALE_CALLBACK_ID,
        "method": "fs/read_text_file",
        "params": {"sessionId": FIXTURE_SESSION, "path": "/private/secret.txt"}
    })
}

fn reject_request(
    peer: &mut dyn GrokAcpClientMcpPeer,
    capture: &mut FrameCapture,
    id: Value,
) -> Result<(), GrokAcpClientMcpError> {
    let bytes =
        encode_error(id, -32600, "stale callback rejected").map_err(|_| GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
    let outbound: Value =
        serde_json::from_slice(&bytes[..bytes.len() - 1]).map_err(|_| GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
    capture.push(FrameDirection::Outbound, outbound.clone());
    let _ = peer.push_outbound(outbound);
    Ok(())
}

struct FrameCapture {
    frames: Vec<GrokAcpClientMcpFrame>,
    truncated: bool,
}

impl FrameCapture {
    fn new() -> Self {
        Self {
            frames: Vec::new(),
            truncated: false,
        }
    }

    fn push(&mut self, direction: FrameDirection, message: Value) {
        if self.frames.len() >= MAXIMUM_FRAMES {
            self.truncated = true;
            return;
        }
        self.frames
            .push(GrokAcpClientMcpFrame { direction, message });
    }
}

fn method_of(message: &Value) -> Option<&str> {
    message.get("method").and_then(Value::as_str)
}

fn response_result(frames: &[GrokAcpClientMcpFrame], id: u64) -> Option<&Value> {
    let id = json!(id);
    frames.iter().find_map(|frame| {
        if frame.is_outbound() || frame.message.get("id") != Some(&id) {
            None
        } else {
            frame.message.get("result")
        }
    })
}

fn response_error(frames: &[GrokAcpClientMcpFrame], id: u64) -> Option<&Value> {
    let id = json!(id);
    frames.iter().find_map(|frame| {
        if frame.is_outbound() || frame.message.get("id") != Some(&id) {
            None
        } else {
            frame.message.get("error")
        }
    })
}

fn request_succeeded(frames: &[GrokAcpClientMcpFrame], id: u64) -> bool {
    response_error(frames, id).is_none() && response_result(frames, id).is_some()
}

fn session_id_from(frames: &[GrokAcpClientMcpFrame], id: u64) -> Option<String> {
    response_result(frames, id)?
        .get("sessionId")
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn echo_named_tool_called(frames: &[GrokAcpClientMcpFrame]) -> bool {
    frames.iter().any(|frame| {
        !frame.is_outbound()
            && update_kind(&frame.message) == Some("tool_call")
            && echo_tool_title(&frame.message)
    })
}

fn echo_tool_title(message: &Value) -> bool {
    let title = message
        .pointer("/params/update/title")
        .and_then(Value::as_str);
    let name = message
        .pointer("/params/update/name")
        .and_then(Value::as_str);
    [title, name]
        .into_iter()
        .flatten()
        .any(|value| value.eq_ignore_ascii_case(ECHO_MCP_TOOL))
}

fn echo_tool_called(frames: &[GrokAcpClientMcpFrame]) -> bool {
    frames.iter().any(|frame| {
        !frame.is_outbound()
            && update_kind(&frame.message) == Some("tool_call")
            && json_contains_client_mcp_server(&frame.message)
    })
}

fn echo_tool_completed(frames: &[GrokAcpClientMcpFrame]) -> bool {
    frames.iter().any(|frame| {
        !frame.is_outbound()
            && update_kind(&frame.message) == Some("tool_call_update")
            && frame
                .message
                .pointer("/params/update/status")
                .and_then(Value::as_str)
                == Some("completed")
            && (json_contains_client_mcp_server(&frame.message) || echo_tool_called(frames))
    })
}

fn update_kind(message: &Value) -> Option<&str> {
    message
        .pointer("/params/update/sessionUpdate")
        .and_then(Value::as_str)
}

fn json_contains_client_mcp_server(value: &Value) -> bool {
    match value {
        Value::String(text) => text.to_ascii_lowercase().contains(ECHO_MCP_SERVER_NAME),
        Value::Array(items) => items.iter().any(json_contains_client_mcp_server),
        Value::Object(map) => map.values().any(json_contains_client_mcp_server),
        _ => false,
    }
}

fn error_rejects_client_mcp(error: &Value) -> bool {
    json_mentions_mcp_servers(error) || json_contains_client_mcp_server(error)
}

fn json_mentions_mcp_servers(value: &Value) -> bool {
    match value {
        Value::String(text) => text.to_ascii_lowercase().contains("mcpservers"),
        Value::Array(items) => items.iter().any(json_mentions_mcp_servers),
        Value::Object(map) => map.values().any(json_mentions_mcp_servers),
        _ => false,
    }
}

fn prompt_turn_completed(frames: &[GrokAcpClientMcpFrame]) -> bool {
    let Some(prompt) = frames
        .iter()
        .find(|frame| frame.is_outbound() && method_of(&frame.message) == Some("session/prompt"))
    else {
        return false;
    };
    let Some(id) = prompt.message.get("id") else {
        return false;
    };
    frames.iter().any(|frame| {
        !frame.is_outbound()
            && frame.message.get("id") == Some(id)
            && frame.message.get("result").is_some()
    })
}

fn permission_was_rejected(frames: &[GrokAcpClientMcpFrame]) -> bool {
    frames.iter().any(|frame| {
        !frame.is_outbound()
            && method_of(&frame.message) == Some("session/request_permission")
            && permission_response_rejected(frames, frame.message.get("id"), &frame.message)
    })
}

fn permission_response_rejected(
    frames: &[GrokAcpClientMcpFrame],
    id: Option<&Value>,
    request: &Value,
) -> bool {
    let Some(id) = id else {
        return false;
    };
    let Some(response) = frames.iter().find(|frame| {
        frame.is_outbound()
            && frame.message.get("id") == Some(id)
            && frame.message.get("result").is_some()
    }) else {
        return false;
    };
    let Some(option_id) = response
        .message
        .pointer("/result/outcome/optionId")
        .and_then(Value::as_str)
    else {
        return response
            .message
            .pointer("/result/outcome/outcome")
            .and_then(Value::as_str)
            == Some("cancelled");
    };
    request
        .pointer("/params/options")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|option| {
            option.get("optionId").and_then(Value::as_str) == Some(option_id)
                && option
                    .get("kind")
                    .and_then(Value::as_str)
                    .is_some_and(|kind| kind.starts_with("reject"))
        })
}

fn permission_response(id: Value, request: &Value) -> Value {
    match permission_option_id(request) {
        Some(option_id) => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "outcome": {
                    "outcome": "selected",
                    "optionId": option_id
                }
            }
        }),
        None => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {"outcome": {"outcome": "cancelled"}}
        }),
    }
}

fn permission_option_id(request: &Value) -> Option<&str> {
    let options = request
        .pointer("/params/options")
        .and_then(Value::as_array)?;
    options.iter().find_map(|option| {
        (option.get("kind").and_then(Value::as_str) == Some("allow_once"))
            .then(|| option.get("optionId").and_then(Value::as_str))
            .flatten()
    })
}

fn redact_value(value: Value) -> Value {
    match value {
        Value::String(text) => Value::String(redact_string(&text)),
        Value::Array(items) => Value::Array(items.into_iter().map(redact_value).collect()),
        Value::Object(map) => {
            let mut redacted = serde_json::Map::new();
            for (key, child) in map {
                let lower = key.to_ascii_lowercase();
                if key_is_secret(&lower) {
                    redacted.insert(key, Value::String("<redacted>".to_owned()));
                } else {
                    redacted.insert(key, redact_value(child));
                }
            }
            Value::Object(redacted)
        }
        other => other,
    }
}

fn key_is_secret(key: &str) -> bool {
    key.contains("token")
        || key.contains("secret")
        || key.contains("password")
        || key.contains("authorization")
        || key.contains("credential")
        || key.contains("cookie")
        || key == "command"
        || key == "cwd"
        || key == "path"
        || key == "args"
        || key == "env"
}

fn redact_string(text: &str) -> String {
    if text.starts_with('/') || text.contains(":\\") || text.contains("Bearer ") {
        "<redacted>".to_owned()
    } else {
        text.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_session_new_is_inconclusive() {
        let frames = [GrokAcpClientMcpFrame {
            direction: FrameDirection::Inbound,
            message: json!({
                "jsonrpc": "2.0",
                "method": "session/update",
                "params": {
                    "update": {
                        "sessionUpdate": "tool_call",
                        "title": "echo"
                    }
                }
            }),
        }];
        assert_eq!(
            grok_acp_client_mcp_verdict_from_frames(&frames),
            ClientMcpVerdict::Inconclusive
        );
    }

    #[test]
    fn echo_handler_lists_one_tool_and_rejects_resources() {
        let list = grok_acp_echo_mcp_reply(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list"
        }))
        .expect("tools/list replies");
        let tools = list["result"]["tools"].as_array().expect("tools");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], ECHO_MCP_TOOL);
        let denied = grok_acp_echo_mcp_reply(&json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "resources/read",
            "params": {"uri": "file:///tmp/secret"}
        }))
        .expect("unknown methods error");
        assert_eq!(denied["error"]["code"], json!(-32601));
        let echoed = grok_acp_echo_mcp_reply(&json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {"name": "echo", "arguments": {"text": "ping"}}
        }))
        .expect("echo replies");
        assert_eq!(echoed["result"]["content"][0]["text"], json!("ping"));
    }

    #[test]
    fn live_open_without_gate_does_not_spawn() {
        assert!(!desktop_grok_acp_client_mcp_probe_is_gated_open());
        match open_desktop_live_grok_acp_peer(Path::new("/bin/true"), Path::new("/bin/true")) {
            Ok(_) => panic!("live peer opened without the Desktop gate"),
            Err(error) => assert_eq!(error.kind(), GrokAcpClientMcpErrorKind::LiveProbeGated),
        }
    }

    #[test]
    fn echo_stdio_frame_is_newline_delimited_json() {
        let reply = grok_acp_echo_mcp_reply(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "fixture"}}
        }))
        .expect("initialize reply");
        let bytes = grok_acp_echo_mcp_stdio_frame(&reply).expect("stdio frame");
        let text = std::str::from_utf8(&bytes).expect("utf8");
        assert!(text.ends_with('\n'));
        assert_eq!(text.matches('\n').count(), 1);
        assert!(!text.to_ascii_lowercase().contains("content-length"));
        let parsed: Value = serde_json::from_str(text.trim_end()).expect("json");
        assert_eq!(parsed["result"]["serverInfo"]["name"], ECHO_MCP_SERVER_NAME);
    }

    #[test]
    fn authenticate_failure_does_not_send_session_new_and_is_inconclusive() {
        let mut peer = FakeAcpPeer::authenticate_failed();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("capsule");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::Inconclusive);
        assert!(!capsule.truncated());
        assert!(!capsule.frames().iter().any(|frame| {
            frame.is_outbound() && method_of(frame.message()) == Some("session/new")
        }));
    }

    #[test]
    fn session_new_auth_error_without_mcp_servers_is_inconclusive() {
        let mut peer = FakeAcpPeer::session_unauthorized();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("capsule");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::Inconclusive);
        assert!(capsule.frames().iter().any(|frame| {
            frame.is_outbound() && method_of(frame.message()) == Some("session/new")
        }));
    }

    #[test]
    fn session_new_mcp_servers_error_is_rejects() {
        let capsule =
            grok_acp_client_mcp_fixture_probe("1.0.5", ClientMcpVerdict::RejectsClientMcp)
                .expect("rejects fixture");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::RejectsClientMcp);
        let error = capsule
            .frames()
            .iter()
            .find(|frame| !frame.is_outbound() && frame.message().get("error").is_some())
            .expect("error frame");
        assert!(error_rejects_client_mcp(
            error.message().get("error").expect("error")
        ));
    }

    #[test]
    fn capture_overflow_returns_truncated_inconclusive_capsule() {
        let mut peer = FakeAcpPeer::chatty();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("overflow still writes a capsule");
        assert!(capsule.truncated());
        assert_eq!(capsule.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(capsule.frames().len(), MAXIMUM_FRAMES);
        assert_eq!(capsule.to_json()["truncated"], json!(true));
    }

    #[test]
    fn session_new_without_prompt_is_inconclusive() {
        let frames = [
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Outbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 3,
                    "method": "session/new",
                    "params": {"cwd": "<host-approved-resource>", "mcpServers": [{"name": ECHO_MCP_SERVER_NAME}]}
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Inbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 3,
                    "result": {"sessionId": FIXTURE_SESSION}
                }),
            },
        ];
        assert_eq!(
            grok_acp_client_mcp_verdict_from_frames(&frames),
            ClientMcpVerdict::Inconclusive
        );
    }

    #[test]
    fn prompt_timeout_without_result_is_inconclusive() {
        let mut peer = FakeAcpPeer::prompt_silent();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("capsule");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::Inconclusive);
        assert!(capsule.frames().iter().any(|frame| {
            frame.is_outbound() && method_of(frame.message()) == Some("session/prompt")
        }));
        assert!(!capsule.frames().iter().any(|frame| {
            !frame.is_outbound()
                && frame.message().get("result").is_some()
                && frame.message().get("id") == Some(&json!(4))
        }));
    }

    #[test]
    fn permission_allow_uses_request_option_id() {
        let mut peer = FakeAcpPeer::asks_permission();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("capsule");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::AcceptsClientMcp);
        let selected = capsule.frames().iter().find_map(|frame| {
            frame
                .is_outbound()
                .then(|| frame.message().pointer("/result/outcome/optionId"))
                .flatten()
                .and_then(Value::as_str)
        });
        assert_eq!(selected, Some("allow_once"));
        assert!(!capsule.to_json().to_string().contains("allow-once"));
    }

    #[test]
    fn rejected_permission_without_echo_is_inconclusive() {
        let frames = [
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Outbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 3,
                    "method": "session/new",
                    "params": {"cwd": "<host-approved-resource>", "mcpServers": [{"name": ECHO_MCP_SERVER_NAME}]}
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Inbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 3,
                    "result": {"sessionId": FIXTURE_SESSION}
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Outbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 4,
                    "method": "session/prompt",
                    "params": {"sessionId": FIXTURE_SESSION}
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Inbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 900,
                    "method": "session/request_permission",
                    "params": {
                        "toolCall": {"toolCallId": "echo-1", "title": "echo"},
                        "options": [
                            {"optionId": "allow_once", "kind": "allow_once"},
                            {"optionId": "reject_once", "kind": "reject_once"}
                        ]
                    }
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Outbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 900,
                    "result": {"outcome": {"outcome": "selected", "optionId": "reject_once"}}
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Inbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 4,
                    "result": {"stopReason": "end_turn"}
                }),
            },
        ];
        assert_eq!(
            grok_acp_client_mcp_verdict_from_frames(&frames),
            ClientMcpVerdict::Inconclusive
        );
    }

    #[test]
    fn native_echo_title_without_client_mcp_server_is_inconclusive() {
        let frames = [
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Outbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 3,
                    "method": "session/new",
                    "params": {"cwd": "<host-approved-resource>", "mcpServers": [{"name": ECHO_MCP_SERVER_NAME}]}
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Inbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 3,
                    "result": {"sessionId": FIXTURE_SESSION}
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Outbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 4,
                    "method": "session/prompt",
                    "params": {"sessionId": FIXTURE_SESSION, "prompt": [{"type": "text", "text": "x"}]}
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Inbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "update": {
                            "sessionUpdate": "tool_call",
                            "title": "echo",
                            "status": "completed"
                        }
                    }
                }),
            },
            GrokAcpClientMcpFrame {
                direction: FrameDirection::Inbound,
                message: json!({
                    "jsonrpc": "2.0",
                    "id": 4,
                    "result": {"stopReason": "end_turn"}
                }),
            },
        ];
        assert_eq!(
            grok_acp_client_mcp_verdict_from_frames(&frames),
            ClientMcpVerdict::Inconclusive
        );
    }

    #[test]
    fn live_none_stale_callback_still_records_rejection() {
        struct SilentStale(FakeAcpPeer);
        impl GrokAcpClientMcpPeer for SilentStale {
            fn push_outbound(&mut self, message: Value) -> Result<(), GrokAcpClientMcpError> {
                self.0.push_outbound(message)
            }
            fn take_inbound(&mut self) -> Result<Vec<Value>, GrokAcpClientMcpError> {
                self.0.take_inbound()
            }
            fn close(&mut self) -> GrokAcpClientMcpCleanup {
                self.0.close()
            }
            fn stale_callback_request(&mut self) -> Option<Value> {
                None
            }
        }
        let mut peer = SilentStale(FakeAcpPeer::new(ClientMcpVerdict::IgnoresClientMcp));
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("capsule");
        assert!(capsule.stale_callback_rejected());
        assert_eq!(capsule.verdict(), ClientMcpVerdict::IgnoresClientMcp);
    }

    #[test]
    fn redaction_strips_paths_and_secrets() {
        let redacted = redact_value(json!({
            "cwd": "/Users/tom/.grok",
            "command": "/opt/grok/echo",
            "token": "secret-value",
            "name": ECHO_MCP_SERVER_NAME
        }));
        assert_eq!(redacted["cwd"], json!("<redacted>"));
        assert_eq!(redacted["command"], json!("<redacted>"));
        assert_eq!(redacted["token"], json!("<redacted>"));
        assert_eq!(redacted["name"], json!(ECHO_MCP_SERVER_NAME));
    }
}
