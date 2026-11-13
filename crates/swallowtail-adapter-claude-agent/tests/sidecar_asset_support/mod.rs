//! Drives the shipped sidecar asset under Node over its real private wire.
//!
//! Every wait here is bounded by the sidecar-death guard, and the child is
//! killed on drop, so a wedged sidecar fails the test instead of hanging the
//! suite.

use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
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
    identity_field: Option<&'static str>,
    identity_value: Option<String>,
    model_evidence_value: Option<String>,
    sdk_version: Option<String>,
    native_version: Option<String>,
    native_manifest_mode: Option<&'static str>,
}

impl Default for Fixture {
    fn default() -> Self {
        Self {
            native_lifetime_ms: "50",
            scenario: "read-only",
            identity_field: None,
            identity_value: None,
            model_evidence_value: None,
            sdk_version: None,
            native_version: None,
            native_manifest_mode: None,
        }
    }
}

/// Everything the native-proof harness hands the shipped sidecar, including the
/// environment allowlist it must launch with. Only the native-proof test binary
/// constructs it, and every test binary compiles this module tree separately.
#[allow(dead_code)]
pub struct NativeProofLaunch<'a> {
    pub directory: &'a Path,
    pub sdk_module: &'a Path,
    pub native_binary: &'a Path,
    pub native_manifest: &'a Path,
    pub scenario: &'a str,
    pub native_lifetime_ms: &'a str,
    pub environment: &'a [(String, String)],
}

