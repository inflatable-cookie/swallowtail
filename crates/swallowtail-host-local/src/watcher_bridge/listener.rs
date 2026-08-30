use super::failure::{malformed_failure, oversized_failure, unknown_failure};
use super::http::{HttpReject, configure_stream, read_request, write_json};
use super::protocol::{
    DecodedRequest, authenticate, correlation_id, decode_request, decoded_request_id, dispatch,
    error_http_status, error_message, jsonrpc_error, recoverable_request_id,
};
use super::state::LiveLease;
use crate::output::failure;
use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use swallowtail_runtime::RuntimeFailure;

pub(super) fn bind_loopback() -> Result<(TcpListener, std::net::SocketAddr), RuntimeFailure> {
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).map_err(|_| {
        failure(
            "swallowtail.watcher_bridge.bind_failed",
            "Watcher bridge could not bind a loopback listener",
        )
    })?;
    listener.set_nonblocking(false).map_err(|_| {
        failure(
            "swallowtail.watcher_bridge.bind_failed",
            "Watcher bridge could not bind a loopback listener",
        )
    })?;
    let addr = listener.local_addr().map_err(|_| {
        failure(
            "swallowtail.watcher_bridge.bind_failed",
            "Watcher bridge could not bind a loopback listener",
        )
    })?;
    Ok((listener, addr))
}

pub(super) fn wake_accept(addr: std::net::SocketAddr) {
    let _ = TcpStream::connect_timeout(&addr, Duration::from_millis(100));
}

pub(super) fn spawn_accept(
    live: Arc<LiveLease>,
    listener: TcpListener,
) -> Result<(), RuntimeFailure> {
    let accept_live = Arc::clone(&live);
    let thread = thread::Builder::new()
        .name("swallowtail-watcher-bridge".to_owned())
        .spawn(move || accept_loop(accept_live, listener))
        .map_err(|_| {
            failure(
                "swallowtail.watcher_bridge.spawn_failed",
                "Watcher bridge could not start its listener",
            )
        })?;
    *live
        .accept_thread
        .lock()
        .expect("watcher bridge accept thread lock poisoned") = Some(thread);
    Ok(())
}

fn accept_loop(live: Arc<LiveLease>, listener: TcpListener) {
    loop {
        if live.is_closed() {
            break;
        }
        match listener.accept() {
            Ok((stream, _)) => {
                if live.is_closed() {
                    break;
                }
                if live.admit_connection().is_err() {
                    drop(stream);
                    continue;
                }
                let handler_live = Arc::clone(&live);
                match thread::Builder::new()
                    .name("swallowtail-watcher-bridge-conn".to_owned())
                    .spawn(move || handle_connection(handler_live, stream))
                {
                    Ok(thread) => live.retain_connection(thread),
                    Err(_) => live.release_connection(),
                }
            }
            Err(error) if error.kind() == ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
}

fn handle_connection(live: Arc<LiveLease>, mut stream: TcpStream) {
    let _guard = ConnectionGuard(&live);
    if configure_stream(&stream).is_err() {
        return;
    }
    let request = match read_request(&mut stream) {
        Ok(request) => request,
        Err(HttpReject::Oversized) => {
            let _ = write_failure(&mut stream, &oversized_failure(), None);
            return;
        }
        Err(HttpReject::Method) => {
            let _ = write_json(
                &mut stream,
                405,
                "Method Not Allowed",
                &jsonrpc_error(None, -32600, unknown_failure().diagnostic().message()),
            );
            return;
        }
        Err(HttpReject::Malformed) => {
            let _ = write_failure(&mut stream, &malformed_failure(), None);
            return;
        }
    };
    if let Err(error) = authenticate(&live, request.bearer.as_deref().map(String::as_str)) {
        let _ = write_failure(&mut stream, &error, recoverable_request_id(&request.body));
        return;
    }
    let decoded = match decode_request(&request.body) {
        Ok(decoded) => decoded,
        Err(error) => {
            let _ = write_failure(&mut stream, &error, recoverable_request_id(&request.body));
            return;
        }
    };
    let request_id = decoded_request_id(&decoded);
    let admitted = match request_correlation(&decoded) {
        Ok(Some(id)) => match live.admit_request(&id) {
            Ok(()) => true,
            Err(error) => {
                let _ = write_failure(&mut stream, &error, request_id);
                return;
            }
        },
        Ok(None) => false,
        Err(error) => {
            let _ = write_failure(&mut stream, &error, request_id);
            return;
        }
    };
    let dispatched = dispatch(&live, decoded);
    if admitted {
        live.release_request();
    }
    match dispatched {
        Ok(Some(body)) => {
            let _ = write_json(&mut stream, 200, "OK", &body);
        }
        Ok(None) => {
            let _ = write_json(&mut stream, 202, "Accepted", "{}");
        }
        Err(error) => {
            let _ = write_failure(&mut stream, &error, request_id);
        }
    }
}

fn request_correlation(
    decoded: &DecodedRequest,
) -> Result<Option<String>, swallowtail_runtime::RuntimeFailure> {
    match decoded {
        DecodedRequest::Initialized => Ok(None),
        DecodedRequest::Initialize { id }
        | DecodedRequest::ToolsList { id }
        | DecodedRequest::ToolsCall { id, .. } => correlation_id(id).map(Some),
    }
}

fn write_failure(
    stream: &mut TcpStream,
    error: &RuntimeFailure,
    id: Option<serde_json::Value>,
) -> Result<(), RuntimeFailure> {
    let (status, reason, code) = error_http_status(error);
    write_json(
        stream,
        status,
        reason,
        &jsonrpc_error(id, code, error_message(error)),
    )
}

struct ConnectionGuard<'a>(&'a Arc<LiveLease>);

impl Drop for ConnectionGuard<'_> {
    fn drop(&mut self) {
        self.0.release_connection();
    }
}

pub(super) fn endpoint_url(addr: std::net::SocketAddr) -> String {
    format!(
        "http://127.0.0.1:{}{}",
        addr.port(),
        swallowtail_runtime::WATCHER_BRIDGE_HTTP_PATH
    )
}
