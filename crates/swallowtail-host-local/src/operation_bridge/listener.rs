//! The single operation-scoped private loopback listener.
//!
//! This module owns the socket, accept loop, HTTP framing, bearer admission,
//! namespace/path demux, and every connection join. Closed profiles only
//! register frame handlers here; they never bind or accept independently.

use crate::output::failure;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use swallowtail_runtime::RuntimeFailure;
use zeroize::Zeroizing;

const IO_TIMEOUT: Duration = Duration::from_secs(5);
const HEADER_TERMINATOR: &[u8] = b"\r\n\r\n";
const WATCHER_NAMESPACE: &str = "watcher";
const REGISTERED_TOOL_NAMESPACE: &str = "registered-tool";

/// Stable namespace label used by watcher route registration.
pub(crate) const fn namespace_watcher() -> &'static str {
    WATCHER_NAMESPACE
}

/// Stable namespace label used by registered-tool route registration.
pub(crate) const fn namespace_registered_tool() -> &'static str {
    REGISTERED_TOOL_NAMESPACE
}

/// One frame after the operation kernel has authenticated its route.
pub(crate) struct OperationBridgeFrame {
    /// The accepted TCP connection identity, stable across its frames.
    pub(crate) connection_id: u64,
    /// The bounded JSON-RPC body. The bearer is deliberately absent.
    pub(crate) body: Vec<u8>,
}

/// A profile response written by the operation-owned connection loop.
pub(crate) struct OperationBridgeResponse {
    pub(crate) status: u16,
    pub(crate) reason: &'static str,
    pub(crate) body: Vec<u8>,
    pub(crate) connection: &'static str,
}

impl OperationBridgeResponse {
    pub(crate) fn close(status: u16, reason: &'static str, body: Vec<u8>) -> Self {
        Self {
            status,
            reason,
            body,
            connection: "close",
        }
    }

    pub(crate) fn keep_alive(status: u16, reason: &'static str, body: Vec<u8>) -> Self {
        Self {
            status,
            reason,
            body,
            connection: "keep-alive",
        }
    }
}

pub(crate) type FrameHandler =
    Arc<dyn Fn(OperationBridgeFrame) -> OperationBridgeResponse + Send + Sync>;

pub(crate) struct OperationBridgeRouteSpec {
    pub(crate) path: &'static str,
    pub(crate) transport_generation: Option<u64>,
    pub(crate) server_name: Option<&'static str>,
    pub(crate) handler: FrameHandler,
}

struct Route {
    namespace: &'static str,
    generation: u64,
    transport_generation: Option<u64>,
    server_name: Option<&'static str>,
    handler: FrameHandler,
}

/// A registration held by one profile lease. Dropping it removes the route;
/// the operation listener remains until the registry observes that no profile
/// lease remains for the operation.
pub(crate) struct OperationBridgeRoute {
    owner: Arc<OperationBridgeListener>,
    path: String,
}

impl Drop for OperationBridgeRoute {
    fn drop(&mut self) {
        self.owner
            .routes
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(&self.path);
    }
}

/// One accepted connection's join handle and its shutdown wake.
///
/// The wake is a `try_clone` of the accepted stream. Shutting its read side
/// down ends the connection loop's blocking read on an event instead of on
/// `IO_TIMEOUT`; the write side stays open so a handler already in flight
/// still delivers its response before the loop exits. The entry lives only
/// while the thread runs: the thread removes it on exit so the clone's file
/// descriptor never outlives the connection and a peer's end-of-stream still
/// lands exactly when the connection loop returns.
struct AcceptedConnection {
    connection_id: u64,
    thread: JoinHandle<()>,
    wake: Option<TcpStream>,
}

/// One listener and one accept loop for one operation.
pub(crate) struct OperationBridgeListener {
    listener_addr: SocketAddr,
    bearer: Zeroizing<String>,
    listener: Mutex<Option<TcpListener>>,
    routes: Mutex<BTreeMap<String, Route>>,
    closed: AtomicBool,
    accept_thread: Mutex<Option<JoinHandle<()>>>,
    connections: Mutex<Vec<AcceptedConnection>>,
    next_connection_id: AtomicU64,
}

