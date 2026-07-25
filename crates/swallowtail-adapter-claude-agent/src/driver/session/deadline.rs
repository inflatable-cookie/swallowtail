use super::*;
use std::future::poll_fn;
use std::task::Poll;

pub(super) fn spawn_deadline(
    services: &HostServices,
    connection: Arc<AcpConnection>,
    turn: Arc<ActiveTurn>,
    deadline: Option<swallowtail_runtime::Deadline>,
) -> Result<Option<Box<dyn JoinedTask>>, RuntimeFailure> {
    let Some(deadline) = deadline else {
        return Ok(None);
    };
    let mut wait = services
        .time()
        .expect("validated time service")
        .wait_until(deadline);
    let mut finished = Box::pin(turn.finished_future());
    let deadline_turn = Arc::clone(&turn);
    let scope = ScopeId::new(format!(
        "claude-agent-acp:deadline:{}",
        turn.runtime_id().as_str()
    ))
    .map_err(|_| malformed())?;
    services
        .task()
        .expect("validated task service")
        .spawn(
            scope,
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
                    deadline_turn.timeout();
                    let _ = connection
                        .notify(
                            "session/cancel",
                            json!({"sessionId": deadline_turn.session_id()}),
                        )
                        .await;
                }
            }),
        )
        .map(Some)
}
