use crate::failure::failure;
use crate::realtime_protocol::{RealtimeServerEvent, parse_server_event};
use futures_channel::{mpsc, oneshot};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc as sync_mpsc;
use std::sync::{Arc, Mutex};
use swallowtail_core::ProviderRequestRef;
use swallowtail_runtime::RuntimeFailure;
use tungstenite::{Error, Message};

const UPDATE_CAPACITY: usize = 32;

mod transport;

use transport::{Socket, open_socket};

pub(super) enum WorkerUpdate {
    Event(RealtimeServerEvent),
    Failed(RuntimeFailure),
    Disconnected,
}

enum WorkerCommand {
    Send(String, oneshot::Sender<Result<(), RuntimeFailure>>),
    Close(oneshot::Sender<()>),
}

pub(super) struct ConnectionWorker {
    socket: Socket,
    commands: sync_mpsc::Receiver<WorkerCommand>,
    updates: mpsc::Sender<WorkerUpdate>,
}

#[derive(Clone)]
pub(super) struct WorkerHandle {
    commands: sync_mpsc::Sender<WorkerCommand>,
    updates: Arc<Mutex<Option<mpsc::Receiver<WorkerUpdate>>>>,
    request_ref: ProviderRequestRef,
    closer: SocketCloser,
}

#[derive(Clone)]
struct SocketCloser(Arc<Mutex<Option<TcpStream>>>);

impl SocketCloser {
    fn shutdown(&self) {
        if let Some(stream) = self.0.lock().expect("closer lock poisoned").take() {
            let _ = stream.shutdown(Shutdown::Both);
        }
    }
}

impl ConnectionWorker {
    pub(super) fn open(
        endpoint: &str,
        secret: &[u8],
    ) -> Result<(Self, WorkerHandle), RuntimeFailure> {
        let (socket, request_ref, control) = open_socket(endpoint, secret)?;
        let closer = SocketCloser(Arc::new(Mutex::new(Some(control))));
        let (command_sender, commands) = sync_mpsc::channel();
        let (updates, update_receiver) = mpsc::channel(UPDATE_CAPACITY);
        let handle = WorkerHandle {
            commands: command_sender,
            updates: Arc::new(Mutex::new(Some(update_receiver))),
            request_ref,
            closer,
        };
        Ok((
            Self {
                socket,
                commands,
                updates,
            },
            handle,
        ))
    }

    pub(super) fn run(mut self) -> Result<(), RuntimeFailure> {
        loop {
            match self.commands.try_recv() {
                Ok(WorkerCommand::Send(frame, acknowledgement)) => {
                    let result = self
                        .socket
                        .send(Message::Text(frame.into()))
                        .map_err(|_| disconnected());
                    let failed = result.is_err();
                    let _ = acknowledgement.send(result);
                    if failed {
                        return Err(disconnected());
                    }
                    continue;
                }
                Ok(WorkerCommand::Close(acknowledgement)) => {
                    let _ = self.socket.close(None);
                    let _ = acknowledgement.send(());
                    return Ok(());
                }
                Err(sync_mpsc::TryRecvError::Disconnected) => return Ok(()),
                Err(sync_mpsc::TryRecvError::Empty) => {}
            }
            match self.socket.read() {
                Ok(Message::Text(frame)) => {
                    let update = match parse_server_event(frame.as_bytes()) {
                        Ok(event) => WorkerUpdate::Event(event),
                        Err(error) => WorkerUpdate::Failed(error),
                    };
                    let terminal = matches!(update, WorkerUpdate::Failed(_));
                    if self.updates.try_send(update).is_err() {
                        return Err(ingress_overflow());
                    }
                    if terminal {
                        return Ok(());
                    }
                }
                Ok(Message::Ping(bytes)) => {
                    self.socket
                        .send(Message::Pong(bytes))
                        .map_err(|_| disconnected())?;
                }
                Ok(Message::Pong(_)) => {}
                Ok(Message::Close(_)) => {
                    let _ = self.updates.try_send(WorkerUpdate::Disconnected);
                    return Ok(());
                }
                Ok(Message::Binary(_) | Message::Frame(_)) => {
                    let _ = self.updates.try_send(WorkerUpdate::Failed(failure(
                        "swallowtail.openai.realtime_frame_type_rejected",
                        "OpenAI Realtime returned an unsupported frame type",
                    )));
                    return Ok(());
                }
                Err(Error::Io(error))
                    if matches!(
                        error.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) => {}
                Err(Error::ConnectionClosed | Error::AlreadyClosed) => {
                    let _ = self.updates.try_send(WorkerUpdate::Disconnected);
                    return Ok(());
                }
                Err(_) => {
                    let _ = self.updates.try_send(WorkerUpdate::Disconnected);
                    return Ok(());
                }
            }
        }
    }
}

impl WorkerHandle {
    pub(super) fn take_updates(&self) -> Option<mpsc::Receiver<WorkerUpdate>> {
        self.updates.lock().expect("updates lock poisoned").take()
    }

    pub(super) fn request_ref(&self) -> &ProviderRequestRef {
        &self.request_ref
    }

    pub(super) async fn send(&self, event: serde_json::Value) -> Result<(), RuntimeFailure> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .send(WorkerCommand::Send(event.to_string(), sender))
            .map_err(|_| disconnected())?;
        receiver.await.map_err(|_| disconnected())?
    }

    pub(super) async fn close(&self) -> Result<(), RuntimeFailure> {
        let (sender, receiver) = oneshot::channel();
        if self.commands.send(WorkerCommand::Close(sender)).is_err() {
            self.closer.shutdown();
            return Ok(());
        }
        if receiver.await.is_err() {
            self.closer.shutdown();
        }
        Ok(())
    }

    pub(super) fn abort(&self) {
        self.closer.shutdown();
    }
}

fn disconnected() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_disconnected",
        "OpenAI Realtime connection ended before terminal response truth",
    )
}

fn ingress_overflow() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_ingress_overflow",
        "OpenAI Realtime event ingress exceeded its bounded capacity",
    )
}