impl OperationBridgeListener {
    /// Binds the operation listener exactly once and starts its sole accept
    /// loop. The private bearer is minted by this owner, not by a profile.
    pub(crate) fn bind() -> Result<Arc<Self>, RuntimeFailure> {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).map_err(|_| {
            failure(
                "swallowtail.operation_bridge.bind_failed",
                "Operation bridge could not bind a loopback listener",
            )
        })?;
        listener.set_nonblocking(false).map_err(|_| {
            failure(
                "swallowtail.operation_bridge.bind_failed",
                "Operation bridge could not configure its listener",
            )
        })?;
        let listener_addr = listener.local_addr().map_err(|_| {
            failure(
                "swallowtail.operation_bridge.bind_failed",
                "Operation bridge could not inspect its listener",
            )
        })?;
        let owner = Arc::new(Self {
            listener_addr,
            bearer: super::secret::generate_operation_secret()?,
            listener: Mutex::new(Some(listener)),
            routes: Mutex::new(BTreeMap::new()),
            closed: AtomicBool::new(false),
            accept_thread: Mutex::new(None),
            connections: Mutex::new(Vec::new()),
            next_connection_id: AtomicU64::new(1),
        });
        let accept_owner = Arc::clone(&owner);
        let thread = thread::Builder::new()
            .name("swallowtail-operation-bridge".to_owned())
            .spawn(move || accept_owner.accept_loop())
            .map_err(|_| {
                failure(
                    "swallowtail.operation_bridge.spawn_failed",
                    "Operation bridge could not start its listener",
                )
            })?;
        *owner
            .accept_thread
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(thread);
        Ok(owner)
    }

    pub(crate) fn endpoint(&self) -> SocketAddr {
        self.listener_addr
    }

    pub(crate) fn bearer(&self) -> Zeroizing<String> {
        self.bearer.clone()
    }

    /// Registers one closed-profile namespace/path against this operation.
    pub(crate) fn register(
        self: &Arc<Self>,
        namespace: &'static str,
        generation: u64,
        spec: OperationBridgeRouteSpec,
    ) -> Result<OperationBridgeRoute, RuntimeFailure> {
        let mut routes = self
            .routes
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.closed.load(Ordering::Acquire) || routes.contains_key(spec.path) {
            return Err(failure(
                "swallowtail.operation_bridge.route_unavailable",
                "Operation bridge route is unavailable",
            ));
        }
        routes.insert(
            spec.path.to_owned(),
            Route {
                namespace,
                generation,
                transport_generation: spec.transport_generation,
                server_name: spec.server_name,
                handler: spec.handler,
            },
        );
        Ok(OperationBridgeRoute {
            owner: Arc::clone(self),
            path: spec.path.to_owned(),
        })
    }

    /// Closes the one accept loop and joins every connection accepted by it.
    pub(crate) fn close(&self) {
        if self.closed.swap(true, Ordering::AcqRel) {
            return;
        }
        // Wake every accepted connection's blocking read so the joins below
        // wait for work, not for `IO_TIMEOUT`. Read-only shutdown keeps the
        // write side of each socket open, so a handler already in flight
        // still writes its response before its loop exits. The read timeout
        // remains the backstop for a peer that resists shutdown, and for a
        // stream whose clone could not be taken.
        for wake in self
            .connections
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .filter_map(|connection| connection.wake.as_ref())
        {
            let _ = wake.shutdown(Shutdown::Read);
        }
        wake_accept(self.listener_addr);
        if let Some(thread) = self
            .accept_thread
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
        {
            let _ = thread.join();
        }
        let _ = self
            .listener
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        let connections = std::mem::take(
            &mut *self
                .connections
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        );
        for connection in connections {
            if let Some(wake) = connection.wake.as_ref() {
                let _ = wake.shutdown(Shutdown::Read);
            }
            let _ = connection.thread.join();
        }
    }

    fn accept_loop(self: Arc<Self>) {
        loop {
            if self.closed.load(Ordering::Acquire) {
                break;
            }
            let accepted = {
                let listener = self
                    .listener
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                let Some(listener) = listener.as_ref() else {
                    break;
                };
                listener.accept()
            };
            match accepted {
                Ok((stream, _)) => {
                    // Registration, the closed re-check, and the spawn share
                    // one lock section so a concurrent close either sees this
                    // connection in the registry or observes `closed` before
                    // spawning; a spawned connection is always joined.
                    let mut connections = self
                        .connections
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    if self.closed.load(Ordering::Acquire) {
                        drop(stream);
                        break;
                    }
                    let connection_owner = Arc::clone(&self);
                    let connection_id = self.next_connection_id.fetch_add(1, Ordering::Relaxed);
                    let wake = stream.try_clone().ok();
                    let exit_owner = Arc::clone(&self);
                    if let Ok(thread) = thread::Builder::new()
                        .name("swallowtail-operation-bridge-conn".to_owned())
                        .spawn(move || {
                            connection_owner.connection_loop(stream, connection_id);
                            // Retire this entry so the wake clone's descriptor
                            // closes with the connection instead of pinning
                            // the socket open until close().
                            let mut connections = exit_owner
                                .connections
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner);
                            connections
                                .retain(|connection| connection.connection_id != connection_id);
                        })
                    {
                        connections.retain(|connection| !connection.thread.is_finished());
                        connections.push(AcceptedConnection {
                            connection_id,
                            thread,
                            wake,
                        });
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            }
        }
    }

    fn connection_loop(&self, mut stream: TcpStream, connection_id: u64) {
        if configure_stream(&stream).is_err() {
            return;
        }
        loop {
            if self.closed.load(Ordering::Acquire) {
                break;
            }
            let frame = match read_request(&mut stream) {
                Ok(frame) => frame,
                Err(reject) => {
                    let _ = write_response(&mut stream, reject.response());
                    break;
                }
            };
            let route = self
                .routes
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .get(&frame.path)
                .map(|route| {
                    (
                        route.namespace,
                        route.generation,
                        route.transport_generation,
                        route.server_name,
                        Arc::clone(&route.handler),
                    )
                });
            let Some((_namespace, generation, transport_generation, server_name, handler)) = route
            else {
                let _ = write_response(
                    &mut stream,
                    OperationBridgeResponse::close(
                        404,
                        "Not Found",
                        json_error("Operation bridge route was not found"),
                    ),
                );
                break;
            };
            // This is the sole bearer admission point. The route's live
            // registration is the generation/lease admission point. No
            // profile handler sees a frame until both checks have passed.
            if !constant_time_eq(
                self.bearer.as_bytes(),
                frame
                    .bearer
                    .as_ref()
                    .map_or("", |bearer| bearer.as_str())
                    .as_bytes(),
            ) {
                let _ = write_response(
                    &mut stream,
                    OperationBridgeResponse::keep_alive(
                        401,
                        "Unauthorized",
                        json_error("Operation bridge request was not authenticated"),
                    ),
                );
                continue;
            }
            if transport_generation.is_some_and(|expected| {
                frame.transport_generation != Some(expected)
                    || frame.lease_generation != Some(generation)
                    || frame.server_name.as_deref() != server_name
            }) {
                let _ = write_response(
                    &mut stream,
                    OperationBridgeResponse::keep_alive(
                        409,
                        "Conflict",
                        json_error("Operation bridge generation or lease was not current"),
                    ),
                );
                continue;
            }
            let response = handler(OperationBridgeFrame {
                connection_id,
                body: frame.body,
            });
            let keep_alive = response.connection == "keep-alive";
            if write_response(&mut stream, response).is_err() || !keep_alive {
                let _ = stream.shutdown(Shutdown::Write);
                break;
            }
        }
    }
}

