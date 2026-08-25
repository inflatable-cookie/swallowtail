struct TurnWork {
    request: DirectContinuationTurnRequest,
    attempt: DirectInferenceAttempt,
    wire: Request,
    credential: SecretBytes,
    call_sender: Option<mpsc::Sender<Result<DirectToolCall, RuntimeFailure>>>,
    submitter: Option<Arc<ResultSubmitter>>,
    result_receiver: Option<oneshot::Receiver<Vec<DirectToolResult>>>,
}

struct TurnContext {
    plan: PreflightPlan,
    scope: swallowtail_runtime::ScopeId,
    services: HostServices,
    transport: crate::transport::CurlTransport,
    endpoint: String,
    tools: Arc<Vec<ToolSpec>>,
    reasoning: Option<swallowtail_core::ReasoningMode>,
    thinking: Option<crate::AnthropicThinkingMode>,
    state: Arc<Mutex<DirectContinuationState>>,
    history: Arc<Mutex<History>>,
    usable: Arc<AtomicBool>,
    cancelled: Arc<AtomicBool>,
    cancel_receiver: oneshot::Receiver<()>,
}

async fn run_turn(
    mut work: TurnWork,
    mut context: TurnContext,
    events: RuntimeEventSender,
    cancellation: Arc<TurnCancellation>,
) -> TerminalOutcome {
    let mut sequence = 1;
    let activity = crate::activity::AnthropicActivityProjection::new(
        swallowtail_runtime::ActivityOperationId::Turn(work.request.turn_id().clone()),
    );
    let mut deadline = context
        .services
        .time()
        .expect("validated time")
        .wait_until(work.request.deadline());
    let result = run_attempt(
        &work.attempt,
        work.wire.clone(),
        &work,
        &mut context,
        &events,
        &mut sequence,
        &mut deadline,
        &cancellation,
        &activity,
    )
    .await;
    let result = match result {
        Ok(AttemptOutcome::Tool { call, private }) if work.attempt.ordinal().get() == 1 => {
            if !context
                .tools
                .iter()
                .any(|tool| tool.name == call.tool_name())
            {
                return provider_terminal(
                    failure(
                        "swallowtail.anthropic.tool_undeclared",
                        "Anthropic returned an undeclared consumer tool",
                    ),
                    &context,
                );
            }
            let pause = context
                .state
                .lock()
                .expect("continuation state lock poisoned")
                .pause_for_tool_calls(&work.attempt, std::slice::from_ref(&call));
            if let Err(error) = pause {
                Err(TurnFailure::Runtime(error, CleanupOutcome::Clean))
            } else {
                async {
                context
                    .history
                    .lock()
                    .expect("history lock poisoned")
                    .record_tool(&work.request, &call, private)
                    .and_then(|_| {
                        work.submitter
                            .as_ref()
                            .expect("first attempt has submitter")
                            .open(call.call_id().clone())
                    })
                    .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))
                    .and_then(|_| {
                        work.call_sender
                            .as_mut()
                            .expect("first attempt has call sender")
                            .try_send(Ok(call.clone()))
                            .map_err(|_| {
                                TurnFailure::Runtime(
                                    failure(
                                        "swallowtail.anthropic.tool_channel_failed",
                                        "Anthropic tool-call channel could not deliver the pending call",
                                    ),
                                    CleanupOutcome::Clean,
                                )
                            })
                    })
                    .map(|_| ())
                    .and_then(|_| {
                        emit(
                            &events,
                            &mut sequence,
                            RuntimeEventKind::DirectToolCallAvailable(call.call_id().clone()),
                        )
                        .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))
                    })?;
                work.call_sender.take();
                match wait_results(
                    work.result_receiver
                        .as_mut()
                        .expect("first attempt has result receiver"),
                    &mut context.cancel_receiver,
                    &mut deadline,
                    &cancellation,
                )
                .await
                {
                    Err(stop) => Err(TurnFailure::Stopped(stop, CleanupOutcome::Clean)),
                    Ok(results) => {
                        let maximum_result = context
                            .plan
                            .requirements()
                            .direct_continuation()
                            .expect("validated continuation")
                            .config()
                            .maximum_tool_result_bytes()
                            .get() as usize;
                        if results
                            .iter()
                            .any(|result| result.content().byte_len() > maximum_result)
                        {
                            return Err(TurnFailure::Runtime(
                                failure(
                                    "swallowtail.anthropic.tool_result_exceeded",
                                    "Anthropic tool result exceeded the selected bound",
                                ),
                                CleanupOutcome::Clean,
                            ));
                        }
                        let next = context
                            .state
                            .lock()
                            .expect("continuation state lock poisoned")
                            .authorize_tool_results(&results)
                            .map_err(|error| {
                                TurnFailure::Runtime(error, CleanupOutcome::Clean)
                            })?;
                        context
                            .history
                            .lock()
                            .expect("history lock poisoned")
                            .record_result(results.first().ok_or_else(|| {
                                TurnFailure::Runtime(
                                    failure(
                                        "swallowtail.anthropic.tool_result_missing",
                                        "Anthropic tool result exchange returned no result",
                                    ),
                                    CleanupOutcome::Clean,
                                )
                            })?)
                            .map_err(|error| {
                                TurnFailure::Runtime(error, CleanupOutcome::Clean)
                            })?;
                        let messages = context
                            .history
                            .lock()
                            .expect("history lock poisoned")
                            .continuation_messages()
                            .map_err(|error| {
                                TurnFailure::Runtime(error, CleanupOutcome::Clean)
                            })?;
                        let wire = Request::direct_message(
                            context.plan.model_id().expect("validated model").as_str(),
                            messages,
                            &context.tools,
                            context
                                .plan
                                .requirements()
                                .direct_continuation()
                                .expect("validated continuation")
                                .config()
                                .maximum_output_tokens_per_attempt()
                                .get(),
                            context.reasoning.as_ref(),
                            context.thinking,
                        )
                        .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))?;
                        context.cancelled.store(false, Ordering::SeqCst);
                        run_attempt(
                            &next,
                            wire,
                            &work,
                            &mut context,
                            &events,
                            &mut sequence,
                            &mut deadline,
                            &cancellation,
                            &activity,
                        )
                        .await
                    }
                }
                }
                .await
            }
        }
        other => other,
    };
    // A provider tool call outside the qualified direct-tool exchange is a
    // turn failure: the exchange must not stay open, and the terminal match
    // below owns the single conversion to the provider failure outcome.
    let tool_failure = matches!(&result, Ok(AttemptOutcome::Tool { .. }));
    if (result.is_err() || tool_failure)
        && let Some(submitter) = work.submitter.as_ref()
    {
        submitter.abandon();
    }
    match result {
        Ok(AttemptOutcome::Final(output)) => {
            let completed = {
                let mut history = context.history.lock().expect("history lock poisoned");
                if work.attempt.ordinal().get() == 1 {
                    history.record_answer(&output)
                } else {
                    Ok(())
                }
            }
            .and_then(|_| {
                context
                    .state
                    .lock()
                    .expect("continuation state lock poisoned")
                    .complete_turn()
            });
            if let Err(error) = completed {
                context.usable.store(false, Ordering::SeqCst);
                return TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    CleanupOutcome::Clean,
                );
            }
            TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean)
                .with_output(OperationContent::new(output).expect("final output is nonempty"))
        }
        Ok(AttemptOutcome::Tool { .. }) => {
            invalidate(&context);
            TerminalOutcome::new(
                TerminalStatus::ProviderFailed(
                    failure(
                        "swallowtail.anthropic.tool_call_unexpected",
                        "Anthropic returned a tool call outside the qualified exchange",
                    )
                    .diagnostic()
                    .clone(),
                ),
                CleanupOutcome::Clean,
            )
        }
        Err(TurnFailure::Stopped(stop, cleanup)) => {
            invalidate(&context);
            TerminalOutcome::new(
                if stop == StopSignal::TimedOut {
                    TerminalStatus::TimedOut
                } else {
                    TerminalStatus::Cancelled
                },
                cleanup,
            )
        }
        Err(TurnFailure::Provider(error, cleanup)) => {
            invalidate(&context);
            TerminalOutcome::new(
                TerminalStatus::ProviderFailed(error.diagnostic().clone()),
                cleanup,
            )
        }
        Err(TurnFailure::Runtime(error, cleanup)) => {
            invalidate(&context);
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup,
            )
        }
    }
}
