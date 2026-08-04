#[path = "websocket/failure.rs"]
mod failure;
#[path = "websocket/worker.rs"]
mod worker;

pub(super) use self::failure::resync_failure;
use self::failure::{disconnected, protocol_failure, turn_timeout};
use crate::failure::failure as runtime_failure;
use futures_channel::{mpsc, oneshot};
use futures_core::Stream;
use std::net::{Shutdown, TcpStream};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc as sync_mpsc;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swallowtail_runtime::{BoxFuture, Deadline, HostServices, RuntimeFailure, ScopeId};

const UPDATE_CAPACITY: usize = 128;
const FRAME_LIMIT: usize = 64 * 1024;

enum Update {
    Ready {
        current_seq: u64,
        current_epoch: Option<String>,
    },
    Event(Vec<u8>),
}

enum Command {
    Abort {
        frame: String,
        id: String,
        result: oneshot::Sender<Result<(), RuntimeFailure>>,
    },
    Close,
}

pub(super) struct Subscription {
    updates: mpsc::Receiver<Result<Update, RuntimeFailure>>,
    commands: sync_mpsc::Sender<Command>,
    work: Option<BoxFuture<'static, Result<(), RuntimeFailure>>>,
    cancelled: Arc<AtomicBool>,
    control: Arc<Mutex<Option<TcpStream>>>,
    replay_target: (u64, Option<String>),
}

pub(super) struct SubscriptionInput {
    pub(super) scope: ScopeId,
    pub(super) endpoint: String,
    pub(super) secret: Vec<u8>,
    pub(super) session_id: String,
    pub(super) cursor_seq: u64,
    pub(super) cursor_epoch: Option<String>,
    pub(super) deadline: Option<Deadline>,
}

#[derive(Clone)]
pub(super) struct SubscriptionControl {
    commands: sync_mpsc::Sender<Command>,
}

impl Subscription {
    pub(super) async fn open(
        input: SubscriptionInput,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        let SubscriptionInput {
            scope,
            endpoint,
            secret,
            session_id,
            cursor_seq,
            cursor_epoch,
            deadline,
        } = input;
        let blocking = services.blocking_work().cloned().ok_or_else(|| {
            runtime_failure(
                "swallowtail.kimi.local_server.blocking_service_missing",
                "Kimi local-server WebSocket requires a blocking-work service",
            )
        })?;
        let (updates, receiver) = mpsc::channel(UPDATE_CAPACITY);
        let (commands, command_receiver) = sync_mpsc::channel();
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_cancelled = Arc::clone(&cancelled);
        let control = Arc::new(Mutex::new(None));
        let worker_control = Arc::clone(&control);
        let initial_replay_target = (cursor_seq, cursor_epoch.clone());
        let work = blocking.run(
            scope,
            Box::new(move || {
                worker::run(
                    endpoint,
                    secret,
                    session_id,
                    cursor_seq,
                    cursor_epoch,
                    updates,
                    command_receiver,
                    worker_cancelled,
                    worker_control,
                )
            }),
        );
        let mut subscription = Self {
            updates: receiver,
            commands,
            work: Some(work),
            cancelled,
            control,
            replay_target: initial_replay_target,
        };
        subscription.replay_target = match subscription.wait_ready(deadline, services).await {
            Ok(target) => target,
            Err(error) => {
                let _ = subscription.close().await;
                return Err(error);
            }
        };
        Ok(subscription)
    }

    pub(super) fn replay_target(&self) -> (u64, Option<&str>) {
        (self.replay_target.0, self.replay_target.1.as_deref())
    }

    pub(super) fn control(&self) -> SubscriptionControl {
        SubscriptionControl {
            commands: self.commands.clone(),
        }
    }

    pub(super) fn poll_next(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<Vec<u8>, RuntimeFailure>>> {
        if let Poll::Ready(item) = Pin::new(&mut self.updates).poll_next(context)
            && item.is_some()
        {
            return Poll::Ready(item.map(|item| {
                item.and_then(|update| match update {
                    Update::Event(frame) => Ok(frame),
                    Update::Ready { .. } => Err(protocol_failure()),
                })
            }));
        }
        let work = self
            .work
            .as_mut()
            .map_or(Poll::Ready(Ok(())), |work| work.as_mut().poll(context));
        match work {
            Poll::Ready(result) => {
                self.work = None;
                match result {
                    Err(error) => Poll::Ready(Some(Err(error))),
                    Ok(()) => Pin::new(&mut self.updates).poll_next(context).map(|item| {
                        item.map(|item| {
                            item.and_then(|update| match update {
                                Update::Event(frame) => Ok(frame),
                                Update::Ready { .. } => Err(protocol_failure()),
                            })
                        })
                    }),
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }

    pub(super) async fn close(mut self) -> Result<(), RuntimeFailure> {
        self.cancelled.store(true, Ordering::SeqCst);
        let _ = self.commands.send(Command::Close);
        if let Some(stream) = self.control.lock().expect("control lock poisoned").take() {
            let _ = stream.shutdown(Shutdown::Both);
        }
        match self.work.take() {
            Some(work) => work.await,
            None => Ok(()),
        }
    }

    async fn wait_ready(
        &mut self,
        deadline: Option<Deadline>,
        services: &HostServices,
    ) -> Result<(u64, Option<String>), RuntimeFailure> {
        let mut timer =
            deadline.and_then(|deadline| services.time().map(|time| time.wait_until(deadline)));
        std::future::poll_fn(|context| {
            if let Poll::Ready(item) = Pin::new(&mut self.updates).poll_next(context) {
                return Poll::Ready(match item {
                    Some(Ok(Update::Ready {
                        current_seq,
                        current_epoch,
                    })) => Ok((current_seq, current_epoch)),
                    Some(Ok(Update::Event(_))) => Err(protocol_failure()),
                    Some(Err(error)) => Err(error),
                    None => Err(disconnected()),
                });
            }
            let Some(work) = self.work.as_mut() else {
                return Poll::Ready(Err(disconnected()));
            };
            match work.as_mut().poll(context) {
                Poll::Ready(Ok(())) => Poll::Ready(Err(disconnected())),
                Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
                Poll::Pending => {
                    if timer
                        .as_mut()
                        .is_some_and(|timer| timer.as_mut().poll(context).is_ready())
                    {
                        self.cancelled.store(true, Ordering::SeqCst);
                        Poll::Ready(Err(turn_timeout()))
                    } else {
                        Poll::Pending
                    }
                }
            }
        })
        .await
    }
}

impl SubscriptionControl {
    pub(super) fn close(&self) -> Result<(), RuntimeFailure> {
        self.commands
            .send(Command::Close)
            .map_err(|_| disconnected())
    }

    pub(super) async fn abort(
        &self,
        session_id: &str,
        prompt_id: &str,
    ) -> Result<(), RuntimeFailure> {
        let id = "swallowtail-abort".to_owned();
        let frame = serde_json::json!({
            "type": "abort",
            "id": id,
            "payload": {"session_id": session_id, "prompt_id": prompt_id}
        })
        .to_string();
        let (sender, receiver) = oneshot::channel();
        self.commands
            .send(Command::Abort {
                frame,
                id,
                result: sender,
            })
            .map_err(|_| disconnected())?;
        receiver.await.map_err(|_| disconnected())?
    }
}
