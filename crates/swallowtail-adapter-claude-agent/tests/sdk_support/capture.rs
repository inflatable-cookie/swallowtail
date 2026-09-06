//! Provider-free capture of the same wire projection used by the bounded live
//! harness. It retains labels, fixed codes, and typed lifecycle evidence only.

use super::host::SdkFixtureHost;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, ProcessExit, ProcessHandle, ProcessInputChunk,
    ProcessOutputChunk, ProcessOutputStream, ProcessRequest, ProcessService, RuntimeFailure,
    ScopeId, TerminalOutcome, TerminalStatus,
};

/// Frozen result-field names projected by the sidecar.
pub const SDK_RESULT_FIELD_NAMES: &[&str] = &[
    "type",
    "subtype",
    "duration_ms",
    "duration_api_ms",
    "ttft_ms",
    "ttft_stream_ms",
    "time_to_request_ms",
    "user_message_uuid",
    "user_message_uuids",
    "request_sent_wall_ms",
    "time_to_request_from_spawn_ms",
    "warm_spare_claimed",
    "time_origin_ms",
    "is_error",
    "api_error_status",
    "num_turns",
    "result",
    "stop_reason",
    "total_cost_usd",
    "usage",
    "modelUsage",
    "permission_denials",
    "queued_turn_count",
    "structured_output",
    "deferred_tool_use",
    "terminal_reason",
    "fast_mode_state",
    "fast_mode_disabled_reason",
    "origin",
    "uuid",
    "session_id",
    "errors",
];

/// Safe evidence retained from the sidecar wire.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SanitizedWireCapture {
    /// Fixed failure code from an open response, if one was emitted.
    pub open_sidecar_code: Option<String>,
    /// Presence map for every projected top-level SDK result field.
    pub result_fields: BTreeMap<String, bool>,
    /// Safe result subtype.
    pub result_subtype: Option<String>,
    /// Safe result error flag.
    pub result_is_error: Option<bool>,
    /// Safe result turn count.
    pub result_num_turns: Option<u64>,
    /// Safe result duration.
    pub result_duration_ms: Option<u64>,
    /// Whether error text was present; the text itself is never retained.
    pub result_error_text_present: Option<bool>,
    /// Type label for error text; the text itself is never retained.
    pub result_error_text_type: Option<String>,
    /// Redacted stderr-tail posture.
    pub stderr_tail: Option<String>,
    /// Exact bounded close labels.
    pub close_timeline: Vec<String>,
    /// Native exit event label.
    pub native_exit_event: Option<String>,
    /// Native exit code.
    pub native_exit_code: Option<i64>,
    /// Native exit signal label.
    pub native_exit_signal: Option<String>,
    /// Native join posture.
    pub native_join: Option<String>,
    /// Native exit observation boolean.
    pub native_exit_observed: Option<bool>,
    /// Root process exit observation from the host process handle.
    pub root_exit: Option<ProcessExit>,
}

/// Sanitized route-level record used by the provider-free harness proof.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SanitizedHarnessRecord {
    /// Stable runtime diagnostic code, if the route rejected open.
    pub route_code: Option<String>,
    /// The safe diagnostic message, including the fixed sidecar subcode.
    pub diagnostic_message: Option<String>,
    /// Typed turn outcome.
    pub terminal_status: Option<TerminalStatus>,
    /// Safe diagnostic code when the terminal status is a failure.
    pub terminal_diagnostic_code: Option<String>,
    /// Typed route cleanup outcome.
    pub cleanup_outcome: Option<CleanupOutcome>,
    /// Safe diagnostic code when cleanup degraded or failed.
    pub cleanup_diagnostic_code: Option<String>,
    /// Wire and process evidence.
    pub wire: SanitizedWireCapture,
}

/// Append-only, flush-and-sync journal for sanitized capture snapshots.
///
/// A live wrapper can disappear before its final record is assembled. Future
/// harnesses may pass this journal to [`captured_services_with_journal`] so
/// every observed wire update is durable before the next await. The journal
/// contains only fixed codes, labels, bounded numbers, and field presence;
/// it never writes paths, account values, provider payloads, or raw errors.
pub struct SanitizedCaptureJournal {
    file: File,
}

