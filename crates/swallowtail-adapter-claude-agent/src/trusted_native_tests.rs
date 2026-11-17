//! Bounded disposable-runner qualification through the actual prepared route.
use crate::*;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use swallowtail_core::*;
use swallowtail_host_local::*;
use swallowtail_runtime::*;

#[allow(dead_code)]
#[path = "../tests/acp_guard_scope.rs"]
mod scope;

thread_local! { static ADMITTED: RefCell<Option<String>> = const { RefCell::new(None) }; }

pub(crate) fn admitted(version: &str) -> bool {
    ADMITTED.with(|value| value.borrow().as_deref() == Some(version))
}

struct Admission;
impl Drop for Admission {
    fn drop(&mut self) {
        ADMITTED.with(|value| *value.borrow_mut() = None);
    }
}

fn hash(file: &Path) -> [u8; 32] {
    Sha256::digest(std::fs::read(file).expect("qualification source")).into()
}

fn runtime(file: &Path) -> LocalTrustedRuntime {
    use std::os::unix::fs::MetadataExt;
    let file = file.canonicalize().expect("runtime path");
    let users = vec![0, nix::unistd::geteuid().as_raw()];
    let groups = file
        .ancestors()
        .filter_map(|p| p.metadata().ok().map(|m| m.gid()))
        .collect();
    LocalTrustedRuntime::capture_with_trust(
        &file,
        hash(&file),
        &[],
        LocalRuntimeTrust::new(users, groups).expect("trusted disposable OS"),
    )
    .expect("exact selected runtime")
}

