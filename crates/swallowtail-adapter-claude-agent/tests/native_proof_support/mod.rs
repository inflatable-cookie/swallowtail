//! Fake-backed native-proof harness for the `claude-agent.sdk` route.
//!
//! The shipped sidecar is launched by real Node code against three controlled
//! peers: a fake SDK module that speaks the native stdio wire, a fake native
//! process on the other end of that wire, and a loopback endpoint that records
//! any attempt to leave the machine. Nothing here installs a package, reads a
//! credential, or reaches a provider.
//!
//! The harness exists so that the *observation points* an authorised
//! original-artifact run needs are exercised and bounded first: exact identity
//! admission, the native argv, the initialize handshake, streamed input,
//! assistant/result/usage frames, the permission round trip, pre-callback
//! denial, interrupt, native exit, joined cleanup and egress silence.
//!
//! A run in `original` mode is refused unless the committed plan enables it and
//! the caller presents the authority token the plan names. The committed plan
//! has that mode disabled, so the refusal is the only reachable outcome today.

use crate::sidecar_asset_support::{NativeProofLaunch, SidecarProcess};
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub const PLAN: &str = include_str!("../fixtures/claude-agent-sdk-native-proof/probe-plan.json");
pub const PLAN_SCHEMA: &str =
    include_str!("../fixtures/claude-agent-sdk-native-proof/probe-plan.schema.json");
pub const SELECTED_FLOW: &str =
    include_str!("../fixtures/claude-agent-sdk-native-proof/selected-flow.json");

const SDK_FIXTURE_IDENTITY: &str =
    include_str!("../fixtures/claude-agent-sdk-0.3.322/identity.json");

/// How long a fixture may wait for a peer before the harness fails closed.
pub const PROOF_BOUND: Duration = Duration::from_secs(120);

/// One exact SDK/native tuple from the plan, with both platform artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hop {
    pub sdk_version: String,
    pub native_version: String,
    pub wrapper_tarball_sha256: String,
    pub platforms: Vec<PlatformArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformArtifact {
    pub platform: String,
    pub binary_sha256: String,
    pub binary_bytes: u64,
}

pub fn plan() -> Value {
    serde_json::from_str(PLAN).expect("the committed probe plan is valid JSON")
}

pub fn hops() -> Vec<Hop> {
    let plan = plan();
    let mut hops = Vec::new();
    for artifact in plan["artifacts"]
        .as_array()
        .expect("plan artifacts are an array")
    {
        let mut platforms = Vec::new();
        for (platform, row) in artifact["platforms"]
            .as_object()
            .expect("plan platforms are an object")
        {
            platforms.push(PlatformArtifact {
                platform: platform.clone(),
                binary_sha256: row["binary_sha256"]
                    .as_str()
                    .expect("platform digest is a string")
                    .to_owned(),
                binary_bytes: row["binary_bytes"]
                    .as_u64()
                    .expect("platform size is an integer"),
            });
        }
        hops.push(Hop {
            sdk_version: artifact["sdk_version"]
                .as_str()
                .expect("hop sdk version")
                .to_owned(),
            native_version: artifact["native_version"]
                .as_str()
                .expect("hop native version")
                .to_owned(),
            wrapper_tarball_sha256: artifact["wrapper_tarball_sha256"]
                .as_str()
                .expect("hop tarball digest")
                .to_owned(),
            platforms,
        });
    }
    hops
}

fn fixture_path(relative: &str) -> PathBuf {
    let crate_directory = Path::new(env!("CARGO_MANIFEST_DIR"));
    // Plan paths are repository-relative so a reviewer can read them; the test
    // binary resolves them from the crate or from the workspace root.
    match relative.strip_prefix("crates/") {
        Some(_) => crate_directory
            .parent()
            .and_then(Path::parent)
            .expect("the crate lives inside the workspace")
            .join(relative),
        None => crate_directory.join(relative),
    }
}