impl SanitizedCaptureJournal {
    /// Creates a fresh journal at the caller-owned path.
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;
        Ok(Self { file })
    }

    /// Persists one complete sanitized wire snapshot as one JSONL record.
    pub fn append_snapshot(&mut self, capture: &SanitizedWireCapture) -> io::Result<()> {
        let record = json!({
            "openSidecarCode": capture.open_sidecar_code,
            "resultFieldPresence": capture.result_fields,
            "resultSubtype": capture.result_subtype,
            "resultIsError": capture.result_is_error,
            "resultNumTurns": capture.result_num_turns,
            "resultDurationMs": capture.result_duration_ms,
            "resultErrorTextPresent": capture.result_error_text_present,
            "resultErrorTextType": capture.result_error_text_type,
            "stderrTailPresent": capture.stderr_tail.is_some(),
            "closeTimeline": capture.close_timeline,
            "nativeExitEvent": capture.native_exit_event,
            "nativeExitCode": capture.native_exit_code,
            "nativeExitSignal": capture.native_exit_signal,
            "nativeJoin": capture.native_join,
            "nativeExitObserved": capture.native_exit_observed,
            "rootExitObserved": capture.root_exit.is_some(),
        });
        serde_json::to_writer(&mut self.file, &record)?;
        self.file.write_all(b"\n")?;
        self.file.flush()?;
        self.file.sync_data()
    }
}

/// Builds services that capture the same stdout/stderr process boundary used
/// by the disposable live harness, while retaining only sanitized evidence.
pub fn captured_services(
    fixture: &SdkFixtureHost,
    host: swallowtail_core::ExecutionHostId,
) -> (HostServices, Arc<Mutex<SanitizedWireCapture>>) {
    captured_services_with_journal(fixture, host, None)
}

/// Builds capture services with an optional durable journal for future live
/// lanes. Existing callers keep the in-memory-only behavior by using
/// [`captured_services`].
pub fn captured_services_with_journal(
    fixture: &SdkFixtureHost,
    host: swallowtail_core::ExecutionHostId,
    journal: Option<Arc<Mutex<SanitizedCaptureJournal>>>,
) -> (HostServices, Arc<Mutex<SanitizedWireCapture>>) {
    let capture = Arc::new(Mutex::new(SanitizedWireCapture::default()));
    let services = fixture
        .services(host)
        .with_process(Arc::new(CapturingProcessService {
            inner: Arc::new(fixture.clone()),
            capture: Arc::clone(&capture),
            journal,
        }) as Arc<dyn ProcessService>);
    (services, capture)
}

/// Records a route open rejection, retaining diagnostic message text only as
/// the already-safe route diagnostic and retaining no provider payload.
pub fn record_open_failure(
    error: &RuntimeFailure,
    capture: &Arc<Mutex<SanitizedWireCapture>>,
) -> SanitizedHarnessRecord {
    SanitizedHarnessRecord {
        route_code: Some(error.diagnostic().code().to_owned()),
        diagnostic_message: Some(error.diagnostic().message().to_owned()),
        terminal_status: None,
        terminal_diagnostic_code: None,
        cleanup_outcome: None,
        cleanup_diagnostic_code: None,
        wire: capture.lock().expect("capture lock").clone(),
    }
}

/// Records a successful turn and its independent route cleanup outcome.
pub fn record_success(
    terminal: &TerminalOutcome,
    cleanup: &CleanupOutcome,
    capture: &Arc<Mutex<SanitizedWireCapture>>,
) -> SanitizedHarnessRecord {
    SanitizedHarnessRecord {
        route_code: None,
        diagnostic_message: None,
        terminal_status: Some(terminal.status().clone()),
        terminal_diagnostic_code: terminal
            .status()
            .failure()
            .map(|failure| failure.diagnostic().code().to_owned()),
        cleanup_outcome: Some(cleanup.clone()),
        cleanup_diagnostic_code: cleanup
            .diagnostic()
            .map(|diagnostic| diagnostic.code().to_owned()),
        wire: capture.lock().expect("capture lock").clone(),
    }
}

struct CapturingProcessService {
    inner: Arc<dyn ProcessService>,
    capture: Arc<Mutex<SanitizedWireCapture>>,
    journal: Option<Arc<Mutex<SanitizedCaptureJournal>>>,
}

struct CapturingProcessHandle {
    inner: Box<dyn ProcessHandle>,
    capture: Arc<Mutex<SanitizedWireCapture>>,
    journal: Option<Arc<Mutex<SanitizedCaptureJournal>>>,
    stdout_buffer: Mutex<Vec<u8>>,
}

impl ProcessService for CapturingProcessService {
    fn start(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        let inner = Arc::clone(&self.inner);
        let capture = Arc::clone(&self.capture);
        let journal = self.journal.as_ref().map(Arc::clone);
        Box::pin(async move {
            let handle = inner.start(scope, request).await?;
            Ok(Box::new(CapturingProcessHandle {
                inner: handle,
                capture,
                journal,
                stdout_buffer: Mutex::new(Vec::new()),
            }) as Box<dyn ProcessHandle>)
        })
    }
}

