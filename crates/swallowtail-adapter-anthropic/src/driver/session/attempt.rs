fn provider_terminal(error: RuntimeFailure, context: &TurnContext) -> TerminalOutcome {
    invalidate(context);
    TerminalOutcome::new(
        TerminalStatus::ProviderFailed(error.diagnostic().clone()),
        CleanupOutcome::Clean,
    )
}

#[allow(clippy::too_many_arguments)]
async fn run_attempt(
    attempt: &DirectInferenceAttempt,
    wire: Request,
    work: &TurnWork,
    context: &mut TurnContext,
    events: &RuntimeEventSender,
    sequence: &mut u64,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &TurnCancellation,
    activity: &crate::activity::AnthropicActivityProjection,
) -> Result<AttemptOutcome, TurnFailure> {
    let mut subscription = context
        .transport
        .subscribe(
            context.scope.clone(),
            context.endpoint.clone(),
            work.credential.copy(),
            wire,
            &context.services,
            Arc::clone(&context.cancelled),
        )
        .map_err(|error| TurnFailure::Provider(error, CleanupOutcome::NotApplicable))?;
    let mut parser = AttemptParser::new(
        attempt.attempt_id().clone(),
        context
            .plan
            .requirements()
            .direct_continuation()
            .expect("validated continuation")
            .config()
            .maximum_tool_argument_bytes()
            .get() as usize,
    );
    loop {
        match next_signal(
            &mut subscription,
            &mut context.cancel_receiver,
            deadline,
            cancellation,
        )
        .await
        {
            StreamSignal::Item(Ok(StreamItem::Headers(headers))) => {
                emit_request(events, sequence, &headers)
                    .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))?;
            }
            StreamSignal::Item(Ok(StreamItem::Frame(frame))) => {
                let event = parse_event(&frame)
                    .map_err(|error| TurnFailure::Provider(error, CleanupOutcome::Clean))?;
                parser
                    .apply(event, events, sequence, activity)
                    .map_err(|error| TurnFailure::Provider(error, CleanupOutcome::Clean))?;
            }
            StreamSignal::Item(Err(error)) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(if cancellation.is_requested() {
                    TurnFailure::Stopped(cancellation.stop_reason(), cleanup)
                } else {
                    TurnFailure::Provider(error, cleanup)
                });
            }
            StreamSignal::Closed => {
                let cleanup = cleanup_result(subscription.close().await);
                return parser
                    .finish()
                    .map_err(|error| TurnFailure::Provider(error, cleanup));
            }
            StreamSignal::Stopped(stop) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(TurnFailure::Stopped(stop, cleanup));
            }
        }
    }
}

enum AttemptOutcome {
    Tool(DirectToolCall),
    Final(String),
}

struct AttemptParser {
    attempt_id: swallowtail_runtime::DirectInferenceAttemptId,
    maximum_arguments: usize,
    started: bool,
    message_id: Option<String>,
    assistant_phase: Option<swallowtail_runtime::ActivityAssistantPhase>,
    active: Option<ContentBlock>,
    tool_id: Option<String>,
    tool_name: Option<String>,
    arguments: String,
    output: String,
    stop_reason: Option<String>,
    stopped: bool,
}

impl AttemptParser {
    fn new(
        attempt_id: swallowtail_runtime::DirectInferenceAttemptId,
        maximum_arguments: usize,
    ) -> Self {
        Self {
            attempt_id,
            maximum_arguments,
            started: false,
            message_id: None,
            assistant_phase: None,
            active: None,
            tool_id: None,
            tool_name: None,
            arguments: String::new(),
            output: String::new(),
            stop_reason: None,
            stopped: false,
        }
    }