/// Cross-checks every digest in the plan against the committed identity
/// fixtures. A typo, a re-pack or a drifted ledger fails here rather than
/// silently steering an original run at the wrong bytes.
pub fn verify_plan_against_fixtures() -> Result<(), String> {
    let plan = plan();
    let identity: Value = serde_json::from_str(SDK_FIXTURE_IDENTITY)
        .map_err(|error| format!("SDK identity fixture is not JSON: {error}"))?;
    let ledger = identity["hops"]
        .as_array()
        .ok_or("SDK identity fixture has no hop ledger")?;
    if plan["schema"] != "claude-agent-sdk-native-proof-plan-v1" {
        return Err("plan schema tag is not the expected one".to_owned());
    }
    if plan["authorized_stage"]["original_artifact_execution"]["enabled"] != json!(false) {
        return Err("the committed plan must leave original execution disabled".to_owned());
    }
    let expected_hops = ["0.3.318", "0.3.319", "0.3.321", "0.3.322"];
    let planned: Vec<String> = hops().into_iter().map(|hop| hop.sdk_version).collect();
    for version in expected_hops {
        if !planned.iter().any(|value| value == version) {
            return Err(format!("plan is missing published hop {version}"));
        }
    }
    for hop in hops() {
        let row = ledger
            .iter()
            .find(|row| row["sdk_version"] == json!(hop.sdk_version))
            .ok_or_else(|| format!("hop {} is not in the SDK ledger", hop.sdk_version))?;
        if row["native_version"] != json!(hop.native_version) {
            return Err(format!(
                "hop {} native version diverges from the ledger",
                hop.sdk_version
            ));
        }
        if row["tarball_sha256"] != json!(hop.wrapper_tarball_sha256) {
            return Err(format!(
                "hop {} tarball digest diverges from the ledger",
                hop.sdk_version
            ));
        }
        for platform in &hop.platforms {
            let manifest_row = &row["native_manifest"]["platforms"][&platform.platform];
            if manifest_row["sha256"] != json!(platform.binary_sha256) {
                return Err(format!(
                    "hop {} {} binary digest diverges from the manifest",
                    hop.sdk_version, platform.platform
                ));
            }
            if manifest_row["bytes"] != json!(platform.binary_bytes) {
                return Err(format!(
                    "hop {} {} binary size diverges from the manifest",
                    hop.sdk_version, platform.platform
                ));
            }
        }
    }
    // Where the plan names a CLI cross-binding, the digest must appear in that
    // fixture. This keeps the coupling visible without re-deriving the CLI row.
    for artifact in plan["artifacts"].as_array().expect("plan artifacts") {
        let Some(relative) = artifact["cli_cross_binding"].as_str() else {
            continue;
        };
        let text = std::fs::read_to_string(fixture_path(relative))
            .map_err(|error| format!("CLI cross-binding {relative} is unreadable: {error}"))?;
        for platform in ["darwin-arm64", "linux-x64"] {
            let digest = artifact["platforms"][platform]["binary_sha256"]
                .as_str()
                .expect("platform digest");
            if !text.contains(digest) {
                return Err(format!(
                    "hop {} {platform} digest is not bound in {relative}",
                    artifact["sdk_version"]
                ));
            }
        }
    }
    Ok(())
}

/// The two harness modes. `Original` executes vendor artifacts and is disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Fake,
    Original,
}

impl Mode {
    /// Parses the requested mode. Whether `original` may run is decided by
    /// `resolve_mode` against the plan and the presented authority.
    pub fn from_request(requested: Option<&str>) -> Result<Self, ModeRefusal> {
        match requested {
            None | Some("") | Some("fake") => Ok(Mode::Fake),
            Some("original") => Ok(Mode::Original),
            Some(other) => Err(ModeRefusal::UnknownMode(other.to_owned())),
        }
    }
}

/// Resolves a mode the way the harness does, with the plan and token supplied
/// explicitly so the decision is testable without mutating process state.
pub fn resolve_mode(
    plan: &Value,
    requested: Option<&str>,
    authority: Option<&str>,
) -> Result<Mode, ModeRefusal> {
    let mode = Mode::from_request(requested)?;
    if mode == Mode::Fake {
        return Ok(mode);
    }
    let stage = &plan["authorized_stage"]["original_artifact_execution"];
    if stage["enabled"] != json!(true) {
        return Err(ModeRefusal::Disabled);
    }
    let expected = stage["required_authority"].as_str().unwrap_or_default();
    match authority {
        Some(token) if token == expected => Ok(Mode::Original),
        _ => Err(ModeRefusal::AuthorityMissing),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeRefusal {
    Disabled,
    AuthorityMissing,
    UnknownMode(String),
}

impl std::fmt::Display for ModeRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeRefusal::Disabled => write!(
                formatter,
                "original-artifact execution is disabled: the committed plan needs an \
                 operator authority statement and a brief amendment naming the exact digests"
            ),
            ModeRefusal::AuthorityMissing => write!(
                formatter,
                "original-artifact execution needs the authority token the plan names"
            ),
            ModeRefusal::UnknownMode(mode) => {
                write!(formatter, "unknown native proof mode {mode:?}")
            }
        }
    }
}

