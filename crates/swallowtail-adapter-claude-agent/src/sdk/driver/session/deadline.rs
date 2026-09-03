//! Host-deadline work for one turn: arming the bound, and reaping a finished
//! turn's task before the next turn may start.

use super::ActiveSlot;
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use crate::sdk::turn::SdkActiveTurn;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::json;
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::task::Poll;
use swallowtail_runtime::{Deadline, HostServices, JoinedTask, RuntimeFailure, ScopeId};

/// Drops a finished turn and joins its host-deadline task before a new turn
/// may start, so deadline work never outlives the turn it bounds.
/// Spawns the host-deadline task for one turn. Expiry interrupts provider work
/// through the SDK's own control surface and marks the turn timed out; it
/// never claims the provider stopped.
pub(super) fn spawn_turn_deadline(
    services: &HostServices,
    connection: Arc<SdkConnection>,
    turn: Arc<SdkActiveTurn>,
    deadline: Deadline,
) -> Result<(Box<dyn JoinedTask>, ScopeId), RuntimeFailure> {
    let mut wait = services
        .time()
        .cloned()
        .expect("validated sidecar time service")
        .wait_until(deadline);
    let mut finished = Box::pin(turn.finished_future());
    let scope = ScopeId::new(format!(
        "claude-agent-sdk:deadline:{}",
        turn.runtime_id().as_str()
    ))
    .map_err(|_| {
        failure(
            "swallowtail.claude-agent.sdk.scope_invalid",
            "Claude Agent SDK sidecar turn scope was invalid",
        )
    })?;
    let spawn_scope = scope.clone();
    services
        .task()
        .expect("validated sidecar task service")
        .spawn(
            spawn_scope,
            Box::pin(async move {
                let timed_out = poll_fn(|context| {
                    if finished.as_mut().poll(context).is_ready() {
                        Poll::Ready(false)
                    } else if wait.as_mut().poll(context).is_ready() {
                        Poll::Ready(true)
                    } else {
                        Poll::Pending
                    }
                })
                .await;
                if timed_out {
                    // The terminal outcome is completed first and never waits
                    // on a receipt. An accepted turn whose sidecar stops
                    // answering still resolves at its deadline; the interrupt
                    // stays a request, and bounded session close owns the
                    // descendant termination.
                    turn.mark_timed_out();
                    turn.fail_connection(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.claude-agent.sdk.turn_deadline_elapsed",
                        "Claude Agent SDK sidecar turn reached its host deadline",
                    ));
                    let id = format!("deadline-interrupt:{}", turn.runtime_id().as_str());
                    let _ = connection
                        .send(id, ClaudeAgentSdkCommand::Interrupt, json!({}))
                        .await;
                }
            }),
        )
        .map(|task| (task, scope))
}

pub(super) async fn reap_finished(
    active: &ActiveSlot,
    services: &HostServices,
    execution_host_id: &swallowtail_core::ExecutionHostId,
    deadline: Deadline,
) {
    let bounded = crate::sdk::bounded::HostBound::new(
        services
            .time()
            .cloned()
            .expect("validated sidecar time service"),
        deadline,
    );
    let finished = {
        let mut active = active.lock().expect("SDK sidecar active lock poisoned");
        if active
            .as_ref()
            .is_some_and(|active| active.turn.is_finished())
        {
            active.take()
        } else {
            None
        }
    };
    if let Some(mut active) = finished
        && let Some(task) = active.deadline_task.take()
    {
        // Bounded by the incoming turn's own deadline: joining a task that has
        // not finished would otherwise block this thread inside the join on
        // hosts whose handles own their worker.
        let owner = crate::sdk::guardian::TaskOwner::new(
            services,
            execution_host_id,
            &active.deadline_scope,
        );
        let _ = crate::sdk::guardian::bounded_join(&bounded, &owner, task).await;
    }
}
