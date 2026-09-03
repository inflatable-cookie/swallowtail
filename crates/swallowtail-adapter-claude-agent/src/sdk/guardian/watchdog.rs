//! The close guard: a host task that makes the declared descendant
//! termination request even when a cooperative close stage never answers.

use super::{Signal, bounded_join};
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_runtime::{Deadline, HostServices, JoinedTask, RuntimeFailure, ScopeId};

/// Guards close: the host termination request happens on the caller's deadline
/// even if a cooperative stage never answers.
pub(crate) struct EscalationWatchdog {
    signal: Arc<Signal>,
    task: Mutex<Option<Box<dyn JoinedTask>>>,
}

impl EscalationWatchdog {
    /// Arms before the cooperative close stages, so no stalled stage can skip
    /// the termination request.
    pub(crate) fn arm(
        services: &HostServices,
        connection: Arc<SdkConnection>,
        request_id: &str,
        deadline: Deadline,
    ) -> Result<Self, RuntimeFailure> {
        let signal = Arc::new(Signal::default());
        let scope =
            ScopeId::new(format!("claude-agent-sdk:close-guard:{request_id}")).map_err(|_| {
                failure(
                    "swallowtail.claude-agent.sdk.scope_invalid",
                    "Claude Agent SDK sidecar close-guard scope was invalid",
                )
            })?;
        let time = services
            .time()
            .cloned()
            .expect("validated sidecar time service");
        let task_signal = Arc::clone(&signal);
        let task = services
            .task()
            .expect("validated sidecar task service")
            .spawn(
                scope,
                Box::pin(async move {
                    let mut expiry = time.wait_until(deadline);
                    let mut fired = Box::pin(task_signal.future());
                    std::future::poll_fn(|context| {
                        if fired.as_mut().poll(context).is_ready()
                            || expiry.as_mut().poll(context).is_ready()
                        {
                            Poll::Ready(())
                        } else {
                            Poll::Pending
                        }
                    })
                    .await;
                    // Unconditional: this is the declared descendant
                    // termination attempt, and it is a request, not a join.
                    let _ = connection.escalate().await;
                }),
            )?;
        Ok(Self {
            signal,
            task: Mutex::new(Some(task)),
        })
    }

    /// Asks for the termination request now, then joins the guard task while
    /// the caller's bound allows.
    pub(crate) async fn terminate(&self, bounded: &crate::sdk::bounded::HostBound) -> bool {
        self.signal.trigger();
        let task = self
            .task
            .lock()
            .expect("SDK close-guard task lock poisoned")
            .take();
        match task {
            Some(task) => bounded_join(bounded, task).await,
            None => false,
        }
    }
}
