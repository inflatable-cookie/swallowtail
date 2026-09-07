//! Private loopback registered-tool carrier and rendezvous lifecycle.

use super::wire::{
    REGISTERED_TOOL_PROXY_HTTP_PATH, REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES,
    REGISTERED_TOOL_PROXY_MCP_PROTOCOL_VERSION, REGISTERED_TOOL_PROXY_SERVER_NAME,
    RegisteredToolProxyRendezvousDocument, RegisteredToolProxyRequest, decode_request, error,
    initialize_result, result, tool_result, tools_list_result,
};
use crate::operation_bridge::generate_operation_secret;
use crate::operation_bridge::{
    OperationBridgeFrame, OperationBridgeListener, OperationBridgeResponse, OperationBridgeRoute,
    OperationBridgeRouteSpec, namespace_registered_tool,
};
use crate::output::failure;
use futures_executor::block_on;
use serde_json::{Map, Value, json};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use swallowtail_runtime::{
    Deadline, RegisteredToolBridgeLease, RegisteredToolCallId, RegisteredToolCallRequest,
    RegisteredToolExecutionKind, RegisteredToolOperationKernel, RegisteredToolPayload,
    RegisteredToolSchemaMediaType, RegisteredToolSelection, RuntimeFailure, TimeService,
};
use zeroize::Zeroizing;

const READY_WAIT: Duration = Duration::from_secs(10);
/// One opened private HTTP carrier owned by the registered-tool lease.
pub(crate) struct RegisteredToolProxyServer {
    state: Arc<ProxyState>,
    route: Mutex<Option<OperationBridgeRoute>>,
}

struct ProxyState {
    endpoint: SocketAddr,
    bearer: Zeroizing<String>,
    kernel: Arc<RegisteredToolOperationKernel>,
    selection: RegisteredToolSelection,
    time: Arc<dyn TimeService>,
    deadline: Deadline,
    connection_claimed: std::sync::atomic::AtomicU64,
    rendezvous_claimed: AtomicBool,
    rendezvous: Mutex<Option<Arc<RendezvousState>>>,
    closed: AtomicBool,
    ready: Mutex<bool>,
    ready_changed: Condvar,
}

impl RegisteredToolProxyServer {
    /// Binds the one shared operation listener and starts its joined reader.
    pub(crate) fn bind(
        listener: Arc<OperationBridgeListener>,
        kernel: Arc<RegisteredToolOperationKernel>,
        selection: RegisteredToolSelection,
        time: Arc<dyn TimeService>,
        deadline: Deadline,
    ) -> Result<Self, RuntimeFailure> {
        let endpoint = listener.endpoint();
        let bearer = listener.bearer();
        let generation = kernel.binding().lease_generation().get();
        let transport_generation = kernel.binding().transport_generation().get();
        let state = Arc::new(ProxyState {
            endpoint,
            bearer,
            kernel,
            selection,
            time,
            deadline,
            connection_claimed: AtomicU64::new(0),
            rendezvous_claimed: AtomicBool::new(false),
            rendezvous: Mutex::new(None),
            closed: AtomicBool::new(false),
            ready: Mutex::new(false),
            ready_changed: Condvar::new(),
        });
        let handler_state = Arc::clone(&state);
        let handler =
            Arc::new(move |frame: OperationBridgeFrame| handle_frame(&handler_state, frame));
        let route = listener.register(
            namespace_registered_tool(),
            generation,
            OperationBridgeRouteSpec {
                path: REGISTERED_TOOL_PROXY_HTTP_PATH,
                transport_generation: Some(transport_generation),
                server_name: Some(REGISTERED_TOOL_PROXY_SERVER_NAME),
                handler,
            },
        )?;
        Ok(Self {
            state,
            route: Mutex::new(Some(route)),
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
            self.expire_rendezvous();
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
        if self.state.rendezvous_claimed.swap(true, Ordering::AcqRel) {
            return Err(failure(
                "swallowtail.registered_tool.proxy_rendezvous_unavailable",
                "Registered tool proxy already has a courier launch",
            ));
        }
        let document = RegisteredToolProxyRendezvousDocument::new(
            self.endpoint(),
            self.state.bearer.to_string(),
            lease.generation().get(),
            lease.transport_generation().get(),
            lease.selection().protocol_version(),
            connect_timeout_millis(self.state.deadline, self.state.time.now()),
        );
        let rendezvous = RegisteredToolProxyRendezvous::create(document)?;
        *self
            .state
            .rendezvous
            .lock()
            .expect("proxy rendezvous lock poisoned") = Some(Arc::clone(&rendezvous.state));
        Ok(rendezvous)
    }

    fn expire_rendezvous(&self) {
        if let Some(rendezvous) = self
            .state
            .rendezvous
            .lock()
            .expect("proxy rendezvous lock poisoned")
            .as_ref()
        {
            rendezvous.expire();
        }
    }

    /// Freezes this profile route. The operation owner joins the listener and
    /// every accepted connection exactly once after all profile leases close.
    pub(crate) fn close(&self) {
        self.state.closed.store(true, Ordering::Release);
        self.expire_rendezvous();
        self.state.ready_changed.notify_all();
        drop(
            self.route
                .lock()
                .expect("proxy thread lock poisoned")
                .take(),
        );
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
    state: Arc<RendezvousState>,
}

struct RendezvousState {
    directory: PathBuf,
    path: PathBuf,
    expired: AtomicBool,
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
            state: Arc::new(RendezvousState {
                directory,
                path,
                expired: AtomicBool::new(false),
            }),
        })
    }

