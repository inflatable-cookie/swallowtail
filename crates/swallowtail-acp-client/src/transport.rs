use crate::diagnostics::AcpDiagnostics;
use serde_json::Value;
use std::sync::Arc;
#[cfg(feature = "remote")]
use swallowtail_core::SafeDiagnostic;
use swallowtail_protocol_acp::{
    FramingLimits, Message, NdjsonDecoder, encode_error, encode_notification, encode_request,
    encode_result,
};
use swallowtail_runtime::{
    CleanupOutcome, ProcessHandle, ProcessInputChunk, ProcessOutputStream, RuntimeFailure,
};
#[cfg(feature = "remote")]
use swallowtail_transport_acp_remote::{RemoteAcpConnection, RemoteAcpError, RemoteAcpSender};

/// Physical transport backing an [`crate::AcpConnection`].
pub enum AcpTransport {
    /// Local stdio subprocess framed with bounded NDJSON.
    Stdio {
        /// Spawned child process handle.
        process: Arc<dyn ProcessHandle>,
        /// Bounded framing limits for stdout NDJSON decoding.
        framing_limits: FramingLimits,
    },
    /// Host-authorized remote ACP HTTP/SSE or WebSocket transport.
    #[cfg(feature = "remote")]
    Remote {
        /// Full-duplex remote ACP transport connection.
        connection: Box<RemoteAcpConnection>,
    },
}

impl AcpTransport {
    /// Creates a stdio transport with explicit NDJSON framing limits.
    #[must_use]
    pub fn stdio(process: Arc<dyn ProcessHandle>, framing_limits: FramingLimits) -> Self {
        Self::Stdio {
            process,
            framing_limits,
        }
    }

    /// Creates a remote ACP transport from a connected [`RemoteAcpConnection`].
    ///
    /// Available only with the `remote` feature.
    #[cfg(feature = "remote")]
    #[must_use]
    pub fn remote(connection: RemoteAcpConnection) -> Self {
        Self::Remote {
            connection: Box::new(connection),
        }
    }

    pub(crate) fn split(self) -> (AcpTransportWriter, AcpTransportReader) {
        match self {
            Self::Stdio {
                process,
                framing_limits,
            } => (
                AcpTransportWriter::Stdio(Arc::clone(&process)),
                AcpTransportReader::Stdio {
                    process,
                    decoder: NdjsonDecoder::new(framing_limits),
                },
            ),
            #[cfg(feature = "remote")]
            Self::Remote { connection } => {
                let sender = connection.sender();
                (
                    AcpTransportWriter::Remote(sender),
                    AcpTransportReader::Remote { connection },
                )
            }
        }
    }
}

pub(crate) enum AcpTransportWriter {
    Stdio(Arc<dyn ProcessHandle>),
    #[cfg(feature = "remote")]
    Remote(RemoteAcpSender),
}

impl AcpTransportWriter {
    pub(crate) async fn send_request(
        &self,
        id: u64,
        method: &'static str,
        params: Value,
        diagnostics: &AcpDiagnostics,
    ) -> Result<(), RuntimeFailure> {
        match self {
            Self::Stdio(process) => {
                let bytes = encode_request(id, method, params)
                    .map_err(|_| (diagnostics.protocol_failure)())?;
                process.write_stdin(ProcessInputChunk::new(bytes)).await
            }
            #[cfg(feature = "remote")]
            Self::Remote(sender) => sender
                .send(Message::Request {
                    id: Value::from(id),
                    method: method.to_owned(),
                    params,
                })
                .await
                .map_err(remote_error_to_runtime),
        }
    }

    pub(crate) async fn send_notification(
        &self,
        method: &'static str,
        params: Value,
        diagnostics: &AcpDiagnostics,
    ) -> Result<(), RuntimeFailure> {
        match self {
            Self::Stdio(process) => {
                let bytes = encode_notification(method, params)
                    .map_err(|_| (diagnostics.protocol_failure)())?;
                process.write_stdin(ProcessInputChunk::new(bytes)).await
            }
            #[cfg(feature = "remote")]
            Self::Remote(sender) => sender
                .send(Message::Notification {
                    method: method.to_owned(),
                    params,
                })
                .await
                .map_err(remote_error_to_runtime),
        }
    }

