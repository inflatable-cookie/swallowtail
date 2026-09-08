//! Provider-free Grok ACP client-MCP probe harness.
//!
//! Verdicts are decided from captured ACP frames plus the disposable echo MCP
//! stdio transcript. ACP v1 cannot name the client MCP server on a tool call,
//! so `accepts_client_mcp` requires a `tools/call` on that transcript.
//! Protocol admission (`initialize`) and tool listing (`tools/list`) are
//! explicit capsule fields; a verdict cannot discard them. `ignores_client_mcp`
//! is only the completed-turn case where a proven-spawnable echo helper was
//! never reached. An unproven or unspawnable helper is `inconclusive` with
//! `echo_liveness_unproven`, so a copy/chmod failure cannot freeze
//! `provider_limitation`. `inconclusive` names its cause, including
//! `no_turn_result` versus `turn_completed_without_tool_call`. Crate tests
//! drive the fake ACP fixture.
//!
//! The probe is a conforming ACP client during every exchange, including
//! `session/new`: inbound client requests are answered from a small allowlist
//! ([`grok_acp_client_request_reply`]) and every answer is captured.
//! `session/new` failure therefore separates an inbound request the probe
//! failed to answer (`session_new_unanswered`, harness-shaped) from the
//! provider never responding within [`LIVE_SESSION_NEW_WAIT`]
//! (`session_new_bound_exceeded`).
//!
//! Bounds are sized for a live model, not for protocol round trips. Only
//! `initialize` and `authenticate` keep a protocol-scale bound
//! ([`LIVE_PROTOCOL_WAIT`]); `session/new` spawns MCP servers
//! ([`LIVE_SESSION_NEW_WAIT`]) and `session/prompt` waits on a real turn that
//! reasons and calls a tool ([`LIVE_PROMPT_WAIT`], minutes-scale). Capture is
//! bounded the same way: the capacity is spent on streaming chatter, and the
//! `session/new` request, tool-call and tool-result updates, correlated
//! responses, and the turn result are never evicted. `truncated` therefore
//! means uninteresting middle frames were elided; it is not a verdict, and
//! [`InconclusiveCause::Truncated`] is reachable only if capacity is filled
//! by decisive frames alone.
//!
//! The live installed-Grok entrypoint refuses to spawn unless Desktop sets
//! [`DESKTOP_GROK_ACP_CLIENT_MCP_PROBE_GATE`] and an isolated `GROK_HOME`
//! directory exists.

use serde_json::{Value, json};
use std::collections::VecDeque;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
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
/// Env var the echo stdio server appends received method names to.
pub const ECHO_MCP_TRANSCRIPT_ENV: &str = "SWALLOWTAIL_ECHO_MCP_TRANSCRIPT";
/// CLI flag carrying the same transcript path, so a dropped `env` is not silent.
pub const ECHO_MCP_TRANSCRIPT_FLAG: &str = "--transcript";

static ECHO_MCP_TRANSCRIPT_SEQ: AtomicU64 = AtomicU64::new(0);

/// Target frame count for one capsule.
///
/// A live turn streams one `session/update` notification per output chunk, so
/// a real session emits far more frames than a hand-reviewable capsule should
/// carry. This budget is spent on the elidable frames only:
/// [`FrameCapture::push`] evicts the oldest of those to make room, and when
/// none is left the capsule grows instead of dropping evidence.
const MAXIMUM_FRAMES: usize = 512;
/// Hard ceiling on retained frames, guarding memory when nothing in the
/// capture may be dropped.
///
/// [`MAXIMUM_FRAMES`] is the capsule's target shape, not a licence to lose
/// evidence: a session whose frames are all irreplaceable — many distinct
/// tool calls, each with its own result — grows past that target rather than
/// discarding the answer. This ceiling bounds that growth. Reaching it needs
/// thousands of distinct requests and tool calls inside one probe turn, which
/// is the only shape left that can score
/// [`InconclusiveCause::Truncated`].
const MAXIMUM_RETAINED_FRAMES: usize = 8_192;
/// Directive prompt recorded verbatim on every capsule.
pub const ECHO_PROMPT: &str = "You must call the tool named echo with argument text set to ping. Do not finish the turn until that tool call has returned.";
/// Session id used by the offline fixtures. Never sent to a provider.
const FIXTURE_SESSION: &str = "grok-fixture-session";
/// Placeholder `cwd` on fixture capsules. Already in redacted spelling, so a
/// fixture capsule and a live one read the same.
const FIXTURE_CWD: &str = "<host-approved-resource>";
/// Placeholder MCP server command on fixture capsules. Same redacted spelling
/// the live path writes, so no fixture leaks a host path.
const FIXTURE_COMMAND: &str = "<redacted-command>";
/// Sentinel JSON-RPC id for the post-close callback the harness must reject.
/// Probe request ids start at 1 and increment once per exchange, four at
/// most, so this cannot collide with a real correlation id.
const STALE_CALLBACK_ID: u64 = 9001;
/// Gap that ends one inbound burst.
///
/// Not a liveness bound. [`exchange`] holds one absolute deadline and
/// re-enters the drain until the response arrives or that deadline expires,
/// so a longer inter-frame gap only splits a burst across two reads. Sized to
/// batch a streaming turn's chunks without spinning on an idle pipe.
const LIVE_IDLE: Duration = Duration::from_millis(200);
/// Inbound bound for exchanges that are pure protocol round trips with no
/// model turn behind them: `initialize` and `authenticate`.
///
/// Neither runs inference, so these are the module's genuinely-immediate
/// exchanges. The bound is still tens of seconds rather than single digits
/// because `initialize` lands on a cold agent process that is still starting
/// its runtime and loading modules, and `authenticate` may touch a keychain.
const LIVE_PROTOCOL_WAIT: Duration = Duration::from_secs(30);
/// Inbound bound for the `session/new` exchange.
///
/// Session establishment spawns every declared MCP server as a child process
/// and enumerates its tools before answering, so this is process startup plus
/// stdio handshakes, not one round trip. Two minutes keeps a slow spawn on a
/// cold or loaded host from being recorded as provider silence. Successful
/// establishment inside the previous minute is not evidence that a minute is
/// enough for the slowest host.
pub const LIVE_SESSION_NEW_WAIT: Duration = Duration::from_secs(120);
/// Inbound bound for the `session/prompt` exchange, the only exchange that
/// waits on a live model.
///
/// The turn reasons about the directive, issues a tool call, waits for the
/// echo MCP round trip, and only then finishes. That is minutes of wall
/// clock. A protocol-scale bound here scores our own impatience as
/// `no_turn_result`, which is what ended the 2026-09-08 rerun on `1.0.5`, so
/// this bound is minutes-scale and deliberately separate from every other
/// bound in the module.
pub const LIVE_PROMPT_WAIT: Duration = Duration::from_secs(300);
/// Grace for the agent child to exit after its stdin closes.
///
/// A Node agent flushes session state and telemetry on shutdown. A grace
/// shorter than that kills a cleanly-exiting child and reports the kill as
/// cleanup evidence, so ten seconds buys the shutdown path real room while
/// still bounding the probe.
const LIVE_JOIN: Duration = Duration::from_secs(10);
/// Bound for the echo helper's own `initialize` self-check.
///
/// The helper is a local, freshly built binary answering from memory, so this
/// is immediate in principle; the cost is a first spawn paging the binary in
/// on a loaded host. A false negative here turns a real provider result into
/// `echo_liveness_unproven`, so the margin is seconds. No provider-facing
/// wait is extended by it.
const ECHO_HELPER_LIVE_WAIT: Duration = Duration::from_secs(5);

/// Client-MCP verdict for one exact Grok version segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientMcpVerdict {
    /// Session accepted a non-empty `mcpServers` list and the echo MCP server was called.
    AcceptsClientMcp,
    /// Session accepted a non-empty `mcpServers` list, the prompt turn completed, the echo helper was proven spawnable, and the echo server was never reached.
    IgnoresClientMcp,
    /// Session setup rejected the non-empty `mcpServers` list.
    RejectsClientMcp,
    /// Frames are missing, incomplete, or contradictory. See [`InconclusiveCause`].
    Inconclusive,
}

/// Named reason when [`ClientMcpVerdict::Inconclusive`] is scored.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InconclusiveCause {
    /// Outbound `session/new` is missing.
    MissingSessionNew,
    /// Outbound `session/new` carried an empty `mcpServers` list.
    EmptyMcpServers,
    /// An inbound client request inside the `session/new` exchange was left
    /// without any probe response. Harness-shaped: the responder must answer
    /// every request.
    SessionNewUnanswered,
    /// Every inbound client request in the `session/new` exchange was answered
    /// and the agent still returned no response within
    /// [`LIVE_SESSION_NEW_WAIT`].
    SessionNewBoundExceeded,
    /// `session/new` failed without naming client MCP.
    SessionNewErrorNotClientMcp,
    /// A frame the verdict depends on could not be retained. Reached only
    /// when [`MAXIMUM_FRAMES`] is filled by decisive frames alone; eliding
    /// non-decisive middle frames never scores this.
    Truncated,
    /// A permission request was rejected before echo could run.
    PermissionRejected,
    /// Prompt was missing or never returned a turn result.
    NoTurnResult,
    /// Turn completed after admission, but echo was never called.
    TurnCompletedWithoutToolCall,
    /// ACP showed an echo-titled tool call without echo MCP `initialize`.
    NativeEchoWithoutAdmission,
    /// Echo helper was not proven spawnable; empty transcript cannot score ignore.
    EchoLivenessUnproven,
}

impl InconclusiveCause {
    /// Returns the capsule spelling for this cause.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingSessionNew => "missing_session_new",
            Self::EmptyMcpServers => "empty_mcp_servers",
            Self::SessionNewUnanswered => "session_new_unanswered",
            Self::SessionNewBoundExceeded => "session_new_bound_exceeded",
            Self::SessionNewErrorNotClientMcp => "session_new_error_not_client_mcp",
            Self::Truncated => "truncated",
            Self::PermissionRejected => "permission_rejected",
            Self::NoTurnResult => "no_turn_result",
            Self::TurnCompletedWithoutToolCall => "turn_completed_without_tool_call",
            Self::NativeEchoWithoutAdmission => "native_echo_without_admission",
            Self::EchoLivenessUnproven => "echo_liveness_unproven",
        }
    }
}

/// Offline oracle shapes that keep admission and invocation separate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientMcpOracleShape {
    /// Echo `initialize`, `tools/list`, and `tools/call` all observed.
    AdmittedAndCalled,
    /// Echo `initialize` and `tools/list` observed; turn completed; no `tools/call`.
    AdmittedListedNotCalledCompleted,
    /// Echo `initialize` observed; `tools/list` absent; turn completed; no `tools/call`.
    AdmittedNotListed,
    /// Session accepted `mcpServers`; a spawnable helper never received `initialize`.
    NoAdmission,
    /// Echo helper was not spawnable; empty transcript must not score ignore.
    HelperUnspawnable,
    /// The agent emits a client permission request inside `session/new` and
    /// completes the session only after the probe answers it.
    SessionNewAnswerRequired,
    /// The agent emits a client permission request inside `session/new`, sees
    /// the probe's recorded answer, and still never responds to `session/new`.
    SessionNewSilentAfterAnswer,
}

/// Methods observed by the disposable echo MCP server on its own stdio.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EchoMcpTranscript {
    initialize: bool,
    tools_list: bool,
    tools_call: bool,
}

impl EchoMcpTranscript {
    /// Transcript proving Grok connected and called the echo tool.
    #[must_use]
    pub const fn called() -> Self {
        Self {
            initialize: true,
            tools_list: true,
            tools_call: true,
        }
    }

    /// Echo MCP received `initialize` and Grok never listed or called the tool.
    #[must_use]
    pub const fn observed_idle() -> Self {
        Self {
            initialize: true,
            tools_list: false,
            tools_call: false,
        }
    }

    /// Echo MCP received `initialize` and `tools/list`, and Grok never called the tool.
    #[must_use]
    pub const fn admitted_listed() -> Self {
        Self {
            initialize: true,
            tools_list: true,
            tools_call: false,
        }
    }

