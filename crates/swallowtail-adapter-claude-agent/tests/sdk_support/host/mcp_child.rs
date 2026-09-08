//! Provider-side spawn of declared stdio MCP children.
//!
//! The real SDK constructs `McpStdioServerConfig` and spawns that command.
//! This fixture does the same for the reserved registered-tool courier so
//! Swallowtail never intercepts `ProcessService::start` to stand in for the
//! provider child. Card 084 consumer-declared servers stay unspawned echoes.
//!
//! Card 139: every startup step keeps bounded process output and exit
//! evidence, and the fixture answers `open` only after the courier has
//! claimed its one-shot rendezvous, so startup is an observed event rather
//! than a wall-clock hope.

use super::super::host::Shared;
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use swallowtail_adapter_claude_agent::sdk::registered_tool::CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, RuntimeFailure,
};

const REGISTERED_TOOL_SERVER: &str = CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER;

/// Bound on retained child stderr. Evidence must be useful without becoming
/// an unbounded fixture buffer.
const STDERR_EVIDENCE_CAP: usize = 4_096;

/// Named bound on the courier startup event. Expiry is a broken startup
/// contract, so it reports the child's own exit and output instead of
/// leaving the caller to read a bare failure code.
const STARTUP_BOUND: Duration = Duration::from_secs(30);

/// Poll step for the rendezvous claim. The wait ends on the claim event or on
/// child exit, whichever happens first; this only bounds how quickly either
/// is noticed.
const STARTUP_STEP: Duration = Duration::from_millis(2);

/// Bound on waiting for the stderr reader to reach end of pipe. Observing a
/// child's exit does not establish that its output has been drained, so
/// evidence waits for the drain rather than racing it. Expiry is reported
/// rather than hidden.
const DRAIN_BOUND: Duration = Duration::from_secs(2);

/// Bound on one retained evidence line, so 32 notes cannot become unbounded
/// fixture memory.
const NOTE_CAP: usize = 2_048;

/// Bounded capture of one child's stderr, with an explicit end-of-pipe
/// signal so evidence is never rendered from a half-drained buffer.
#[derive(Default)]
pub(in crate::sdk_support) struct BoundedStderr {
    bytes: Vec<u8>,
    dropped: usize,
}

/// The retained capture plus its drain-completion signal.
#[derive(Default)]
pub(in crate::sdk_support) struct StderrEvidence {
    capture: Mutex<BoundedStderr>,
    drained: AtomicBool,
    drain_changed: Condvar,
    drain_gate: Mutex<()>,
    /// Why the reader stopped, when it stopped for any reason other than
    /// reaching end of pipe. A failed read is not a complete capture.
    fault: Mutex<Option<String>>,
}

impl StderrEvidence {
    fn push(&self, chunk: &[u8]) {
        self.capture
            .lock()
            .expect("courier stderr lock")
            .push(chunk);
    }

    /// Records end of pipe: the capture is complete.
    fn finish(&self) {
        self.drained.store(true, Ordering::Release);
        let _guard = self.drain_gate.lock().expect("courier drain lock");
        self.drain_changed.notify_all();
    }

    /// Records that the reader stopped without reaching end of pipe.
    ///
    /// The wait is released, because no further bytes are coming, but the
    /// capture is never marked complete: a read that failed cannot certify
    /// that it saw everything the child wrote.
    fn fault(&self, reason: String) {
        *self.fault.lock().expect("courier fault lock") = Some(reason);
        let _guard = self.drain_gate.lock().expect("courier drain lock");
        self.drain_changed.notify_all();
    }