    pub(crate) async fn send_result(
        &self,
        id: Value,
        result: Value,
        diagnostics: &AcpDiagnostics,
    ) -> Result<(), RuntimeFailure> {
        match self {
            Self::Stdio(process) => {
                let bytes =
                    encode_result(id, result).map_err(|_| (diagnostics.protocol_failure)())?;
                process.write_stdin(ProcessInputChunk::new(bytes)).await
            }
            #[cfg(feature = "remote")]
            Self::Remote(sender) => sender
                .send(Message::Response {
                    id,
                    result: Ok(result),
                })
                .await
                .map_err(remote_error_to_runtime),
        }
    }

    pub(crate) async fn send_error(
        &self,
        id: Value,
        code: i64,
        message: &'static str,
        diagnostics: &AcpDiagnostics,
    ) -> Result<(), RuntimeFailure> {
        match self {
            Self::Stdio(process) => {
                let bytes = encode_error(id, code, message)
                    .map_err(|_| (diagnostics.protocol_failure)())?;
                process.write_stdin(ProcessInputChunk::new(bytes)).await
            }
            #[cfg(feature = "remote")]
            Self::Remote(sender) => {
                let decoded = Message::Response {
                    id,
                    result: Err(swallowtail_protocol_acp::RpcError::new(code, message)),
                };
                sender.send(decoded).await.map_err(remote_error_to_runtime)
            }
        }
    }

    pub(crate) async fn begin_close(&self) {
        match self {
            Self::Stdio(process) => {
                let _ = process.close_stdin().await;
                let _ = process.request_stop().await;
            }
            #[cfg(feature = "remote")]
            Self::Remote(sender) => {
                sender.request_close().await;
            }
        }
    }

    pub(crate) async fn force_stop(&self) -> Result<(), RuntimeFailure> {
        match self {
            Self::Stdio(process) => process.force_stop().await,
            #[cfg(feature = "remote")]
            Self::Remote(sender) => sender.cancel().await.map_err(remote_error_to_runtime),
        }
    }
}

pub(crate) enum AcpTransportReader {
    Stdio {
        process: Arc<dyn ProcessHandle>,
        decoder: NdjsonDecoder,
    },
    #[cfg(feature = "remote")]
    Remote {
        connection: Box<RemoteAcpConnection>,
    },
}

pub(crate) enum ReadStep {
    Messages(Vec<Message>),
    EndOfStream,
}

impl AcpTransportReader {
    pub(crate) async fn next_batch(
        &mut self,
        diagnostics: &AcpDiagnostics,
    ) -> Result<ReadStep, (&'static str, RuntimeFailure)> {
        match self {
            Self::Stdio { process, decoder } => loop {
                match process.read_output().await {
                    Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                        return decoder
                            .push(chunk.bytes())
                            .map(ReadStep::Messages)
                            .map_err(|_| ("acp.pump.decode", (diagnostics.protocol_failure)()));
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => return Ok(ReadStep::EndOfStream),
                    Err(error) => return Err(("acp.pump.read", error)),
                }
            },
            #[cfg(feature = "remote")]
            Self::Remote { connection } => match connection.next_event().await {
                Some(Ok(message)) => Ok(ReadStep::Messages(vec![message])),
                Some(Err(error)) => Err(("acp.pump.read", remote_error_to_runtime(error))),
                None => Ok(ReadStep::EndOfStream),
            },
        }
    }

    pub(crate) async fn finish_and_cleanup(
        self,
        had_failure: bool,
        diagnostics: &AcpDiagnostics,
    ) -> (Option<(&'static str, RuntimeFailure)>, CleanupOutcome) {
        match self {
            Self::Stdio { process, decoder } => {
                let mut finish_error = None;
                if !had_failure && decoder.finish().is_err() {
                    finish_error = Some(("acp.pump.finish", (diagnostics.protocol_failure)()));
                }
                if had_failure || finish_error.is_some() {
                    let _ = process.force_stop().await;
                }
                let cleanup = if process.wait().await.is_ok() {
                    CleanupOutcome::Clean
                } else {
                    CleanupOutcome::Failed((diagnostics.process_cleanup_failed)())
                };
                (finish_error, cleanup)
            }
            #[cfg(feature = "remote")]
            Self::Remote { connection } => {
                let cleanup = connection.close().await;
                (None, cleanup)
            }
        }
    }
}

#[cfg(feature = "remote")]
fn remote_error_to_runtime(error: RemoteAcpError) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        error.diagnostic().code(),
        error.diagnostic().message(),
    ))
}