    /// Returns whether echo MCP received `initialize`. That is protocol admission, not a verdict.
    #[must_use]
    pub const fn initialize(&self) -> bool {
        self.initialize
    }

    /// Returns whether echo MCP received `tools/list`.
    #[must_use]
    pub const fn tools_list(&self) -> bool {
        self.tools_list
    }

    /// Returns whether a `tools/call` for echo was observed.
    #[must_use]
    pub const fn tools_call(&self) -> bool {
        self.tools_call
    }

    fn methods(&self) -> Vec<&'static str> {
        let mut methods = Vec::new();
        if self.initialize {
            methods.push("initialize");
        }
        if self.tools_list {
            methods.push("tools/list");
        }
        if self.tools_call {
            methods.push("tools/call");
        }
        methods
    }
}

/// Appends one MCP method name to the echo-server transcript file.
pub fn append_echo_mcp_transcript(path: &Path, method: &str) -> Result<(), GrokAcpClientMcpError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|_| GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        })?;
    writeln!(file, "{method}").map_err(|_| GrokAcpClientMcpError {
        kind: GrokAcpClientMcpErrorKind::Transport,
    })
}

/// Exclusive empty per-run transcript under `grok_home`. Fails if that path exists.
pub fn create_echo_mcp_transcript(grok_home: &Path) -> Result<PathBuf, GrokAcpClientMcpError> {
    let seq = ECHO_MCP_TRANSCRIPT_SEQ.fetch_add(1, Ordering::Relaxed);
    let path = grok_home.join(format!(
        "echo-mcp-transcript-{}-{seq}.ndjson",
        std::process::id()
    ));
    create_echo_mcp_transcript_at(&path)?;
    Ok(path)
}

fn create_echo_mcp_transcript_at(path: &Path) -> Result<(), GrokAcpClientMcpError> {
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(_) => Ok(()),
        Err(error) if error.kind() == ErrorKind::AlreadyExists => Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::LiveTranscriptExists,
        }),
        Err(_) => Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::Transport,
        }),
    }
}

fn refuse_nonempty_transcript(path: Option<&Path>) -> Result<(), GrokAcpClientMcpError> {
    let Some(path) = path else {
        return Ok(());
    };
    if fs::metadata(path).is_ok_and(|meta| meta.len() > 0) {
        return Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::LiveTranscriptExists,
        });
    }
    Ok(())
}

/// Reads method names from an echo-server transcript file.
#[must_use]
pub fn read_echo_mcp_transcript(path: &Path) -> EchoMcpTranscript {
    let Ok(text) = fs::read_to_string(path) else {
        return EchoMcpTranscript::default();
    };
    let mut transcript = EchoMcpTranscript::default();
    for line in text.lines() {
        match line.trim() {
            "initialize" => transcript.initialize = true,
            "tools/list" => transcript.tools_list = true,
            "tools/call" => transcript.tools_call = true,
            _ => {}
        }
    }
    transcript
}

/// Isolated Grok home used as live `HOME` and session `cwd`. Fail closed.
pub fn isolated_grok_home() -> Result<PathBuf, GrokAcpClientMcpError> {
    isolated_grok_home_from(std::env::var("GROK_HOME").ok().as_deref())
}

fn isolated_grok_home_from(value: Option<&str>) -> Result<PathBuf, GrokAcpClientMcpError> {
    let Some(value) = value.filter(|home| !home.is_empty()) else {
        return Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::LiveIsolationMissing,
        });
    };
    let path = PathBuf::from(value);
    if path.is_dir() {
        Ok(path)
    } else {
        Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::LiveIsolationMissing,
        })
    }
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
    inconclusive_cause: Option<InconclusiveCause>,
    prompt: String,
    prompt_turn_completed: bool,
    stop_reason: Option<String>,
    client_mcp_admitted: bool,
    client_mcp_tools_listed: bool,
    echo_helper_live: bool,
    frames: Vec<GrokAcpClientMcpFrame>,
    stale_callback_rejected: bool,
    cleanup: GrokAcpClientMcpCleanup,
    truncated: bool,
    echo_mcp_methods: Vec<String>,
}

impl GrokAcpClientMcpCapsule {
    /// Exact version segment recorded on the capsule.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Frame-decided verdict, using echo MCP stdio evidence when present.
    #[must_use]
    pub const fn verdict(&self) -> ClientMcpVerdict {
        self.verdict
    }

    /// Named cause when the verdict is [`ClientMcpVerdict::Inconclusive`].
    #[must_use]
    pub const fn inconclusive_cause(&self) -> Option<InconclusiveCause> {
        self.inconclusive_cause
    }

    /// Exact prompt text sent on `session/prompt`.
    #[must_use]
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Returns whether the prompt turn produced a result.
    #[must_use]
    pub const fn prompt_turn_completed(&self) -> bool {
        self.prompt_turn_completed
    }

    /// ACP `stopReason` from the prompt result, when present.
    #[must_use]
    pub fn stop_reason(&self) -> Option<&str> {
        self.stop_reason.as_deref()
    }

    /// Echo-server `initialize` observed. Independent of the verdict.
    #[must_use]
    pub const fn client_mcp_admitted(&self) -> bool {
        self.client_mcp_admitted
    }

    /// Echo-server `tools/list` observed. Independent of the verdict.
    #[must_use]
    pub const fn client_mcp_tools_listed(&self) -> bool {
        self.client_mcp_tools_listed
    }

    /// Returns whether this run proved the echo helper spawnable before scoring.
    #[must_use]
    pub const fn echo_helper_live(&self) -> bool {
        self.echo_helper_live
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

    /// Returns whether non-decisive middle frames were elided to stay under
    /// the capture bound. Decisive frames survive truncation, so this is not
    /// by itself a verdict.
    #[must_use]
    pub const fn truncated(&self) -> bool {
        self.truncated
    }

    /// Echo MCP methods observed on the disposable server's stdio.
    #[must_use]
    pub fn echo_mcp_methods(&self) -> &[String] {
        &self.echo_mcp_methods
    }

    /// Bounded redacted JSON object with exact frames and the verdict.
    #[must_use]
    pub fn to_json(&self) -> Value {
        json!({
            "route": "grok-build.acp",
            "version": self.version,
            "verdict": self.verdict.as_str(),
            "inconclusive_cause": self.inconclusive_cause.map(InconclusiveCause::as_str),
            "prompt": self.prompt,
            "prompt_turn_completed": self.prompt_turn_completed,
            "stop_reason": self.stop_reason,
            "client_mcp_admitted": self.client_mcp_admitted,
            "client_mcp_tools_listed": self.client_mcp_tools_listed,
            "echo_helper_live": self.echo_helper_live,
            "stale_callback_rejected": self.stale_callback_rejected,
            "cleanup_joined": self.cleanup.joined,
            "truncated": self.truncated,
            "echo_mcp_methods": self.echo_mcp_methods,
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
    /// Live spawn was requested without an isolated `GROK_HOME` directory.
    LiveIsolationMissing,
    /// Live transcript path already exists; a prior run must not be scored again.
    LiveTranscriptExists,
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
            GrokAcpClientMcpErrorKind::LiveIsolationMissing => {
                "live Grok ACP client-MCP probe requires an isolated GROK_HOME directory"
            }
            GrokAcpClientMcpErrorKind::LiveTranscriptExists => {
                "live Grok ACP client-MCP probe refuses a transcript path that already exists"
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
    /// Returns every inbound object now available, spending at most `bound`
    /// in total. A peer that keeps streaming must still return by then, so an
    /// exchange deadline cannot be outlived. Deterministic peers may ignore
    /// the bound.
    fn take_inbound_within(
        &mut self,
        bound: Duration,
    ) -> Result<Vec<Value>, GrokAcpClientMcpError> {
        let _ = bound;
        self.take_inbound()
    }
    /// Disconnects and reports whether the wait joined.
    fn close(&mut self) -> GrokAcpClientMcpCleanup;
    /// Optional post-close callback the harness must reject.
    fn stale_callback_request(&mut self) -> Option<Value>;
    /// Echo MCP stdio methods observed for this peer. Live reads the transcript file.
    fn echo_mcp_transcript(&mut self) -> EchoMcpTranscript {
        EchoMcpTranscript::default()
    }
    /// Whether this run proved the echo helper spawnable. Default is unproven.
    fn echo_helper_live(&mut self) -> bool {
        false
    }
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

/// Runs the fake ACP fixture for one admission/invocation oracle shape.
pub fn grok_acp_client_mcp_oracle_fixture_probe(
    version: &str,
    shape: ClientMcpOracleShape,
) -> Result<GrokAcpClientMcpCapsule, GrokAcpClientMcpError> {
    let mut peer = FakeAcpPeer::oracle(shape);
    let capsule = run_grok_acp_client_mcp_probe(&mut peer, version, FIXTURE_COMMAND, FIXTURE_CWD)?;
    let expected = match shape {
        ClientMcpOracleShape::AdmittedAndCalled
        | ClientMcpOracleShape::SessionNewAnswerRequired => ClientMcpVerdict::AcceptsClientMcp,
        ClientMcpOracleShape::NoAdmission => ClientMcpVerdict::IgnoresClientMcp,
        ClientMcpOracleShape::AdmittedListedNotCalledCompleted
        | ClientMcpOracleShape::AdmittedNotListed
        | ClientMcpOracleShape::HelperUnspawnable
        | ClientMcpOracleShape::SessionNewSilentAfterAnswer => ClientMcpVerdict::Inconclusive,
    };
    if capsule.verdict != expected {
        return Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::InvalidVerdict,
        });
    }
    Ok(capsule)
}

/// Drives ACP initialize, non-empty `session/new`, one echo prompt, cleanup, and
/// stale-callback rejection. Verdict uses ACP frames plus the echo MCP transcript.
pub fn run_grok_acp_client_mcp_probe(
    peer: &mut dyn GrokAcpClientMcpPeer,
    version: &str,
    echo_command: &str,
    cwd: &str,
) -> Result<GrokAcpClientMcpCapsule, GrokAcpClientMcpError> {
    run_grok_acp_client_mcp_probe_with_transcript(peer, version, echo_command, cwd, None)
}

/// Same as [`run_grok_acp_client_mcp_probe`], reading echo stdio methods from `transcript`.
pub fn run_grok_acp_client_mcp_probe_with_transcript(
    peer: &mut dyn GrokAcpClientMcpPeer,
    version: &str,
    echo_command: &str,
    cwd: &str,
    transcript_path: Option<&Path>,
) -> Result<GrokAcpClientMcpCapsule, GrokAcpClientMcpError> {
    refuse_nonempty_transcript(transcript_path)?;
    let mut capture = FrameCapture::new();
    let mut next_id = 1_u64;
    let (args, env) = match transcript_path {
        Some(path) => {
            let path = path.to_string_lossy();
            (
                json!([ECHO_MCP_TRANSCRIPT_FLAG, path]),
                json!([{
                    "name": ECHO_MCP_TRANSCRIPT_ENV,
                    "value": path
                }]),
            )
        }
        None => (json!([]), json!([])),
    };

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
    exchange(
        peer,
        &mut capture,
        initialize_id,
        "initialize",
        initialize,
        LIVE_PROTOCOL_WAIT,
    )?;
    next_id += 1;

    if request_succeeded(&capture.frames, initialize_id, "initialize") {
        let authenticate_id = next_id;
        exchange(
            peer,
            &mut capture,
            authenticate_id,
            "authenticate",
            json!({"methodId": "cached_token", "_meta": {"headless": true}}),
            LIVE_PROTOCOL_WAIT,
        )?;
        next_id += 1;
        if request_succeeded(&capture.frames, authenticate_id, "authenticate") {
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
                        "args": args,
                        "env": env
                    }]
                }),
                LIVE_SESSION_NEW_WAIT,
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
                    LIVE_PROMPT_WAIT,
                )?;
            }
        }
    }

    let stale_callback_rejected = reject_stale_callback(peer, &mut capture)?;
    let cleanup = peer.close();
    let transcript = match transcript_path {
        Some(path) => read_echo_mcp_transcript(path),
        None => peer.echo_mcp_transcript(),
    };
    let echo_helper_live = peer.echo_helper_live();
    let truncated = capture.truncated;
    let decisive_frame_lost = capture.decisive_frame_lost;
    let redacted: Vec<GrokAcpClientMcpFrame> = capture
        .frames
        .into_iter()
        .map(|frame| GrokAcpClientMcpFrame {
            direction: frame.direction,
            message: redact_value(frame.message),
        })
        .collect();
    let decision = grok_acp_client_mcp_verdict_decision(
        &redacted,
        &transcript,
        decisive_frame_lost,
        echo_helper_live,
    );
    let (prompt_turn_completed, stop_reason) = prompt_turn_observation(&redacted);
    Ok(GrokAcpClientMcpCapsule {
        version: version.to_owned(),
        verdict: decision.verdict,
        inconclusive_cause: decision.inconclusive_cause,
        prompt: ECHO_PROMPT.to_owned(),
        prompt_turn_completed,
        stop_reason,
        client_mcp_admitted: transcript.initialize(),
        client_mcp_tools_listed: transcript.tools_list(),
        echo_helper_live,
        frames: redacted,
        stale_callback_rejected,
        cleanup,
        truncated,
        echo_mcp_methods: transcript
            .methods()
            .into_iter()
            .map(str::to_owned)
            .collect(),
    })
}

