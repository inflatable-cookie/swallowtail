//! Private loopback registered-tool carrier and rendezvous lifecycle.

use super::wire::{
    REGISTERED_TOOL_PROXY_HTTP_PATH, REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES,
    REGISTERED_TOOL_PROXY_MCP_PROTOCOL_VERSION, RegisteredToolProxyRendezvousDocument,
    RegisteredToolProxyRequest, decode_request, error, initialize_result, result, tool_result,
    tools_list_result,
};
use crate::operation_bridge::generate_operation_secret;
use crate::output::failure;
use futures_executor::block_on;
use serde_json::{Map, Value, json};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use swallowtail_runtime::{
    Deadline, RegisteredToolBridgeLease, RegisteredToolCallId, RegisteredToolCallRequest,
    RegisteredToolOperationKernel, RegisteredToolPayload, RegisteredToolSchemaMediaType,
    RegisteredToolSelection, RuntimeFailure,
};
use zeroize::Zeroizing;

const READY_WAIT: Duration = Duration::from_secs(10);
const MAX_HEADER_BYTES: usize = 64 * 1024;

/// One opened private HTTP carrier owned by the registered-tool lease.
pub(crate) struct RegisteredToolProxyServer {
    state: Arc<ProxyState>,
    thread: Mutex<Option<JoinHandle<()>>>,
}

struct ProxyState {
    endpoint: SocketAddr,
    bearer: Zeroizing<String>,
    kernel: Arc<RegisteredToolOperationKernel>,
    selection: RegisteredToolSelection,
    deadline: Deadline,
    connection_claimed: AtomicBool,
    closed: AtomicBool,
    ready: Mutex<bool>,
    ready_changed: Condvar,
}

impl RegisteredToolProxyServer {
    /// Binds the one shared operation listener and starts its joined reader.
    pub(crate) fn bind(
        kernel: Arc<RegisteredToolOperationKernel>,
        selection: RegisteredToolSelection,
        deadline: Deadline,
    ) -> Result<Self, RuntimeFailure> {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).map_err(|_| {
            failure(
                "swallowtail.registered_tool.proxy_bind_failed",
                "Registered tool proxy could not bind a loopback listener",
            )
        })?;
        listener.set_nonblocking(false).map_err(|_| {
            failure(
                "swallowtail.registered_tool.proxy_bind_failed",
                "Registered tool proxy could not configure its listener",
            )
        })?;
        let endpoint = listener.local_addr().map_err(|_| {
            failure(
                "swallowtail.registered_tool.proxy_bind_failed",
                "Registered tool proxy could not inspect its listener",
            )
        })?;
        let state = Arc::new(ProxyState {
            endpoint,
            bearer: generate_operation_secret()?,
            kernel,
            selection,
            deadline,
            connection_claimed: AtomicBool::new(false),
            closed: AtomicBool::new(false),
            ready: Mutex::new(false),
            ready_changed: Condvar::new(),
        });
        let accept_state = Arc::clone(&state);
        let thread = thread::Builder::new()
            .name("swallowtail-registered-tool-proxy".to_owned())
            .spawn(move || accept_loop(accept_state, listener))
            .map_err(|_| {
                failure(
                    "swallowtail.registered_tool.proxy_spawn_failed",
                    "Registered tool proxy could not start its listener",
                )
            })?;
        Ok(Self {
            state,
            thread: Mutex::new(Some(thread)),
        })
    }

    /// Returns the host-private endpoint used only in the rendezvous file.
    pub(crate) fn endpoint(&self) -> String {
        format!(
            "http://127.0.0.1:{}{}",
            self.state.endpoint.port(),
            REGISTERED_TOOL_PROXY_HTTP_PATH
        )
    }

    /// Returns whether the courier completed the MCP ready barrier.
    pub(crate) fn wait_until_ready(&self) -> Result<(), RuntimeFailure> {
        let deadline = Instant::now() + READY_WAIT;
        let mut ready = self.state.ready.lock().expect("proxy ready lock poisoned");
        while !*ready && !self.state.closed.load(Ordering::Acquire) {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(failure(
                    "swallowtail.registered_tool.proxy_not_ready",
                    "Registered tool proxy did not reach its ready barrier",
                ));
            }
            let (next, timeout) = self
                .state
                .ready_changed
                .wait_timeout(ready, remaining)
                .expect("proxy ready wait lock poisoned");
            ready = next;
            if timeout.timed_out() {
                return Err(failure(
                    "swallowtail.registered_tool.proxy_not_ready",
                    "Registered tool proxy did not reach its ready barrier",
                ));
            }
        }
        if *ready {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.registered_tool.proxy_closed",
                "Registered tool proxy closed before its ready barrier",
            ))
        }
    }

    /// Creates one private rendezvous file for the exact opened lease.
    pub(crate) fn rendezvous(
        &self,
        lease: &RegisteredToolBridgeLease,
    ) -> Result<RegisteredToolProxyRendezvous, RuntimeFailure> {
        let document = RegisteredToolProxyRendezvousDocument::new(
            self.endpoint(),
            self.state.bearer.to_string(),
            lease.generation().get(),
            lease.transport_generation().get(),
            lease.selection().protocol_version(),
        );
        RegisteredToolProxyRendezvous::create(document)
    }

    /// Freezes the listener and joins the accept/connection thread.
    pub(crate) fn close(&self) {
        self.state.closed.store(true, Ordering::Release);
        self.state.ready_changed.notify_all();
        let _ = TcpStream::connect_timeout(&self.state.endpoint, Duration::from_millis(100));
        if let Some(thread) = self
            .thread
            .lock()
            .expect("proxy thread lock poisoned")
            .take()
        {
            let _ = thread.join();
        }
    }
}

