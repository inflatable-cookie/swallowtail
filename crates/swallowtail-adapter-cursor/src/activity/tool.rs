impl AcpActivityProjection {
    fn tool_start(
        &mut self,
        call: &AcpToolCall,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let key = format!("tool:{}", call.tool_call_id.as_str());
        if self.closed.contains(&key) {
            return Ok(Vec::new());
        }
        let status = tool_status(call.status);
        let label = activity_label(call.title.as_str());
        let content = tool_content(&call.content)?;
        if let Some(mut activity) = self.open.get(&key).cloned() {
            if label.is_some() {
                activity.label = label;
            }
            let status = reconcile_status(activity.status, status);
            let phase = lifecycle_phase(status);
            activity.status = status;
            let observation = self.observation(&activity, phase, status, content)?;
            self.retain_or_close(key, activity, status);
            return Ok(vec![observation]);
        }
        if status.is_terminal() {
            return self.synthetic_terminal_tool(
                key,
                call.tool_call_id.as_str(),
                label,
                status,
                content,
            );
        }
        let mut activity = self.open_or_insert(
            &key,
            Some(call.tool_call_id.as_str()),
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            status,
        )?;
        activity.label = label;
        self.open.insert(key, activity.clone());
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            status,
            content,
        )?])
    }

    fn tool_update(
        &mut self,
        update: &AcpToolCallUpdate,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let key = format!("tool:{}", update.tool_call_id.as_str());
        if self.closed.contains(&key) {
            return Ok(Vec::new());
        }
        let label = update
            .title
            .as_ref()
            .and_then(|title| activity_label(title.as_str()));
        let content = update
            .content_replacement
            .as_ref()
            .map(|content| tool_content(content))
            .transpose()?
            .flatten();
        let Some(mut activity) = self.open.get(&key).cloned() else {
            let Some(status) = update
                .status
                .map(tool_status)
                .filter(|status| status.is_terminal())
            else {
                return Ok(Vec::new());
            };
            return self.synthetic_terminal_tool(
                key,
                update.tool_call_id.as_str(),
                label,
                status,
                content,
            );
        };
        if label.is_some() {
            activity.label = label;
        }
        let status = update.status.map_or(activity.status, |status| {
            reconcile_status(activity.status, tool_status(status))
        });
        let observation = self.observation(&activity, lifecycle_phase(status), status, content)?;
        activity.status = status;
        self.retain_or_close(key, activity, status);
        Ok(vec![observation])
    }

    fn synthetic_terminal_tool(
        &mut self,
        key: String,
        provider_id: &str,
        label: Option<ActivityLabel>,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut activity = self.open_or_insert(
            &key,
            Some(provider_id),
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            ActivityStatus::InProgress,
        )?;
        activity.label = label;
        let started = self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
        )?;
        let completed = self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            status,
            content,
        )?;
        self.open.remove(&key);
        self.closed.insert(key);
        Ok(vec![started, completed])
    }

    fn retain_or_close(&mut self, key: String, activity: OpenActivity, status: ActivityStatus) {
        if status.is_terminal() {
            self.open.remove(&key);
            self.closed.insert(key);
        } else {
            self.open.insert(key, activity);
        }
    }
}

