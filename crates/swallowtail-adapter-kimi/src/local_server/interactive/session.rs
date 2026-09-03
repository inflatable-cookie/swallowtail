#[path = "session/handle.rs"]
mod handle;
#[path = "session/pump.rs"]
mod pump;
#[path = "session/turn.rs"]
mod turn;

pub(in crate::local_server) use self::handle::{
    ActiveSlot, ActiveTurn, TurnCancellation, TurnDetachment,
};
pub(super) use self::handle::{KimiTurnHandle, SessionCancellation, close_active, reap};
use super::access::{SessionAccess, merge};
use super::websocket::{Subscription, SubscriptionInput};
use crate::failure::failure;
use crate::local_server::protocol::{PromptStatus, decode_prompt_submission};
use crate::local_server::transport::CurlTransport;
use futures_channel::oneshot;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use swallowtail_core::SessionRef;
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, InteractiveSessionHandle,
    ProviderSessionManagementBinding, RequestId, RuntimeFailure, RuntimeSessionId, RuntimeTurnId,
    SessionResumeBinding, TurnHandle, TurnRequest, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 256;

pub(super) struct CursorState {
    pub(super) seq: u64,
    pub(super) epoch: Option<String>,
}

pub(in crate::local_server) struct KimiInteractiveSession {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) provider_ref: SessionRef,
    pub(super) provider_session_id: String,
    pub(super) resume: SessionResumeBinding,
    pub(super) management: Option<ProviderSessionManagementBinding>,
    pub(super) model_id: swallowtail_core::ModelId,
    pub(super) options: swallowtail_runtime::SessionOptions,
    pub(super) configuration: super::KimiLocalServerSessionConfiguration,
    pub(super) cursor: Arc<Mutex<CursorState>>,
    pub(super) access: Option<SessionAccess>,
    pub(super) services: HostServices,
    pub(super) transport: CurlTransport,
    pub(in crate::local_server) active: ActiveSlot,
    pub(super) cancellation: SessionCancellation,
}

