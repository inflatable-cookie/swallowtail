use super::OpenAiRealtimeDriver;
use super::access::AccessLeases;
use super::lifecycle::{ActiveSlot, SessionCancellation, cleanup, merge};
use super::worker::{WorkerHandle, WorkerUpdate};
use crate::failure::failure;
use crate::realtime_protocol::{ClientEvent, RealtimeServerEvent};
use futures_channel::mpsc;
use futures_core::Stream;
use std::future::poll_fn;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::{Arc, Mutex};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, OpenRealtimeMediaSessionRequest,
    RealtimeMediaSessionDriver, RealtimeMediaSessionHandle, RealtimeMediaSessionState, RequestId,
    RuntimeFailure, RuntimeSessionId, ScopeId,
};

pub(super) struct OpenAiRealtimeSession {
    pub(super) request_id: RequestId,
    pub(super) session_id: RuntimeSessionId,
    pub(super) config: swallowtail_core::RealtimeMediaConfig,
    pub(super) services: HostServices,
    pub(super) worker: WorkerHandle,
    pub(super) worker_work: Option<BoxFuture<'static, Result<(), RuntimeFailure>>>,
    pub(super) updates: Arc<Mutex<Option<mpsc::Receiver<WorkerUpdate>>>>,
    pub(super) access: Option<AccessLeases>,
    pub(super) state: Arc<Mutex<RealtimeMediaSessionState>>,
    pub(super) reusable: Arc<AtomicBool>,
    pub(super) next_event_sequence: Arc<AtomicU64>,
    pub(super) active: ActiveSlot,
    pub(super) cancellation: Arc<SessionCancellation>,
    pub(super) turn_index: u32,
    pub(super) next_append_event: u64,
    pub(super) deadline: Option<swallowtail_runtime::Deadline>,
}

impl RealtimeMediaSessionDriver for OpenAiRealtimeDriver {
    fn open_realtime_media_session(
        &self,
        plan: PreflightPlan,
        request: OpenRealtimeMediaSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RealtimeMediaSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(plan.execution_host_id())?;
            Self::validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "openai-realtime:session:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| invalid_scope())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let (worker, worker_work) = match access.connect(scope.clone(), &services).await {
                Ok(connection) => connection,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let mut updates = worker.take_updates().expect("new worker owns updates");
            let configured = configure(&worker, &mut updates).await;
            if let Err(error) = configured {
                let _ = worker.close().await;
                let _ = worker_work.await;
                let _ = access.release(&services).await;
                return Err(error);
            }
            let session_id =
                RuntimeSessionId::new(format!("openai-realtime:{}", request.request_id().as_str()))
                    .map_err(|_| invalid_scope())?;
            let config = request.config().clone();
            let state = Arc::new(Mutex::new(RealtimeMediaSessionState::new(
                session_id.clone(),
                config.clone(),
            )));
            let reusable = Arc::new(AtomicBool::new(true));
            let active = Arc::new(Mutex::new(None));
            let cancellation = Arc::new(SessionCancellation::new(
                worker.clone(),
                Arc::clone(&active),
                Arc::clone(&reusable),
            ));
            Ok(Box::new(OpenAiRealtimeSession {
                request_id: request.request_id().clone(),
                session_id,
                config,
                services,
                worker,
                worker_work: Some(worker_work),
                updates: Arc::new(Mutex::new(Some(updates))),
                access: Some(access),
                state,
                reusable,
                next_event_sequence: Arc::new(AtomicU64::new(1)),
                active,
                cancellation,
                turn_index: 0,
                next_append_event: 1,
                deadline: request.deadline(),
            }) as Box<dyn RealtimeMediaSessionHandle>)
        })
    }
}

async fn configure(
    worker: &WorkerHandle,
    updates: &mut mpsc::Receiver<WorkerUpdate>,
) -> Result<(), RuntimeFailure> {
    expect_session(update(updates).await?)?;
    worker.send(ClientEvent::SessionUpdate.to_json()).await?;
    expect_session(update(updates).await?)
}

fn expect_session(update: WorkerUpdate) -> Result<(), RuntimeFailure> {
    match update {
        WorkerUpdate::Event(RealtimeServerEvent::SessionConfigured) => Ok(()),
        WorkerUpdate::Event(_) => Err(failure(
            "swallowtail.openai.realtime_session_order_invalid",
            "OpenAI Realtime session handshake ordering was invalid",
        )),
        WorkerUpdate::Failed(error) => Err(error),
        WorkerUpdate::Disconnected => Err(disconnected()),
    }
}

pub(super) async fn update(
    updates: &mut mpsc::Receiver<WorkerUpdate>,
) -> Result<WorkerUpdate, RuntimeFailure> {
    poll_fn(|context| Pin::new(&mut *updates).poll_next(context))
        .await
        .ok_or_else(disconnected)
}

impl OpenAiRealtimeSession {
    pub(super) async fn close_inner(&mut self) -> CleanupOutcome {
        self.reusable
            .store(false, std::sync::atomic::Ordering::SeqCst);
        self.state
            .lock()
            .expect("media state lock poisoned")
            .close();
        let active = {
            self.active
                .lock()
                .expect("active response lock poisoned")
                .as_ref()
                .map(|active| Arc::clone(&active.cancellation))
        };
        if let Some(active) = active {
            let _ = swallowtail_runtime::CancellationControl::request(active.as_ref()).await;
        }
        let connection = cleanup(self.worker.close().await);
        let work = match self.worker_work.take() {
            Some(work) => cleanup(work.await),
            None => CleanupOutcome::NotApplicable,
        };
        let response = super::lifecycle::join_active(&self.active).await;
        let credential = match self.access.as_mut() {
            Some(access) => access.release(&self.services).await,
            None => CleanupOutcome::NotApplicable,
        };
        merge(merge(merge(response, connection), work), credential)
    }
}

fn invalid_scope() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_identity_invalid",
        "OpenAI Realtime runtime identity was invalid",
    )
}

fn disconnected() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_disconnected",
        "OpenAI Realtime connection ended before session configuration",
    )
}
