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
        LocalProcessLimits, LocalTrustedRuntime,
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
        let mode = input["mode"].as_str().ok_or("missing mode")?;
        let control = input["control"].as_bool().ok_or("missing control")?;
        let node_digest = hash(&read(&node)?);
        let vendor = if mode == "fake" {
            hash(b"repository-owned idle component peer")
        } else if mode == "original" {
            let platform = if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
                "darwin-arm64"
            } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
                "linux-x64"
            } else {
                return Err("unadmitted platform".into());
            };
            let expected = if platform == "darwin-arm64" {
                "18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572"
            } else {
                "3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327"
            };
            if node_digest != decode(expected)? {
                return Err("original component runtime not admitted".into());
            }
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
            if hash(&read(&root.join("idle.mjs"))?)
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
        let idle = if mode == "fake" {
            include_bytes!("../tests/guard_component_probe/idle.mjs").to_vec()
        } else {
            read(&root.join("idle.mjs"))?
        };
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
        let runtime =
            LocalTrustedRuntime::capture_with_dependencies(&node, node_digest, &dependencies)
                .map_err(|error| format!("runtime custody refused: {error}"))?;
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
            hash(if control {
                b"probe-only unguarded counterexample"
            } else {
                b"guarded component witness"
            }),
        )
        .map_err(|_| "component recipe refused")?;
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
            while let Some(chunk) = process
                .read_output()
                .await
                .map_err(|_| "component output limit or read failed")?
            {
                if chunk.stream() == ProcessOutputStream::Stdout {
                    stdout.extend_from_slice(chunk.bytes());
                }
            }
            let exit = process
                .wait()
                .await
                .map_err(|_| "component cleanup unconfirmed")?;
            if !exit.success() {
                return Err("component process failed".to_owned());
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
            serde_json::json!({"scope":"idle-class-component-only","mode":mode,"control":control,"nativeExecuted":false,"routeQualified":false,"runtimeSha256":format!("{:x}",Sha256::digest(read(&node)?)),"compoundFingerprint":fingerprint.iter().map(|b|format!("{b:02x}")).collect::<String>(),"witness":value})
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
