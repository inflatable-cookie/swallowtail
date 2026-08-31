use super::worker::{WorkerHandle, WorkerUpdate};
use crate::failure::failure;
use crate::realtime_protocol::{ClientEvent, RealtimeServerEvent};
use futures_channel::mpsc;
use futures_core::Stream;
use std::future::poll_fn;
use std::pin::Pin;
use swallowtail_runtime::RuntimeFailure;

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
    pub(super) const fn unknown(failure: RuntimeFailure) -> Self {
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

pub(super) async fn configure(
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

fn disconnected() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_disconnected",
        "OpenAI Realtime connection ended before session configuration",
    )
}