/// A loopback endpoint that records every connection and answers locally.
pub struct FakeEndpoint {
    port: u16,
    contacts: Arc<AtomicUsize>,
    requests: Arc<Mutex<Vec<String>>>,
    stop: Arc<AtomicBool>,
}

impl FakeEndpoint {
    pub fn start() -> Self {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("loopback endpoint binds");
        listener
            .set_nonblocking(true)
            .expect("loopback endpoint is non-blocking");
        let port = listener.local_addr().expect("endpoint address").port();
        let contacts = Arc::new(AtomicUsize::new(0));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_contacts = Arc::clone(&contacts);
        let thread_requests = Arc::clone(&requests);
        let thread_stop = Arc::clone(&stop);
        std::thread::spawn(move || {
            while !thread_stop.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        thread_contacts.fetch_add(1, Ordering::Relaxed);
                        record_request(stream, &thread_requests);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(_) => break,
                }
            }
        });
        Self {
            port,
            contacts,
            requests,
            stop,
        }
    }

    pub fn address(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }

    pub fn contacts(&self) -> usize {
        self.contacts.load(Ordering::Relaxed)
    }

    pub fn requests(&self) -> Vec<String> {
        self.requests
            .lock()
            .expect("endpoint request log lock")
            .clone()
    }
}

fn record_request(mut stream: TcpStream, log: &Mutex<Vec<String>>) {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
    let mut text = String::new();
    let mut buffer = [0u8; 2048];
    let deadline = Instant::now() + Duration::from_secs(2);
    while !text.contains("\r\n\r\n") && Instant::now() < deadline {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => text.push_str(&String::from_utf8_lossy(&buffer[..read])),
            Err(_) => break,
        }
    }
    log.lock()
        .expect("endpoint request log lock")
        .push(text.lines().next().unwrap_or_default().to_owned());
    let body = "{\"error\":\"fixture endpoint\"}";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
}

impl Drop for FakeEndpoint {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

pub struct FixtureOptions<'a> {
    pub scenario: &'a str,
    pub sdk_version: &'a str,
    pub native_version: &'a str,
    /// Overrides the packaged SDK identity without touching the peer's code.
    pub declared_sdk_version: Option<&'a str>,
    pub declared_native_version: Option<&'a str>,
}

impl<'a> FixtureOptions<'a> {
    pub fn new(scenario: &'a str, sdk_version: &'a str, native_version: &'a str) -> Self {
        Self {
            scenario,
            sdk_version,
            native_version,
            declared_sdk_version: None,
            declared_native_version: None,
        }
    }
}

/// One isolated fake-backed proof fixture.
pub struct NativeProofFixture {
    sidecar: SidecarProcess,
    endpoint: FakeEndpoint,
    root: PathBuf,
}

impl NativeProofFixture {
    pub fn start(options: &FixtureOptions<'_>) -> Self {
        let root = std::env::temp_dir().join(format!(
            "swallowtail-native-proof-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time follows epoch")
                .as_nanos()
        ));
        for directory in ["", "home", "workspace", "settings", "tmp", "peer"] {
            std::fs::create_dir_all(root.join(directory)).expect("fixture directory is created");
        }
        // Controlled settings live beside the fixture but are not selected by
        // the route's empty settingSources; the native sees no path to them.
        std::fs::write(
            root.join("settings/settings.json"),
            json!({"permissions": {"deny": ["Bash(rm:*)"]}}).to_string(),
        )
        .expect("controlled settings are written");

        let sdk_module = root.join("peer/fake-sdk-native-wire.mjs");
        std::fs::write(&sdk_module, include_str!("fake-sdk-native-wire.mjs"))
            .expect("fake SDK module is written");
        std::fs::write(
            root.join("peer/package.json"),
            json!({
                "name": "@anthropic-ai/claude-agent-sdk",
                "version": options
                    .declared_sdk_version
                    .unwrap_or(options.sdk_version),
            })
            .to_string(),
        )
        .expect("fake SDK package identity is written");
        let native = root.join("peer/fake-native.mjs");
        std::fs::write(&native, include_str!("fake-native.mjs")).expect("fake native is written");
        let manifest = root.join("peer/manifest.json");
        std::fs::write(
            &manifest,
            json!({
                "version": options
                    .declared_native_version
                    .unwrap_or(options.native_version),
            })
            .to_string(),
        )
        .expect("fake native manifest is written");