    /// Renders the capture after waiting, up to `DRAIN_BOUND`, for the reader
    /// to reach end of pipe. A wait that expires says so.
    fn describe_drained(&self) -> String {
        if !self.settled() {
            let deadline = Instant::now() + DRAIN_BOUND;
            let mut guard = self.drain_gate.lock().expect("courier drain lock");
            while !self.settled() {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    break;
                }
                let (next, _) = self
                    .drain_changed
                    .wait_timeout(guard, remaining)
                    .expect("courier drain wait");
                guard = next;
            }
        }
        self.render(true)
    }

    /// Whether the reader has stopped, either at end of pipe or on a fault.
    fn settled(&self) -> bool {
        self.drained.load(Ordering::Acquire)
            || self.fault.lock().expect("courier fault lock").is_some()
    }

    /// Renders without waiting, for a child that is still running and so has
    /// no end of pipe to wait for.
    fn describe_snapshot(&self) -> String {
        self.render(false)
    }

    /// Renders the capture and its completion status under one lock.
    ///
    /// The reader appends under `capture` and only then sets `drained`, so a
    /// completion observed while holding `capture` covers every byte in this
    /// rendering. Reading the flag after unlocking would let a final append
    /// certify an older rendering as complete.
    fn render(&self, waited: bool) -> String {
        let capture = self.capture.lock().expect("courier stderr lock");
        let complete = self.drained.load(Ordering::Acquire);
        let fault = self.fault.lock().expect("courier fault lock").clone();
        let rendered = capture.describe();
        drop(capture);
        match (complete, fault) {
            (_, Some(reason)) => format!("{rendered} (capture incomplete: {reason})"),
            (true, None) => rendered,
            // A snapshot that never waited must not claim an expired wait.
            (false, None) if waited => {
                format!("{rendered} (drain incomplete after {DRAIN_BOUND:?})")
            }
            (false, None) => format!("{rendered} (snapshot; drain pending)"),
        }
    }
}

impl BoundedStderr {
    fn push(&mut self, chunk: &[u8]) {
        let room = STDERR_EVIDENCE_CAP.saturating_sub(self.bytes.len());
        let taken = room.min(chunk.len());
        self.bytes.extend_from_slice(&chunk[..taken]);
        self.dropped += chunk.len() - taken;
    }

    fn describe(&self) -> String {
        if self.bytes.is_empty() && self.dropped == 0 {
            return "stderr empty".to_owned();
        }
        let text = String::from_utf8_lossy(&self.bytes);
        if self.dropped == 0 {
            format!("stderr {text:?}")
        } else {
            format!("stderr {text:?} (+{} bytes dropped)", self.dropped)
        }
    }
}

pub(in crate::sdk_support) struct SpawnedMcpChild {
    command: String,
    arguments: Vec<String>,
    stdin: Mutex<Option<ChildStdin>>,
    stdout: Mutex<Option<BufReader<ChildStdout>>>,
    child: Mutex<Child>,
    stderr: Arc<StderrEvidence>,
}

impl SpawnedMcpChild {
    fn spawn(
        command: &str,
        args: &[String],
        env: &[(String, String)],
    ) -> Result<Arc<Self>, String> {
        let mut launched = Command::new(command);
        launched
            .args(args)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (key, value) in env {
            launched.env(key, value);
        }
        let mut child = launched
            .spawn()
            .map_err(|error| describe_spawn_error(command, args, &error))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| format!("courier {command:?} exposed no stdin pipe"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| format!("courier {command:?} exposed no stdout pipe"))?;
        let stderr = Arc::new(StderrEvidence::default());
        if let Some(mut handle) = child.stderr.take() {
            let retained = Arc::clone(&stderr);
            std::thread::spawn(move || {
                let mut buffer = [0_u8; 1_024];
                loop {
                    match handle.read(&mut buffer) {
                        // End of pipe. Evidence may now be rendered completely.
                        Ok(0) => {
                            retained.finish();
                            return;
                        }
                        Ok(read) => retained.push(&buffer[..read]),
                        Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                        // A failed read is not an end of pipe. Ending the loop
                        // here as though it were would certify a capture that
                        // may be missing whatever the child wrote next.
                        Err(error) => {
                            retained.fault(format!("stderr read failed: {error}"));
                            return;
                        }
                    }
                }
            });
        } else {
            stderr.finish();
        }
        Ok(Arc::new(Self {
            command: command.to_owned(),
            arguments: args.to_vec(),
            stdin: Mutex::new(Some(stdin)),
            stdout: Mutex::new(Some(BufReader::new(stdout))),
            child: Mutex::new(child),
            stderr,
        }))
    }

