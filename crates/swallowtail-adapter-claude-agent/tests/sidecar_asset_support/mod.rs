//! Drives the shipped sidecar asset under Node over its real private wire.
//!
//! Every wait here is bounded, and the child is killed on drop, so a wedged
//! sidecar fails the test instead of hanging the suite.

use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::Duration;
use swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_SIDECAR_SOURCE;

const WIRE_BOUND: Duration = Duration::from_secs(20);

pub struct SidecarProcess {
    child: Child,
    stdin: ChildStdin,
    lines: Receiver<String>,
    directory: PathBuf,
    pending: Vec<Value>,
    offered: Vec<String>,
}

impl SidecarProcess {
    /// Starts the asset with a fake native child that exits promptly.
    pub fn start() -> Self {
        Self::start_with_native_lifetime("50")
    }

    /// Starts the asset with a fake native child that outlives any bound the
    /// test declares.
    pub fn start_with_surviving_native_child() -> Self {
        Self::start_with_native_lifetime("30000")
    }

    fn start_with_native_lifetime(lifetime_ms: &str) -> Self {
        let directory = temporary_directory();
        let entry = directory.join("claude-agent-sdk-sidecar.mjs");
        std::fs::write(&entry, CLAUDE_AGENT_SDK_SIDECAR_SOURCE).expect("asset is written");
        std::fs::write(directory.join("fake-sdk.mjs"), include_str!("fake-sdk.mjs"))
            .expect("fake SDK is written");
        std::fs::write(
            directory.join("manifest.json"),
            json!({"version": "2.1.259"}).to_string(),
        )
        .expect("fake manifest is written");

        let node = std::env::var("SWALLOWTAIL_CLAUDE_AGENT_SDK_NODE")
            .unwrap_or_else(|_| "node".to_owned());
        let mut child = Command::new(&node)
            .arg(&entry)
            .env_clear()
            .env("PATH", std::env::var("PATH").unwrap_or_default())
            .env(
                "CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE",
                directory.join("fake-sdk.mjs"),
            )
            .env(
                "CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY",
                directory.join("claude"),
            )
            .env(
                "CLAUDE_AGENT_SDK_SIDECAR_MANIFEST",
                directory.join("manifest.json"),
            )
            .env("FAKE_SDK_OBSERVATIONS", directory.join("observations.json"))
            .env("FAKE_SDK_NATIVE_LIFETIME_MS", lifetime_ms)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap_or_else(|error| {
                panic!(
                    "the sidecar asset proof requires Node on PATH or \
                     SWALLOWTAIL_CLAUDE_AGENT_SDK_NODE: {error}"
                )
            });

        let stdin = child.stdin.take().expect("sidecar stdin");
        let stdout = child.stdout.take().expect("sidecar stdout");
        let (sender, lines) = channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                if sender.send(line).is_err() {
                    return;
                }
            }
        });
        Self {
            child,
            stdin,
            lines,
            directory,
            pending: Vec::new(),
            offered: Vec::new(),
        }
    }

    pub fn cwd(&self) -> String {
        self.directory.to_string_lossy().into_owned()
    }

    /// Sends one command and returns its correlated response, collecting any
    /// callbacks that arrive first.
    pub fn command(&mut self, id: &str, command: &str, params: Value) -> Value {
        self.write(json!({"type": "command", "id": id, "command": command, "params": params}));
        loop {
            let record = self.next_record();
            match record["type"].as_str() {
                Some("response") if record["id"] == id => return record,
                Some("callback") => self.hold_callback(record),
                Some("terminal") => panic!("sidecar terminated: {record}"),
                _ => {}
            }
        }
    }

    /// Returns the next `canUseTool` request, waiting for it if needed.
    pub fn next_callback(&mut self) -> Value {
        if !self.pending.is_empty() {
            return self.pending.remove(0);
        }
        loop {
            let record = self.next_record();
            if record["type"] == "callback" {
                self.hold_callback(record);
                return self.pending.remove(0);
            }
        }
    }

    pub fn respond_callback(&mut self, id: &str, decision: &str) {
        self.write(json!({"type": "callback_response", "id": id, "decision": decision}));
    }

    /// Every tool name the sidecar offered the host, in arrival order.
    pub fn callback_tool_names(&self) -> Vec<String> {
        self.offered.clone()
    }

    /// The decisions the fake SDK observed for `tools`, waited for until every
    /// named tool has been decided.
    pub fn admissions(&self, tools: &[&str]) -> Value {
        self.await_observations(|value| {
            tools
                .iter()
                .all(|tool| value["admissions"].get(tool).is_some())
        })["admissions"]
            .clone()
    }

    pub fn observed_options(&self) -> Value {
        self.await_observations(|value| value["options"].is_object())["options"].clone()
    }

    fn await_observations(&self, ready: impl Fn(&Value) -> bool) -> Value {
        let path = self.directory.join("observations.json");
        let deadline = std::time::Instant::now() + WIRE_BOUND;
        loop {
            if let Ok(text) = std::fs::read_to_string(&path)
                && let Ok(value) = serde_json::from_str::<Value>(&text)
                && ready(&value)
            {
                return value;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "fake SDK never recorded the expected observations"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn hold_callback(&mut self, record: Value) {
        if let Some(name) = record["toolName"].as_str() {
            self.offered.push(name.to_owned());
        }
        self.pending.push(record);
    }

    fn write(&mut self, record: Value) {
        let line = format!("{record}\n");
        self.stdin.write_all(line.as_bytes()).expect("wire write");
        self.stdin.flush().expect("wire flush");
    }

    fn next_record(&mut self) -> Value {
        match self.lines.recv_timeout(WIRE_BOUND) {
            Ok(line) => serde_json::from_str(&line).unwrap_or_else(|error| {
                panic!("sidecar wrote a non-record line {line:?}: {error}")
            }),
            Err(RecvTimeoutError::Timeout) => panic!("sidecar produced no record inside its bound"),
            Err(RecvTimeoutError::Disconnected) => panic!("sidecar closed its wire unexpectedly"),
        }
    }
}

impl Drop for SidecarProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

fn temporary_directory() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time follows epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "swallowtail-claude-agent-sdk-{}-{nanos}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path).expect("fixture directory is created");
    path
}