impl Drop for RegisteredToolProxyServer {
    fn drop(&mut self) {
        self.close();
    }
}

/// One-shot, operation-scoped rendezvous path.
///
/// The file is never serializable or cloneable. Its contents are removed by a
/// courier's first reader and the path is removed again at drop or expiry.
pub struct RegisteredToolProxyRendezvous {
    directory: PathBuf,
    path: PathBuf,
    expired: bool,
}

impl RegisteredToolProxyRendezvous {
    fn create(document: RegisteredToolProxyRendezvousDocument) -> Result<Self, RuntimeFailure> {
        let root = std::env::temp_dir().join("swallowtail-mediated-stdio");
        fs::create_dir_all(&root).map_err(|_| rendezvous_failure())?;
        set_private_directory(&root)?;
        let name = generate_operation_secret()?.to_string();
        let directory = root.join(name);
        fs::create_dir(&directory).map_err(|_| rendezvous_failure())?;
        set_private_directory(&directory)?;
        let path = directory.join("rendezvous.json");
        let body = document.encode().map_err(|_| rendezvous_failure())?;
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&path).map_err(|_| rendezvous_failure())?;
        set_private_file(&file)?;
        if file.write_all(&body).is_err() || file.sync_all().is_err() {
            let _ = fs::remove_file(&path);
            let _ = fs::remove_dir(&directory);
            return Err(rendezvous_failure());
        }
        Ok(Self {
            directory,
            path,
            expired: false,
        })
    }

    /// Returns the non-authoritative path passed to the courier.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns exactly the fixed wire tag and rendezvous path arguments.
    #[must_use]
    pub fn courier_arguments(&self) -> [String; 2] {
        [
            swallowtail_runtime::REGISTERED_TOOL_PROXY_WIRE_TAG.to_owned(),
            self.path.to_string_lossy().into_owned(),
        ]
    }

    /// Expires the rendezvous at the ready barrier.
    pub fn expire(&mut self) {
        if !self.expired {
            self.expired = true;
            remove_rendezvous(&self.path, &self.directory);
        }
    }
}

impl Drop for RegisteredToolProxyRendezvous {
    fn drop(&mut self) {
        self.expire();
    }
}