    /// Returns the non-authoritative path passed to the courier.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.state.path
    }

    /// Returns exactly the fixed wire tag and rendezvous path arguments.
    #[must_use]
    pub fn courier_arguments(&self) -> [String; 2] {
        [
            swallowtail_runtime::REGISTERED_TOOL_PROXY_WIRE_TAG.to_owned(),
            self.state.path.to_string_lossy().into_owned(),
        ]
    }

    /// Expires the rendezvous at the ready barrier.
    pub fn expire(&self) {
        if !self.state.expired.swap(true, Ordering::AcqRel) {
            remove_rendezvous(&self.state.path, &self.state.directory);
        }
    }
}

impl Drop for RegisteredToolProxyRendezvous {
    fn drop(&mut self) {
        self.expire();
    }
}

impl RendezvousState {
    fn expire(&self) {
        if !self.expired.swap(true, Ordering::AcqRel) {
            remove_rendezvous(&self.path, &self.directory);
        }
    }
}

fn handle_frame(state: &Arc<ProxyState>, frame: OperationBridgeFrame) -> OperationBridgeResponse {
    if state.closed.load(Ordering::Acquire) {
        return OperationBridgeResponse::close(
            410,
            "Gone",
            error(None, -32006, "Registered tool proxy is closed"),
        );
    }
    let claimed = state.connection_claimed.load(Ordering::Acquire);
    if claimed == 0
        && state
            .connection_claimed
            .compare_exchange(0, frame.connection_id, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
    {
        return OperationBridgeResponse::close(
            409,
            "Conflict",
            error(None, -32005, "Registered tool proxy allows one connection"),
        );
    }
    if state.connection_claimed.load(Ordering::Acquire) != frame.connection_id {
        return OperationBridgeResponse::close(
            409,
            "Conflict",
            error(None, -32005, "Registered tool proxy allows one connection"),
        );
    }
    let (mut status, mut body) = dispatch_request(state, &frame.body);
    if body.len() > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES {
        status = 413;
        body = error(
            None,
            -32600,
            "Registered tool proxy response exceeds its bound",
        );
    }
    OperationBridgeResponse::keep_alive(status, response_reason(status), body)
}

fn response_reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        401 => "Unauthorized",
        413 => "Payload Too Large",
        _ => "Error",
    }
}

fn connect_timeout_millis(deadline: Deadline, now: swallowtail_runtime::MonotonicInstant) -> u64 {
    let remaining_nanos = deadline.instant().ticks().saturating_sub(now.ticks());
    remaining_nanos
        .saturating_add(999_999)
        .saturating_div(1_000_000)
        .clamp(1, 10_000)
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
            if state.kernel.mark_ready().is_err() {
                return (
                    200,
                    error(Some(id), -32006, "Registered-tool proxy is not ready"),
                );
            }
            let mut ready = state.ready.lock().expect("proxy ready lock poisoned");
            *ready = true;
            state.ready_changed.notify_all();
            if let Some(rendezvous) = state
                .rendezvous
                .lock()
                .expect("proxy rendezvous lock poisoned")
                .as_ref()
            {
                rendezvous.expire();
            }
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
            if declaration.kind() != RegisteredToolExecutionKind::Mcp {
                return None;
            }
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
    if state
        .selection
        .snapshot()
        .declaration(&tool)
        .is_none_or(|declaration| declaration.kind() != RegisteredToolExecutionKind::Mcp)
    {
        return (200, error(Some(id), -32602, "Unknown registered tool"));
    }
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
    let _ = fs::remove_file(path.with_extension("claimed"));
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