impl Drop for OperationBridgeListener {
    fn drop(&mut self) {
        self.close();
    }
}

struct HttpRequest {
    path: String,
    bearer: Option<Zeroizing<String>>,
    body: Vec<u8>,
    lease_generation: Option<u64>,
    transport_generation: Option<u64>,
    server_name: Option<String>,
}

#[derive(Clone, Copy)]
enum HttpReject {
    Malformed,
    Oversized,
    Method,
}

impl HttpReject {
    fn response(self) -> OperationBridgeResponse {
        let (status, reason, message) = match self {
            Self::Malformed => (400, "Bad Request", "Malformed operation bridge request"),
            Self::Oversized => (
                413,
                "Payload Too Large",
                "Operation bridge request exceeded its bound",
            ),
            Self::Method => (
                405,
                "Method Not Allowed",
                "Operation bridge only accepts POST",
            ),
        };
        OperationBridgeResponse::close(status, reason, json_error(message))
    }
}

fn read_request(stream: &mut TcpStream) -> Result<HttpRequest, HttpReject> {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 512];
    let header_end = loop {
        if buffer.len() > swallowtail_runtime::WATCHER_BRIDGE_MAX_HEADER_BYTES {
            return Err(HttpReject::Oversized);
        }
        let read = stream.read(&mut chunk).map_err(|_| HttpReject::Malformed)?;
        if read == 0 {
            return Err(HttpReject::Malformed);
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(index) = find_subslice(&buffer, HEADER_TERMINATOR) {
            break index;
        }
    };
    let header_text =
        std::str::from_utf8(&buffer[..header_end]).map_err(|_| HttpReject::Malformed)?;
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().ok_or(HttpReject::Malformed)?;
    let mut parts = request_line.split(' ');
    let method = parts.next().ok_or(HttpReject::Malformed)?;
    let path = parts.next().ok_or(HttpReject::Malformed)?.to_owned();
    let version = parts.next().ok_or(HttpReject::Malformed)?;
    if parts.next().is_some() {
        return Err(HttpReject::Malformed);
    }
    if method != "POST" {
        return Err(HttpReject::Method);
    }
    if version != "HTTP/1.0" && version != "HTTP/1.1" {
        return Err(HttpReject::Malformed);
    }
    let mut bearer = None;
    let mut content_type = None;
    let mut content_length = None;
    let mut lease_generation = None;
    let mut transport_generation = None;
    let mut server_name = None;
    let mut header_count = 0_usize;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        header_count += 1;
        if header_count > swallowtail_runtime::WATCHER_BRIDGE_MAX_HEADER_COUNT {
            return Err(HttpReject::Oversized);
        }
        let (name, value) = line.split_once(':').ok_or(HttpReject::Malformed)?;
        let name = name.trim();
        let value = value.trim();
        if name.eq_ignore_ascii_case("authorization") {
            bearer = Some(parse_bearer(value)?);
        } else if name.eq_ignore_ascii_case("content-type") {
            content_type = Some(value.to_ascii_lowercase());
        } else if name.eq_ignore_ascii_case("content-length") {
            content_length = Some(value.parse::<usize>().map_err(|_| HttpReject::Malformed)?);
        } else if name.eq_ignore_ascii_case("transfer-encoding") {
            return Err(HttpReject::Malformed);
        } else if name.eq_ignore_ascii_case("x-swallowtail-lease-generation") {
            lease_generation = Some(value.parse::<u64>().map_err(|_| HttpReject::Malformed)?);
        } else if name.eq_ignore_ascii_case("x-swallowtail-transport-generation") {
            transport_generation = Some(value.parse::<u64>().map_err(|_| HttpReject::Malformed)?);
        } else if name.eq_ignore_ascii_case("x-swallowtail-server-name") {
            server_name = Some(value.to_owned());
        }
    }
    let content_type = content_type.ok_or(HttpReject::Malformed)?;
    if content_type != "application/json" && content_type != "application/json; charset=utf-8" {
        return Err(HttpReject::Malformed);
    }
    let content_length = content_length.ok_or(HttpReject::Malformed)?;
    let mut body = buffer.split_off(header_end + HEADER_TERMINATOR.len());
    if content_length > swallowtail_runtime::WATCHER_BRIDGE_MAX_BODY_BYTES {
        let drain_to = content_length
            .min(swallowtail_runtime::WATCHER_BRIDGE_MAX_BODY_BYTES.saturating_add(4096));
        while body.len() < drain_to {
            let read = stream.read(&mut chunk).map_err(|_| HttpReject::Malformed)?;
            if read == 0 {
                break;
            }
            body.extend_from_slice(&chunk[..read]);
        }
        return Err(HttpReject::Oversized);
    }
    while body.len() < content_length {
        let read = stream.read(&mut chunk).map_err(|_| HttpReject::Malformed)?;
        if read == 0 {
            return Err(HttpReject::Malformed);
        }
        body.extend_from_slice(&chunk[..read]);
    }
    body.truncate(content_length);
    Ok(HttpRequest {
        path,
        bearer,
        body,
        lease_generation,
        transport_generation,
        server_name,
    })
}

