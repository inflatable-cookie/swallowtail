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

/// Exact provider acknowledgement observed while the session was configured.
pub(crate) enum RealtimeAcknowledgement {
    /// The session-start request selected no reasoning effort.
    NotRequested,
    /// The provider acknowledged exactly the selected reasoning effort.
    Effective(String),
}

/// Exact rejection observed while the session was configured.
pub(crate) struct RealtimeOpenRejection {
    failure: RuntimeFailure,
    rejected_effort: Option<String>,
}

impl RealtimeOpenRejection {
    const fn unknown(failure: RuntimeFailure) -> Self {
        Self {
            failure,
            rejected_effort: None,
        }
    }

    /// Returns the exact well-formed differing effort the provider acknowledged.
    pub(crate) fn rejected_effort(&self) -> Option<&str> {
        self.rejected_effort.as_deref()
    }

    pub(crate) fn into_failure(self) -> RuntimeFailure {
        self.failure
    }
}

/// Open Realtime session paired with the acknowledgement its setup observed.
pub(crate) type RealtimeOpenResult =
    Result<(Box<dyn RealtimeMediaSessionHandle>, RealtimeAcknowledgement), RealtimeOpenRejection>;

/// Opens one Realtime media connection and reports its exact acknowledgement.
///
/// Both public prepared open methods share this private lifecycle, so
/// transport, setup, `session.updated` validation, handle construction,
/// failure, and cleanup stay identical.
pub(crate) fn open_realtime_lifecycle(
    plan: PreflightPlan,
    request: OpenRealtimeMediaSessionRequest,
    services: HostServices,
) -> BoxFuture<'static, RealtimeOpenResult> {
    Box::pin(async move {
        services
            .require_execution_host(plan.execution_host_id())
            .map_err(RealtimeOpenRejection::unknown)?;
        OpenAiRealtimeDriver::validate(&plan, &request, &services)
            .map_err(RealtimeOpenRejection::unknown)?;
        open_configured_session(plan, request, services).await
    })
}

impl RealtimeMediaSessionDriver for OpenAiRealtimeDriver {
    fn open_realtime_media_session(
        &self,
        plan: PreflightPlan,
        request: OpenRealtimeMediaSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RealtimeMediaSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            open_realtime_lifecycle(plan, request, services)
                .await
                .map(|(handle, _)| handle)
                .map_err(RealtimeOpenRejection::into_failure)
        })
    }
}

async fn open_configured_session(
    plan: PreflightPlan,
    request: OpenRealtimeMediaSessionRequest,
    services: HostServices,
) -> RealtimeOpenResult {
    let scope = ScopeId::new(format!(
        "openai-realtime:session:{}",
        request.request_id().as_str()
    ))
    .map_err(|_| RealtimeOpenRejection::unknown(invalid_scope()))?;
    let mut access = AccessLeases::acquire(&plan, scope.clone(), &services)
        .await
        .map_err(RealtimeOpenRejection::unknown)?;
    let (worker, worker_work) = match access.connect(scope.clone(), &services).await {
        Ok(connection) => connection,
        Err(error) => {
            let _ = access.release(&services).await;
            return Err(RealtimeOpenRejection::unknown(error));
        }
    };
    let mut updates = worker.take_updates().expect("new worker owns updates");
    let configured = configure(
        &worker,
        &mut updates,
        request.maximum_output_tokens(),
        request
            .reasoning_mode()
            .and_then(crate::realtime_reasoning::session_effort),
    )
    .await;
    let acknowledgement = match configured {
        Ok(acknowledgement) => acknowledgement,
        Err(rejection) => {
            let _ = worker.close().await;
            let _ = worker_work.await;
            let _ = access.release(&services).await;
            return Err(rejection);
        }
    };
    let session_id =
        RuntimeSessionId::new(format!("openai-realtime:{}", request.request_id().as_str()))
            .map_err(|_| RealtimeOpenRejection::unknown(invalid_scope()))?;
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
    let handle = Box::new(OpenAiRealtimeSession {
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
    }) as Box<dyn RealtimeMediaSessionHandle>;
    Ok((handle, acknowledgement))
}

async fn configure(
    worker: &WorkerHandle,
    updates: &mut mpsc::Receiver<WorkerUpdate>,
    maximum_output_tokens: Option<std::num::NonZeroU64>,
    reasoning_effort: Option<&str>,
) -> Result<RealtimeAcknowledgement, RealtimeOpenRejection> {
    expect_created(next_update(updates).await?)?;
    worker
        .send(
            ClientEvent::SessionUpdate {
                maximum_output_tokens,
                reasoning_effort,
            }
            .to_json(),
        )
        .await
        .map_err(RealtimeOpenRejection::unknown)?;
    expect_updated(next_update(updates).await?, reasoning_effort)
}

async fn next_update(
    updates: &mut mpsc::Receiver<WorkerUpdate>,
) -> Result<WorkerUpdate, RealtimeOpenRejection> {
    update(updates)
        .await
        .map_err(RealtimeOpenRejection::unknown)
}

fn expect_created(update: WorkerUpdate) -> Result<(), RealtimeOpenRejection> {
    match update {
        WorkerUpdate::Event(RealtimeServerEvent::SessionCreated) => Ok(()),
        WorkerUpdate::Event(_) => Err(RealtimeOpenRejection::unknown(order_invalid())),
        WorkerUpdate::Failed(error) => Err(RealtimeOpenRejection::unknown(error)),
        WorkerUpdate::Disconnected => Err(RealtimeOpenRejection::unknown(disconnected())),
    }
}

/// Classifies the exact `session.updated` reasoning acknowledgement.
///
/// Only a matching effort proves provider-effective reasoning. Only an exact,
/// well-formed differing effort carries a rejected state. Missing, malformed,
/// out-of-order, transport, timeout, and disconnect evidence carries none.
fn expect_updated(
    update: WorkerUpdate,
    expected_effort: Option<&str>,
) -> Result<RealtimeAcknowledgement, RealtimeOpenRejection> {
    match update {
        WorkerUpdate::Event(RealtimeServerEvent::SessionUpdated { reasoning }) => {
            let Some(wanted) = expected_effort else {
                return Ok(RealtimeAcknowledgement::NotRequested);
            };
            match reasoning {
                crate::realtime_protocol::SessionReasoningAck::Effort(got) if got == wanted => {
                    Ok(RealtimeAcknowledgement::Effective(got))
                }
                crate::realtime_protocol::SessionReasoningAck::Effort(got)
                    if crate::realtime_reasoning::is_session_effort(&got) =>
                {
                    Err(RealtimeOpenRejection {
                        failure: acknowledgement_invalid(),
                        rejected_effort: Some(got),
                    })
                }
                _ => Err(RealtimeOpenRejection::unknown(acknowledgement_invalid())),
            }
        }
        WorkerUpdate::Event(_) => Err(RealtimeOpenRejection::unknown(order_invalid())),
        WorkerUpdate::Failed(error) => Err(RealtimeOpenRejection::unknown(error)),
        WorkerUpdate::Disconnected => Err(RealtimeOpenRejection::unknown(disconnected())),
    }
}

fn order_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_session_order_invalid",
        "OpenAI Realtime session handshake ordering was invalid",
    )
}

fn acknowledgement_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_reasoning_acknowledgement_invalid",
        "OpenAI Realtime session reasoning acknowledgement did not match the selected effort",
    )
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