        let endpoint = FakeEndpoint::start();
        // The sidecar projects a fixed child-environment allowlist, so the peer
        // reads its scenario, observation path and endpoint from this file.
        std::fs::write(
            root.join("peer/native-config.json"),
            json!({
                "scenario": options.scenario,
                "observations": root.join("native-observations.json").to_string_lossy(),
                "endpoint": endpoint.address(),
            })
            .to_string(),
        )
        .expect("peer configuration is written");
        let environment = vec![
            (
                "PATH".to_owned(),
                std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin".to_owned()),
            ),
            (
                "HOME".to_owned(),
                root.join("home").to_string_lossy().into_owned(),
            ),
            (
                "TMPDIR".to_owned(),
                root.join("tmp").to_string_lossy().into_owned(),
            ),
            ("LANG".to_owned(), "en_US.UTF-8".to_owned()),
            ("LC_ALL".to_owned(), "en_US.UTF-8".to_owned()),
            ("LC_CTYPE".to_owned(), "en_US.UTF-8".to_owned()),
            ("USER".to_owned(), "proof-fixture".to_owned()),
            ("SHELL".to_owned(), "/bin/sh".to_owned()),
            ("TERM".to_owned(), "xterm-256color".to_owned()),
            ("COLORTERM".to_owned(), "truecolor".to_owned()),
        ];
        let sidecar = SidecarProcess::start_native_proof(&NativeProofLaunch {
            directory: &root,
            sdk_module: &sdk_module,
            native_binary: &native,
            native_manifest: &manifest,
            scenario: options.scenario,
            native_lifetime_ms: "0",
            environment: &environment,
        });
        Self {
            sidecar,
            endpoint,
            root,
        }
    }

    pub fn sidecar(&mut self) -> &mut SidecarProcess {
        &mut self.sidecar
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn endpoint(&self) -> &FakeEndpoint {
        &self.endpoint
    }

    pub fn sdk_observations(&self) -> Value {
        read_json(&self.root.join("observations.json"), "SDK observations")
    }

    pub fn native_observations(&self) -> Value {
        read_json(
            &self.root.join("native-observations.json"),
            "native observations",
        )
    }

    /// Waits, bounded, until the native process recorded its exit.
    pub fn wait_for_native_exit(&self) -> Value {
        let deadline = Instant::now() + PROOF_BOUND;
        loop {
            let path = self.root.join("native-observations.json");
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(observations) = serde_json::from_str::<Value>(&text) {
                    if observations["exitCode"].is_number() {
                        return observations;
                    }
                }
            }
            assert!(
                Instant::now() < deadline,
                "the fake native did not record an exit within the proof bound"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Waits, bounded, until the SDK module observed the native exit.
    pub fn wait_for_sdk_native_exit(&self) -> Value {
        let deadline = Instant::now() + PROOF_BOUND;
        loop {
            let observations = self.sdk_observations();
            if observations["nativeExit"].is_object() {
                return observations;
            }
            assert!(
                Instant::now() < deadline,
                "the fake SDK did not observe the native exit within the proof bound"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn native_environment_keys(&self) -> Vec<String> {
        self.native_observations()["envKeys"]
            .as_array()
            .expect("native recorded its environment keys")
            .iter()
            .map(|value| value.as_str().expect("environment key").to_owned())
            .collect()
    }
}

fn read_json(path: &Path, what: &str) -> Value {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("{what} at {path:?} are unreadable: {error}"));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{what} are not JSON: {error}"))
}

/// Names in the plan's forbidden list that appear in a peer's environment.
pub fn forbidden_environment_hits(keys: &[String]) -> Vec<String> {
    let plan = plan();
    let forbidden: Vec<String> = plan["environment"]["forbidden_names"]
        .as_array()
        .expect("plan forbidden names")
        .iter()
        .map(|value| value.as_str().expect("forbidden name").to_owned())
        .collect();
    keys.iter()
        .filter(|key| forbidden.iter().any(|name| name == *key))
        .cloned()
        .collect()
}

/// The value that follows `flag` in an argv list, when the flag is present.
pub fn arg_value<'a>(argv: &'a [Value], flag: &str) -> Option<&'a Value> {
    let index = argv.iter().position(|value| value == &json!(flag))?;
    argv.get(index + 1)
}

/// `--flag=value` entries, which carry their value inline.
pub fn arg_with_value<'a>(argv: &'a [Value], flag: &str) -> Option<&'a Value> {
    argv.iter().find(|value| {
        value
            .as_str()
            .is_some_and(|text| text == flag || text.starts_with(&format!("{flag}=")))
    })
}

pub fn argv_strings(observations: &Value) -> Vec<Value> {
    observations["argv"]
        .as_array()
        .expect("peer recorded its argv")
        .clone()
}