pub struct SidecarProcess {
    child: Child,
    stdin: ChildStdin,
    lines: Receiver<String>,
    directory: PathBuf,
    node_version: String,
    pending: Vec<Value>,
    held: Vec<Value>,
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
            native_lifetime_ms: if scenario.starts_with("usage-") {
                "0"
            } else {
                "50"
            },
            ..Fixture::default()
        })
    }

    /// Starts the sidecar with one shared model-evidence fixture value in the
    /// fake SDK's first-turn init message.
    pub fn start_model_evidence_case(scenario: &'static str, value: &str) -> Self {
        Self::start_with(&Fixture {
            scenario,
            model_evidence_value: Some(value.to_owned()),
            ..Fixture::default()
        })
    }

    /// Starts the sidecar with one shared identity-boundary fixture value in
    /// the fake SDK package manifest.
    pub fn start_sdk_identity_case(field: &'static str, value: &str) -> Self {
        Self::start_with(&Fixture {
            scenario: "sdk-identity-boundary",
            identity_field: Some(field),
            identity_value: Some(value.to_owned()),
            ..Fixture::default()
        })
    }

    /// Starts the shipped sidecar with one exact fake package/native pair.
    pub fn start_package_native_pair(sdk_version: &str, native_version: &str) -> Self {
        Self::start_with(&Fixture {
            sdk_version: Some(sdk_version.to_owned()),
            native_version: Some(native_version.to_owned()),
            ..Fixture::default()
        })
    }

    /// Starts the sidecar with an absent, malformed, or unreadable native manifest.
    pub fn start_native_manifest_case(mode: &'static str) -> Self {
        Self::start_with(&Fixture {
            native_manifest_mode: Some(mode),
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
        let sdk_module = write_sdk_fixture(
            &directory,
            scenario,
            fixture.identity_field,
            fixture.identity_value.as_deref(),
            fixture.sdk_version.as_deref().unwrap_or("0.3.293"),
        );
        let native_manifest = directory.join("manifest.json");
        match fixture.native_manifest_mode {
            Some("missing") => {}
            Some("malformed") => {
                std::fs::write(&native_manifest, b"{").expect("malformed fake manifest is written")
            }
            Some("unreadable") => {
                std::fs::create_dir(&native_manifest).expect("unreadable fake manifest is created")
            }
            Some(mode) => panic!("unknown native manifest fixture mode: {mode}"),
            None => std::fs::write(
                native_manifest.clone(),
                json!({"version": fixture.native_version.as_deref().unwrap_or("2.1.293")})
                    .to_string(),
            )
            .expect("fake manifest is written"),
        }

        // The historical fixture intentionally exposes provider-shaped names so
        // the sidecar's environment handling is observable. The native-proof
        // launcher supplies its own allowlist instead.
        let environment = vec![
            ("PATH".to_owned(), std::env::var("PATH").unwrap_or_default()),
            ("HOME".to_owned(), "/fixture/home".to_owned()),
            ("TMPDIR".to_owned(), "/fixture/tmp".to_owned()),
            ("LANG".to_owned(), "en_US.UTF-8".to_owned()),
            ("LC_ALL".to_owned(), "en_US.UTF-8".to_owned()),
            ("LC_CTYPE".to_owned(), "en_US.UTF-8".to_owned()),
            ("USER".to_owned(), "fixture-user".to_owned()),
            ("SHELL".to_owned(), "/bin/sh".to_owned()),
            ("TERM".to_owned(), "xterm-256color".to_owned()),
            ("COLORTERM".to_owned(), "truecolor".to_owned()),
            ("__CF_USER_TEXT_ENCODING".to_owned(), "0x0:0:0".to_owned()),
            ("XPC_FLAGS".to_owned(), "0x0".to_owned()),
            ("XPC_SERVICE_NAME".to_owned(), "fixture.service".to_owned()),
            ("MallocNanoZone".to_owned(), "0".to_owned()),
            ("COMMAND_MODE".to_owned(), "unix2003".to_owned()),
            ("ANTHROPIC_API_KEY".to_owned(), "fixture-secret".to_owned()),
            ("CLAUDE_CONFIG_DIR".to_owned(), "/fixture/claude".to_owned()),
            ("CLAUDE_CODE_USE_BEDROCK".to_owned(), "1".to_owned()),
            ("OPENAI_API_KEY".to_owned(), "fixture-secret".to_owned()),
            ("AWS_ACCESS_KEY_ID".to_owned(), "fixture-secret".to_owned()),
            (
                "AWS_SECRET_ACCESS_KEY".to_owned(),
                "fixture-secret".to_owned(),
            ),
            ("GOOGLE_API_KEY".to_owned(), "fixture-secret".to_owned()),
            ("RANDOM_UNRELATED".to_owned(), "fixture-value".to_owned()),
        ];
        let historical_native = directory.join("claude");
        Self::launch(
            directory,
            entry,
            sdk_module,
            historical_native,
            native_manifest,
            scenario,
            lifetime_ms,
            fixture.model_evidence_value.clone(),
            &environment,
        )
    }

    #[allow(dead_code)]
    /// Starts the shipped sidecar against an explicit fake-SDK module, fake
    /// native and environment. Used by the native-proof harness, which owns the
    /// fixture tree and the credential-free allowlist.
    pub fn start_native_proof(launch: &NativeProofLaunch) -> Self {
        let directory = launch.directory.to_path_buf();
        let entry = directory.join("claude-agent-sdk-sidecar.mjs");
        std::fs::write(&entry, CLAUDE_AGENT_SDK_SIDECAR_SOURCE).expect("asset is written");
        Self::launch(
            directory,
            entry,
            launch.sdk_module.to_path_buf(),
            launch.native_binary.to_path_buf(),
            launch.native_manifest.to_path_buf(),
            launch.scenario,
            launch.native_lifetime_ms,
            None,
            launch.environment,
        )
    }

    #[allow(dead_code)]
    /// The fixture directory this process was launched in.
    pub fn fixture_directory(&self) -> &Path {
        &self.directory
    }

    #[allow(clippy::too_many_arguments)]
    fn launch(
        directory: PathBuf,
        entry: PathBuf,
        sdk_module: PathBuf,
        native_binary: PathBuf,
        native_manifest: PathBuf,
        scenario: &str,
        lifetime_ms: &str,
        model_evidence_value: Option<String>,
        environment: &[(String, String)],
    ) -> Self {
        let node = std::env::var("SWALLOWTAIL_CLAUDE_AGENT_SDK_NODE")
            .unwrap_or_else(|_| "node".to_owned());
        let version = Command::new(&node)
            .arg("--version")
            .output()
            .unwrap_or_else(|error| {
                panic!("selected Node executable {node:?} cannot run: {error}")
            });
        assert!(
            version.status.success(),
            "selected Node executable {node:?} failed --version: {}",
            String::from_utf8_lossy(&version.stderr)
        );
        let node_version = String::from_utf8(version.stdout)
            .expect("selected Node version is UTF-8")
            .trim()
            .strip_prefix('v')
            .expect("Node --version uses the v-prefixed semantic version")
            .to_owned();
        let mut command = Command::new(&node);
        command.arg(&entry).env_clear();
        for (name, value) in environment {
            command.env(name, value);
        }
        let mut child = command
            .env("CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE", &sdk_module)
            .env("CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY", &native_binary)
            .env("CLAUDE_AGENT_SDK_SIDECAR_MANIFEST", &native_manifest)
            .env("FAKE_SDK_OBSERVATIONS", directory.join("observations.json"))
            .env("FAKE_SDK_NATIVE_LIFETIME_MS", lifetime_ms)
            .env("FAKE_SDK_SCENARIO", scenario)
            .envs(
                model_evidence_value
                    .iter()
                    .map(|value| ("FAKE_SDK_MODEL_EVIDENCE_VALUE", value)),
            )
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
            node_version,
            pending: Vec::new(),
            held: Vec::new(),
            offered: Vec::new(),
        }
    }

    pub fn cwd(&self) -> String {
        self.directory.to_string_lossy().into_owned()
    }

    /// Reports the version printed by the exact Node executable used to
    /// launch this sidecar process.
    pub fn node_version(&self) -> &str {
        &self.node_version
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

    /// Models the calls that reached the fake SDK's `setModel` method.
    pub fn observed_model_set_calls(&self) -> Vec<String> {
        self.read_observations()["modelSetCalls"]
            .as_array()
            .expect("fake SDK model-change observations are an array")
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .expect("fake SDK model-change observation is a string")
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
    pub fn command(&mut self, id: &str, command: &str, mut params: Value) -> Value {
        if matches!(command, "open" | "list_sessions") {
            let object = params
                .as_object_mut()
                .expect("sidecar command parameters are an object");
            object
                .entry("expectedSdkVersion")
                .or_insert_with(|| json!("0.3.293"));
            object
                .entry("expectedNativeVersion")
                .or_insert_with(|| json!("2.1.293"));
        }
        self.write(json!({"type": "command", "id": id, "command": command, "params": params}));
        loop {
            let record = self.next_received_record();
            match record["type"].as_str() {
                Some("response") if record["id"] == id => return record,
                Some("callback") => self.hold_callback(record),
                Some("terminal") => panic!("sidecar terminated: {record}"),
                _ => self.held.push(record),
            }
        }
    }

    /// Reads the next wire event, retaining the same held-record ordering as commands.
    pub fn next_event(&mut self) -> Value {
        let record = self.next_record();
        assert_eq!(record["type"], "event", "expected event: {record}");
        record
    }

    /// Reads the next bounded diagnostic record, retaining the sidecar's
    /// record ordering so callers can compare it with the following response.
    pub fn next_diagnostic(&mut self) -> Value {
        let record = self.next_record();
        assert_eq!(
            record["type"], "diagnostic",
            "expected diagnostic: {record}"
        );
        record
    }

    /// Sends a query and returns the terminal record emitted for an unmapped
    /// fake-SDK message. The raw message type is never part of that record.
    pub fn terminal_after_query(&mut self, id: &str, params: Value) -> Value {
        self.write(json!({
            "type": "command",
            "id": id,
            "command": "query",
            "params": params
        }));
        loop {
            let record = self.next_record();
            match record["type"].as_str() {
                Some("callback") => self.hold_callback(record),
                Some("terminal") => return record,
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
        let _ = self.wait_for_turn_end_record();
    }

    /// Returns the sanitized terminal event, holding any callback that arrives
    /// before it. The sidecar's projection intentionally excludes SDK error
    /// text while retaining the typed result evidence needed by the live log.
    pub fn wait_for_turn_end_record(&mut self) -> Value {
        loop {
            let record = self.next_record();
            match record["type"].as_str() {
                Some("callback") => self.hold_callback(record),
                Some("terminal") => panic!("sidecar terminated: {record}"),
                Some("event") if record["event"] == "turn_ended" => return record,
                _ => {}
            }
        }
    }

    pub fn observed_prompt_stream_state(&self) -> Option<String> {
        self.read_observations()["promptStreamState"]
            .as_str()
            .map(ToOwned::to_owned)
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

    pub fn observed_list_sessions(&self) -> Value {
        let observations = self.read_observations();
        assert!(
            observations["listSessions"].is_object(),
            "fake SDK did not record listSessions options"
        );
        observations["listSessions"].clone()
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

    pub fn observed_query_input_calls(&self) -> usize {
        self.read_observations()["queryInputCalls"]
            .as_u64()
            .expect("fake SDK recorded query input calls") as usize
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

    pub fn observed_close_calls(&self) -> usize {
        self.read_observations()["closeCalls"]
            .as_u64()
            .expect("fake SDK recorded close calls") as usize
    }

    pub fn observed_mcp_hits(&self) -> Vec<String> {
        self.read_observations()["mcpHits"]
            .as_array()
            .expect("fake SDK MCP hits are an array")
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .expect("fake SDK MCP hit is a string")
                    .to_owned()
            })
            .collect()
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

    #[allow(dead_code)]
    /// Writes one record without waiting for a reply. Used where the reply may
    /// never arrive because the peer already died.
    pub fn send_without_reply(&mut self, record: Value) {
        self.write(record);
    }

    #[allow(dead_code)]
    /// Reads the next record within `timeout`, reporting a closed wire, a
    /// timeout, or an unreadable line as `None`.
    pub fn next_record_within(&mut self, timeout: Duration) -> Option<Value> {
        if let Some(record) = self.held.first().cloned() {
            self.held.remove(0);
            return Some(record);
        }
        match self.lines.recv_timeout(timeout) {
            Ok(line) => Some(serde_json::from_str(&line).unwrap_or_else(|error| {
                panic!("sidecar wrote a non-record line {line:?}: {error}")
            })),
            Err(_) => None,
        }
    }

    #[allow(dead_code)]
    /// Reads the next record, reporting a closed or unreadable wire as `None`
    /// instead of panicking. Used where a peer's termination is the observation.
    pub fn next_record_or_closed(&mut self) -> Option<Value> {
        if let Some(record) = self.held.first().cloned() {
            self.held.remove(0);
            return Some(record);
        }
        match self.lines.recv_timeout(SIDECAR_DEATH_GUARD) {
            Ok(line) => Some(serde_json::from_str(&line).unwrap_or_else(|error| {
                panic!("sidecar wrote a non-record line {line:?}: {error}")
            })),
            Err(_) => None,
        }
    }

    fn next_record(&mut self) -> Value {
        if !self.held.is_empty() {
            return self.held.remove(0);
        }
        self.next_received_record()
    }

    fn next_received_record(&self) -> Value {
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
        // Nucleus permits a fixture to tear down temporary roots it created in
        // this run. Guard that: only a direct child of the platform temporary
        // directory carrying this fixture's own prefix is removed, so a
        // mis-pointed root can never be deleted.
        let temporary = std::env::temp_dir();
        let owned = self
            .directory
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("swallowtail-claude-agent-sdk-"))
            && self.directory.parent() == Some(temporary.as_path());
        if owned {
            let _ = std::fs::remove_dir_all(&self.directory);
        }
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

fn write_sdk_fixture(
    directory: &Path,
    scenario: &str,
    identity_field: Option<&str>,
    identity_value: Option<&str>,
    sdk_version: &str,
) -> PathBuf {
    let nested_identity = matches!(
        scenario,
        "sdk-identity-nested-match"
            | "sdk-identity-unrelated-ancestor"
            | "sdk-identity-malformed"
            | "sdk-identity-unreadable"
    );
    let ambiguous_identity = scenario == "sdk-identity-ambiguous";
    let sdk_module = if nested_identity {
        directory.join(
            "node_modules/fixture-wrapper/node_modules/@anthropic-ai/claude-agent-sdk/dist/fake-sdk.mjs",
        )
    } else if ambiguous_identity {
        directory.join("node_modules/@anthropic-ai/fake-sdk.mjs")
    } else {
        directory.join("fake-sdk.mjs")
    };
    std::fs::create_dir_all(sdk_module.parent().expect("SDK module has a parent"))
        .expect("SDK module directory is created");
    std::fs::write(&sdk_module, include_str!("fake-sdk.mjs")).expect("fake SDK is written");

    let package_manifest = directory.join("package.json");
    let correct_manifest = json!({
        "name": "@anthropic-ai/claude-agent-sdk",
        "version": sdk_version
    })
    .to_string();
    let mismatch_manifest = json!({
        "name": "fixture-consumer",
        "version": "9.9.9"
    })
    .to_string();
    match scenario {
        "sdk-identity-missing" => {}
        "sdk-identity-mismatch" => std::fs::write(
            package_manifest,
            json!({
                "name": "@anthropic-ai/claude-agent-sdk",
                "version": "0.3.258"
            })
            .to_string(),
        )
        .expect("mismatching SDK package identity is written"),
        "sdk-identity-nested-match" => {
            let root = sdk_module
                .parent()
                .expect("nested SDK module has a dist directory")
                .parent()
                .expect("nested SDK dist has a package root");
            std::fs::write(root.join("package.json"), correct_manifest)
                .expect("nested SDK package identity is written");
        }
        "sdk-identity-unrelated-ancestor" => {
            let wrapper = directory.join("node_modules/fixture-wrapper");
            std::fs::write(wrapper.join("package.json"), &mismatch_manifest)
                .expect("unrelated wrapper identity is written");
            std::fs::write(package_manifest, mismatch_manifest)
                .expect("unrelated consumer identity is written");
        }
        "sdk-identity-malformed" => {
            let root = sdk_module
                .parent()
                .expect("nested SDK module has a dist directory")
                .parent()
                .expect("nested SDK dist has a package root");
            std::fs::write(root.join("package.json"), b"{")
                .expect("malformed SDK package identity is written");
        }
        "sdk-identity-unreadable" => {
            let root = sdk_module
                .parent()
                .expect("nested SDK module has a dist directory")
                .parent()
                .expect("nested SDK dist has a package root");
            std::fs::create_dir(root.join("package.json"))
                .expect("unreadable SDK package identity is created");
        }
        "sdk-identity-ambiguous" => {
            std::fs::write(package_manifest, mismatch_manifest)
                .expect("unrelated consumer identity is written");
        }
        "sdk-identity-boundary" => {
            let manifest_field = match identity_field {
                Some("loadedSdkPackage") => "name",
                Some("loadedSdkVersion") => "version",
                _ => panic!("unknown identity boundary fixture field"),
            };
            let mut manifest = json!({
                "name": "@anthropic-ai/claude-agent-sdk",
                "version": sdk_version
            });
            manifest[manifest_field] = json!(identity_value.expect("identity boundary value"));
            std::fs::write(package_manifest, manifest.to_string())
                .expect("identity boundary manifest is written");
        }
        _ => std::fs::write(package_manifest, correct_manifest)
            .expect("fake SDK package identity is written"),
    }
    sdk_module
}