    fn apply(
        &mut self,
        event: Event,
        events: &RuntimeEventSender,
        sequence: &mut u64,
        activity: &crate::activity::AnthropicActivityProjection,
    ) -> Result<(), RuntimeFailure> {
        match event {
            Event::Unknown => Ok(()),
            Event::Ping => emit(events, sequence, RuntimeEventKind::Keepalive),
            Event::MessageStart { id, usage } if !self.started => {
                self.started = true;
                self.message_id = Some(id);
                emit_attempt_usage(events, sequence, &self.attempt_id, usage)
            }
            Event::ContentStart(block) if self.started && self.active.is_none() => {
                if let ContentBlock::ToolUse { id, name } = &block {
                    self.tool_id = Some(id.clone());
                    self.tool_name = Some(name.clone());
                    self.assistant_phase =
                        Some(swallowtail_runtime::ActivityAssistantPhase::Intermediate);
                    let call_id = DirectToolCallId::new(id.clone()).map_err(|_| {
                        failure(
                            "swallowtail.anthropic.tool_call_invalid",
                            "Anthropic tool call identity was invalid",
                        )
                    })?;
                    emit(
                        events,
                        sequence,
                        RuntimeEventKind::Activity(activity.assistant_started(
                            crate::activity::attempt_assistant_id(&self.attempt_id)?,
                            self.message_id.as_deref().expect("message identity exists"),
                            swallowtail_runtime::ActivityAssistantPhase::Intermediate,
                        )?),
                    )?;
                    emit(
                        events,
                        sequence,
                        RuntimeEventKind::Activity(activity.consumer_tool(
                            &call_id,
                            swallowtail_runtime::ActivityLifecyclePhase::Started,
                            swallowtail_runtime::ActivityStatus::Pending,
                        )?),
                    )?;
                } else if block == ContentBlock::Text {
                    self.assistant_phase = Some(swallowtail_runtime::ActivityAssistantPhase::Final);
                    emit(
                        events,
                        sequence,
                        RuntimeEventKind::Activity(activity.assistant_started(
                            crate::activity::attempt_assistant_id(&self.attempt_id)?,
                            self.message_id.as_deref().expect("message identity exists"),
                            swallowtail_runtime::ActivityAssistantPhase::Final,
                        )?),
                    )?;
                }
                if matches!(
                    block,
                    ContentBlock::SearchUse { .. } | ContentBlock::SearchResult { .. }
                ) {
                    return Err(failure(
                        "swallowtail.anthropic.provider_tool_unexpected",
                        "Anthropic provider-owned search appeared in a consumer-tool session",
                    ));
                }
                self.active = Some(block);
                Ok(())
            }
            Event::OutputDelta(delta) if self.active == Some(ContentBlock::Text) => {
                self.output.push_str(&delta);
                emit(
                    events,
                    sequence,
                    RuntimeEventKind::Activity(activity.assistant_delta(
                        crate::activity::attempt_assistant_id(&self.attempt_id)?,
                        self.message_id.as_deref().expect("message identity exists"),
                        swallowtail_runtime::ActivityAssistantPhase::Final,
                        &delta,
                    )?),
                )?;
                emit_content(events, sequence, RuntimeEventKind::OutputDelta, delta)
            }
            Event::InputJsonDelta(delta)
                if matches!(self.active, Some(ContentBlock::ToolUse { .. })) =>
            {
                if self.arguments.len().saturating_add(delta.len()) > self.maximum_arguments {
                    return Err(failure(
                        "swallowtail.anthropic.tool_arguments_exceeded",
                        "Anthropic tool arguments exceeded the selected bound",
                    ));
                }
                self.arguments.push_str(&delta);
                let call_id =
                    DirectToolCallId::new(self.tool_id.clone().expect("tool identity exists"))
                        .map_err(|_| {
                            failure(
                                "swallowtail.anthropic.tool_call_invalid",
                                "Anthropic tool call identity was invalid",
                            )
                        })?;
                emit(
                    events,
                    sequence,
                    RuntimeEventKind::Activity(activity.consumer_tool(
                        &call_id,
                        swallowtail_runtime::ActivityLifecyclePhase::Updated,
                        swallowtail_runtime::ActivityStatus::InProgress,
                    )?),
                )?;
                Ok(())
            }
            Event::ContentStop if self.active.is_some() => {
                if matches!(self.active, Some(ContentBlock::ToolUse { .. })) {
                    let call_id =
                        DirectToolCallId::new(self.tool_id.clone().expect("tool identity exists"))
                            .map_err(|_| {
                                failure(
                                    "swallowtail.anthropic.tool_call_invalid",
                                    "Anthropic tool call identity was invalid",
                                )
                            })?;
                    emit(
                        events,
                        sequence,
                        RuntimeEventKind::Activity(activity.consumer_tool(
                            &call_id,
                            swallowtail_runtime::ActivityLifecyclePhase::Completed,
                            swallowtail_runtime::ActivityStatus::Completed,
                        )?),
                    )?;
                }
                self.active.take();
                Ok(())
            }
            Event::Usage(usage, reason) if self.active.is_none() => {
                self.stop_reason = Some(reason.clone());
                emit_attempt_usage(events, sequence, &self.attempt_id, usage)?;
                let finish = match reason.as_str() {
                    "tool_use" => return Ok(()),
                    "end_turn" | "stop_sequence" => ProviderFinishReason::Stop,
                    "max_tokens" => ProviderFinishReason::Length,
                    _ => {
                        return Err(failure(
                            "swallowtail.anthropic.finish_reason_invalid",
                            "Anthropic finish reason was not qualified",
                        ));
                    }
                };
                emit(
                    events,
                    sequence,
                    RuntimeEventKind::ProviderObservation(
                        ProviderObservation::DirectAttemptFinish(
                            DirectAttemptFinishObservation::new(self.attempt_id.clone(), finish),
                        ),
                    ),
                )
            }
            Event::MessageStop if self.stop_reason.is_some() => {
                let phase = self.assistant_phase.ok_or_else(|| {
                    failure(
                        "swallowtail.anthropic.activity_invalid",
                        "Anthropic message completed without a qualified assistant phase",
                    )
                })?;
                let output = (phase == swallowtail_runtime::ActivityAssistantPhase::Final)
                    .then_some(self.output.as_str());
                emit(
                    events,
                    sequence,
                    RuntimeEventKind::Activity(activity.assistant_completed(
                        crate::activity::attempt_assistant_id(&self.attempt_id)?,
                        self.message_id.as_deref().expect("message identity exists"),
                        phase,
                        output,
                    )?),
                )?;
                if let Some(output) = output {
                    emit_content(
                        events,
                        sequence,
                        RuntimeEventKind::OutputAvailable,
                        output.to_owned(),
                    )?;
                }
                self.stopped = true;
                Ok(())
            }
            Event::ProviderFailed(kind) => Err(provider_failure(kind, "message stream")),
            _ => Err(failure(
                "swallowtail.anthropic.stream_order_invalid",
                "Anthropic direct-continuation stream order was invalid",
            )),
        }
    }

