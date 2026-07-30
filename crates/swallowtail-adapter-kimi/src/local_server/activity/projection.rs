impl KimiLocalActivityProjection {
    pub(super) fn new(turn_id: RuntimeTurnId) -> Self {
        Self {
            operation_id: ActivityOperationId::Turn(turn_id),
            assistant: None,
            reasoning: None,
            steps: BTreeMap::new(),
            tools: BTreeMap::new(),
            commands: BTreeMap::new(),
            subagents: BTreeMap::new(),
            tasks: BTreeMap::new(),
            compaction: None,
            next_id: 0,
        }
    }

    pub(super) fn project(
        &mut self,
        event: &WsEvent,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match event {
            WsEvent::AssistantDelta { .. } => {
                let activity = self.open_assistant()?;
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Updated,
                    ActivityStatus::InProgress,
                    None,
                )?])
            }
            WsEvent::ThinkingDelta { delta, .. } => {
                let activity = self.open_reasoning()?;
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Updated,
                    ActivityStatus::InProgress,
                    Some(content(
                        delta,
                        ActivityContentChangeKind::Delta,
                        ActivityContentStream::ReasoningSummaryText,
                    )?),
                )?])
            }
            WsEvent::StepStarted {
                turn_id,
                step,
                step_id,
            } => self.started(
                key(turn_id, *step),
                step_id.as_deref(),
                ActivityKind::Task,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                ActivityBucket::Step,
                None,
            ),
            WsEvent::StepEnded {
                turn_id,
                step,
                failed,
                ..
            } => self.ended(&key(turn_id, *step), *failed, ActivityBucket::Step),
            WsEvent::ToolStarted { call_id, name, .. } => self.started(
                call_id.clone(),
                Some(call_id),
                ActivityKind::ProviderOwnedTool,
                ActivityDisclosure::ProviderDisplayContent,
                ActivityBucket::Tool,
                activity_label(name),
            ),
            WsEvent::ToolUpdated { call_id, .. } => self.updated(call_id, ActivityBucket::Tool),
            WsEvent::ToolEnded {
                call_id, failed, ..
            } => self.ended(call_id, *failed, ActivityBucket::Tool),
            WsEvent::ShellStarted { command_id } => self.started(
                command_id.clone(),
                Some(command_id),
                ActivityKind::CommandExecution,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                ActivityBucket::Command,
                None,
            ),
            WsEvent::ShellUpdated { command_id } => {
                self.updated(command_id, ActivityBucket::Command)
            }
            WsEvent::ShellEnded { command_id, failed } => {
                self.ended(command_id, *failed, ActivityBucket::Command)
            }
            WsEvent::SubagentSpawned {
                subagent_id,
                name: _,
            } => self.started(
                subagent_id.clone(),
                Some(subagent_id),
                ActivityKind::SubagentOrCollaboration,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                ActivityBucket::Subagent,
                None,
            ),
            WsEvent::SubagentUpdated { subagent_id } => {
                self.updated(subagent_id, ActivityBucket::Subagent)
            }
            WsEvent::SubagentEnded {
                subagent_id,
                failed,
            } => self.ended(subagent_id, *failed, ActivityBucket::Subagent),
            WsEvent::CompactionStarted => {
                if self.compaction.is_some() {
                    return Err(activity_drift());
                }
                let activity = self.open(
                    "compaction",
                    None,
                    ActivityKind::ContextCompaction,
                    None,
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                )?;
                self.compaction = Some(activity.clone());
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::InProgress,
                    None,
                )?])
            }
            WsEvent::CompactionEnded { failed } => {
                let activity = self.compaction.take().ok_or_else(activity_drift)?;
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Completed,
                    terminal_status(*failed),
                    None,
                )?])
            }
            WsEvent::TaskStarted { task_id } => self.started(
                task_id.clone(),
                Some(task_id),
                ActivityKind::Task,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                ActivityBucket::Task,
                None,
            ),
            WsEvent::TaskEnded { task_id, failed } => {
                self.ended(task_id, *failed, ActivityBucket::Task)
            }
            WsEvent::Retrying { .. } => self.milestone(
                ActivityKind::Unknown(namespace("kimi.local-server.retry")?),
                "retry",
            ),
            WsEvent::Warning => self.milestone(ActivityKind::WarningOrError, "warning"),
            WsEvent::Unknown(event_type) => self.milestone(
                ActivityKind::Unknown(namespace(&format!("kimi.local-server.{event_type}"))?),
                "unknown",
            ),
            WsEvent::TurnEnded { reason, .. } => {
                let status = match reason {
                    TurnEndReason::Completed => ActivityStatus::Completed,
                    TurnEndReason::Cancelled => ActivityStatus::Cancelled,
                    TurnEndReason::Failed | TurnEndReason::Blocked => ActivityStatus::Failed,
                };
                self.complete(status)
            }
            WsEvent::TurnStarted { .. }
            | WsEvent::AwaitingApproval
            | WsEvent::AwaitingQuestion
            | WsEvent::SessionAborted
            | WsEvent::Progress
            | WsEvent::ProviderError => Ok(Vec::new()),
        }
    }

    pub(super) fn complete(
        &mut self,
        status: ActivityStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut open = Vec::new();
        if let Some(activity) = self.reasoning.take() {
            open.push(activity);
        }
        if let Some(activity) = self.assistant.take() {
            open.push(activity);
        }
        open.extend(std::mem::take(&mut self.steps).into_values());
        open.extend(std::mem::take(&mut self.tools).into_values());
        open.extend(std::mem::take(&mut self.commands).into_values());
        open.extend(std::mem::take(&mut self.subagents).into_values());
        open.extend(std::mem::take(&mut self.tasks).into_values());
        if let Some(activity) = self.compaction.take() {
            open.push(activity);
        }
        open.into_iter()
            .map(|activity| {
                self.observation(&activity, ActivityLifecyclePhase::Completed, status, None)
            })
            .collect()
    }

    fn open_assistant(&mut self) -> Result<OpenActivity, RuntimeFailure> {
        if let Some(activity) = self.assistant.clone() {
            return Ok(activity);
        }
        let activity = self.open(
            "assistant",
            None,
            ActivityKind::AssistantMessage,
            Some(ActivityAssistantPhase::ProviderUnspecified),
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )?;
        self.assistant = Some(activity.clone());
        Ok(activity)
    }

    fn open_reasoning(&mut self) -> Result<OpenActivity, RuntimeFailure> {
        if let Some(activity) = self.reasoning.clone() {
            return Ok(activity);
        }
        let activity = self.open(
            "reasoning",
            None,
            ActivityKind::ReasoningSummary,
            None,
            ActivityDisclosure::ProviderDisplayContent,
        )?;
        self.reasoning = Some(activity.clone());
        Ok(activity)
    }

    #[allow(clippy::too_many_arguments)]
    fn started(
        &mut self,
        key: String,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        disclosure: ActivityDisclosure,
        bucket: ActivityBucket,
        label: Option<ActivityLabel>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if self.bucket(bucket).contains_key(&key) {
            return Err(activity_drift());
        }
        let mut activity = self.open(bucket.label(), provider_ref, kind, None, disclosure)?;
        activity.label = label;
        self.bucket_mut(bucket).insert(key, activity.clone());
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
        )?])
    }

    fn updated(
        &self,
        key: &str,
        bucket: ActivityBucket,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self
            .bucket(bucket)
            .get(key)
            .cloned()
            .ok_or_else(activity_drift)?;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            None,
        )?])
    }

    fn ended(
        &mut self,
        key: &str,
        failed: bool,
        bucket: ActivityBucket,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self
            .bucket_mut(bucket)
            .remove(key)
            .ok_or_else(activity_drift)?;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            terminal_status(failed),
            None,
        )?])
    }

    fn milestone(
        &mut self,
        kind: ActivityKind,
        label: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self.open(
            label,
            None,
            kind,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )?;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
        )?])
    }

    fn bucket(&self, bucket: ActivityBucket) -> &BTreeMap<String, OpenActivity> {
        match bucket {
            ActivityBucket::Step => &self.steps,
            ActivityBucket::Tool => &self.tools,
            ActivityBucket::Command => &self.commands,
            ActivityBucket::Subagent => &self.subagents,
            ActivityBucket::Task => &self.tasks,
        }
    }

    fn bucket_mut(&mut self, bucket: ActivityBucket) -> &mut BTreeMap<String, OpenActivity> {
        match bucket {
            ActivityBucket::Step => &mut self.steps,
            ActivityBucket::Tool => &mut self.tools,
            ActivityBucket::Command => &mut self.commands,
            ActivityBucket::Subagent => &mut self.subagents,
            ActivityBucket::Task => &mut self.tasks,
        }
    }

    fn open(
        &mut self,
        label: &str,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
    ) -> Result<OpenActivity, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        Ok(OpenActivity {
            id: ActivityId::new(format!("kimi-local:{label}:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            provider_ref: provider_ref
                .map(ProviderActivityRef::new)
                .transpose()
                .map_err(|_| activity_drift())?,
            kind,
            assistant_phase,
            disclosure,
            label: None,
        })
    }

    fn observation(
        &self,
        activity: &OpenActivity,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let mut observation = ActivityObservation::new(
            activity.id.clone(),
            self.operation_id.clone(),
            activity.kind.clone(),
            phase,
            status,
            activity.assistant_phase,
            activity.disclosure,
        )
        .map_err(|_| activity_drift())?;
        if let Some(reference) = activity.provider_ref.clone() {
            observation = observation.with_provider_activity_ref(reference);
        }
        if let Some(label) = activity.label.clone() {
            observation = observation
                .with_label(label)
                .map_err(|_| activity_drift())?;
        }
        if let Some(content) = content {
            observation = observation
                .with_content(content)
                .map_err(|_| activity_drift())?;
        }
        Ok(observation)
    }
}
