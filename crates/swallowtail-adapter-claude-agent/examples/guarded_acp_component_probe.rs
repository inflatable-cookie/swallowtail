//! Bounded original idle-class witness, never whole-route/native qualification.
//! Original mode requires separate exact authority and the confined owner wrapper.

#[cfg(unix)]
mod probe {
    use futures_executor::block_on;
    use sha2::{Digest, Sha256};
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
        time::Duration,
    };
    use swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE;
    use swallowtail_host_local::{
        LocalCapturedGraph, LocalCapturedImport, LocalCapturedNodeRecipe, LocalCapturedSources,
        LocalCheckedNodeLaunch, LocalExecutableLaunch, LocalExpectedSource, LocalProcessHost,
        LocalProcessLimits, LocalRuntimeTrust, LocalTrustedRuntime,
    };
    use swallowtail_runtime::{
        CheckedProcessRef, ExecutableRef, ProcessOutputStream, ProcessRequest, ProcessService,
        ScopeId,
    };

    fn hash(bytes: &[u8]) -> [u8; 32] {
        Sha256::digest(bytes).into()
    }
    fn decode(value: &str) -> Result<[u8; 32], String> {
        if value.len() != 64
            || !value
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("invalid digest".into());
        }
        let mut out = [0; 32];
        for (byte, pair) in out.iter_mut().zip(value.as_bytes().as_chunks::<2>().0) {
            let n = |b: u8| if b <= b'9' { b - b'0' } else { b - b'a' + 10 };
            *byte = n(pair[0]) * 16 + n(pair[1]);
        }
        Ok(out)
    }
    fn read(path: &Path) -> Result<Vec<u8>, String> {
        std::fs::read(path).map_err(|_| "probe input unavailable".into())
    }
    fn path(input: &serde_json::Value, name: &str) -> Result<PathBuf, String> {
        let path = PathBuf::from(input[name].as_str().ok_or("missing path")?);
        if !path.is_absolute() || path.canonicalize().map_err(|_| "noncanonical input")? != path {
            return Err("noncanonical input".into());
        }
        Ok(path)
    }
    pub fn run() -> Result<(), String> {
        if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            return Err("component owner requires Darwin arm64".into());
        }
        let args: Vec<_> = std::env::args_os().collect();
        if args.len() != 2 {
            return Err("use the source-bound confined probe owner".into());
        }
        let input: serde_json::Value = serde_json::from_slice(&read(Path::new(&args[1]))?)
            .map_err(|_| "invalid probe input")?;
        let root = path(&input, "root")?;
        let node = path(&input, "node")?;
        // The owner supplies the declared OS policy; these are independent
        // kernel counterexamples, not a caller's "sandbox active" marker.
        match std::fs::File::open(root.join("outside-denied")) {
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {}
            _ => return Err("OS filesystem confinement not observed".into()),
        }
        #[cfg(target_os = "macos")]
        match std::net::TcpStream::connect_timeout(
            &"127.0.0.2:1".parse().expect("literal loopback"),
            Duration::from_millis(100),
        ) {
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {}
            _ => return Err("OS network confinement not observed".into()),
        }
        #[cfg(target_os = "linux")]
        {
            let routes = std::fs::read_to_string("/proc/net/route")
                .map_err(|_| "network namespace unavailable")?;
            let devices = std::fs::read_to_string("/proc/net/dev")
                .map_err(|_| "network namespace unavailable")?;
            if routes.lines().skip(1).any(|line| !line.trim().is_empty())
                || devices.lines().skip(2).any(|line| {
                    line.split(':')
                        .next()
                        .is_none_or(|name| name.trim() != "lo")
                })
            {
                return Err("network namespace has non-loopback routes or devices".into());
            }
        }
        // Readability distinguishes an execution veto from a missing file.
        std::fs::File::open("/bin/echo").map_err(|_| "exec canary unreadable")?;
        match std::process::Command::new("/bin/echo")
            .arg("DENIED_EXEC_CANARY")
            .output()
        {
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {}
            _ => return Err("OS execution confinement not observed".into()),
        }
        let mode = input["mode"].as_str().ok_or("missing mode")?;
        let control = input["control"].as_bool().ok_or("missing control")?;
        let idle = if mode == "fake" {
            include_bytes!("../tests/guard_component_probe/idle.mjs").to_vec()
        } else {
            read(&root.join("idle.mjs"))?
        };
        let node_digest = hash(&read(&node)?);
        if node_digest != decode(input["nodeSha256"].as_str().ok_or("missing Node binding")?)? {
            return Err("Node executable identity changed".into());
        }
        let vendor = if mode == "fake" {
            hash(b"repository-owned idle component peer")
        } else if mode == "original" {
            let hop = input["hop"].as_str().ok_or("missing exact hop")?;
            let manifest = match hop {
                "0.101.0" => include_str!("../sidecar/claude-agent-acp-guard-graphs/0.101.0.json"),
                "0.101.1" => include_str!("../sidecar/claude-agent-acp-guard-graphs/0.101.1.json"),
                "0.102.0" => include_str!("../sidecar/claude-agent-acp-guard-graphs/0.102.0.json"),
                "0.103.0" => include_str!("../sidecar/claude-agent-acp-guard-graphs/0.103.0.json"),
                "0.104.0" => include_str!("../sidecar/claude-agent-acp-guard-graphs/0.104.0.json"),
                "0.104.1" => include_str!("../sidecar/claude-agent-acp-guard-graphs/0.104.1.json"),
                "0.105.0" => include_str!("../sidecar/claude-agent-acp-guard-graphs/0.105.0.json"),
                "0.106.0" => include_str!("../sidecar/claude-agent-acp-guard-graphs/0.106.0.json"),
                _ => return Err("unadmitted exact hop".into()),
            };
            let manifest: serde_json::Value =
                serde_json::from_str(manifest).map_err(|_| "invalid packaged graph")?;
            let original = String::from_utf8(idle.clone()).map_err(|_| "non-UTF8 idle source")?;
            // Appending a patch cannot intercept top-level evaluation. This
            // exact audited prefix contains constants, a bounded intrinsic Set
            // allocation and function declarations. The pinned class has no
            // extends, static initializer/block or computed member.
            let (prefix, class) = original
                .split_once("export class IdleCompaction {")
                .ok_or("idle declaration mismatch")?;
            if hash(prefix.as_bytes())
                != decode("9178cba38b0cd5451ff1425b2cfa762882fb13662c30fd13f7530f6673fd653b")?
                || class
                    .lines()
                    .any(|line| line.starts_with("    static ") || line.starts_with("    ["))
                || !class.ends_with("\n}\n")
            {
                return Err("unaudited original top-level evaluation".into());
            }
            if hash(&idle)
                != decode(
                    manifest["files"]["acp/dist/idle-compaction.js"]["sha256"]
                        .as_str()
                        .ok_or("missing class identity")?,
                )?
            {
                return Err("original idle artifact mismatch".into());
            }
            decode(
                manifest["vendor"]
                    .as_str()
                    .ok_or("missing vendor identity")?,
            )?
        } else {
            return Err("unadmitted mode".into());
        };
        let entry = include_bytes!("../tests/guard_component_probe/entry.mjs");

        let sources = LocalCapturedSources::capture(
            &root,
            &[
                LocalExpectedSource::new("entry.mjs", hash(entry)),
                LocalExpectedSource::new("idle.mjs", hash(&idle)),
            ],
        )
        .map_err(|_| "captured component input refused")?;
        let graph = LocalCapturedGraph::new(
            &sources,
            Path::new("entry.mjs"),
            Path::new("idle.mjs"),
            &[LocalCapturedImport::new(
                "entry.mjs",
                "./idle.mjs",
                "idle.mjs",
            )],
        )
        .map_err(|_| "component graph refused")?;
        let mut dependencies = Vec::new();
        for row in input["dependencies"]
            .as_array()
            .ok_or("missing runtime dependency boundary")?
        {
            let file = PathBuf::from(row["path"].as_str().ok_or("invalid dependency path")?);
            dependencies.push((
                file,
                decode(row["sha256"].as_str().ok_or("missing dependency digest")?)?,
            ));
        }
        let tree_owner = &input["systemTreeOwner"];
        if tree_owner["path"] != "/bin/sleep" {
            return Err("tree owner identity absent".into());
        }
        dependencies.push((
            PathBuf::from("/bin/sleep"),
            decode(
                tree_owner["sha256"]
                    .as_str()
                    .ok_or("missing tree owner digest")?,
            )?,
        ));
        let principals = |name: &str| -> Result<Vec<u32>, String> {
            input[name]
                .as_array()
                .ok_or_else(|| "missing host principals".to_string())?
                .iter()
                .map(|id| {
                    id.as_u64()
                        .and_then(|id| u32::try_from(id).ok())
                        .ok_or_else(|| "invalid host principal".to_string())
                })
                .collect()
        };
        let runtime = LocalTrustedRuntime::capture_with_trust(
            &node,
            node_digest,
            &dependencies,
            LocalRuntimeTrust::new(principals("trustedUsers")?, principals("trustedGroups")?)
                .map_err(|_| "invalid host trust")?,
        )
        .map_err(|error| format!("runtime custody refused: {error}"))?;
        let runtime_identity = *runtime.digest();
        let runtime_source = *runtime.source_digest();
        let host_policy = hash(b"swallowtail.darwin-acp-idle-component-v2");
        if input["rustRuntimeSha256"]
            .as_str()
            .is_some_and(|expected| decode(expected).ok() != Some(runtime_identity))
            || input["launchPolicyRevisionSha256"]
                .as_str()
                .is_some_and(|expected| decode(expected).ok() != Some(host_policy))
            || (mode == "original"
                && (!input["rustRuntimeSha256"].is_string()
                    || !input["launchPolicyRevisionSha256"].is_string()))
        {
            return Err("operator-approved runtime/policy identity mismatch".into());
        }
        let loader = if control {
            let needle = "if (url === idleUrl) source +=";
            if CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE.matches(needle).count() != 1 {
                return Err("control source anchor mismatch".into());
            }
            CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE
                .replace(needle, "if (false && url === idleUrl) source +=")
        } else {
            CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE.to_owned()
        };
        let recipe = LocalCapturedNodeRecipe::for_trusted_runtime(
            &graph,
            &loader,
            vendor,
            &runtime,
            hash(&[host_policy.as_slice(), &[u8::from(control)]].concat()),
        )
        .map_err(|_| "component recipe refused")?;
        let graph_scope: serde_json::Value =
            serde_json::from_slice(graph.configuration()).map_err(|_| "invalid graph summary")?;
        let graph_summary = serde_json::json!({"files":graph_scope["files"].as_array().map(Vec::len),"nativeBindings":usize::from(graph_scope["native"].is_object()),"platformPrograms":graph_scope["platformPrograms"].as_array().map_or(0,Vec::len),"entrySha256":format!("{:x}",Sha256::digest(entry)),"idleSha256":format!("{:x}",Sha256::digest(&idle))});
        let fingerprint = *recipe.fingerprint().digest();
        let target = ExecutableRef::new("probe.component").expect("literal target");
        let binding = CheckedProcessRef::new("probe.component.checked").expect("literal binding");
        let launch = LocalExecutableLaunch::new(&node)
            .with_prefix_arguments(recipe.prefix_arguments().map(std::ffi::OsStr::to_os_string))
            .with_node_startup_environment_filter();
        let approval =
            LocalCheckedNodeLaunch::new(target.clone(), runtime, recipe, fingerprint, vec![vec![]])
                .map_err(|_| "component registration refused")?;
        let host = LocalProcessHost::builder(LocalProcessLimits::new(
            32,
            256 * 1024,
            1024,
            1024 * 1024,
            1024 * 1024,
        ))
        .approve_executable_launch(target.clone(), launch)
        .approve_checked_node(binding.clone(), approval)
        .map_err(|_| "checked registration refused")?
        .build();
        let process: Arc<dyn swallowtail_runtime::ProcessHandle> = Arc::from(
            block_on(host.start_checked(
                ScopeId::new("component").expect("scope"),
                binding,
                ProcessRequest::new(target),
            ))
            .map_err(|_| "checked component launch refused")?,
        );
        let (stop, receiver) = std::sync::mpsc::channel();
        let watched = process.clone();
        let watchdog = std::thread::spawn(move || {
            if receiver.recv_timeout(Duration::from_secs(10)).is_err() {
                let _ = block_on(watched.force_stop());
                let _ = block_on(watched.wait());
            }
        });
        let result = block_on(async {
            process
                .close_stdin()
                .await
                .map_err(|_| "component input close failed")?;
            let mut stdout = Vec::new();
            let mut stderr = Vec::new();
            while let Some(chunk) = process
                .read_output()
                .await
                .map_err(|_| "component output limit or read failed")?
            {
                if chunk.stream() == ProcessOutputStream::Stdout {
                    stdout.extend_from_slice(chunk.bytes());
                } else {
                    stderr.extend_from_slice(chunk.bytes());
                }
            }
            let exit = process
                .wait()
                .await
                .map_err(|_| "component cleanup unconfirmed")?;
            if !exit.success() {
                return Err(format!(
                    "component process failed: {}",
                    String::from_utf8_lossy(&stderr[stderr.len().saturating_sub(1024)..])
                ));
            }
            let value: serde_json::Value =
                serde_json::from_slice(&stdout).map_err(|_| "component output malformed")?;
            if value["entryRan"] != true
                || value["joined"] != true
                || value["settingsUnchanged"] != true
            {
                return Err("component witness incomplete".into());
            }
            let valid = if control {
                value["armed"].as_u64().unwrap_or(0) > 0
                    && value["fired"].as_u64().unwrap_or(0) > 0
                    && value["prototypeLocked"] == false
                    && value["ownLocked"] == false
            } else {
                value["armed"] == 0
                    && value["fired"] == 0
                    && value["prototypeLocked"] == true
                    && value["ownLocked"] == true
            };
            if !valid {
                return Err("component counterexample failed".into());
            }
            Ok(value)
        });
        if result.is_err() {
            let _ = block_on(process.force_stop());
            let _ = block_on(process.wait());
        }
        let _ = stop.send(());
        watchdog
            .join()
            .map_err(|_| "watchdog cleanup unconfirmed")?;
        let value = result?;
        println!(
            "{}",
            serde_json::json!({"scope":"idle-class-component-only","mode":mode,"control":control,"graph":graph_summary,"runtimeSha256":runtime_source.iter().map(|b|format!("{b:02x}")).collect::<String>(),"rustRuntimeSha256":runtime_identity.iter().map(|b|format!("{b:02x}")).collect::<String>(),"launchPolicyRevisionSha256":host_policy.iter().map(|b|format!("{b:02x}")).collect::<String>(),"compoundFingerprint":fingerprint.iter().map(|b|format!("{b:02x}")).collect::<String>(),"witness":value})
        );
        Ok(())
    }
}

fn main() {
    #[cfg(unix)]
    if let Err(error) = probe::run() {
        eprintln!("component probe refused: {error}");
        std::process::exit(78);
    }
    #[cfg(not(unix))]
    {
        eprintln!("component probe platform unavailable");
        std::process::exit(78);
    }
}
