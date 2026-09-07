//! Provider-side spawn of declared stdio MCP children.
//!
//! The real SDK constructs `McpStdioServerConfig` and spawns that command.
//! This fixture does the same for the reserved registered-tool courier so
//! Swallowtail never intercepts `ProcessService::start` to stand in for the
//! provider child. Card 084 consumer-declared servers stay unspawned echoes.

use super::super::host::Shared;
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{Arc, Mutex};
use swallowtail_adapter_claude_agent::sdk::registered_tool::CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, RuntimeFailure,
};

const REGISTERED_TOOL_SERVER: &str = CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER;

pub(in crate::sdk_support) struct SpawnedMcpChild {
    stdin: Mutex<Option<ChildStdin>>,
    stdout: Mutex<Option<BufReader<ChildStdout>>>,
    child: Mutex<Child>,
}

impl SpawnedMcpChild {
    fn spawn(command: &str, args: &[String], env: &[(String, String)]) -> Result<Arc<Self>, ()> {
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
            stdout: Mutex::new(Some(BufReader::new(stdout))),
            child: Mutex::new(child),
        }))
    }

    fn kill(&self) {
        let _ = self.child.lock().expect("mcp child lock").kill();
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

pub(super) fn spawn_registered_courier(shared: &Shared, params: &Value) -> Result<(), ()> {
    let Some(servers) = params.get("mcpServers").and_then(Value::as_array) else {
        return Ok(());
    };
    for server in servers {
        if server["name"].as_str() != Some(REGISTERED_TOOL_SERVER) {
            continue;
        }
        let command = server["command"].as_str().ok_or(())?;
        let args = server["args"]
            .as_array()
            .ok_or(())?
            .iter()
            .map(|value| value.as_str().map(str::to_owned).ok_or(()))
            .collect::<Result<Vec<_>, _>>()?;
        let env = match server.get("env") {
            None | Some(Value::Null) => Vec::new(),
            Some(Value::Object(values)) => values
                .iter()
                .map(|(key, value)| {
                    value
                        .as_str()
                        .map(|value| (key.clone(), value.to_owned()))
                        .ok_or(())
                })
                .collect::<Result<Vec<_>, _>>()?,
            Some(_) => return Err(()),
        };
        let child = SpawnedMcpChild::spawn(command, &args, &env)?;
        shared
            .spawned_mcp
            .lock()
            .expect("spawned mcp lock")
            .push(child);
    }
    Ok(())
}

pub(super) fn kill_spawned(shared: &Shared) {
    let children = shared
        .spawned_mcp
        .lock()
        .expect("spawned mcp lock")
        .drain(..)
        .collect::<Vec<_>>();
    for child in children {
        child.kill();
    }
}

fn mcp_closed() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.mcp_child.closed",
        "fixture MCP child closed",
    ))
}