    fn kill(&self) {
        let _ = self.child.lock().expect("mcp child lock").kill();
    }

    /// Waits for an already-killed child, so its stderr pipe is closed and the
    /// drain completes instead of expiring against a live writer.
    fn reap(&self) {
        let _ = self.child.lock().expect("mcp child lock").wait();
    }

    /// Returns the child's observed exit, or `None` while it is still running.
    fn exit_status(&self) -> Option<String> {
        match self.child.lock().expect("mcp child lock").try_wait() {
            Ok(Some(status)) => Some(format!("exited {status}")),
            Ok(None) => None,
            Err(error) => Some(format!("exit unobservable: {error}")),
        }
    }

    /// Bounded process output and exit evidence for a child that has ended.
    ///
    /// Waits for end of pipe, so a child's final output cannot be lost. Only
    /// call this once the child has exited or been reaped; a live writer keeps
    /// the pipe open and would cost the whole drain bound.
    pub(in crate::sdk_support) fn evidence(&self) -> String {
        let exit = self
            .exit_status()
            .unwrap_or_else(|| "still running".to_owned());
        let stderr = self.stderr.describe_drained();
        format!(
            "courier {:?} args {:?}: {exit}; {stderr}",
            self.command, self.arguments
        )
    }

    /// The same evidence for a child that may still be running, taken without
    /// waiting for a drain that a live writer will not complete.
    pub(in crate::sdk_support) fn evidence_snapshot(&self) -> String {
        let exit = self
            .exit_status()
            .unwrap_or_else(|| "still running".to_owned());
        let stderr = self.stderr.describe_snapshot();
        format!(
            "courier {:?} args {:?}: {exit}; {stderr}",
            self.command, self.arguments
        )
    }
}

impl ProcessHandle for SpawnedMcpChild {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = (|| {
            let mut stdin = self.stdin.lock().expect("mcp stdin lock");
            stdin
                .as_mut()
                .ok_or_else(mcp_closed)?
                .write_all(chunk.bytes())
                .map_err(|_| mcp_closed())?;
            Ok(())
        })();
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        drop(self.stdin.lock().expect("mcp stdin lock").take());
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        let result = (|| {
            let mut stdout = self.stdout.lock().expect("mcp stdout lock");
            let reader = stdout.as_mut().ok_or_else(mcp_closed)?;
            let mut line = Vec::new();
            let read = reader
                .read_until(b'\n', &mut line)
                .map_err(|_| mcp_closed())?;
            if read == 0 {
                return Ok(None);
            }
            Ok(Some(ProcessOutputChunk::new(
                ProcessOutputStream::Stdout,
                line,
            )))
        })();
        Box::pin(async move { result })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.kill();
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.kill();
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        let status = self.child.lock().expect("mcp child lock").wait();
        Box::pin(async move {
            status
                .map(|status| ProcessExit::new(true, status.code()))
                .map_err(|_| mcp_closed())
        })
    }
}