fn observations(root: &Path) -> Vec<Value> {
    std::fs::read_to_string(root.join("model-observations.json"))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn native_observations(root: &Path) -> Vec<Value> {
    std::fs::read_to_string(root.join("native-waits.jsonl"))
        .unwrap_or_default()
        .lines()
        .map(|s| serde_json::from_str(s).expect("native observation"))
        .collect()
}

fn observed_loader(root: &Path) -> String {
    let output = serde_json::to_string(&root.join("native-waits.jsonl")).expect("trace path");
    let source = CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE;
    let needle = "return trackChild(original.spawn(check(command,args,options),args,options));";
    assert_eq!(source.matches(needle).count(), 1);
    source.replace(needle, &format!(r#"const child=trackChild(original.spawn(check(command,args,options),args,options));
const record=value=>{{ const fs=originalFs; fs.appendFileSync({output},JSON.stringify(value)+'\\n'); }};
record({{phase:'start',pid:child.pid,parent:process.pid,command,args,cwd:options.cwd}});
child.once('exit',(code,signal)=>record({{phase:'wait',pid:child.pid,code,signal}}));
return child;"#))
        .replace("import { readFileSync, realpathSync, lstatSync } from 'node:fs';", "import * as originalFs from 'node:fs';\nimport { readFileSync, realpathSync, lstatSync } from 'node:fs';")
}

fn recipe(config: &Value, root: &Path, node: &LocalTrustedRuntime) -> LocalCapturedNodeRecipe {
    let native_path = PathBuf::from(config["native"].as_str().expect("native path"))
        .canonicalize()
        .expect("native absolute path");
    let native = runtime(&native_path);
    let platform = config["platform"].as_str().expect("platform");
    let graph = if config["original"] == true {
        assert_eq!(std::env::var("GITHUB_ACTIONS").as_deref(), Ok("true"));
        let version = config["hop"].as_str().expect("frozen hop");
        // Embedded copies keep the compiled runner independent of checkout paths.
        let manifest = match version {
            "0.101.0" => {
                include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.101.0.json").as_slice()
            }
            "0.101.1" => {
                include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.101.1.json").as_slice()
            }
            "0.102.0" => {
                include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.102.0.json").as_slice()
            }
            "0.103.0" => {
                include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.103.0.json").as_slice()
            }
            "0.104.0" => {
                include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.104.0.json").as_slice()
            }
            "0.104.1" => {
                include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.104.1.json").as_slice()
            }
            "0.105.0" => {
                include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.105.0.json").as_slice()
            }
            "0.106.0" => {
                include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.106.0.json").as_slice()
            }
            _ => panic!("unlisted original version"),
        };
        let ps = runtime(Path::new("/bin/ps"));
        let graph = LocalCapturedGraph::capture_manifest(
            Path::new(config["sourceRoot"].as_str().expect("source root")),
            manifest,
            platform,
            &native,
            Some(&ps),
        )
        .expect("frozen original graph and native hash");
        ADMITTED.with(|value| *value.borrow_mut() = Some(version.to_owned()));
        graph
    } else {
        let entry =
            include_str!("../tests/guard_full_route/entry.mjs").replace("0.106.0", "0.100.0");
        let sdk = include_str!("../tests/guard_full_route/sdk.mjs")
            .replace(
                "__NATIVE_PATH__",
                &serde_json::to_string(&native_path).expect("Node path"),
            )
            .replace(
                "__NATIVE_PROGRAM__",
                &serde_json::to_string(include_str!("../tests/guard_full_route/native.cjs"))
                    .expect("owned native"),
            );
        let idle = "export class IdleCompaction {constructor(){this.armed=0;this.fired=0;} onIdle(){this.armed++;} fire(){this.fired++;}}";
        let files = [
            ("entry.mjs", entry.as_str()),
            ("sdk.mjs", sdk.as_str()),
            ("idle.mjs", idle),
        ];
        let mut inventory = serde_json::Map::new();
        for (name, data) in files {
            std::fs::write(root.join(name), data).expect("owned source");
            inventory.insert(
                name.into(),
                json!({"format":"module","sha256":format!("{:x}",Sha256::digest(data.as_bytes()))}),
            );
        }
        let image = json!({"sha256":format!("{:x}",Sha256::digest(std::fs::read(&native_path).expect("owned runtime"))),"bytes":std::fs::metadata(&native_path).expect("runtime size").len(),"specifier":"@owned/native"});
        let manifest = json!({"vendor":"11".repeat(32),"entry":"entry.mjs","idle":"idle.mjs","files":inventory,"imports":[{"parent":"entry.mjs","specifier":"./sdk.mjs","target":"sdk.mjs"},{"parent":"entry.mjs","specifier":"./idle.mjs","target":"idle.mjs"}],"builtins":[{"parent":"entry.mjs","specifier":"node:fs","target":"node:fs"},{"parent":"sdk.mjs","specifier":"node:fs","target":"node:fs"},{"parent":"sdk.mjs","specifier":"node:child_process","target":"node:child_process"}],"require_parents":[],"native_parents":["sdk.mjs"],"native":{platform:image},"platform_programs":[],"source_cautions":[],"denied_dynamic":[]});
        LocalCapturedGraph::capture_manifest(
            root,
            &serde_json::to_vec(&manifest).expect("owned manifest"),
            platform,
            &native,
            None,
        )
        .expect("owned graph")
    };
    LocalCapturedNodeRecipe::for_trusted_runtime(
        &graph,
        &observed_loader(root),
        [1; 32],
        node,
        [8; 32],
    )
    .expect("actual Rust captured recipe")
}

fn prepare(
    config: &Value,
    root: &Path,
) -> (
    ClaudeAgentPreparedSession,
    LocalHostServices,
    HostServices,
    scope::FixtureScope,
) {
    let node_path = Path::new(config["node"].as_str().expect("Node"));
    let node = runtime(node_path);
    let recipe = recipe(config, root, &node);
    let fingerprint = *recipe.fingerprint().digest();
    let executable = ExecutableRef::new("acp239.executable").expect("executable");
    let environment = EnvironmentRef::new("acp239.environment").expect("environment");
    let resource = WorkingResourceRef::new("acp239.workspace").expect("resource");
    let checked = CheckedProcessRef::new("acp239.checked").expect("checked");
    let mut env: Vec<(std::ffi::OsString, std::ffi::OsString)> = config["environment"]
        .as_object()
        .expect("isolated environment")
        .iter()
        .map(|(k, v)| (k.into(), v.as_str().expect("env value").into()))
        .collect();
    env.push(("SWALLOWTAIL_FAKE_ROOT".into(), root.into()));
    if let Some(port) = config["fixturePort"].as_u64() {
        env.push((
            "SWALLOWTAIL_FAKE_ENDPOINT_PORT".into(),
            port.to_string().into(),
        ));
    }
    let host = ExecutionHostId::new("acp239.disposable").expect("host");
    let local = LocalProcessHost::builder(LocalProcessLimits::new(
        256,
        1024 * 1024,
        8 * 1024 * 1024,
        512 * 1024,
        512 * 1024,
    ))
    .approve_executable_launch(
        executable.clone(),
        LocalExecutableLaunch::new(node_path)
            .with_prefix_arguments(recipe.prefix_arguments().map(std::ffi::OsStr::to_os_string))
            .with_node_startup_environment_filter(),
    )
    .approve_environment(environment.clone(), env)
    .approve_working_resource(resource.clone(), root.join("workspace"))
    .approve_checked_node(
        checked.clone(),
        LocalCheckedNodeLaunch::new(
            executable.clone(),
            node,
            recipe,
            fingerprint,
            vec![vec![], vec!["--version".into()]],
        )
        .expect("hash-bound test launch"),
    )
    .expect("checked registration")
    .build_services(host.clone());
    let (scope, process) = scope::FixtureScope::new(local.process_host().as_ref().clone());
    scope.renew_behaviour_budget();
    let services = local.services().clone().with_process(process);
    let access = AccessProfileId::new("acp239.dummy-access").expect("access");
    let input = ClaudeAgentPreparationInput::new(
        ConfiguredInstanceId::new("acp239.instance").expect("instance"),
        InstanceRevision::new("1").expect("revision"),
        host,
        InstalledExecutableTarget::new(
            executable,
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("axis"),
        ),
        environment,
        AccessProfile::new(
            access.clone(),
            CredentialMechanism::LocalUnauthenticated,
            EntitlementMetering::SubscriptionAllowance,
            EndpointAudience::new("api.anthropic.com").expect("audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access,
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        )),
    )
    .with_checked_process(checked);
    let prepared = block_on(prepare_claude_agent(
        input,
        ClaudeAgentPreparationProbe::new(
            RequestId::new("trusted.discovery").expect("request"),
            ScopeId::new("trusted.discovery").expect("scope"),
            local.deadline_after(Duration::from_secs(45)),
            DiscoveryCancellation::new(),
        ),
        services.clone(),
    ))
    .expect("original prepared discovery");
    let profile = prepared
        .prepare_session(
            ClaudeAgentSessionProfileInput::new(
                RequestId::new("trusted.open").expect("open"),
                ClaudeAgentModelSelection::new(
                    ModelRouteId::new("owned.model").expect("route"),
                    ModelRouteRevision::new("1").expect("revision"),
                    ModelId::new("claude-sonnet-4-6").expect("model"),
                ),
                resource,
                SessionOptions::default(),
            )
            .with_consumer_mediated_permissions(),
        )
        .expect("real prepared profile");
    // Retain the real local task/time/resource composition; only checked process
    // ownership is wrapped by the existing joined fixture scope.
    (profile, local, services, scope)
}

fn exercise(config: &Value) {
    let _admission = Admission;
    let root = Path::new(config["caseRoot"].as_str().expect("owned case"));
    let mut binding = None;
    let scenario = config["scenario"].as_str().expect("scenario");
    let unsupported = matches!(
        scenario,
        "inherited-user-unsupported" | "inherited-managed-unsupported"
    );
    if unsupported {
        assert_eq!(config["hop"], "0.101.0");
        assert_eq!(config["original"], true);
    }
    let permits = scenario == "allow" || unsupported;
    let read_case = permits || matches!(scenario, "user-deny" | "managed-deny");
    let contexts = if read_case {
        vec!["new", "load", "resume"]
    } else {
        vec!["new"]
    };
    for context in contexts {
        let (profile, local, services, mut scope) = prepare(config, root);
        scope.renew_behaviour_budget();
        let mut session = match context {
            "new" => {
                block_on(profile.open_session(services.clone())).expect("original session/new")
            }
            "load" => {
                block_on(
                    profile
                        .load_session(
                            RequestId::new("trusted.load").expect("load"),
                            binding.clone().expect("prior binding"),
                            services.clone(),
                        )
                        .expect("load derives"),
                )
                .expect("original session/load")
                .into_parts()
                .1
            }
            _ => block_on(
                profile
                    .resume_session(
                        RequestId::new("trusted.resume").expect("resume"),
                        binding.clone().expect("prior binding"),
                        services.clone(),
                    )
                    .expect("resume derives"),
            )
            .expect("original session/resume"),
        };
        binding = Some(
            session
                .resume_binding()
                .expect("persisted session binding")
                .clone(),
        );
        let count = if read_case { 2 } else { 1 };
        for turn_index in 0..count {
            scope.renew_behaviour_budget();
            println!("ACP_TRUSTED_BEGIN {context}/{turn_index}/{scenario}");
            let observed_before = observations(root).len();
            let mut turn = block_on(
                session.start_turn(
                    TurnRequest::new(
                        RuntimeTurnId::new(format!("trusted.{context}.{turn_index}"))
                            .expect("turn"),
                        OperationContent::new(
                            "Read the owned fixture file requested by the model, then continue.",
                        )
                        .expect("prompt"),
                    )
                    .with_deadline(local.deadline_after(Duration::from_secs(
                        if scenario == "deadline" { 2 } else { 45 },
                    ))),
                    services.clone(),
                ),
            )
            .expect("original prompt");
            let mut callbacks = turn.take_callbacks().expect("real callback channel");
            let mut requests = callbacks
                .take_requests()
                .expect("real callback stream")
                .fuse();
            let mut events = turn.take_events().expect("events").fuse();
            let terminal = turn.take_terminal_outcome().expect("terminal");
            if scenario == "cancel" {
                let until = Instant::now() + Duration::from_secs(10);
                while !observations(root)
                    .iter()
                    .skip(observed_before)
                    .any(|r| r["selected"] == true)
                {
                    assert!(Instant::now() < until, "real model request");
                    std::thread::sleep(Duration::from_millis(10));
                }
                block_on(turn.cancellation().request()).expect("original cancel");
            }
            if scenario == "native-exit" {
                let until = Instant::now() + Duration::from_secs(10);
                while !observations(root)
                    .iter()
                    .skip(observed_before)
                    .any(|r| r["selected"] == true)
                {
                    assert!(Instant::now() < until, "real pending native request");
                    std::thread::sleep(Duration::from_millis(10));
                }
                let pid = native_observations(root)
                    .iter()
                    .rev()
                    .find(|r| r["phase"] == "start")
                    .expect("owned native start")["pid"]
                    .as_u64()
                    .expect("owned native PID");
                assert!(
                    Command::new("/bin/kill")
                        .args(["-TERM", &pid.to_string()])
                        .status()
                        .expect("stop only owned native child")
                        .success()
                );
            }
            let mut callback_count = 0;
            let mut kinds = vec![];
            block_on(async {
                loop {
                    futures_util::select! {
                        request=requests.next()=>if let Some(request)=request {
                            let request=request.expect("callback");callback_count+=1;
                            assert!(permits,"inherited denial must not fabricate a callback");
                            assert!(!observations(root).last().is_some_and(|r|r["continuing"]==true),"tool effect before permission");
                            let CallbackRequestKind::Extension(extension)=request.kind() else {panic!("expected actual ACP permission extension")};
                            let offered:Value=serde_json::from_slice(extension.payload()).expect("permission options");
                            let option=offered["options"].as_array().expect("one-shot options").iter().find(|o|o["kind"]=="allow_once").expect("actual allow once")["optionId"].as_str().expect("option id");
                            callbacks.responder().respond(CallbackResponse::new(request.callback_id().clone(),request.turn_id().expect("callback turn").clone(),CallbackResult::Success(CallbackPayload::new(serde_json::to_vec(&json!({"optionId":option})).expect("response JSON"),256).expect("permission response")))).await.expect("real permission reply");
                        },
                        event=events.next()=>match event {Some(event)=>kinds.push(format!("{:?}",event.expect("activity/result event").kind())),None=>break},
                    }
                }
            });
            let outcome = block_on(terminal);
            if read_case {
                assert_eq!(outcome.status(), &TerminalStatus::Completed, "{outcome:?}");
                assert_eq!(callback_count, usize::from(permits));
                assert!(
                    outcome
                        .output()
                        .expect("continued output")
                        .as_str()
                        .contains("continuing.")
                );
                let activity = kinds
                    .iter()
                    .position(|s| s.starts_with("Activity") && s.contains("ProviderOwnedTool"))
                    .expect("actual tool activity");
                let output = kinds
                    .iter()
                    .position(|s| s == "OutputDelta")
                    .expect("continued output event");
                assert!(activity < output, "activity must precede continued result");
                if config["original"] == true {
                    let observed = observations(root);
                    assert_eq!(
                        observed
                            .iter()
                            .skip(observed_before)
                            .rev()
                            .find(|r| r["selected"] == true && r["continuing"] == true)
                            .expect("this turn's actual tool result")["secretSeen"],
                        permits
                    );
                    assert!(
                        observed
                            .iter()
                            .filter(|r| r["selected"] == true)
                            .all(|r| r["model"] == "claude-sonnet-4-6"),
                        "model/options must persist through actual new/load/resume: {observed:?}"
                    );
                }
            } else if scenario == "cancel" {
                assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
            } else if scenario == "deadline" {
                assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
            } else {
                assert!(
                    matches!(
                        outcome.status(),
                        TerminalStatus::RuntimeFailed(_) | TerminalStatus::ProviderFailed(_)
                    ),
                    "actual native/provider failure must surface: {outcome:?}"
                );
            }
            println!(
                "ACP_TRUSTED_TURN {}",
                json!({"hop":config["hop"],"context":context,"scenario":scenario,"callbacks":callback_count,"status":format!("{:?}",outcome.status()),"events":kinds})
            );
            assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        }
        assert_eq!(
            block_on(session.close(
                SessionCleanupRequest::new(local.deadline_after(Duration::from_secs(5))),
                services
            )),
            CleanupOutcome::Clean
        );
        let cleanup = scope.finish();
        assert_eq!(cleanup["timedOut"], false);
        assert_eq!(cleanup["ownDescriptorEof"], true);
        local.shutdown_task_reapers().expect("real task join");
        println!("ACP_TRUSTED_CLEANUP {cleanup}");
    }
    {
        let rows = native_observations(root);
        for start in rows.iter().filter(|r| r["phase"] == "start") {
            assert!(
                rows.iter()
                    .any(|r| r["phase"] == "wait" && r["pid"] == start["pid"]),
                "native child wait missing: {start}"
            );
        }
        assert!(
            rows.iter().any(|r| r["phase"] == "start"),
            "no actual original native spawn observed"
        );
        println!("ACP_TRUSTED_NATIVE_WAITS {}", json!(rows));
    }
}

#[test]
#[ignore = "explicit trusted disposable runner only"]
fn original_case() {
    assert_eq!(std::env::var("GITHUB_ACTIONS").as_deref(), Ok("true"));
    let config: Value = serde_json::from_slice(
        &std::fs::read(std::env::var_os("SWALLOWTAIL_ACP_TRUSTED_CASE").expect("case config"))
            .expect("config bytes"),
    )
    .expect("case JSON");
    assert_eq!(config["original"], true);
    exercise(&config);
}

struct OwnedPeer(std::process::Child);
impl Drop for OwnedPeer {
    fn drop(&mut self) {
        drop(self.0.stdin.take());
        let until = Instant::now() + Duration::from_secs(5);
        while self.0.try_wait().expect("fixture wait").is_none() {
            if Instant::now() >= until {
                self.0.kill().expect("stop owned fixture");
                self.0.wait().expect("join owned fixture");
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

#[test]
#[ignore = "explicit owned harness self-test selector"]
fn owned_self_test() {
    let output = Command::new("/usr/bin/mktemp")
        .arg("-d")
        .arg(std::env::temp_dir().join("acp239-owned.XXXXXXXX"))
        .output()
        .expect("mktemp");
    assert!(output.status.success());
    let root = PathBuf::from(
        String::from_utf8(output.stdout)
            .expect("fixture path")
            .trim(),
    );
    let node = std::env::split_paths(&std::env::var_os("PATH").expect("PATH"))
        .map(|p| p.join("node"))
        .find(|p| p.is_file())
        .expect("installed Node")
        .canonicalize()
        .expect("Node path");
    let node = if std::env::var("GITHUB_ACTIONS").as_deref() == Ok("true") {
        // GitHub's shared toolcache may be writable. Use a job-owned copy;
        // never chmod or transact over the toolcache itself.
        let owned = root.join("owned-node");
        std::fs::copy(&node, &owned).expect("copy installed CI Node for owned fake");
        owned
    } else {
        node
    };
    std::fs::create_dir(root.join("workspace")).expect("workspace");
    std::fs::create_dir(root.join("home")).expect("home");
    std::fs::write(
        root.join("settings.json"),
        r#"{"user":{"permissions":{"denyInsteadOfPrompting":false}}}"#,
    )
    .expect("owned native settings");
    std::fs::write(root.join("tool.txt"), "owned tool").expect("owned tool");
    let script =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/claude-acp-trusted-model.py");
    let _peer = OwnedPeer(
        Command::new("python3")
            .arg("-I")
            .arg("-B")
            .arg(script)
            .arg(&root)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("owned loopback fixture"),
    );
    let until = Instant::now() + Duration::from_secs(5);
    while !root.join("fixture-port").is_file() {
        assert!(Instant::now() < until, "fixture startup");
        std::thread::sleep(Duration::from_millis(10));
    }
    let port: u16 = std::fs::read_to_string(root.join("fixture-port"))
        .expect("port")
        .parse()
        .expect("port number");
    let config = json!({"original":false,"hop":"0.100.0","node":node,"native":node,"caseRoot":root,"scenario":"allow","platform":if cfg!(target_os="macos"){"darwin-arm64"}else{"linux-x64"},"fixturePort":port,"environment":{"HOME":root.join("home").to_str().expect("home"),"PATH":"/usr/bin:/bin","ANTHROPIC_API_KEY":"owned-dummy-key","ANTHROPIC_BASE_URL":format!("http://127.0.0.1:{port}")}});
    exercise(&config);
    drop(_peer);
    std::fs::remove_dir_all(root).expect("remove only fresh owned fixture");
}