/// Decides the verdict from redacted frames. Missing `session/new` is inconclusive.
/// Frames alone cannot prove echo helper liveness, so a completed empty
/// transcript cannot score [`ClientMcpVerdict::IgnoresClientMcp`].
#[must_use]
pub fn grok_acp_client_mcp_verdict_from_frames(
    frames: &[GrokAcpClientMcpFrame],
) -> ClientMcpVerdict {
    grok_acp_client_mcp_verdict(frames, &EchoMcpTranscript::default())
}

/// Decides the verdict from ACP frames plus the echo MCP stdio transcript.
/// Helper liveness is unproven; use [`grok_acp_client_mcp_verdict_with_helper`]
/// to score ignore.
#[must_use]
pub fn grok_acp_client_mcp_verdict(
    frames: &[GrokAcpClientMcpFrame],
    transcript: &EchoMcpTranscript,
) -> ClientMcpVerdict {
    grok_acp_client_mcp_verdict_decision(frames, transcript, false, false).verdict
}

/// Same as [`grok_acp_client_mcp_verdict`], with per-run echo helper liveness.
#[must_use]
pub fn grok_acp_client_mcp_verdict_with_helper(
    frames: &[GrokAcpClientMcpFrame],
    transcript: &EchoMcpTranscript,
    echo_helper_live: bool,
) -> ClientMcpVerdict {
    grok_acp_client_mcp_verdict_decision(frames, transcript, false, echo_helper_live).verdict
}

struct VerdictDecision {
    verdict: ClientMcpVerdict,
    inconclusive_cause: Option<InconclusiveCause>,
}

fn inconclusive(cause: InconclusiveCause) -> VerdictDecision {
    VerdictDecision {
        verdict: ClientMcpVerdict::Inconclusive,
        inconclusive_cause: Some(cause),
    }
}

fn grok_acp_client_mcp_verdict_decision(
    frames: &[GrokAcpClientMcpFrame],
    transcript: &EchoMcpTranscript,
    decisive_frame_lost: bool,
    echo_helper_live: bool,
) -> VerdictDecision {
    // Eliding chatter is not a failure to answer. Only a capture that could
    // not retain a frame the verdict depends on is scored `truncated`.
    if decisive_frame_lost {
        return inconclusive(InconclusiveCause::Truncated);
    }
    let Some(session_new_at) = frames
        .iter()
        .position(|frame| frame.is_outbound() && method_of(&frame.message) == Some("session/new"))
    else {
        return inconclusive(InconclusiveCause::MissingSessionNew);
    };
    let session_new = &frames[session_new_at];
    let servers = session_new
        .message
        .get("params")
        .and_then(|params| params.get("mcpServers"))
        .and_then(Value::as_array);
    if servers.is_none_or(|list| list.is_empty()) {
        return inconclusive(InconclusiveCause::EmptyMcpServers);
    }
    let Some(id) = session_new.message.get("id").cloned() else {
        return inconclusive(InconclusiveCause::MissingSessionNew);
    };
    // The response must follow the request: never correlate an inbound
    // frame that precedes the outbound `session/new`.
    let response = frames
        .iter()
        .skip(session_new_at + 1)
        .find(|frame| !frame.is_outbound() && frame.message.get("id") == Some(&id));
    match response {
        None => session_new_no_response_cause(frames, session_new_at),
        Some(frame) if frame.message.get("error").is_some() => {
            if error_rejects_client_mcp(frame.message.get("error").expect("error present")) {
                VerdictDecision {
                    verdict: ClientMcpVerdict::RejectsClientMcp,
                    inconclusive_cause: None,
                }
            } else {
                inconclusive(InconclusiveCause::SessionNewErrorNotClientMcp)
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
            if transcript.tools_call() {
                VerdictDecision {
                    verdict: ClientMcpVerdict::AcceptsClientMcp,
                    inconclusive_cause: None,
                }
            } else if permission_was_rejected(frames) {
                inconclusive(InconclusiveCause::PermissionRejected)
            } else if transcript.initialize() {
                if prompt_turn_completed(frames) {
                    inconclusive(InconclusiveCause::TurnCompletedWithoutToolCall)
                } else {
                    inconclusive(InconclusiveCause::NoTurnResult)
                }
            } else if echo_named_tool_called(frames) {
                inconclusive(InconclusiveCause::NativeEchoWithoutAdmission)
            } else if prompt_turn_completed(frames) {
                if echo_helper_live {
                    VerdictDecision {
                        verdict: ClientMcpVerdict::IgnoresClientMcp,
                        inconclusive_cause: None,
                    }
                } else {
                    inconclusive(InconclusiveCause::EchoLivenessUnproven)
                }
            } else {
                inconclusive(InconclusiveCause::NoTurnResult)
            }
        }
        Some(_) => session_new_no_response_cause(frames, session_new_at),
    }
}

/// Classifies a `session/new` that never resolved. A request inside the
/// exchange that the probe failed to answer is harness-shaped; otherwise the
/// agent simply did not respond within [`LIVE_SESSION_NEW_WAIT`].
fn session_new_no_response_cause(
    frames: &[GrokAcpClientMcpFrame],
    session_new_at: usize,
) -> VerdictDecision {
    let unanswered =
        frames
            .iter()
            .enumerate()
            .skip(session_new_at + 1)
            .any(|(request_at, frame)| {
                !frame.is_outbound()
                    && method_of(&frame.message).is_some()
                    && frame.message.get("id").is_some_and(|id| !id.is_null())
                    && !request_was_answered(
                        frames,
                        request_at,
                        frame.message.get("id").expect("id present"),
                    )
            });
    if unanswered {
        inconclusive(InconclusiveCause::SessionNewUnanswered)
    } else {
        inconclusive(InconclusiveCause::SessionNewBoundExceeded)
    }
}

/// Returns whether an outbound frame after the inbound request at
/// `request_at` answers `id` with a result or error. An answer captured
/// before the request it responds to is never correlated.
fn request_was_answered(frames: &[GrokAcpClientMcpFrame], request_at: usize, id: &Value) -> bool {
    frames[request_at + 1..].iter().any(|frame| {
        frame.is_outbound()
            && frame.message.get("id") == Some(id)
            && (frame.message.get("result").is_some() || frame.message.get("error").is_some())
    })
}

/// Decides the conforming client reply for one inbound ACP request.
///
/// Returns `None` for notifications and non-requests. `session/request_permission`
/// selects the agent's own `allow_once` option, or cancels when none exists.
/// Filesystem, terminal, and every other method outside that allowlist is
/// refused with a recorded JSON-RPC error: the probe declares no such client
/// capability and never reads or writes the filesystem, runs a shell, or
/// touches the network. Every inbound request leaves the probe answered, so a
/// provider is never left waiting on probe silence.
#[must_use]
pub fn grok_acp_client_request_reply(request: &Value) -> Option<Value> {
    if request.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        return None;
    }
    let id = request.get("id").cloned()?;
    let method = request.get("method").and_then(Value::as_str)?;
    Some(match method {
        "session/request_permission" => permission_response(id, request),
        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32601,
                "message": "method not in the probe client allowlist"
            }
        }),
    })
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

/// Spawns `echo_mcp` and requires an `initialize` reply. Does not touch the
/// probe transcript, spawn Grok, or require the Desktop gate.
#[must_use]
pub fn echo_mcp_helper_is_live(echo_mcp: &Path) -> bool {
    if !echo_mcp.is_file() {
        return false;
    }
    let Ok(mut child) = Command::new(echo_mcp)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        return false;
    };
    let live = echo_mcp_helper_initialize_replies(&mut child);
    let _ = child.kill();
    let _ = child.wait();
    live
}

fn echo_mcp_helper_initialize_replies(child: &mut Child) -> bool {
    let Some(mut stdin) = child.stdin.take() else {
        return false;
    };
    let Some(stdout) = child.stdout.take() else {
        return false;
    };
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "swallowtail-echo-liveness"}
        }
    });
    let Ok(frame) = grok_acp_echo_mcp_stdio_frame(&request) else {
        return false;
    };
    if stdin.write_all(&frame).is_err() || stdin.flush().is_err() {
        return false;
    }
    let (sender, incoming) = mpsc::channel();
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        let _ = sender.send(line);
    });
    let Ok(line) = incoming.recv_timeout(ECHO_HELPER_LIVE_WAIT) else {
        return false;
    };
    let Ok(reply) = serde_json::from_str::<Value>(line.trim()) else {
        return false;
    };
    reply
        .pointer("/result/serverInfo/name")
        .and_then(Value::as_str)
        == Some(ECHO_MCP_SERVER_NAME)
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
    let grok_home = isolated_grok_home()?;
    if !grok_executable.is_file() || !echo_mcp.is_file() {
        return Err(GrokAcpClientMcpError {
            kind: GrokAcpClientMcpErrorKind::LivePathMissing,
        });
    }
    let echo_helper_live = echo_mcp_helper_is_live(echo_mcp);
    LiveGrokAcpPeer::spawn(grok_executable, &grok_home, echo_helper_live)
}

/// Live ACP stdio child. Constructed only after the Desktop gate succeeds.
pub struct LiveGrokAcpPeer {
    child: Child,
    stdin: Option<ChildStdin>,
    incoming: Receiver<Result<Value, GrokAcpClientMcpError>>,
    echo_helper_live: bool,
}