fn accept_loop(state: Arc<ProxyState>, listener: TcpListener) {
    while !state.closed.load(Ordering::Acquire) {
        match listener.accept() {
            Ok((stream, _)) => {
                if state.connection_claimed.swap(true, Ordering::AcqRel) {
                    drop(stream);
                    break;
                }
                handle_connection(&state, stream);
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
}

fn handle_connection(state: &Arc<ProxyState>, mut stream: TcpStream) {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(100)));
    while !state.closed.load(Ordering::Acquire) {
        let request = match read_http_request(&mut stream) {
            Ok(Some(request)) => request,
            Ok(None) => break,
            Err(_) => break,
        };
        let (mut status, mut body) = match authenticate(state, request.authorization.as_deref()) {
            Ok(()) if request.path == REGISTERED_TOOL_PROXY_HTTP_PATH => {
                dispatch_request(state, &request.body)
            }
            Ok(()) => (
                404,
                error(None, -32601, "Unknown registered-tool proxy path"),
            ),
            Err(_) => (
                401,
                error(None, -32001, "Unauthorized registered-tool proxy request"),
            ),
        };
        if body.len() > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES {
            status = 413;
            body = error(
                None,
                -32600,
                "Registered tool proxy response exceeds its bound",
            );
        }
        if write_http_response(&mut stream, status, &body).is_err() {
            break;
        }
    }
    let _ = stream.shutdown(Shutdown::Both);
}

struct HttpRequest {
    path: String,
    authorization: Option<String>,
    body: Vec<u8>,
}

fn read_http_request(stream: &mut TcpStream) -> Result<Option<HttpRequest>, ()> {
    let mut header = Vec::with_capacity(1024);
    let mut byte = [0_u8; 1];
    loop {
        match stream.read(&mut byte) {
            Ok(0) => return Ok(None),
            Ok(1) => {
                header.push(byte[0]);
                if header.ends_with(b"\r\n\r\n") {
                    break;
                }
                if header.len() > MAX_HEADER_BYTES {
                    return Err(());
                }
            }
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue;
            }
            Err(_) => return Err(()),
            _ => return Err(()),
        }
    }
    let header_text = std::str::from_utf8(&header[..header.len() - 4]).map_err(|_| ())?;
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().ok_or(())?;
    let mut request_parts = request_line.split_whitespace();
    if request_parts.next() != Some("POST") {
        return Err(());
    }
    let path = request_parts.next().ok_or(())?.to_owned();
    if request_parts.next().is_none() {
        return Err(());
    }
    let mut authorization = None;
    let mut content_length = None;
    for line in lines {
        let (name, value) = line.split_once(':').ok_or(())?;
        if name.eq_ignore_ascii_case("authorization") {
            authorization = Some(value.trim().to_owned());
        } else if name.eq_ignore_ascii_case("content-length") {
            content_length = Some(value.trim().parse::<usize>().map_err(|_| ())?);
        }
    }
    let length = content_length.ok_or(())?;
    if length > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES {
        return Err(());
    }
    let mut body = vec![0_u8; length];
    stream.read_exact(&mut body).map_err(|_| ())?;
    Ok(Some(HttpRequest {
        path,
        authorization,
        body,
    }))
}

fn write_http_response(stream: &mut TcpStream, status: u16, body: &[u8]) -> Result<(), ()> {
    let reason = match status {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        401 => "Unauthorized",
        413 => "Payload Too Large",
        _ => "Error",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes()).map_err(|_| ())?;
    stream.write_all(body).map_err(|_| ())?;
    stream.flush().map_err(|_| ())
}

fn authenticate(state: &ProxyState, value: Option<&str>) -> Result<(), ()> {
    let Some(value) = value.and_then(|value| value.strip_prefix("Bearer ")) else {
        return Err(());
    };
    if constant_time_eq(state.bearer.as_bytes(), value.as_bytes()) {
        Ok(())
    } else {
        Err(())
    }
}

fn dispatch_request(state: &Arc<ProxyState>, body: &[u8]) -> (u16, Vec<u8>) {
    let request = match decode_request(body) {
        Ok(request) => request,
        Err(_) => {
            return (
                400,
                error(None, -32600, "Malformed registered-tool proxy request"),
            );
        }
    };
    match request {
        RegisteredToolProxyRequest::Initialize {
            id,
            protocol_version,
        } => {
            if protocol_version != REGISTERED_TOOL_PROXY_MCP_PROTOCOL_VERSION {
                return (
                    200,
                    error(Some(id), -32602, "Unsupported MCP protocol version"),
                );
            }
            let mut ready = state.ready.lock().expect("proxy ready lock poisoned");
            *ready = true;
            state.ready_changed.notify_all();
            (200, result(id, initialize_result()))
        }
        RegisteredToolProxyRequest::Initialized => {
            if !is_ready(state) {
                return (
                    200,
                    error(None, -32006, "Registered-tool proxy handshake required"),
                );
            }
            (202, Vec::new())
        }
        RegisteredToolProxyRequest::ToolsList { id } => {
            if !is_ready(state) {
                return (
                    200,
                    error(Some(id), -32006, "Registered-tool proxy handshake required"),
                );
            }
            (200, result(id, tools_list(state)))
        }
        RegisteredToolProxyRequest::ToolsCall {
            id,
            name,
            arguments,
        } => {
            if !is_ready(state) {
                return (
                    200,
                    error(Some(id), -32006, "Registered-tool proxy handshake required"),
                );
            }
            dispatch_tool(state, id, &name, arguments)
        }
    }
}

fn is_ready(state: &ProxyState) -> bool {
    *state.ready.lock().expect("proxy ready lock poisoned")
}

fn tools_list(state: &ProxyState) -> Value {
    let tools = state
        .selection
        .selected()
        .iter()
        .filter_map(|id| {
            let declaration = state.selection.snapshot().declaration(id)?;
            let schema = serde_json::from_str::<Value>(
                declaration.input_schema().document().expose_for_execution(),
            )
            .unwrap_or_else(|_| json!({ "type": "object" }));
            Some(json!({
                "name": id.to_string(),
                "description": format!("Registered tool {}", id),
                "inputSchema": schema,
            }))
        })
        .collect();
    tools_list_result(tools)
}

