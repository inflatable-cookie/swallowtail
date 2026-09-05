//! Drives the shipped sidecar asset under Node over its real private wire.
//!
//! Every wait here is bounded by the sidecar-death guard, and the child is
//! killed on drop, so a wedged sidecar fails the test instead of hanging the
//! suite.

use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::Duration;
use swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_SIDECAR_SOURCE;

const SIDECAR_DEATH_GUARD: Duration = Duration::from_secs(5 * 60);
static NEXT_TEMPORARY_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

/// How one sidecar-asset process is started.
struct Fixture {
    native_lifetime_ms: &'static str,
    scenario: &'static str,
}

impl Default for Fixture {
    fn default() -> Self {
        Self {
            native_lifetime_ms: "50",
            scenario: "read-only",
        }
    }
}

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
        Self::start_with(&Fixture::default())
    }

    /// Starts the asset against a named provider-free fake-SDK scenario.
    pub fn start_scenario(scenario: &'static str) -> Self {
        Self::start_with(&Fixture {
            scenario,
            ..Fixture::default()
        })
    }

    /// Starts the asset with a fake native child that outlives any bound the
    /// test declares.
    pub fn start_with_surviving_native_child() -> Self {
        Self::start_with(&Fixture {
            native_lifetime_ms: "30000",
            ..Fixture::default()
        })
    }

    /// Starts the asset against the multi-turn editing fake SDK, which writes
    /// real files under the leased cwd for every admitted write.
    pub fn start_editing() -> Self {
        Self::start_with(&Fixture {
            scenario: "editing",
            ..Fixture::default()
        })
    }

    /// Starts the asset against the fake SDK's two-turn Bash session.
    pub fn start_bash() -> Self {
        Self::start_with(&Fixture {
            scenario: "bash",
            ..Fixture::default()
        })
    }

    fn start_with(fixture: &Fixture) -> Self {
        let lifetime_ms = fixture.native_lifetime_ms;
        let scenario = fixture.scenario;
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
            .env("FAKE_SDK_SCENARIO", scenario)
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

    /// Reads one file under the leased cwd, or `None` when it was never
    /// written. A denied write must leave nothing behind.
    pub fn file_under_cwd(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.directory.join(name)).ok()
    }

    /// Reports whether the fake SDK was ever constructed. A rejection before
    /// launch leaves no observations at all.
    pub fn sdk_was_constructed(&self) -> bool {
        self.directory.join("observations.json").exists()
    }

    /// Every permission mode the fake SDK was asked to apply, in order.
    pub fn observed_permission_modes(&mut self) -> Vec<String> {
        // The caller has consumed the command response for each mode change;
        // the fake SDK publishes its observation before that response.
        let observations = self.read_observations();
        observations["permissionModes"]
            .as_array()
            .expect("fake SDK permission-mode observations are an array")
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .expect("fake SDK permission-mode observations are strings")
                    .to_owned()
            })
            .collect()
    }

    /// Per-turn write outcomes the fake SDK recorded before the turn-ended
    /// wire event.
    pub fn writes(&mut self, turns: usize) -> Vec<Value> {
        self.wait_for_turn_end();
        let observations = self.read_observations();
        let writes = observations["writes"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        assert!(
            writes.len() >= turns,
            "fake SDK did not record the expected write outcomes"
        );
        writes
    }

    /// Per-turn Bash outcomes the fake SDK recorded before the turn-ended
    /// wire event.
    pub fn bash_outcomes(&mut self, turns: usize) -> Vec<Value> {
        self.wait_for_turn_end();
        let observations = self.read_observations();
        let outcomes = observations["bash"].as_array().cloned().unwrap_or_default();
        assert!(
            outcomes.len() >= turns,
            "fake SDK did not record the expected Bash outcomes"
        );
        outcomes
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

    /// Waits for the live turn to end, holding any callback that arrives
    /// first. A new query before the turn ends is refused by the sidecar.
    pub fn wait_for_turn_end(&mut self) {
        loop {
            let record = self.next_record();
            match record["type"].as_str() {
                Some("callback") => self.hold_callback(record),
                Some("terminal") => panic!("sidecar terminated: {record}"),
                Some("event") if record["event"] == "turn_ended" => return,
                _ => {}
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

    /// The decisions the fake SDK observed for the named tools, after the
    /// turn-ended wire event that follows all of them.
    pub fn admissions(&mut self, tools: &[&str]) -> Value {
        self.wait_for_turn_end();
        let observations = self.read_observations();
        assert!(
            tools
                .iter()
                .all(|tool| observations["admissions"].get(tool).is_some()),
            "fake SDK did not record the expected tool admissions"
        );
        observations["admissions"].clone()
    }

    pub fn observed_options(&self) -> Value {
        let observations = self.read_observations();
        assert!(
            observations["options"].is_object(),
            "fake SDK did not record its options before the open response"
        );
        observations["options"].clone()
    }

    pub fn observed_control_calls(&self) -> Vec<String> {
        self.read_observations()["controlCalls"]
            .as_array()
            .expect("fake SDK control observations are an array")
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .expect("fake SDK control observation is a string")
                    .to_owned()
            })
            .collect()
    }

    pub fn first_input_consumed(&self) -> bool {
        self.read_observations()["firstInputConsumed"] == true
    }

    pub fn observed_spawn_hook_argument(&self) -> Value {
        let observations = self.read_observations();
        assert!(
            observations["spawnHookArgument"].is_object(),
            "fake SDK did not record the argument received by the spawn hook"
        );
        observations["spawnHookArgument"].clone()
    }

    pub fn observed_spawn_hook_argument_count(&self) -> usize {
        self.read_observations()["spawnHookArgumentCount"]
            .as_u64()
            .expect("fake SDK recorded the spawn hook argument count") as usize
    }

    fn read_observations(&self) -> Value {
        let path = self.directory.join("observations.json");
        let text = std::fs::read_to_string(path).expect("fake SDK observations are present");
        serde_json::from_str(&text).expect("fake SDK observations are valid JSON")
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
        match self.lines.recv_timeout(SIDECAR_DEATH_GUARD) {
            Ok(line) => serde_json::from_str(&line).unwrap_or_else(|error| {
                panic!("sidecar wrote a non-record line {line:?}: {error}")
            }),
            Err(RecvTimeoutError::Timeout) => {
                panic!("sidecar produced no record before the sidecar-death guard")
            }
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
    let sequence = NEXT_TEMPORARY_DIRECTORY.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "swallowtail-claude-agent-sdk-{}-{nanos}-{sequence}",
        std::process::id()
    ));
    std::fs::create_dir(&path).expect("fixture directory is created without collision");
    path
}