pub(super) fn spawn_registered_courier(shared: &Shared, params: &Value) -> Result<(), String> {
    let Some(servers) = params.get("mcpServers").and_then(Value::as_array) else {
        return Ok(());
    };
    for server in servers {
        if server["name"].as_str() != Some(REGISTERED_TOOL_SERVER) {
            continue;
        }
        let command = server["command"]
            .as_str()
            .ok_or_else(|| declaration_defect(server, "command is not a string"))?;
        let args = server["args"]
            .as_array()
            .ok_or_else(|| declaration_defect(server, "args is not an array"))?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| declaration_defect(server, "an argument is not a string"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let env = match server.get("env") {
            None | Some(Value::Null) => Vec::new(),
            Some(Value::Object(values)) => values
                .iter()
                .map(|(key, value)| {
                    value
                        .as_str()
                        .map(|value| (key.clone(), value.to_owned()))
                        .ok_or_else(|| declaration_defect(server, "an env value is not a string"))
                })
                .collect::<Result<Vec<_>, _>>()?,
            Some(_) => return Err(declaration_defect(server, "env is not an object")),
        };
        let rendezvous = args.get(1).map(PathBuf::from);
        let child = SpawnedMcpChild::spawn(command, &args, &env)?;
        shared
            .spawned_mcp
            .lock()
            .expect("spawned mcp lock")
            .push(Arc::clone(&child));
        await_rendezvous_claim(&child, rendezvous.as_deref())?;
    }
    Ok(())
}

/// Waits for the courier's own startup event: the one-shot rendezvous is
/// claimed and unlinked before it connects, so its disappearance is a
/// positive observation that the child started and read its authority.
///
/// The wait ends early on child exit, and every ending that is not the claim
/// carries the child's bounded output and exit evidence.
fn await_rendezvous_claim(
    child: &SpawnedMcpChild,
    rendezvous: Option<&Path>,
) -> Result<(), String> {
    let Some(rendezvous) = rendezvous else {
        return Ok(());
    };
    let deadline = Instant::now() + STARTUP_BOUND;
    loop {
        if !rendezvous.exists() {
            return Ok(());
        }
        if let Some(exit) = child.exit_status() {
            // One last look: the claim and the exit can land together.
            if !rendezvous.exists() {
                return Ok(());
            }
            return Err(format!(
                "courier exited before claiming its rendezvous {}: {exit}; {}",
                rendezvous.display(),
                child.stderr.describe_drained()
            ));
        }
        if Instant::now() >= deadline {
            return Err(format!(
                "courier never claimed its rendezvous {} within {STARTUP_BOUND:?}: {}",
                rendezvous.display(),
                child.evidence_snapshot()
            ));
        }
        std::thread::sleep(STARTUP_STEP);
    }
}

pub(super) fn kill_spawned(shared: &Shared) {
    let children = shared
        .spawned_mcp
        .lock()
        .expect("spawned mcp lock")
        .drain(..)
        .collect::<Vec<_>>();
    for child in children {
        // Card 139: kill and reap before rendering evidence. Waiting for end
        // of pipe while the child still holds it open costs the whole drain
        // bound and still misses whatever the child writes on its way out.
        child.kill();
        child.reap();
        record_evidence(shared, &child.evidence());
    }
}

/// Retains one bounded evidence line for a later setup failure report.
pub(super) fn record_evidence(shared: &Shared, line: &str) {
    let mut notes = shared.courier_notes.lock().expect("courier notes lock");
    if notes.len() < 32 {
        let mut line = line.to_owned();
        if line.len() > NOTE_CAP {
            line.truncate(
                (0..=NOTE_CAP)
                    .rev()
                    .find(|index| line.is_char_boundary(*index))
                    .unwrap_or(0),
            );
            line.push_str("… (truncated)");
        }
        notes.push(line);
    }
}

/// Names a malformed declaration by its shape, never by its values: the
/// declared `env` carries host-approved recipe values that no failure message
/// needs to echo.
fn declaration_defect(server: &Value, reason: &str) -> String {
    let name = server["name"].as_str().unwrap_or("<unnamed>");
    let keys = server
        .as_object()
        .map(|object| object.keys().cloned().collect::<Vec<_>>().join(", "))
        .unwrap_or_else(|| "<not an object>".to_owned());
    format!("declared courier server {name:?} is malformed ({reason}); declared fields: {keys}")
}

fn describe_spawn_error(command: &str, args: &[String], error: &std::io::Error) -> String {
    let raw = error.raw_os_error().map_or_else(
        || "no os code".to_owned(),
        |code| format!("os error {code}"),
    );
    format!(
        "spawn of courier {command:?} args {args:?} failed: {error} ({raw}, kind {:?})",
        error.kind()
    )
}

fn mcp_closed() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.mcp_child.closed",
        "fixture MCP child closed",
    ))
}