impl InteractiveSessionHandle for KimiInteractiveSession {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        Some(&self.provider_ref)
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        Some(&self.resume)
    }

    fn management_binding(&self) -> Option<&ProviderSessionManagementBinding> {
        self.management.as_ref()
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_turn(&request, &services)?;
            reap(&self.active).await?;
            if self
                .active
                .lock()
                .expect("active turn lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.kimi.local_server.turn_active",
                    "Kimi local-server session already has an active turn",
                ));
            }
            let (endpoint, secret) = {
                let access = self.access.as_ref().ok_or_else(session_closed)?;
                (access.endpoint.clone(), Arc::clone(&access.secret))
            };
            let (cursor_seq, cursor_epoch) = {
                let cursor = self.cursor.lock().expect("cursor lock poisoned");
                (cursor.seq, cursor.epoch.clone())
            };
            let scope = turn_scope(request.turn_id())?;
            let prompt_request = self.prompt_request(&request)?;
            let (callback_hub, callbacks) = self.callback_exchange(request.turn_id(), &services)?;
            let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            event_sender.send(swallowtail_runtime::RuntimeEvent::new(
                1,
                swallowtail_runtime::RuntimeEventKind::Started,
            ))?;
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let terminal_flag = Arc::new(AtomicBool::new(false));
            let checkpoint_ready = Arc::new(AtomicBool::new(false));
            let (pump_sender, pump_receiver) = oneshot::channel();
            let task = services.task().expect("validated task service").spawn(
                scope.clone(),
                Box::pin(turn::run_pump_when_ready(pump_receiver)),
            )?;
            let subscription = match Subscription::open(
                SubscriptionInput {
                    scope: scope.clone(),
                    endpoint: endpoint.clone(),
                    secret: secret.copy(),
                    session_id: self.provider_session_id.clone(),
                    cursor_seq,
                    cursor_epoch,
                    deadline: request.deadline(),
                },
                &services,
            )
            .await
            {
                Ok(subscription) => subscription,
                Err(error) => return turn::fail_setup(pump_sender, task, error).await,
            };
            let control = subscription.control();
            let transport = self.transport.clone();
            let prompt_cancelled = Arc::new(AtomicBool::new(false));
            let prompt = turn::before_turn_deadline(
                transport.request(
                    scope.clone(),
                    endpoint.clone(),
                    prompt_request,
                    Some(secret.copy()),
                    &services,
                    Arc::clone(&prompt_cancelled),
                ),
                request.deadline(),
                &services,
                prompt_cancelled,
            )
            .await
            .and_then(|response| {
                if response.status == 200 {
                    decode_prompt_submission(&response.body)
                } else {
                    Err(failure(
                        "swallowtail.kimi.local_server.prompt_rejected",
                        "Kimi local server rejected the prompt",
                    ))
                }
            });
            let prompt = match prompt {
                Ok(prompt) if prompt.status == PromptStatus::Running => prompt,
                Ok(_) => {
                    let _ = subscription.close().await;
                    return turn::fail_setup(
                        pump_sender,
                        task,
                        failure(
                            "swallowtail.kimi.local_server.prompt_not_running",
                            "Kimi local server did not start the submitted prompt",
                        ),
                    )
                    .await;
                }
                Err(error) => {
                    let _ = subscription.close().await;
                    return turn::fail_setup(pump_sender, task, error).await;
                }
            };
            let cancellation = Arc::new(TurnCancellation {
                control: Mutex::new(control),
                session_id: self.provider_session_id.clone(),
                prompt_id: prompt.id,
                requested: AtomicBool::new(false),
            });
            let detachment = self.configuration.active_turn_detachment().then(|| {
                Arc::new(TurnDetachment {
                    cancellation: Arc::clone(&cancellation),
                    terminal: Arc::clone(&terminal_flag),
                    checkpoint_ready: Arc::clone(&checkpoint_ready),
                    requested: AtomicBool::new(false),
                })
            });
            let pump_input = pump::PumpInput {
                subscription: Some(subscription),
                scope: scope.clone(),
                session_id: self.provider_session_id.clone(),
                runtime_turn_id: request.turn_id().clone(),
                deadline: request.deadline(),
                services: services.clone(),
                transport: self.transport.clone(),
                endpoint,
                secret: Arc::downgrade(&secret),
                cursor: Arc::clone(&self.cursor),
                cancellation: Arc::clone(&cancellation),
                detachment: detachment.clone(),
                callbacks: callback_hub,
                events: event_sender,
                terminal: terminal_sender,
                terminal_flag: Arc::clone(&terminal_flag),
                checkpoint_ready,
                remaining_reattachments: self.configuration.maximum_reattachments(),
            };
            if let Err(input) = pump_sender.send(pump_input) {
                if let Some(subscription) = input.subscription {
                    let _ = subscription.close().await;
                }
                let error = failure(
                    "swallowtail.kimi.local_server.task_start_failed",
                    "Kimi local-server turn task did not accept provider work",
                );
                let _ = task.join().await;
                return Err(error);
            }
            *self.active.lock().expect("active turn lock poisoned") = Some(ActiveTurn {
                task: Some(task),
                cancellation: Arc::clone(&cancellation),
                terminal: Arc::clone(&terminal_flag),
            });
            Ok(Box::new(KimiTurnHandle {
                runtime_id: request.turn_id().clone(),
                provider_ref: None,
                events: Some(Box::pin(event_stream)),
                callbacks,
                terminal: Some(Box::pin(terminal)),
                cancellation,
                detachment,
                terminal_flag,
                active: Arc::clone(&self.active),
            }) as Box<dyn TurnHandle>)
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(
        mut self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        let execution_host_id = self.services.execution_host_id().clone();
        swallowtail_runtime::bound_session_cleanup(
            execution_host_id,
            request,
            services,
            Box::pin(async move {
                let active = close_active(&self.active).await;
                let access = match self.access.as_mut() {
                    Some(access) => access.release(&self.services).await,
                    None => CleanupOutcome::NotApplicable,
                };
                merge(active, access)
            }),
        )
    }
}

fn turn_scope(turn: &RuntimeTurnId) -> Result<swallowtail_runtime::ScopeId, RuntimeFailure> {
    swallowtail_runtime::ScopeId::new(format!("kimi-local:turn:{}", turn.as_str()))
        .map_err(|_| protocol_failure())
}

fn session_closed() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.session_closed",
        "Kimi local-server session is closed",
    )
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.turn_protocol_failed",
        "Kimi local-server turn protocol failed",
    )
}