fn dispatch_tool(
    state: &Arc<ProxyState>,
    id: Value,
    name: &str,
    arguments: Map<String, Value>,
) -> (u16, Vec<u8>) {
    let Some(tool) = state
        .selection
        .selected()
        .iter()
        .find(|candidate| candidate.to_string() == name)
        .cloned()
    else {
        return (200, error(Some(id), -32602, "Unknown registered tool"));
    };
    let call_id = RegisteredToolCallId::new(courier_call_id(&id));
    let Ok(call_id) = call_id else {
        return (
            200,
            error(Some(id), -32600, "Invalid registered tool correlation"),
        );
    };
    let Ok(media_type) = RegisteredToolSchemaMediaType::new("application/json") else {
        return (
            200,
            error(Some(id), -32603, "Registered tool schema unavailable"),
        );
    };
    let arguments = match serde_json::to_vec(&Value::Object(arguments)) {
        Ok(arguments) => arguments,
        Err(_) => {
            return (
                200,
                error(Some(id), -32602, "Invalid registered tool arguments"),
            );
        }
    };
    let bounds = state.selection.effective_bounds();
    let Ok(arguments) =
        RegisteredToolPayload::new(media_type, arguments, bounds.max_argument_bytes())
    else {
        return (
            413,
            error(Some(id), -32600, "Registered tool argument limit exceeded"),
        );
    };
    let outcome = block_on(RegisteredToolOperationKernel::issue(
        &state.kernel,
        RegisteredToolCallRequest::new(call_id, tool, arguments, state.deadline),
    ));
    let Ok(outcome) = outcome else {
        return (
            200,
            error(Some(id), -32000, "Registered tool dispatch failed"),
        );
    };
    match outcome.result() {
        Some(tool_result_value) => (
            200,
            result(
                id,
                tool_result(tool_result_value.payload().expose_for_execution(), false),
            ),
        ),
        None => (
            200,
            result(
                id,
                tool_result(
                    outcome
                        .failure()
                        .map_or(b"registered tool call failed".as_slice(), |_| {
                            b"registered tool call failed"
                        }),
                    true,
                ),
            ),
        ),
    }
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .fold(0_u8, |acc, (left, right)| acc | (left ^ right))
            == 0
}

fn courier_call_id(id: &Value) -> String {
    match id {
        Value::String(value) => format!("stdio-string:{value}"),
        Value::Number(value) => format!("stdio-number:{value}"),
        Value::Null | Value::Bool(_) | Value::Array(_) | Value::Object(_) => {
            "stdio-invalid".to_owned()
        }
    }
}

/// Private, bounded host material for one courier process.
pub struct RegisteredToolProxyLaunch {
    request: swallowtail_runtime::ProcessRequest,
    rendezvous: RegisteredToolProxyRendezvous,
    server: Arc<RegisteredToolProxyServer>,
}

impl RegisteredToolProxyLaunch {
    pub(crate) fn new(
        request: swallowtail_runtime::ProcessRequest,
        rendezvous: RegisteredToolProxyRendezvous,
        server: Arc<RegisteredToolProxyServer>,
    ) -> Self {
        Self {
            request,
            rendezvous,
            server,
        }
    }

    /// Returns the host-approved process request for the provider's launcher.
    #[must_use]
    pub const fn process_request(&self) -> &swallowtail_runtime::ProcessRequest {
        &self.request
    }

    /// Returns the non-authoritative rendezvous path.
    #[must_use]
    pub fn rendezvous_path(&self) -> &Path {
        self.rendezvous.path()
    }

    /// Waits for the courier to authenticate and complete MCP initialization.
    pub fn wait_until_ready(&mut self) -> Result<(), RuntimeFailure> {
        let result = self.server.wait_until_ready();
        self.rendezvous.expire();
        result
    }
}

fn rendezvous_failure() -> RuntimeFailure {
    failure(
        "swallowtail.registered_tool.proxy_rendezvous_failed",
        "Registered tool proxy rendezvous could not be created",
    )
}

fn remove_rendezvous(path: &Path, directory: &Path) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_dir(directory);
}

#[cfg(unix)]
fn set_private_directory(path: &Path) -> Result<(), RuntimeFailure> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(|_| rendezvous_failure())
}

#[cfg(not(unix))]
fn set_private_directory(_path: &Path) -> Result<(), RuntimeFailure> {
    Ok(())
}

#[cfg(unix)]
fn set_private_file(file: &File) -> Result<(), RuntimeFailure> {
    use std::os::unix::fs::PermissionsExt;
    file.set_permissions(fs::Permissions::from_mode(0o600))
        .map_err(|_| rendezvous_failure())
}

#[cfg(not(unix))]
fn set_private_file(_file: &File) -> Result<(), RuntimeFailure> {
    Ok(())
}
