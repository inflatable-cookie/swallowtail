use super::*;

pub(super) async fn close_provider_session(
    connection: &AcpConnection,
    provider_id: &str,
    negotiated_and_qualified: bool,
) -> CleanupOutcome {
    if !negotiated_and_qualified || connection.is_closed() {
        return CleanupOutcome::NotApplicable;
    }
    match connection.close_session(provider_id).await {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Degraded(error.diagnostic().clone()),
    }
}

pub(super) async fn join_connection(session: &mut ClaudeAgentSessionHandle) -> CleanupOutcome {
    match session.pump_task.take() {
        Some(task) => match task.join().await {
            Ok(()) => session.connection.cleanup_outcome(),
            Err(_) => cleanup_failure(
                "task_join_failed",
                "Claude Agent ACP protocol task did not join cleanly",
            ),
        },
        None => CleanupOutcome::NotApplicable,
    }
}

pub(super) async fn finish_cleanup(
    mut session: Box<ClaudeAgentSessionHandle>,
    native_close: CleanupOutcome,
    task: CleanupOutcome,
) -> CleanupOutcome {
    let resource = release_resource(session.resource.take(), &session.services).await;
    let credential = release_credential(session.credential.take(), &session.services).await;
    merge_cleanup(
        merge_cleanup(merge_cleanup(native_close, task), resource),
        credential,
    )
}

pub(in crate::driver) fn cleanup_failure(
    code: &'static str,
    message: &'static str,
) -> CleanupOutcome {
    CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.claude_agent.acp.cleanup_failed",
        format!("{message} ({code})"),
    ))
}

pub(in crate::driver) fn merge_cleanup(
    left: CleanupOutcome,
    right: CleanupOutcome,
) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(error), _) | (_, CleanupOutcome::Failed(error)) => {
            CleanupOutcome::Failed(error)
        }
        (CleanupOutcome::Degraded(error), _) | (_, CleanupOutcome::Degraded(error)) => {
            CleanupOutcome::Degraded(error)
        }
        (CleanupOutcome::Clean, CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
        | (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}
