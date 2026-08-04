use crate::protocol::{
    BackgroundStatus, ResponseSnapshot, parse_deletion, parse_snapshot, require_success,
};
use swallowtail_core::OwnedRemoteResourceKind;
use swallowtail_runtime::{
    ProviderCancellationOutcome, RemoteResourceDeletionOutcome, TerminalStatus, TokenUsage,
};

#[allow(clippy::too_many_arguments)]
async fn pump_run(
    transport: CurlTransport,
    scope: ScopeId,
    response_id: String,
    mut subscription: Subscription,
    mut stream: BackgroundStream,
    mut access: AccessLeases,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    mut sequence: u64,
    cancellation: Arc<RunCancellation>,
    detachment: Option<Arc<RunDetachment>>,
    mut deadline: Option<
        swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    >,
    activity: crate::activity::OpenAiBackgroundActivityProjection,
) -> TerminalOutcome {
    let endpoint = access.endpoint.clone();
    let credential = SecretMaterial(
        access
            .secret()
            .expect("validated credential remains available")
            .to_vec(),
    );
    let mut output = String::new();
    let mut output_done = None;
    let mut reattached = false;
    let mut cleanup = CleanupOutcome::NotApplicable;
    let mut final_state = loop {
        let exit = pump_attachment(
            &mut subscription,
            &mut stream,
            &mut output,
            &mut output_done,
            &events,
            &mut sequence,
            &cancellation,
            detachment.as_deref(),
            &mut deadline,
            &activity,
        )
        .await;
        cleanup = merge_cleanup(cleanup, cleanup_result(subscription.close().await));
        match exit {
            AttachmentExit::Terminal(state) => break *state,
            AttachmentExit::Cancelled => {
                break stop_remote(
                    &transport,
                    &scope,
                    &response_id,
                    &endpoint,
                    &credential,
                    &services,
                    &events,
                    &mut sequence,
                    LocalStop::Cancelled,
                )
                .await;
            }
            AttachmentExit::Deadline => {
                cancellation.stop_active();
                break stop_remote(
                    &transport,
                    &scope,
                    &response_id,
                    &endpoint,
                    &credential,
                    &services,
                    &events,
                    &mut sequence,
                    LocalStop::TimedOut,
                )
                .await;
            }
            AttachmentExit::Detached => break FinalState::detached(),
            AttachmentExit::Disconnected if !reattached => {
                reattached = true;
                match open_reattachment(
                    &transport,
                    &scope,
                    &response_id,
                    stream.last_sequence().expect("identity established a cursor"),
                    &endpoint,
                    &credential,
                    &services,
                    &cancellation,
                ) {
                    Ok(next) => subscription = next,
                    Err(_) => {
                        break retrieve_terminal(
                            &transport,
                            &scope,
                            &response_id,
                            &endpoint,
                            &credential,
                            &services,
                            &events,
                            &mut sequence,
                        )
                        .await;
                    }
                }
            }
            AttachmentExit::Disconnected => {
                break retrieve_terminal(
                    &transport,
                    &scope,
                    &response_id,
                    &endpoint,
                    &credential,
                    &services,
                    &events,
                    &mut sequence,
                )
                .await;
            }
        }
    };

    if let Some(usage) = final_state.usage {
        let kind = RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage));
        if emit(&events, &mut sequence, kind).is_err() {
            cleanup = merge_cleanup(
                cleanup,
                CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.openai.final_evidence_dropped",
                    "OpenAI final provider evidence could not be delivered",
                )),
            );
        }
    }
    if let Some(content) = final_state.output.as_ref() {
        let result = activity.completed(content).and_then(|observation| {
            emit(
                &events,
                &mut sequence,
                RuntimeEventKind::Activity(observation),
            )
        });
        if let Err(error) = result {
            cleanup = merge_cleanup(
                cleanup,
                CleanupOutcome::Degraded(error.diagnostic().clone()),
            );
            final_state.status = TerminalStatus::RuntimeFailed(error.diagnostic().clone());
        }
    }
    if let Some(content) = final_state.output.as_ref()
        && emit_content(
            &events,
            &mut sequence,
            RuntimeEventKind::OutputAvailable,
            content.clone(),
        )
        .is_err()
    {
        cleanup = merge_cleanup(
            cleanup,
            CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.openai.final_output_dropped",
                "OpenAI final output could not be delivered",
            )),
        );
    }
    let response_deletion = if matches!(final_state.status, TerminalStatus::Detached) {
        None
    } else {
        let (response_deletion, deletion_cleanup) = delete_response(
            &transport,
            &scope,
            &response_id,
            final_state.provider_terminal_known,
            &endpoint,
            &credential,
            &services,
        )
        .await;
        cleanup = merge_cleanup(cleanup, deletion_cleanup);
        Some(response_deletion)
    };
    cleanup = merge_cleanup(cleanup, access.release(&services).await);
    let mut outcome = TerminalOutcome::new(final_state.status, cleanup);
    if let Some(response_deletion) = response_deletion {
        outcome = outcome.with_remote_resource_deletion(
            OwnedRemoteResourceKind::Response,
            response_deletion,
        );
    }
    if let Some(cancellation) = final_state.cancellation {
        outcome = outcome.with_provider_cancellation(cancellation);
    }
    if let Some(output) = final_state.output
        && let Ok(output) = OperationContent::new(output)
    {
        outcome = outcome.with_output(output);
    }
    outcome
}

enum AttachmentExit {
    Terminal(Box<FinalState>),
    Cancelled,
    Deadline,
    Detached,
    Disconnected,
}

struct FinalState {
    status: TerminalStatus,
    provider_terminal_known: bool,
    cancellation: Option<ProviderCancellationOutcome>,
    output: Option<String>,
    usage: Option<TokenUsage>,
}

impl FinalState {
    fn new(status: TerminalStatus) -> Self {
        Self {
            status,
            provider_terminal_known: false,
            cancellation: None,
            output: None,
            usage: None,
        }
    }

    fn provider_terminal(status: TerminalStatus) -> Self {
        Self {
            provider_terminal_known: true,
            ..Self::new(status)
        }
    }

    fn detached() -> Self {
        Self::new(TerminalStatus::Detached)
    }
}

async fn delete_response(
    transport: &CurlTransport,
    scope: &ScopeId,
    response_id: &str,
    provider_terminal_known: bool,
    endpoint: &str,
    credential: &SecretMaterial,
    services: &HostServices,
) -> (RemoteResourceDeletionOutcome, CleanupOutcome) {
    let degraded = || {
        CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.openai.response_deletion_unconfirmed",
            "OpenAI response deletion could not be confirmed",
        ))
    };
    if !provider_terminal_known {
        return (RemoteResourceDeletionOutcome::Unconfirmed, degraded());
    }
    let result = async {
        let response = transport
            .request(
                scope.clone(),
                endpoint.to_owned(),
                credential.0.clone(),
                Request::delete(response_id)?,
                services,
                Arc::new(AtomicBool::new(false)),
            )
            .await?;
        require_success(&response)?;
        let deletion = parse_deletion(&response.body)?;
        if deletion.response_id != response_id || !deletion.deleted {
            return Err(failure(
                "swallowtail.openai.response_deletion_mismatch",
                "OpenAI response deletion acknowledgement did not match the active response",
            ));
        }
        Ok(())
    }
    .await;
    match result {
        Ok(()) => (
            RemoteResourceDeletionOutcome::Confirmed,
            CleanupOutcome::Clean,
        ),
        Err(_) => (RemoteResourceDeletionOutcome::Unconfirmed, degraded()),
    }
}

include!("pump/attachment.rs");
include!("pump/management.rs");
