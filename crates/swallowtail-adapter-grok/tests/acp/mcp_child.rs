// Provider-side spawn of ACP-declared stdio MCP children.
//
// Grok spawns the servers a client declares in `session/new`. This fixture
// does the same for the reserved registered-tool courier, so Swallowtail never
// intercepts `ProcessService::start` to stand in for the provider child.

struct SpawnedMcpChild {
    stdin: Mutex<Option<std::process::ChildStdin>>,
    stdout: Mutex<Option<std::io::BufReader<std::process::ChildStdout>>>,
    child: Mutex<std::process::Child>,
}

impl SpawnedMcpChild {
    fn spawn(command: &str, args: &[String], env: &[(String, String)]) -> Result<Arc<Self>, ()> {
        use std::io::Read;
        let mut launched = std::process::Command::new(command);
        launched
            .args(args)
            .env_clear()
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        for (key, value) in env {
            launched.env(key, value);
        }
        let mut child = launched.spawn().map_err(|_| ())?;
        let stdin = child.stdin.take().ok_or(())?;
        let stdout = child.stdout.take().ok_or(())?;
        if let Some(mut stderr) = child.stderr.take() {
            std::thread::spawn(move || {
                let mut discarded = Vec::new();
                let _ = stderr.read_to_end(&mut discarded);
            });
        }
        Ok(Arc::new(Self {
            stdin: Mutex::new(Some(stdin)),
            stdout: Mutex::new(Some(std::io::BufReader::new(stdout))),
            child: Mutex::new(child),
        }))
    }

    fn kill(&self) {
        let _ = self.child.lock().expect("mcp child lock").kill();
    }

    /// Writes one framed record directly, without an executor.
    ///
    /// The provider side runs inside the driver's own `block_on`, so it cannot
    /// nest another executor to reach these futures.
    fn write_line_sync(&self, bytes: &[u8]) -> Result<(), ()> {
        use std::io::Write;
        let mut stdin = self.stdin.lock().expect("mcp stdin lock");
        stdin
            .as_mut()
            .ok_or(())?
            .write_all(bytes)
            .map_err(|_| ())?;
        stdin.as_mut().ok_or(())?.flush().map_err(|_| ())
    }

    /// Reads one framed record directly, without an executor.
    fn read_line_sync(&self) -> Result<Vec<u8>, ()> {
        use std::io::BufRead;
        let mut stdout = self.stdout.lock().expect("mcp stdout lock");
        let reader = stdout.as_mut().ok_or(())?;
        let mut line = Vec::new();
        let read = reader.read_until(b'\n', &mut line).map_err(|_| ())?;
        if read == 0 { Err(()) } else { Ok(line) }
    }
}

impl ProcessHandle for SpawnedMcpChild {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        use std::io::Write;
        let result = (|| {
            let mut stdin = self.stdin.lock().expect("mcp stdin lock");
            stdin
                .as_mut()
                .ok_or_else(fixture_failure)?
                .write_all(chunk.bytes())
                .map_err(|_| fixture_failure())?;
            Ok(())
        })();
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        drop(self.stdin.lock().expect("mcp stdin lock").take());
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        use std::io::BufRead;
        let result = (|| {
            let mut stdout = self.stdout.lock().expect("mcp stdout lock");
            let reader = stdout.as_mut().ok_or_else(fixture_failure)?;
            let mut line = Vec::new();
            let read = reader
                .read_until(b'\n', &mut line)
                .map_err(|_| fixture_failure())?;
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
                .map_err(|_| fixture_failure())
        })
    }
}

/// Spawns every ACP-declared MCP server exactly as the provider would.
///
/// ACP carries `env` as a list of `{name, value}` objects. A declaration this
/// fixture cannot read is a fixture failure, not a silently skipped child.
fn spawn_declared_mcp_servers(params: &Value) -> Result<Vec<Arc<SpawnedMcpChild>>, ()> {
    let Some(servers) = params.get("mcpServers").and_then(Value::as_array) else {
        return Ok(Vec::new());
    };
    let mut spawned = Vec::new();
    for server in servers {
        let command = server["command"].as_str().ok_or(())?;
        let args = server["args"]
            .as_array()
            .ok_or(())?
            .iter()
            .map(|value| value.as_str().map(str::to_owned).ok_or(()))
            .collect::<Result<Vec<_>, _>>()?;
        let env = server["env"]
            .as_array()
            .ok_or(())?
            .iter()
            .map(|value| {
                let name = value["name"].as_str().ok_or(())?;
                let value = value["value"].as_str().ok_or(())?;
                Ok((name.to_owned(), value.to_owned()))
            })
            .collect::<Result<Vec<_>, ()>>()?;
        spawned.push(SpawnedMcpChild::spawn(command, &args, &env)?);
    }
    Ok(spawned)
}

/// Handshakes the spawned courier and fires one call it will not answer yet.
///
/// This is the provider side: Grok connects the MCP server it spawned and
/// calls a tool on it. The response is deliberately not read, so the call stays
/// outstanding while the fixture drives the rest of the lifecycle.
fn drive_courier_call(child: &Arc<SpawnedMcpChild>) -> Result<(), ()> {
    let initialize = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{{\"protocolVersion\":\"{}\"}}}}\n",
        swallowtail_adapter_grok::registered_tool::GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION
    );
    child.write_line_sync(initialize.as_bytes())?;
    child.read_line_sync()?;
    child.write_line_sync(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")?;
    // The response is deliberately not read: the call stays outstanding while
    // the fixture drives the rest of the lifecycle.
    child.write_line_sync(
        b"{\"jsonrpc\":\"2.0\",\"id\":7,\"method\":\"tools/call\",\"params\":{\"name\":\"desktop/reconcile\",\"arguments\":{}}}\n",
    )
}