impl LiveGrokAcpPeer {
    fn spawn(
        executable: &Path,
        grok_home: &Path,
        echo_helper_live: bool,
    ) -> Result<Self, GrokAcpClientMcpError> {
        let mut command = Command::new(executable);
        command
            .args(["--no-auto-update", "agent", "stdio"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("HOME", grok_home)
            .env("GROK_HOME", grok_home);
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
            echo_helper_live,
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
        self.take_inbound_within(LIVE_PROTOCOL_WAIT)
    }

    fn take_inbound_within(
        &mut self,
        bound: Duration,
    ) -> Result<Vec<Value>, GrokAcpClientMcpError> {
        drain_within(&self.incoming, bound)
    }

    fn close(&mut self) -> GrokAcpClientMcpCleanup {
        self.stdin.take();
        let started = std::time::Instant::now();
        while started.elapsed() < LIVE_JOIN {
            match self.child.try_wait() {
                Ok(Some(_)) => return GrokAcpClientMcpCleanup { joined: true },
                // Poll gap only: the exit deadline is `LIVE_JOIN` above.
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

    fn echo_helper_live(&mut self) -> bool {
        self.echo_helper_live
    }
}

/// Reads one inbound burst, spending at most `bound` in total.
///
/// The first message may take the whole budget; after that [`LIVE_IDLE`]
/// ends the burst. The budget is a hard ceiling on both: an agent that
/// streams without pause cannot hold the drain open past the deadline its
/// exchange set, so the probe's worst-case wall clock is the sum of the
/// exchange bounds and nothing else.
fn drain_within(
    incoming: &Receiver<Result<Value, GrokAcpClientMcpError>>,
    bound: Duration,
) -> Result<Vec<Value>, GrokAcpClientMcpError> {
    let deadline = std::time::Instant::now() + bound;
    let mut messages = Vec::new();
    match incoming.recv_timeout(bound) {
        Ok(Ok(value)) => messages.push(value),
        Ok(Err(error)) => return Err(error),
        Err(RecvTimeoutError::Timeout | RecvTimeoutError::Disconnected) => return Ok(messages),
    }
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        match incoming.recv_timeout(LIVE_IDLE.min(remaining)) {
            Ok(Ok(value)) => messages.push(value),
            Ok(Err(error)) => return Err(error),
            Err(RecvTimeoutError::Timeout | RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(messages)
}

#[allow(dead_code)]
enum FakeBehavior {
    Verdict(ClientMcpVerdict),
    Oracle(ClientMcpOracleShape),
    AuthenticateFailed,
    SessionUnauthorized,
    Chatty,
    PromptSilent,
    AsksPermission,
    /// Inbound `session/load` inside `session/new`; the recorded refusal
    /// satisfies this agent, which then completes the session.
    SessionNewForeignRequest,
}

struct FakeAcpPeer {
    behavior: FakeBehavior,
    inbound: VecDeque<Value>,
    closed: bool,
    prompt_id: Option<Value>,
    emitted_echo: bool,
    session_new_id: Option<Value>,
}

impl FakeAcpPeer {
    fn with_behavior(behavior: FakeBehavior) -> Self {
        Self {
            behavior,
            inbound: VecDeque::new(),
            closed: false,
            prompt_id: None,
            emitted_echo: false,
            session_new_id: None,
        }
    }

    fn new(scenario: ClientMcpVerdict) -> Self {
        Self::with_behavior(FakeBehavior::Verdict(scenario))
    }

    fn oracle(shape: ClientMcpOracleShape) -> Self {
        Self::with_behavior(FakeBehavior::Oracle(shape))
    }

    #[cfg(test)]
    fn authenticate_failed() -> Self {
        Self::with_behavior(FakeBehavior::AuthenticateFailed)
    }

    #[cfg(test)]
    fn session_unauthorized() -> Self {
        Self::with_behavior(FakeBehavior::SessionUnauthorized)
    }

    #[cfg(test)]
    fn chatty() -> Self {
        Self::with_behavior(FakeBehavior::Chatty)
    }

    #[cfg(test)]
    fn prompt_silent() -> Self {
        Self::with_behavior(FakeBehavior::PromptSilent)
    }

    #[cfg(test)]
    fn asks_permission() -> Self {
        Self::with_behavior(FakeBehavior::AsksPermission)
    }

    #[cfg(test)]
    fn session_new_foreign_request() -> Self {
        Self::with_behavior(FakeBehavior::SessionNewForeignRequest)
    }

    fn push(&mut self, message: Value) {
        self.inbound.push_back(message);
    }

    fn push_chatter(&mut self, count: usize) {
        for index in 0..count {
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
    }

    /// Emits the echo tool call and its result without ending the turn, so a
    /// fixture can keep streaming after the decisive frames.
    fn emit_echo_accept_without_result(&mut self) {
        let prompt_id = self.prompt_id.take();
        self.emit_echo_accept();
        self.prompt_id = prompt_id;
    }

    fn emit_echo_accept(&mut self) {
        self.emitted_echo = true;
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
                    "content": []
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
                FakeBehavior::Oracle(
                    ClientMcpOracleShape::SessionNewAnswerRequired
                    | ClientMcpOracleShape::SessionNewSilentAfterAnswer,
                )
                | FakeBehavior::SessionNewForeignRequest => {
                    self.session_new_id = id.clone();
                    if matches!(self.behavior, FakeBehavior::SessionNewForeignRequest) {
                        self.push(json!({
                            "jsonrpc": "2.0",
                            "id": 812,
                            "method": "session/load",
                            "params": {"sessionId": FIXTURE_SESSION}
                        }));
                    } else {
                        self.push(json!({
                            "jsonrpc": "2.0",
                            "id": 810,
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
                }
                FakeBehavior::Verdict(
                    ClientMcpVerdict::AcceptsClientMcp | ClientMcpVerdict::IgnoresClientMcp,
                )
                | FakeBehavior::Oracle(
                    ClientMcpOracleShape::AdmittedAndCalled
                    | ClientMcpOracleShape::AdmittedListedNotCalledCompleted
                    | ClientMcpOracleShape::AdmittedNotListed
                    | ClientMcpOracleShape::NoAdmission
                    | ClientMcpOracleShape::HelperUnspawnable,
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
                FakeBehavior::Verdict(ClientMcpVerdict::AcceptsClientMcp)
                | FakeBehavior::Oracle(
                    ClientMcpOracleShape::AdmittedAndCalled
                    | ClientMcpOracleShape::SessionNewAnswerRequired,
                ) => {
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
                                "content": []
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
                FakeBehavior::Verdict(ClientMcpVerdict::IgnoresClientMcp)
                | FakeBehavior::Oracle(
                    ClientMcpOracleShape::AdmittedListedNotCalledCompleted
                    | ClientMcpOracleShape::AdmittedNotListed
                    | ClientMcpOracleShape::NoAdmission
                    | ClientMcpOracleShape::HelperUnspawnable,
                ) => {
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
                    // A real streaming turn: chatter well past the capture
                    // capacity on both sides of the frames that carry the
                    // answer, so overflow must elide the middle and keep the
                    // tool call, the tool result, and the turn result.
                    self.push_chatter(MAXIMUM_FRAMES);
                    self.prompt_id = id.clone();
                    self.emit_echo_accept_without_result();
                    self.push_chatter(MAXIMUM_FRAMES);
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
                | FakeBehavior::Oracle(ClientMcpOracleShape::SessionNewSilentAfterAnswer)
                | FakeBehavior::AuthenticateFailed
                | FakeBehavior::SessionUnauthorized
                | FakeBehavior::SessionNewForeignRequest => {}
            },
            Some("session/request_permission") | Some("fs/read_text_file") => {}
            None => {
                let answered = message.get("id").cloned();
                match self.behavior {
                    FakeBehavior::AsksPermission => {
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
                    FakeBehavior::Oracle(ClientMcpOracleShape::SessionNewAnswerRequired) => {
                        if answered == Some(json!(810))
                            && message.get("result").is_some()
                            && let Some(session_new_id) = self.session_new_id.clone()
                        {
                            self.push(json!({
                                "jsonrpc": "2.0",
                                "id": session_new_id,
                                "result": {"sessionId": FIXTURE_SESSION}
                            }));
                        }
                    }
                    FakeBehavior::Oracle(ClientMcpOracleShape::SessionNewSilentAfterAnswer) => {}
                    FakeBehavior::SessionNewForeignRequest => {
                        if answered == Some(json!(812))
                            && message.get("error").is_some()
                            && let Some(session_new_id) = self.session_new_id.clone()
                        {
                            self.push(json!({
                                "jsonrpc": "2.0",
                                "id": session_new_id,
                                "result": {"sessionId": FIXTURE_SESSION}
                            }));
                        }
                    }
                    _ => {}
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

    fn echo_mcp_transcript(&mut self) -> EchoMcpTranscript {
        if matches!(
            self.behavior,
            FakeBehavior::Verdict(ClientMcpVerdict::AcceptsClientMcp)
                | FakeBehavior::Oracle(
                    ClientMcpOracleShape::AdmittedAndCalled
                        | ClientMcpOracleShape::SessionNewAnswerRequired,
                )
                | FakeBehavior::SessionNewForeignRequest
        ) || self.emitted_echo
        {
            return EchoMcpTranscript::called();
        }
        match self.behavior {
            FakeBehavior::Oracle(ClientMcpOracleShape::AdmittedListedNotCalledCompleted)
            | FakeBehavior::PromptSilent => EchoMcpTranscript::admitted_listed(),
            FakeBehavior::Oracle(ClientMcpOracleShape::AdmittedNotListed)
            | FakeBehavior::AsksPermission => EchoMcpTranscript::observed_idle(),
            FakeBehavior::Verdict(ClientMcpVerdict::IgnoresClientMcp)
            | FakeBehavior::Oracle(
                ClientMcpOracleShape::NoAdmission | ClientMcpOracleShape::HelperUnspawnable,
            ) => EchoMcpTranscript::default(),
            _ => EchoMcpTranscript::default(),
        }
    }

    fn echo_helper_live(&mut self) -> bool {
        !matches!(
            self.behavior,
            FakeBehavior::Oracle(ClientMcpOracleShape::HelperUnspawnable)
        )
    }
}

fn exchange(
    peer: &mut dyn GrokAcpClientMcpPeer,
    capture: &mut FrameCapture,
    id: u64,
    method: &str,
    params: Value,
    inbound_bound: Duration,
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
    // One absolute deadline per exchange: answering a request never extends
    // it, and the drain keeps running past bursts that carry nothing
    // answerable (an agent may emit a notification before its request) until
    // the response to this exchange arrives or the bound expires.
    let deadline = std::time::Instant::now() + inbound_bound;
    let response_id = json!(id);
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            // The bound is spent. A backlog must not buy further passes: a
            // zero-budget drain still returns an already-queued frame, so
            // without this the loop reads one frame per pass forever.
            return Ok(());
        }
        let inbound = peer.take_inbound_within(remaining)?;
        if inbound.is_empty() {
            // The bound expired (or the deterministic peer has nothing left);
            // nothing further can be read or answered.
            return Ok(());
        }
        let mut answered = false;
        let mut saw_response = false;
        for message in inbound {
            if method_of(&message).is_none()
                && message.get("id") == Some(&response_id)
                && (message.get("result").is_some() || message.get("error").is_some())
            {
                saw_response = true;
            }
            capture.push(FrameDirection::Inbound, message.clone());
            if let Some(reply) = grok_acp_client_request_reply(&message) {
                capture.push(FrameDirection::Outbound, reply.clone());
                peer.push_outbound(reply)?;
                answered = true;
            }
        }
        if saw_response && !answered {
            return Ok(());
        }
    }
}

fn reject_stale_callback(
    peer: &mut dyn GrokAcpClientMcpPeer,
    capture: &mut FrameCapture,
) -> Result<bool, GrokAcpClientMcpError> {
    let Some(request) = peer.stale_callback_request() else {
        return Ok(false);
    };
    capture.push(FrameDirection::Inbound, request.clone());
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    reject_request(peer, capture, id)?;
    Ok(true)
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
    decisive_frame_lost: bool,
}

/// Returns whether the verdict can be decided without this frame.
///
/// Everything the scorer reads is decisive: the probe's own outbound frames
/// (its four requests and every recorded answer to an inbound request), any
/// frame carrying a correlation id (responses to those requests, and the
/// agent's own requests such as `session/request_permission`), and the
/// tool-call and tool-result session updates. What is left over is the
/// streaming chatter — `agent_message_chunk`, thought chunks, plan
/// notifications — which no verdict depends on and is evicted first.
fn frame_is_decisive(frame: &GrokAcpClientMcpFrame) -> bool {
    if frame.is_outbound() {
        return true;
    }
    if frame.message.get("id").is_some_and(|id| !id.is_null()) {
        return true;
    }
    matches!(
        update_kind(&frame.message),
        Some("tool_call" | "tool_call_update")
    )
}

impl FrameCapture {
    fn new() -> Self {
        Self {
            frames: Vec::new(),
            truncated: false,
            decisive_frame_lost: false,
        }
    }

    /// Captures one frame, holding the capsule at [`MAXIMUM_FRAMES`] by
    /// eliding frames nothing depends on.
    ///
    /// At the target the oldest evictable frame goes so the incoming frame
    /// still lands, and `truncated` records that middle frames were elided.
    /// When nothing is evictable the capsule grows to
    /// [`MAXIMUM_RETAINED_FRAMES`] rather than dropping evidence, so no
    /// ordinary session — however many distinct tool calls it makes — can
    /// cost us the answer. Only past that ceiling is a frame lost, and a
    /// decisive one then sets `decisive_frame_lost`, the sole route to
    /// [`InconclusiveCause::Truncated`].
    fn push(&mut self, direction: FrameDirection, message: Value) {
        let frame = GrokAcpClientMcpFrame { direction, message };
        if self.frames.len() < MAXIMUM_FRAMES {
            self.frames.push(frame);
            return;
        }
        if let Some(evict_at) = evictable_at(&self.frames) {
            self.frames.remove(evict_at);
            self.truncated = true;
            self.frames.push(frame);
            return;
        }
        if self.frames.len() < MAXIMUM_RETAINED_FRAMES {
            self.frames.push(frame);
            return;
        }
        self.truncated = true;
        if frame_is_decisive(&frame) {
            self.decisive_frame_lost = true;
        }
    }
}

/// Returns the oldest frame overflow is allowed to drop.
///
/// Chatter goes first. After that the only elidable decisive frame is a
/// `tool_call_update` that carries no evidence of its own and is followed by
/// another update for the same `toolCallId` — a bare progress tick. ACP
/// updates are partial refinements, so a later update does not necessarily
/// repeat what an earlier one carried; anything holding `content` or a
/// settled `status` is kept whatever follows it. Everything else — outbound
/// requests and recorded answers, correlated responses, inbound agent
/// requests, and the opening `tool_call` — is irreplaceable and is never
/// returned here.
fn evictable_at(frames: &[GrokAcpClientMcpFrame]) -> Option<usize> {
    if let Some(at) = frames.iter().position(|held| !frame_is_decisive(held)) {
        return Some(at);
    }
    frames
        .iter()
        .enumerate()
        .position(|(at, frame)| tool_call_progress_tick_is_superseded(frames, at, frame))
}

fn tool_call_progress_tick_is_superseded(
    frames: &[GrokAcpClientMcpFrame],
    at: usize,
    frame: &GrokAcpClientMcpFrame,
) -> bool {
    if update_kind(&frame.message) != Some("tool_call_update")
        || tool_call_update_carries_evidence(&frame.message)
    {
        return false;
    }
    let Some(call) = tool_call_id(&frame.message) else {
        return false;
    };
    frames[at + 1..].iter().any(|later| {
        update_kind(&later.message) == Some("tool_call_update")
            && tool_call_id(&later.message) == Some(call)
    })
}

/// Returns whether this `tool_call_update` carries something a later update
/// for the same call may not repeat: any non-empty `content`, or a `status`
/// that is not an in-flight one. Unknown statuses count as evidence, so the
/// conservative answer is always "keep".
fn tool_call_update_carries_evidence(message: &Value) -> bool {
    let update = message.pointer("/params/update");
    let carries_content = update
        .and_then(|update| update.get("content"))
        .is_some_and(|content| match content {
            Value::Null => false,
            Value::Array(items) => !items.is_empty(),
            _ => true,
        });
    let settled_status = update
        .and_then(|update| update.get("status"))
        .and_then(Value::as_str)
        .is_some_and(|status| {
            !status.eq_ignore_ascii_case("pending") && !status.eq_ignore_ascii_case("in_progress")
        });
    carries_content || settled_status
}

fn tool_call_id(message: &Value) -> Option<&str> {
    message
        .pointer("/params/update/toolCallId")
        .and_then(Value::as_str)
}

fn method_of(message: &Value) -> Option<&str> {
    message.get("method").and_then(Value::as_str)
}

/// Returns the result carried by the reply to the outbound `method` request
/// `id`. Only a reply that follows the outbound request is correlated: a
/// response captured before the request it resolves is never accepted.
fn response_result<'a>(
    frames: &'a [GrokAcpClientMcpFrame],
    id: u64,
    method: &str,
) -> Option<&'a Value> {
    let id = json!(id);
    let request_at = frames.iter().rposition(|frame| {
        frame.is_outbound()
            && method_of(&frame.message) == Some(method)
            && frame.message.get("id") == Some(&id)
    })?;
    frames[request_at + 1..].iter().find_map(|frame| {
        if frame.is_outbound() || frame.message.get("id") != Some(&id) {
            None
        } else {
            frame.message.get("result")
        }
    })
}

/// Same as [`response_result`] for an error reply.
fn response_error<'a>(
    frames: &'a [GrokAcpClientMcpFrame],
    id: u64,
    method: &str,
) -> Option<&'a Value> {
    let id = json!(id);
    let request_at = frames.iter().rposition(|frame| {
        frame.is_outbound()
            && method_of(&frame.message) == Some(method)
            && frame.message.get("id") == Some(&id)
    })?;
    frames[request_at + 1..].iter().find_map(|frame| {
        if frame.is_outbound() || frame.message.get("id") != Some(&id) {
            None
        } else {
            frame.message.get("error")
        }
    })
}

fn request_succeeded(frames: &[GrokAcpClientMcpFrame], id: u64, method: &str) -> bool {
    response_error(frames, id, method).is_none() && response_result(frames, id, method).is_some()
}

fn session_id_from(frames: &[GrokAcpClientMcpFrame], id: u64) -> Option<String> {
    response_result(frames, id, "session/new")?
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
    prompt_turn_observation(frames).0
}

fn prompt_turn_observation(frames: &[GrokAcpClientMcpFrame]) -> (bool, Option<String>) {
    let Some(prompt) = frames
        .iter()
        .find(|frame| frame.is_outbound() && method_of(&frame.message) == Some("session/prompt"))
    else {
        return (false, None);
    };
    let Some(id) = prompt.message.get("id") else {
        return (false, None);
    };
    let Some(result) = frames.iter().find_map(|frame| {
        if frame.is_outbound() || frame.message.get("id") != Some(id) {
            None
        } else {
            frame.message.get("result")
        }
    }) else {
        return (false, None);
    };
    let stop_reason = result
        .get("stopReason")
        .and_then(Value::as_str)
        .map(str::to_owned);
    (true, stop_reason)
}

fn permission_was_rejected(frames: &[GrokAcpClientMcpFrame]) -> bool {
    frames.iter().enumerate().any(|(request_at, frame)| {
        !frame.is_outbound()
            && method_of(&frame.message) == Some("session/request_permission")
            && permission_response_rejected(
                frames,
                request_at,
                frame.message.get("id"),
                &frame.message,
            )
    })
}

/// Rejection is decided only from the probe's reply after the inbound
/// permission request: an answer captured before the request is never
/// correlated.
fn permission_response_rejected(
    frames: &[GrokAcpClientMcpFrame],
    request_at: usize,
    id: Option<&Value>,
    request: &Value,
) -> bool {
    let Some(id) = id else {
        return false;
    };
    let Some(response) = frames[request_at + 1..].iter().find(|frame| {
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
        assert_eq!(
            capsule.inconclusive_cause(),
            Some(InconclusiveCause::MissingSessionNew)
        );
        assert_eq!(capsule.prompt(), ECHO_PROMPT);
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
        assert_eq!(
            capsule.inconclusive_cause(),
            Some(InconclusiveCause::SessionNewErrorNotClientMcp)
        );
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
    fn capture_overflow_keeps_the_decisive_frames_and_still_scores() {
        let mut peer = FakeAcpPeer::chatty();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("overflow still writes a capsule");
        assert!(capsule.truncated(), "the chatter must overflow the bound");
        assert_eq!(capsule.frames().len(), MAXIMUM_FRAMES);
        assert_eq!(capsule.to_json()["truncated"], json!(true));
        assert_eq!(capsule.verdict(), ClientMcpVerdict::AcceptsClientMcp);
        assert_eq!(capsule.inconclusive_cause(), None);
        assert!(capsule.prompt_turn_completed());
        assert_eq!(capsule.stop_reason(), Some("end_turn"));
        let frames = capsule.frames();
        assert!(
            frames
                .iter()
                .any(|frame| frame.is_outbound()
                    && method_of(frame.message()) == Some("session/new")),
            "the session/new request must survive truncation"
        );
        assert!(
            frames
                .iter()
                .any(|frame| frame.is_outbound()
                    && method_of(frame.message()) == Some("session/prompt")),
            "the prompt request must survive truncation"
        );
        for kind in ["tool_call", "tool_call_update"] {
            assert!(
                frames
                    .iter()
                    .any(|frame| update_kind(frame.message()) == Some(kind)),
                "the {kind} frame must survive truncation"
            );
        }
        assert!(
            frames.iter().any(|frame| {
                !frame.is_outbound()
                    && frame.message().pointer("/result/stopReason") == Some(&json!("end_turn"))
            }),
            "the turn result must survive truncation"
        );
        assert!(
            frames
                .iter()
                .any(|frame| { update_kind(frame.message()) == Some("agent_message_chunk") }),
            "eviction takes the oldest chatter, not every chunk"
        );
    }

    #[test]
    fn capture_evicts_chatter_before_any_decisive_frame() {
        let mut capture = FrameCapture::new();
        let decisive = json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {
                "sessionId": FIXTURE_SESSION,
                "update": {"sessionUpdate": "tool_call", "toolCallId": "echo-1"}
            }
        });
        capture.push(FrameDirection::Inbound, decisive.clone());
        for index in 0..MAXIMUM_FRAMES * 2 {
            capture.push(
                FrameDirection::Inbound,
                json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": FIXTURE_SESSION,
                        "update": {
                            "sessionUpdate": "agent_message_chunk",
                            "content": {"type": "text", "text": index.to_string()}
                        }
                    }
                }),
            );
        }
        assert!(capture.truncated);
        assert!(!capture.decisive_frame_lost);
        assert_eq!(capture.frames.len(), MAXIMUM_FRAMES);
        assert_eq!(capture.frames[0].message, decisive);
    }

    #[test]
    fn capture_full_of_decisive_frames_names_the_lost_frame() {
        let mut capture = FrameCapture::new();
        for index in 0..=MAXIMUM_RETAINED_FRAMES {
            capture.push(
                FrameDirection::Outbound,
                json!({"jsonrpc": "2.0", "id": index, "method": "session/prompt"}),
            );
        }
        assert!(capture.truncated);
        assert!(capture.decisive_frame_lost);
        assert_eq!(
            grok_acp_client_mcp_verdict_decision(
                &capture.frames,
                &EchoMcpTranscript::called(),
                capture.decisive_frame_lost,
                true
            )
            .inconclusive_cause,
            Some(InconclusiveCause::Truncated)
        );
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
        assert_eq!(
            grok_acp_client_mcp_verdict_decision(
                &frames,
                &EchoMcpTranscript::default(),
                false,
                false,
            )
            .inconclusive_cause,
            Some(InconclusiveCause::NoTurnResult)
        );
    }

    #[test]
    fn prompt_timeout_without_result_is_inconclusive() {
        let mut peer = FakeAcpPeer::prompt_silent();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("capsule");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(
            capsule.inconclusive_cause(),
            Some(InconclusiveCause::NoTurnResult)
        );
        assert!(capsule.client_mcp_admitted());
        assert!(capsule.client_mcp_tools_listed());
        assert!(!capsule.prompt_turn_completed());
        assert_eq!(capsule.stop_reason(), None);
        assert_eq!(capsule.prompt(), ECHO_PROMPT);
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
    fn session_new_permission_request_is_answered_and_session_completes() {
        for version in ["1.0.4", "1.0.5"] {
            let capsule = grok_acp_client_mcp_oracle_fixture_probe(
                version,
                ClientMcpOracleShape::SessionNewAnswerRequired,
            )
            .expect("answer-required fixture completes");
            assert_eq!(capsule.verdict(), ClientMcpVerdict::AcceptsClientMcp);
            assert!(capsule.prompt_turn_completed());
            let answer_at = capsule
                .frames()
                .iter()
                .position(|frame| {
                    !frame.is_outbound()
                        && method_of(frame.message()) == Some("session/request_permission")
                })
                .expect("inbound session/new permission request captured");
            assert!(
                request_was_answered(
                    capsule.frames(),
                    answer_at,
                    capsule.frames()[answer_at]
                        .message()
                        .get("id")
                        .expect("request id")
                ),
                "the probe must answer the request during session/new"
            );
            let selected = capsule.frames().iter().find_map(|frame| {
                frame
                    .is_outbound()
                    .then(|| frame.message().pointer("/result/outcome/optionId"))
                    .flatten()
                    .and_then(Value::as_str)
            });
            assert_eq!(selected, Some("allow_once"));
        }
    }

    #[test]
    fn session_new_silent_after_recorded_answer_is_bound_exceeded() {
        let capsule = grok_acp_client_mcp_oracle_fixture_probe(
            "1.0.5",
            ClientMcpOracleShape::SessionNewSilentAfterAnswer,
        )
        .expect("silent-after-answer fixture");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(
            capsule.inconclusive_cause(),
            Some(InconclusiveCause::SessionNewBoundExceeded)
        );
        let answered_request = capsule
            .frames()
            .iter()
            .enumerate()
            .any(|(request_at, frame)| {
                !frame.is_outbound()
                    && method_of(frame.message()) == Some("session/request_permission")
                    && request_was_answered(
                        capsule.frames(),
                        request_at,
                        frame.message().get("id").expect("request id"),
                    )
            });
        assert!(
            answered_request,
            "the recorded answer must be on the capsule"
        );
        assert!(
            !capsule.frames().iter().any(|frame| {
                !frame.is_outbound() && frame.message().get("sessionId").is_some()
            })
        );
        assert_eq!(
            capsule.to_json()["inconclusive_cause"],
            json!("session_new_bound_exceeded")
        );
    }

    #[test]
    fn session_new_silent_fixture_without_requests_scores_bound_exceeded() {
        let capsule = grok_acp_client_mcp_fixture_probe("1.0.4", ClientMcpVerdict::Inconclusive)
            .expect("silent fixture");
        assert_eq!(
            capsule.inconclusive_cause(),
            Some(InconclusiveCause::SessionNewBoundExceeded)
        );
    }

    #[test]
    fn session_new_foreign_request_refusal_is_recorded_and_non_blocking() {
        let mut peer = FakeAcpPeer::session_new_foreign_request();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("foreign-request fixture completes");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::AcceptsClientMcp);
        let refusal = capsule
            .frames()
            .iter()
            .find(|frame| frame.is_outbound() && frame.message().get("id") == Some(&json!(812)))
            .expect("refusal recorded on the capsule");
        assert_eq!(refusal.message()["error"]["code"], json!(-32601));
    }

    #[test]
    fn unanswered_request_in_session_new_window_is_harness_cause() {
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
                    "id": 800,
                    "method": "fs/write_text_file",
                    "params": {"sessionId": FIXTURE_SESSION, "path": "/tmp/x", "content": "x"}
                }),
            },
        ];
        let decision = grok_acp_client_mcp_verdict_decision(
            &frames,
            &EchoMcpTranscript::default(),
            false,
            true,
        );
        assert_eq!(decision.verdict, ClientMcpVerdict::Inconclusive);
        assert_eq!(
            decision.inconclusive_cause,
            Some(InconclusiveCause::SessionNewUnanswered)
        );
    }

    #[test]
    fn client_request_reply_answers_permission_and_refuses_the_rest() {
        let permission = grok_acp_client_request_reply(&json!({
            "jsonrpc": "2.0",
            "id": 810,
            "method": "session/request_permission",
            "params": {
                "options": [
                    {"optionId": "reject_once", "kind": "reject_once"},
                    {"optionId": "allow_once", "kind": "allow_once"}
                ]
            }
        }))
        .expect("permission request is answered");
        assert_eq!(
            permission["result"]["outcome"]["optionId"],
            json!("allow_once")
        );
        for method in [
            "fs/read_text_file",
            "fs/write_text_file",
            "terminal/create",
            "session/load",
        ] {
            let refused = grok_acp_client_request_reply(&json!({
                "jsonrpc": "2.0",
                "id": 900,
                "method": method,
                "params": {}
            }))
            .expect("every request leaves the probe answered");
            assert_eq!(refused["error"]["code"], json!(-32601));
        }
        assert_eq!(
            grok_acp_client_request_reply(&json!({
                "jsonrpc": "2.0",
                "method": "session/update",
                "params": {}
            })),
            None,
            "notifications are never answered"
        );
        assert_eq!(
            grok_acp_client_request_reply(&json!({
                "jsonrpc": "2.0",
                "id": 4,
                "result": {"stopReason": "end_turn"}
            })),
            None,
            "agent responses are never re-answered"
        );
    }

    #[test]
    fn session_new_inbound_bound_is_realistic_for_mcp_establishment() {
        assert!(LIVE_SESSION_NEW_WAIT >= Duration::from_secs(30));
        assert!(LIVE_SESSION_NEW_WAIT > LIVE_PROTOCOL_WAIT);
    }

    #[test]
    fn prompt_bound_is_minutes_scale_and_separate_from_protocol_bounds() {
        assert!(
            LIVE_PROMPT_WAIT >= Duration::from_secs(180),
            "a live turn that reasons and calls a tool needs minutes"
        );
        assert!(LIVE_PROMPT_WAIT > LIVE_SESSION_NEW_WAIT);
        assert!(LIVE_SESSION_NEW_WAIT > LIVE_PROTOCOL_WAIT);
        assert!(LIVE_PROTOCOL_WAIT > LIVE_IDLE);
    }

    #[test]
    fn delayed_request_after_notification_is_still_answered() {
        let mut peer = SteppedPeer::session_new_scenario();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("delayed-request capsule");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::AcceptsClientMcp);
        let answered = capsule.frames().iter().any(|frame| {
            !frame.is_outbound()
                && frame.message().get("id") == Some(&json!(850))
                && method_of(frame.message()) == Some("fs/write_text_file")
        }) && capsule.frames().iter().any(|frame| {
            frame.is_outbound()
                && frame.message().get("id") == Some(&json!(850))
                && frame.message().get("error").is_some()
        });
        assert!(
            answered,
            "the delayed request and its recorded refusal must both be captured"
        );
        assert!(capsule.prompt_turn_completed());
        // No drain ever waits longer than the exchange it belongs to; the
        // prompt exchange is now the widest of those bounds.
        assert!(
            peer.drain_bounds
                .iter()
                .all(|bound| *bound <= LIVE_PROMPT_WAIT)
        );
        let session_new_at = capsule
            .frames()
            .iter()
            .position(|frame| {
                frame.is_outbound() && method_of(frame.message()) == Some("session/new")
            })
            .expect("session/new frame");
        let request_at = capsule
            .frames()
            .iter()
            .position(|frame| {
                !frame.is_outbound() && frame.message().get("id") == Some(&json!(850))
            })
            .expect("delayed request captured");
        assert!(
            request_at > session_new_at,
            "the fs/write_text_file request must arrive after the outbound session/new"
        );
    }

    /// Deterministic peer that replays the live timing shape the review
    /// oracle cares about: a notification burst, then the client request one
    /// drain later, then the `session/new` response.
    #[derive(Default)]
    struct SteppedPeer {
        steps: VecDeque<Vec<Value>>,
        queued: VecDeque<Value>,
        drain_bounds: Vec<Duration>,
        steps_active: bool,
    }

    impl SteppedPeer {
        fn session_new_scenario() -> Self {
            Self {
                steps: VecDeque::from([
                    vec![json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": FIXTURE_SESSION,
                            "update": {"sessionUpdate": "agent_message_chunk", "content": {"type": "text", "text": "working"}}
                        }
                    })],
                    vec![json!({
                        "jsonrpc": "2.0",
                        "id": 850,
                        "method": "fs/write_text_file",
                        "params": {"sessionId": FIXTURE_SESSION, "path": "<redacted>", "content": "x"}
                    })],
                    vec![json!({
                        "jsonrpc": "2.0",
                        "id": 3,
                        "result": {"sessionId": FIXTURE_SESSION}
                    })],
                ]),
                queued: VecDeque::new(),
                drain_bounds: Vec::new(),
                steps_active: false,
            }
        }
    }

    impl GrokAcpClientMcpPeer for SteppedPeer {
        fn push_outbound(&mut self, message: Value) -> Result<(), GrokAcpClientMcpError> {
            if method_of(&message) == Some("session/new") {
                // The stepped notification/request/response sequence belongs
                // to this exchange only; earlier exchanges drain `queued`.
                self.steps_active = true;
                return Ok(());
            }
            let reply = match method_of(&message) {
                Some("initialize") => json!({
                    "jsonrpc": "2.0",
                    "id": message.get("id").cloned(),
                    "result": {"protocolVersion": ACP_PROTOCOL_VERSION, "agentCapabilities": {}, "authMethods": []}
                }),
                Some("authenticate") => json!({
                    "jsonrpc": "2.0",
                    "id": message.get("id").cloned(),
                    "result": {}
                }),
                Some("session/prompt") => json!({
                    "jsonrpc": "2.0",
                    "id": message.get("id").cloned(),
                    "result": {"stopReason": "end_turn"}
                }),
                _ => return Ok(()),
            };
            self.queued.push_back(reply);
            Ok(())
        }

        fn take_inbound(&mut self) -> Result<Vec<Value>, GrokAcpClientMcpError> {
            self.take_inbound_within(LIVE_PROTOCOL_WAIT)
        }
        fn take_inbound_within(
            &mut self,
            bound: Duration,
        ) -> Result<Vec<Value>, GrokAcpClientMcpError> {
            self.drain_bounds.push(bound);
            if self.steps_active
                && let Some(step) = self.steps.pop_front()
            {
                return Ok(step);
            }
            Ok(self.queued.drain(..).collect())
        }

        fn close(&mut self) -> GrokAcpClientMcpCleanup {
            GrokAcpClientMcpCleanup { joined: true }
        }

        fn stale_callback_request(&mut self) -> Option<Value> {
            None
        }

        fn echo_mcp_transcript(&mut self) -> EchoMcpTranscript {
            EchoMcpTranscript::called()
        }

        fn echo_helper_live(&mut self) -> bool {
            true
        }
    }

    /// Latency the slow-turn peer models: longer than any protocol-scale
    /// bound this module ever used, shorter than [`LIVE_PROMPT_WAIT`].
    const SLOW_TURN_LATENCY: Duration = Duration::from_secs(45);

    /// Deterministic stand-in for a live turn that reasons before it answers.
    ///
    /// It releases the turn frames only when the exchange offers it at least
    /// [`SLOW_TURN_LATENCY`], and otherwise reports the empty drain a real
    /// timeout produces. Shrink the prompt bound back to protocol scale and
    /// this fixture scores `no_turn_result`, exactly as the 2026-09-08 `1.0.5`
    /// rerun did.
    ///
    /// It pins the wiring, not the clock: the bound offered is the signal, so
    /// it spends no wall clock and cannot observe how a drain spends its
    /// budget. `drain_within_returns_inside_its_budget_under_a_continuous_stream`
    /// covers that on a real clock.
    #[derive(Default)]
    struct SlowTurnPeer {
        queued: VecDeque<Value>,
        prompt_id: Option<Value>,
        prompt_bound: Option<Duration>,
    }

    impl GrokAcpClientMcpPeer for SlowTurnPeer {
        fn push_outbound(&mut self, message: Value) -> Result<(), GrokAcpClientMcpError> {
            let id = message.get("id").cloned();
            match method_of(&message) {
                Some("initialize") => self.queued.push_back(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {"protocolVersion": ACP_PROTOCOL_VERSION, "agentCapabilities": {}, "authMethods": []}
                })),
                Some("authenticate") => self.queued.push_back(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {}
                })),
                Some("session/new") => self.queued.push_back(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {"sessionId": FIXTURE_SESSION}
                })),
                Some("session/prompt") => self.prompt_id = id,
                _ => {}
            }
            Ok(())
        }

        fn take_inbound(&mut self) -> Result<Vec<Value>, GrokAcpClientMcpError> {
            self.take_inbound_within(LIVE_PROTOCOL_WAIT)
        }

        fn take_inbound_within(
            &mut self,
            bound: Duration,
        ) -> Result<Vec<Value>, GrokAcpClientMcpError> {
            let Some(prompt_id) = self.prompt_id.clone() else {
                return Ok(self.queued.drain(..).collect());
            };
            self.prompt_bound.get_or_insert(bound);
            if bound < SLOW_TURN_LATENCY {
                // The model was still thinking when our bound expired.
                return Ok(Vec::new());
            }
            self.prompt_id = None;
            Ok(vec![
                json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": FIXTURE_SESSION,
                        "update": {
                            "sessionUpdate": "tool_call",
                            "toolCallId": "echo-1",
                            "title": ECHO_MCP_TOOL,
                            "status": "in_progress",
                            "content": []
                        }
                    }
                }),
                json!({
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
                }),
                json!({
                    "jsonrpc": "2.0",
                    "id": prompt_id,
                    "result": {"stopReason": "end_turn"}
                }),
            ])
        }

        fn close(&mut self) -> GrokAcpClientMcpCleanup {
            GrokAcpClientMcpCleanup { joined: true }
        }

        fn stale_callback_request(&mut self) -> Option<Value> {
            None
        }

        fn echo_mcp_transcript(&mut self) -> EchoMcpTranscript {
            EchoMcpTranscript::called()
        }

        fn echo_helper_live(&mut self) -> bool {
            true
        }
    }

    #[test]
    fn slow_turn_beyond_the_old_prompt_bound_still_scores_accepts() {
        let mut peer = SlowTurnPeer::default();
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("slow-turn capsule");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::AcceptsClientMcp);
        assert_eq!(capsule.inconclusive_cause(), None);
        assert!(capsule.prompt_turn_completed());
        assert_eq!(capsule.stop_reason(), Some("end_turn"));
        let prompt_bound = peer.prompt_bound.expect("prompt exchange drained once");
        assert!(
            prompt_bound >= SLOW_TURN_LATENCY,
            "the prompt exchange must outlive a turn that reasons before answering"
        );
    }

    #[test]
    fn drain_within_returns_inside_its_budget_under_a_continuous_stream() {
        // An agent that never pauses must not hold the drain open past the
        // deadline its exchange set: without a total budget, `LIVE_IDLE`
        // alone keeps re-arming and the exchange bound means nothing.
        let (sender, incoming) = mpsc::channel();
        // The stream must still be running when the drain starts, or a
        // pre-fix drain could pass by finding the channel already exhausted.
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let stream_barrier = std::sync::Arc::clone(&barrier);
        let stream = thread::spawn(move || {
            stream_barrier.wait();
            for index in 0..100 {
                if sender.send(Ok(json!({"chunk": index}))).is_err() {
                    return;
                }
                thread::sleep(Duration::from_millis(20));
            }
        });
        barrier.wait();
        let budget = Duration::from_millis(100);
        let started = std::time::Instant::now();
        let messages = drain_within(&incoming, budget).expect("drain");
        let elapsed = started.elapsed();
        assert!(
            elapsed < Duration::from_millis(1_000),
            "drain spent {elapsed:?} against a {budget:?} budget"
        );
        assert!(!messages.is_empty(), "the stream was read, not skipped");
        drop(incoming);
        let _ = stream.join();
    }

    #[test]
    fn capture_evicts_superseded_tool_call_updates_before_the_answer() {
        // ACP lets one tool call be refined many times. A capacity filled
        // with those refinements must not cost us the turn result.
        let mut capture = FrameCapture::new();
        capture.push(
            FrameDirection::Outbound,
            json!({"jsonrpc": "2.0", "id": 3, "method": "session/new"}),
        );
        capture.push(
            FrameDirection::Inbound,
            json!({
                "jsonrpc": "2.0",
                "method": "session/update",
                "params": {
                    "sessionId": FIXTURE_SESSION,
                    "update": {"sessionUpdate": "tool_call", "toolCallId": "echo-1"}
                }
            }),
        );
        for index in 0..MAXIMUM_FRAMES * 2 {
            capture.push(
                FrameDirection::Inbound,
                json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": FIXTURE_SESSION,
                        "update": {
                            "sessionUpdate": "tool_call_update",
                            "toolCallId": "echo-1",
                            "status": "in_progress",
                            "title": index.to_string()
                        }
                    }
                }),
            );
        }
        let answer = json!({"jsonrpc": "2.0", "id": 4, "result": {"stopReason": "end_turn"}});
        capture.push(FrameDirection::Inbound, answer.clone());

        assert!(capture.truncated);
        assert!(
            !capture.decisive_frame_lost,
            "a superseded refinement must be evicted before the answer"
        );
        assert_eq!(capture.frames.len(), MAXIMUM_FRAMES);
        assert_eq!(
            capture.frames.last().expect("answer retained").message,
            answer
        );
        assert!(
            capture
                .frames
                .iter()
                .any(|frame| { update_kind(&frame.message) == Some("tool_call") }),
            "the opening tool call is never evicted"
        );
        assert!(
            capture.frames.iter().any(
                |frame| frame.is_outbound() && method_of(&frame.message) == Some("session/new")
            ),
            "the session/new request is never evicted"
        );
        let newest_update = capture
            .frames
            .iter()
            .rfind(|frame| update_kind(&frame.message) == Some("tool_call_update"))
            .expect("an update survives");
        assert_eq!(
            newest_update
                .message
                .pointer("/params/update/title")
                .and_then(Value::as_str),
            Some((MAXIMUM_FRAMES * 2 - 1).to_string().as_str()),
            "the newest progress tick for the call must be the one retained"
        );
    }

    #[test]
    fn a_result_carrying_update_survives_a_later_partial_update() {
        // ACP updates are partial: a later `{toolCallId, title}` refinement
        // does not repeat the content an earlier one carried, so the earlier
        // frame is not superseded by it.
        let mut capture = FrameCapture::new();
        let result = json!({
            "jsonrpc": "2.0",
            "method": "session/update",
            "params": {
                "sessionId": FIXTURE_SESSION,
                "update": {
                    "sessionUpdate": "tool_call_update",
                    "toolCallId": "echo-1",
                    "status": "completed",
                    "content": [{"type": "content", "content": {"type": "text", "text": "ping"}}]
                }
            }
        });
        capture.push(FrameDirection::Inbound, result.clone());
        for _ in 0..MAXIMUM_FRAMES * 2 {
            capture.push(
                FrameDirection::Inbound,
                json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": FIXTURE_SESSION,
                        "update": {
                            "sessionUpdate": "tool_call_update",
                            "toolCallId": "echo-1",
                            "title": ECHO_MCP_TOOL
                        }
                    }
                }),
            );
        }
        assert!(
            capture.frames.iter().any(|frame| frame.message == result),
            "an update carrying the tool result is never elided"
        );
        assert!(!capture.decisive_frame_lost);
    }

    #[test]
    fn many_distinct_tool_calls_do_not_cost_the_turn_result() {
        // Nothing limits an agent to one echo invocation, and each call has
        // its own id, so a capacity of irreplaceable frames must grow rather
        // than discard the answer.
        let mut capture = FrameCapture::new();
        capture.push(
            FrameDirection::Outbound,
            json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "session/new",
                "params": {"cwd": FIXTURE_CWD, "mcpServers": [{"name": ECHO_MCP_SERVER_NAME}]}
            }),
        );
        capture.push(
            FrameDirection::Inbound,
            json!({"jsonrpc": "2.0", "id": 3, "result": {"sessionId": FIXTURE_SESSION}}),
        );
        capture.push(
            FrameDirection::Outbound,
            json!({"jsonrpc": "2.0", "id": 4, "method": "session/prompt"}),
        );
        for call in 0..MAXIMUM_FRAMES {
            let id = format!("echo-{call}");
            capture.push(
                FrameDirection::Inbound,
                json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": FIXTURE_SESSION,
                        "update": {
                            "sessionUpdate": "tool_call",
                            "toolCallId": id,
                            "title": ECHO_MCP_TOOL
                        }
                    }
                }),
            );
            capture.push(
                FrameDirection::Inbound,
                json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": FIXTURE_SESSION,
                        "update": {
                            "sessionUpdate": "tool_call_update",
                            "toolCallId": id,
                            "status": "completed",
                            "content": [{"type": "content", "content": {"type": "text", "text": "ping"}}]
                        }
                    }
                }),
            );
        }
        let answer = json!({"jsonrpc": "2.0", "id": 4, "result": {"stopReason": "end_turn"}});
        capture.push(FrameDirection::Inbound, answer.clone());

        assert!(!capture.decisive_frame_lost);
        assert!(capture.frames.len() > MAXIMUM_FRAMES);
        assert!(capture.frames.len() <= MAXIMUM_RETAINED_FRAMES);
        assert_eq!(
            capture.frames.last().expect("answer retained").message,
            answer
        );
        assert_eq!(
            grok_acp_client_mcp_verdict_decision(
                &capture.frames,
                &EchoMcpTranscript::called(),
                capture.decisive_frame_lost,
                true
            )
            .inconclusive_cause,
            None
        );
    }

    #[test]
    fn exchange_stops_when_its_bound_is_spent_even_with_a_backlog() {
        // A zero-budget drain still returns an already-queued frame, so an
        // exchange that keeps looping on a spent deadline reads one frame per
        // pass forever.
        #[derive(Default)]
        struct BacklogPeer {
            drains: usize,
        }

        impl GrokAcpClientMcpPeer for BacklogPeer {
            fn push_outbound(&mut self, _message: Value) -> Result<(), GrokAcpClientMcpError> {
                Ok(())
            }

            fn take_inbound(&mut self) -> Result<Vec<Value>, GrokAcpClientMcpError> {
                self.take_inbound_within(LIVE_PROTOCOL_WAIT)
            }

            fn take_inbound_within(
                &mut self,
                _bound: Duration,
            ) -> Result<Vec<Value>, GrokAcpClientMcpError> {
                self.drains += 1;
                if self.drains > 5_000 {
                    return Ok(Vec::new());
                }
                // A backlog always has the next frame ready, whatever budget
                // it is offered; the small sleep is the read cost.
                thread::sleep(Duration::from_millis(1));
                Ok(vec![json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": FIXTURE_SESSION,
                        "update": {
                            "sessionUpdate": "agent_message_chunk",
                            "content": {"type": "text", "text": "streaming"}
                        }
                    }
                })])
            }

            fn close(&mut self) -> GrokAcpClientMcpCleanup {
                GrokAcpClientMcpCleanup { joined: true }
            }

            fn stale_callback_request(&mut self) -> Option<Value> {
                None
            }
        }

        let mut peer = BacklogPeer::default();
        let mut capture = FrameCapture::new();
        let bound = Duration::from_millis(50);
        let started = std::time::Instant::now();
        exchange(
            &mut peer,
            &mut capture,
            1,
            "session/prompt",
            json!({"sessionId": FIXTURE_SESSION}),
            bound,
        )
        .expect("exchange returns");
        let elapsed = started.elapsed();
        assert!(
            peer.drains < 1_000,
            "the exchange kept draining past its bound: {} passes",
            peer.drains
        );
        assert!(
            elapsed < Duration::from_secs(2),
            "the exchange spent {elapsed:?} against a {bound:?} bound"
        );
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
        assert_eq!(
            grok_acp_client_mcp_verdict_decision(
                &frames,
                &EchoMcpTranscript::default(),
                false,
                false,
            )
            .inconclusive_cause,
            Some(InconclusiveCause::PermissionRejected)
        );
    }

    #[test]
    fn native_echo_title_without_client_mcp_server_is_inconclusive() {
        let frames = unattributed_echo_frames();
        assert!(echo_named_tool_called(&frames));
        assert_eq!(
            grok_acp_client_mcp_verdict_from_frames(&frames),
            ClientMcpVerdict::Inconclusive
        );
        assert_eq!(
            grok_acp_client_mcp_verdict_decision(
                &frames,
                &EchoMcpTranscript::default(),
                false,
                false,
            )
            .inconclusive_cause,
            Some(InconclusiveCause::NativeEchoWithoutAdmission)
        );
    }

    #[test]
    fn live_none_stale_callback_is_not_synthesized() {
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
            fn echo_mcp_transcript(&mut self) -> EchoMcpTranscript {
                self.0.echo_mcp_transcript()
            }
            fn echo_helper_live(&mut self) -> bool {
                self.0.echo_helper_live()
            }
        }
        let mut peer = SilentStale(FakeAcpPeer::new(ClientMcpVerdict::IgnoresClientMcp));
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("capsule");
        assert!(!capsule.stale_callback_rejected());
        assert_eq!(capsule.verdict(), ClientMcpVerdict::IgnoresClientMcp);
        assert!(!capsule.frames().iter().any(|frame| {
            !frame.is_outbound() && method_of(frame.message()) == Some("fs/read_text_file")
        }));
    }

    #[test]
    fn echo_mcp_tools_call_transcript_is_accepts() {
        let frames = unattributed_echo_frames();
        assert_eq!(
            grok_acp_client_mcp_verdict(&frames, &EchoMcpTranscript::called()),
            ClientMcpVerdict::AcceptsClientMcp
        );
    }

    #[test]
    fn unattributed_echo_with_idle_transcript_is_not_ignores() {
        let frames = unattributed_echo_frames();
        let decision = grok_acp_client_mcp_verdict_decision(
            &frames,
            &EchoMcpTranscript::observed_idle(),
            false,
            true,
        );
        assert_eq!(decision.verdict, ClientMcpVerdict::Inconclusive);
        assert_eq!(
            decision.inconclusive_cause,
            Some(InconclusiveCause::TurnCompletedWithoutToolCall)
        );
    }

    #[test]
    fn empty_transcript_file_is_not_echo_liveness() {
        let path = std::env::temp_dir().join(format!(
            "swallowtail-echo-mcp-empty-{}.ndjson",
            std::process::id()
        ));
        std::fs::write(&path, b"").expect("empty transcript");
        let transcript = read_echo_mcp_transcript(&path);
        let _ = std::fs::remove_file(&path);
        assert!(!transcript.initialize());
        assert!(!transcript.tools_call());
        assert_eq!(
            grok_acp_client_mcp_verdict(&unattributed_echo_frames(), &transcript),
            ClientMcpVerdict::Inconclusive
        );
    }

    #[test]
    fn completed_prompt_without_echo_initialize_is_ignores() {
        let mut peer = FakeAcpPeer::new(ClientMcpVerdict::IgnoresClientMcp);
        let capsule =
            run_grok_acp_client_mcp_probe(&mut peer, "1.0.5", FIXTURE_COMMAND, FIXTURE_CWD)
                .expect("capsule");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::IgnoresClientMcp);
        assert!(capsule.echo_helper_live());
        assert!(!capsule.client_mcp_admitted());
        assert!(!capsule.client_mcp_tools_listed());
        assert!(capsule.prompt_turn_completed());
        assert_eq!(capsule.stop_reason(), Some("end_turn"));
        assert_eq!(capsule.prompt(), ECHO_PROMPT);
        assert_eq!(
            grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
            ClientMcpVerdict::Inconclusive
        );
        assert_eq!(
            grok_acp_client_mcp_verdict_with_helper(
                capsule.frames(),
                &EchoMcpTranscript::default(),
                true
            ),
            ClientMcpVerdict::IgnoresClientMcp
        );
    }

    #[test]
    fn frames_only_empty_transcript_cannot_score_ignores() {
        let capsule =
            grok_acp_client_mcp_fixture_probe("1.0.5", ClientMcpVerdict::IgnoresClientMcp)
                .expect("ignores fixture");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::IgnoresClientMcp);
        assert!(capsule.echo_helper_live());
        let decision = grok_acp_client_mcp_verdict_decision(
            capsule.frames(),
            &EchoMcpTranscript::default(),
            false,
            false,
        );
        assert_eq!(decision.verdict, ClientMcpVerdict::Inconclusive);
        assert_eq!(
            decision.inconclusive_cause,
            Some(InconclusiveCause::EchoLivenessUnproven)
        );
        assert_eq!(
            grok_acp_client_mcp_verdict_from_frames(capsule.frames()),
            ClientMcpVerdict::Inconclusive
        );
    }

    #[test]
    fn unspawnable_helper_is_echo_liveness_unproven() {
        let capsule = grok_acp_client_mcp_oracle_fixture_probe(
            "1.0.5",
            ClientMcpOracleShape::HelperUnspawnable,
        )
        .expect("unspawnable helper");
        assert_eq!(capsule.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(
            capsule.inconclusive_cause(),
            Some(InconclusiveCause::EchoLivenessUnproven)
        );
        assert!(!capsule.echo_helper_live());
        assert!(!capsule.client_mcp_admitted());
        assert!(capsule.prompt_turn_completed());
        assert_ne!(capsule.verdict(), ClientMcpVerdict::IgnoresClientMcp);
        assert_eq!(capsule.to_json()["echo_helper_live"], json!(false));
        assert_eq!(
            capsule.to_json()["inconclusive_cause"],
            json!("echo_liveness_unproven")
        );
    }

    #[test]
    fn dead_echo_helper_paths_are_not_live() {
        assert!(!echo_mcp_helper_is_live(Path::new(
            "/no/such/swallowtail-echo-helper"
        )));
        assert!(!echo_mcp_helper_is_live(Path::new("/bin/false")));
        assert!(!echo_mcp_helper_is_live(Path::new("/bin/true")));
    }

    #[test]
    fn review_oracle_forbids_ignores_when_echo_initialize_observed() {
        let frames = unattributed_echo_frames();
        assert!(EchoMcpTranscript::observed_idle().initialize());
        assert_ne!(
            grok_acp_client_mcp_verdict(&frames, &EchoMcpTranscript::observed_idle()),
            ClientMcpVerdict::IgnoresClientMcp
        );
        assert_ne!(
            grok_acp_client_mcp_verdict(&frames, &EchoMcpTranscript::admitted_listed()),
            ClientMcpVerdict::IgnoresClientMcp
        );
    }

    #[test]
    fn oracle_shapes_keep_admission_separate_from_verdict() {
        let called = grok_acp_client_mcp_oracle_fixture_probe(
            "1.0.5",
            ClientMcpOracleShape::AdmittedAndCalled,
        )
        .expect("called");
        assert_eq!(called.verdict(), ClientMcpVerdict::AcceptsClientMcp);
        assert!(called.client_mcp_admitted());
        assert!(called.client_mcp_tools_listed());
        assert!(called.prompt_turn_completed());
        assert_eq!(called.stop_reason(), Some("end_turn"));
        assert_eq!(called.prompt(), ECHO_PROMPT);

        let listed = grok_acp_client_mcp_oracle_fixture_probe(
            "1.0.5",
            ClientMcpOracleShape::AdmittedListedNotCalledCompleted,
        )
        .expect("listed");
        assert_eq!(listed.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(
            listed.inconclusive_cause(),
            Some(InconclusiveCause::TurnCompletedWithoutToolCall)
        );
        assert!(listed.client_mcp_admitted());
        assert!(listed.client_mcp_tools_listed());
        assert!(listed.prompt_turn_completed());
        assert_eq!(listed.stop_reason(), Some("end_turn"));
        assert_ne!(listed.verdict(), ClientMcpVerdict::IgnoresClientMcp);

        let admitted = grok_acp_client_mcp_oracle_fixture_probe(
            "1.0.5",
            ClientMcpOracleShape::AdmittedNotListed,
        )
        .expect("admitted");
        assert_eq!(admitted.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(
            admitted.inconclusive_cause(),
            Some(InconclusiveCause::TurnCompletedWithoutToolCall)
        );
        assert!(admitted.client_mcp_admitted());
        assert!(!admitted.client_mcp_tools_listed());
        assert_ne!(admitted.verdict(), ClientMcpVerdict::IgnoresClientMcp);

        let ignored =
            grok_acp_client_mcp_oracle_fixture_probe("1.0.5", ClientMcpOracleShape::NoAdmission)
                .expect("no admission");
        assert_eq!(ignored.verdict(), ClientMcpVerdict::IgnoresClientMcp);
        assert!(ignored.echo_helper_live());
        assert!(!ignored.client_mcp_admitted());
        assert!(!ignored.client_mcp_tools_listed());
        assert!(ignored.prompt_turn_completed());

        let dead = grok_acp_client_mcp_oracle_fixture_probe(
            "1.0.5",
            ClientMcpOracleShape::HelperUnspawnable,
        )
        .expect("helper unspawnable");
        assert_eq!(dead.verdict(), ClientMcpVerdict::Inconclusive);
        assert_eq!(
            dead.inconclusive_cause(),
            Some(InconclusiveCause::EchoLivenessUnproven)
        );
        assert!(!dead.echo_helper_live());
        assert_ne!(dead.verdict(), ClientMcpVerdict::IgnoresClientMcp);
    }

    #[test]
    fn echo_mcp_transcript_records_method_names() {
        let home = test_grok_home("records");
        let path = create_echo_mcp_transcript(&home).expect("create");
        append_echo_mcp_transcript(&path, "initialize").expect("append");
        append_echo_mcp_transcript(&path, "tools/list").expect("append");
        append_echo_mcp_transcript(&path, "tools/call").expect("append");
        let transcript = read_echo_mcp_transcript(&path);
        let _ = std::fs::remove_dir_all(&home);
        assert!(transcript.tools_call());
        assert_eq!(
            transcript.methods(),
            ["initialize", "tools/list", "tools/call"]
        );
    }

    #[test]
    fn exclusive_transcript_create_fails_if_present() {
        let home = test_grok_home("exists");
        let path = create_echo_mcp_transcript(&home).expect("create");
        std::fs::write(&path, "initialize\ntools/call\n").expect("plant leftover");
        assert_eq!(
            create_echo_mcp_transcript_at(&path).unwrap_err().kind(),
            GrokAcpClientMcpErrorKind::LiveTranscriptExists
        );
        let leftover = read_echo_mcp_transcript(&path);
        let _ = std::fs::remove_dir_all(&home);
        assert!(leftover.tools_call());
    }

    #[test]
    fn reused_grok_home_does_not_inherit_prior_transcript() {
        let home = test_grok_home("reuse");
        std::fs::write(
            home.join("echo-mcp-transcript.ndjson"),
            "initialize\ntools/list\ntools/call\n",
        )
        .expect("plant constant leftover");
        let first = create_echo_mcp_transcript(&home).expect("first run");
        append_echo_mcp_transcript(&first, "initialize").expect("first initialize");
        append_echo_mcp_transcript(&first, "tools/call").expect("first call");
        let second = create_echo_mcp_transcript(&home).expect("second run");
        assert_ne!(first, second);
        assert_ne!(
            second.file_name().and_then(|name| name.to_str()),
            Some("echo-mcp-transcript.ndjson")
        );
        assert!(!read_echo_mcp_transcript(&second).initialize());
        assert!(read_echo_mcp_transcript(&first).tools_call());
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn nonempty_transcript_path_is_refused_before_scoring() {
        let home = test_grok_home("nonempty");
        let leftover = home.join("leftover.ndjson");
        std::fs::write(&leftover, "initialize\ntools/call\n").expect("plant");
        let mut peer = FakeAcpPeer::new(ClientMcpVerdict::IgnoresClientMcp);
        let error = run_grok_acp_client_mcp_probe_with_transcript(
            &mut peer,
            "1.0.5",
            FIXTURE_COMMAND,
            FIXTURE_CWD,
            Some(&leftover),
        )
        .expect_err("leftover transcript");
        let _ = std::fs::remove_dir_all(&home);
        assert_eq!(
            error.kind(),
            GrokAcpClientMcpErrorKind::LiveTranscriptExists
        );
    }

    #[test]
    fn missing_isolated_grok_home_fails_closed() {
        assert_eq!(
            isolated_grok_home_from(None).unwrap_err().kind(),
            GrokAcpClientMcpErrorKind::LiveIsolationMissing
        );
        assert_eq!(
            isolated_grok_home_from(Some("/no/such/swallowtail-grok-home"))
                .unwrap_err()
                .kind(),
            GrokAcpClientMcpErrorKind::LiveIsolationMissing
        );
    }

    fn test_grok_home(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "swallowtail-grok-home-{}-{}",
            label,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("temp grok home");
        path
    }

    fn unattributed_echo_frames() -> [GrokAcpClientMcpFrame; 5] {
        [
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
        ]
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
