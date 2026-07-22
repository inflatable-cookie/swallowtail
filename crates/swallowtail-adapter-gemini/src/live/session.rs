use super::GeminiLiveDriver;
use super::access::AccessLeases;
use super::lifecycle::{
    ActiveSlot, ConnectionRegistry, SessionCancellation, cleanup, join_active, merge,
};
use super::worker::{WorkerHandle, WorkerUpdate};
use crate::failure::failure;
use crate::live_protocol::{ClientFrame, RolloverState, ServerEvent};
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

mod rollover;

pub(super) struct GeminiLiveSession {
    pub(super) request_id: RequestId,
    pub(super) session_id: RuntimeSessionId,
    pub(super) scope: ScopeId,
    pub(super) config: swallowtail_core::RealtimeMediaConfig,
    pub(super) services: HostServices,
    pub(super) worker: WorkerHandle,
    pub(super) worker_work: Option<BoxFuture<'static, Result<(), RuntimeFailure>>>,
    pub(super) updates: Arc<Mutex<Option<mpsc::Receiver<WorkerUpdate>>>>,
    pub(super) access: Option<AccessLeases>,
    pub(super) state: Arc<Mutex<RealtimeMediaSessionState>>,
    pub(super) rollover: Arc<Mutex<RolloverState>>,
    pub(super) reusable: Arc<AtomicBool>,
    pub(super) next_event_sequence: Arc<AtomicU64>,
    pub(super) active: ActiveSlot,
    pub(super) connections: ConnectionRegistry,
    pub(super) cancellation: Arc<SessionCancellation>,
    pub(super) turn_index: u32,
    pub(super) activity_open: bool,
    pub(super) deadline: Option<swallowtail_runtime::Deadline>,
    historical_cleanup: CleanupOutcome,
}

impl RealtimeMediaSessionDriver for GeminiLiveDriver {
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
                "gemini-live:session:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| invalid_identity())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let (worker, worker_work) = match access.connect(scope.clone(), &services).await {
                Ok(connection) => connection,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let connections = ConnectionRegistry::default();
            connections.add(worker.clone());
            let mut updates = worker.take_updates().expect("new worker owns updates");
            if let Err(error) = configure(
                &worker,
                &mut updates,
                ClientFrame::Setup { handle: None }.to_json(),
            )
            .await
            {
                worker.abort();
                let _ = worker_work.await;
                let _ = access.release(&services).await;
                return Err(error);
            }
            let session_id =
                RuntimeSessionId::new(format!("gemini-live:{}", request.request_id().as_str()))
                    .map_err(|_| invalid_identity())?;
            let config = request.config().clone();
            let state = Arc::new(Mutex::new(RealtimeMediaSessionState::new(
                session_id.clone(),
                config.clone(),
            )));
            let reusable = Arc::new(AtomicBool::new(true));
            let active = Arc::new(Mutex::new(None));
            let cancellation = Arc::new(SessionCancellation::new(
                connections.clone(),
                Arc::clone(&active),
                Arc::clone(&reusable),
            ));
            Ok(Box::new(GeminiLiveSession {
                request_id: request.request_id().clone(),
                session_id,
                scope,
                config,
                services,
                worker,
                worker_work: Some(worker_work),
                updates: Arc::new(Mutex::new(Some(updates))),
                access: Some(access),
                state,
                rollover: Arc::new(Mutex::new(RolloverState::default())),
                reusable,
                next_event_sequence: Arc::new(AtomicU64::new(1)),
                active,
                connections,
                cancellation,
                turn_index: 0,
                activity_open: false,
                deadline: request.deadline(),
                historical_cleanup: CleanupOutcome::NotApplicable,
            }) as Box<dyn RealtimeMediaSessionHandle>)
        })
    }
}

async fn configure(
    worker: &WorkerHandle,
    updates: &mut mpsc::Receiver<WorkerUpdate>,
    setup: serde_json::Value,
) -> Result<(), RuntimeFailure> {
    worker.send(setup).await?;
    match next_update(updates).await? {
        WorkerUpdate::Event(ServerEvent::SetupComplete) => Ok(()),
        WorkerUpdate::Event(_) => Err(failure(
            "swallowtail.gemini.live_setup_order_invalid",
            "Gemini Live setup confirmation ordering was invalid",
        )),
        WorkerUpdate::Failed(error) => Err(error),
        WorkerUpdate::Disconnected => Err(disconnected()),
    }
}

pub(super) async fn next_update(
    updates: &mut mpsc::Receiver<WorkerUpdate>,
) -> Result<WorkerUpdate, RuntimeFailure> {
    poll_fn(|context| Pin::new(&mut *updates).poll_next(context))
        .await
        .ok_or_else(disconnected)
}

impl GeminiLiveSession {
    pub(super) async fn close_inner(&mut self) -> CleanupOutcome {
        self.reusable
            .store(false, std::sync::atomic::Ordering::SeqCst);
        self.state
            .lock()
            .expect("media state lock poisoned")
            .close();
        self.connections.abort_all();
        let response = join_active(&self.active).await;
        let connection = cleanup(self.worker.close().await);
        let work = match self.worker_work.take() {
            Some(work) => cleanup(work.await),
            None => CleanupOutcome::NotApplicable,
        };
        self.connections.remove(&self.worker);
        self.rollover
            .lock()
            .expect("rollover state poisoned")
            .clear();
        let credential = match self.access.as_mut() {
            Some(access) => access.release(&self.services).await,
            None => CleanupOutcome::NotApplicable,
        };
        merge(
            self.historical_cleanup.clone(),
            merge(response, merge(connection, merge(work, credential))),
        )
    }
}

fn invalid_identity() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.live_identity_invalid",
        "Gemini Live runtime identity was invalid",
    )
}

fn disconnected() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.live_disconnected",
        "Gemini Live connection ended before setup completed",
    )
}