fn configure_stream(stream: &TcpStream) -> Result<(), RuntimeFailure> {
    stream.set_nodelay(true).map_err(|_| transport_failure())?;
    stream
        .set_read_timeout(Some(IO_TIMEOUT))
        .map_err(|_| transport_failure())?;
    stream
        .set_write_timeout(Some(IO_TIMEOUT))
        .map_err(|_| transport_failure())?;
    Ok(())
}

fn write_response(
    stream: &mut TcpStream,
    response: OperationBridgeResponse,
) -> Result<(), RuntimeFailure> {
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: {}\r\n\r\n",
        response.status,
        response.reason,
        response.body.len(),
        response.connection,
    );
    stream
        .write_all(header.as_bytes())
        .map_err(|_| transport_failure())?;
    stream
        .write_all(&response.body)
        .map_err(|_| transport_failure())?;
    stream.flush().map_err(|_| transport_failure())
}

fn parse_bearer(value: &str) -> Result<Zeroizing<String>, HttpReject> {
    let secret = value.strip_prefix("Bearer ").ok_or(HttpReject::Malformed)?;
    if secret.is_empty() || secret.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(HttpReject::Malformed);
    }
    Ok(Zeroizing::new(secret.to_owned()))
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .fold(0_u8, |acc, (left, right)| acc | (left ^ right))
            == 0
}

fn json_error(message: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": null,
        "error": { "code": -32600, "message": message },
    }))
    .unwrap_or_default()
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn wake_accept(addr: SocketAddr) {
    let _ = TcpStream::connect_timeout(&addr, Duration::from_millis(100));
}

fn transport_failure() -> RuntimeFailure {
    failure(
        "swallowtail.operation_bridge.transport_failed",
        "Operation bridge transport failed",
    )
}
