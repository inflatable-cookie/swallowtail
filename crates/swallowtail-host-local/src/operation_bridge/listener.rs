//! Shared private loopback listener lifecycle for closed operation profiles.

use crate::output::failure;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn bind_loopback() -> Result<(TcpListener, std::net::SocketAddr), RuntimeFailure> {
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
    let addr = listener.local_addr().map_err(|_| {
        failure(
            "swallowtail.operation_bridge.bind_failed",
            "Operation bridge could not inspect its listener",
        )
    })?;
    Ok((listener, addr))
}

pub(crate) fn wake_accept(addr: std::net::SocketAddr) {
    let _ = TcpStream::connect_timeout(&addr, Duration::from_millis(100));
}

/// Starts the shared accept-loop implementation used by closed profiles.
pub(crate) fn spawn_accept_loop(
    listener: TcpListener,
    thread_name: &'static str,
    should_close: Arc<dyn Fn() -> bool + Send + Sync>,
    accept_one: bool,
    handler: Arc<dyn Fn(TcpStream) + Send + Sync>,
) -> Result<JoinHandle<()>, RuntimeFailure> {
    thread::Builder::new()
        .name(thread_name.to_owned())
        .spawn(move || {
            loop {
                if should_close() {
                    break;
                }
                match listener.accept() {
                    Ok((stream, _)) => {
                        handler(stream);
                        if accept_one {
                            break;
                        }
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(_) => break,
                }
            }
        })
        .map_err(|_| {
            failure(
                "swallowtail.operation_bridge.spawn_failed",
                "Operation bridge could not start its listener",
            )
        })
}