impl ProcessHandle for CapturingProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.inner.write_stdin(chunk)
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.inner.close_stdin()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        let pending = self.inner.read_output();
        let capture = Arc::clone(&self.capture);
        let journal = self.journal.as_ref().map(Arc::clone);
        Box::pin(async move {
            let chunk = pending.await?;
            if let Some(chunk_ref) = chunk.as_ref() {
                match chunk_ref.stream() {
                    ProcessOutputStream::Stderr => {
                        if !chunk_ref.bytes().is_empty() {
                            capture.lock().expect("capture lock").stderr_tail =
                                Some("<redacted>".to_owned());
                            persist_snapshot(&capture, journal.as_ref());
                        }
                    }
                    ProcessOutputStream::Stdout => {
                        let mut buffer = self.stdout_buffer.lock().expect("stdout lock");
                        buffer.extend_from_slice(chunk_ref.bytes());
                        while let Some(index) = buffer.iter().position(|byte| *byte == b'\n') {
                            let line: Vec<u8> = buffer.drain(..=index).collect();
                            if let Ok(record) = serde_json::from_slice::<Value>(&line) {
                                project_record(&record, &capture);
                                persist_snapshot(&capture, journal.as_ref());
                            }
                        }
                    }
                }
            }
            Ok(chunk)
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.inner.request_stop()
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.inner.force_stop()
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        let pending = self.inner.wait();
        let capture = Arc::clone(&self.capture);
        let journal = self.journal.as_ref().map(Arc::clone);
        Box::pin(async move {
            let exit = pending.await?;
            capture.lock().expect("capture lock").root_exit = Some(exit);
            persist_snapshot(&capture, journal.as_ref());
            Ok(exit)
        })
    }
}

fn persist_snapshot(
    capture: &Arc<Mutex<SanitizedWireCapture>>,
    journal: Option<&Arc<Mutex<SanitizedCaptureJournal>>>,
) {
    let Some(journal) = journal else {
        return;
    };
    let snapshot = capture.lock().expect("capture lock").clone();
    let _ = journal
        .lock()
        .expect("capture journal lock poisoned")
        .append_snapshot(&snapshot);
}

fn project_record(record: &Value, capture: &Arc<Mutex<SanitizedWireCapture>>) {
    let mut capture = capture.lock().expect("capture lock");
    match record.get("type").and_then(Value::as_str) {
        Some("response") if record.get("command") == Some(&Value::String("open".to_owned())) => {
            capture.open_sidecar_code = record
                .get("failure")
                .and_then(|failure| failure.get("code"))
                .and_then(Value::as_str)
                .map(str::to_owned);
        }
        Some("event") if record.get("event").and_then(Value::as_str) == Some("turn_ended") => {
            capture.result_fields = record
                .get("resultFieldPresence")
                .and_then(Value::as_object)
                .into_iter()
                .flatten()
                .filter_map(|(key, value)| value.as_bool().map(|present| (key.clone(), present)))
                .collect();
            capture.result_subtype = record
                .get("subtype")
                .and_then(Value::as_str)
                .map(str::to_owned);
            capture.result_is_error = record.get("isError").and_then(Value::as_bool);
            capture.result_num_turns = record.get("numTurns").and_then(Value::as_u64);
            capture.result_duration_ms = record.get("durationMs").and_then(Value::as_u64);
            capture.result_error_text_present =
                record.get("errorTextPresent").and_then(Value::as_bool);
            capture.result_error_text_type = record
                .get("errorTextType")
                .and_then(Value::as_str)
                .map(str::to_owned);
        }
        Some("response")
            if record.get("command") == Some(&Value::String("close".to_owned()))
                && record.get("success") == Some(&Value::Bool(true)) =>
        {
            if let Some(data) = record.get("data") {
                capture.close_timeline = data
                    .get("closeTimeline")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect();
                capture.native_exit_event = data
                    .get("nativeExitEvent")
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                capture.native_exit_code = data.get("nativeExitCode").and_then(Value::as_i64);
                capture.native_exit_signal = data
                    .get("nativeExitSignal")
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                capture.native_join = data
                    .get("nativeJoin")
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                capture.native_exit_observed =
                    data.get("nativeExitObserved").and_then(Value::as_bool);
            }
        }
        _ => {}
    }
}