    fn finish(self) -> Result<AttemptOutcome, RuntimeFailure> {
        if !self.stopped {
            return Err(failure(
                "swallowtail.anthropic.stream_disconnected",
                "Anthropic stream closed before message completion",
            ));
        }
        match self.stop_reason.as_deref() {
            Some("tool_use") if self.output.is_empty() => {
                let arguments =
                    DirectToolArguments::new(self.arguments.into_bytes(), self.maximum_arguments)
                        .map_err(|_| {
                        failure(
                            "swallowtail.anthropic.tool_arguments_exceeded",
                            "Anthropic tool arguments exceeded the selected bound",
                        )
                    })?;
                serde_json::from_slice::<serde_json::Value>(arguments.as_bytes()).map_err(
                    |_| {
                        failure(
                            "swallowtail.anthropic.tool_arguments_invalid",
                            "Anthropic tool arguments were not valid JSON",
                        )
                    },
                )?;
                Ok(AttemptOutcome::Tool(
                    DirectToolCall::new(
                        DirectToolCallId::new(self.tool_id.ok_or_else(|| {
                            failure(
                                "swallowtail.anthropic.tool_call_invalid",
                                "Anthropic tool call identity was missing",
                            )
                        })?)
                        .map_err(|_| {
                            failure(
                                "swallowtail.anthropic.tool_call_invalid",
                                "Anthropic tool call identity was invalid",
                            )
                        })?,
                        self.attempt_id,
                        self.tool_name.ok_or_else(|| {
                            failure(
                                "swallowtail.anthropic.tool_call_invalid",
                                "Anthropic tool call name was missing",
                            )
                        })?,
                        arguments,
                    )
                    .map_err(|_| {
                        failure(
                            "swallowtail.anthropic.tool_call_invalid",
                            "Anthropic tool call was invalid",
                        )
                    })?,
                ))
            }
            Some("end_turn" | "stop_sequence" | "max_tokens")
                if self.tool_id.is_none() && !self.output.is_empty() =>
            {
                Ok(AttemptOutcome::Final(self.output))
            }
            _ => Err(failure(
                "swallowtail.anthropic.attempt_semantics_invalid",
                "Anthropic attempt completion did not match the selected operation",
            )),
        }
    }
}

