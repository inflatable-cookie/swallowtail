use crate::protocol::{BackgroundStatus, ResponseSnapshot, parse_snapshot, require_success};
use swallowtail_runtime::{ProviderCancellationOutcome, TerminalStatus, TokenUsage};

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
    mut deadline: Option<
        swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    >,
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
    let final_state = loop {
        let exit = pump_attachment(
            &mut subscription,
            &mut stream,
            &mut output,
            &mut output_done,
            &events,
            &mut sequence,
            &cancellation,
            &mut deadline,
        )
        .await;
        cleanup = merge_cleanup(cleanup, cleanup_result(subscription.close().await));
        match exit {
            AttachmentExit::Terminal(state) => break state,
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
    cleanup = merge_cleanup(cleanup, access.release(&services).await);
    let mut outcome = TerminalOutcome::new(final_state.status, cleanup);
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
    Terminal(FinalState),
    Cancelled,
    Deadline,
    Disconnected,
}

struct FinalState {
    status: TerminalStatus,
    cancellation: Option<ProviderCancellationOutcome>,
    output: Option<String>,
    usage: Option<TokenUsage>,
}

impl FinalState {
    fn new(status: TerminalStatus) -> Self {
        Self {
            status,
            cancellation: None,
            output: None,
            usage: None,
        }
    }
}

include!("pump/attachment.rs");
include!("pump/management.rs");
