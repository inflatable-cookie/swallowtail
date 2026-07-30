use super::{ActivityContent, ActivityLabel, InvalidActivityRecord};
use std::collections::BTreeSet;
use std::fmt;
use swallowtail_core::{ModelId, ProviderActivityRef, ReasoningMode};

const MAX_SUBAGENT_ID_BYTES: usize = 256;
pub(super) const MAX_SUBAGENTS_PER_OBSERVATION: usize = 64;

/// Operation-local identity for provider-owned child work.
///
/// This value is correlation evidence, not authority to address an arbitrary
/// provider session.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubagentId(String);

impl SubagentId {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidActivityRecord> {
        let value = value.into();
        if value.trim().is_empty()
            || value.len() > MAX_SUBAGENT_ID_BYTES
            || value.chars().any(char::is_control)
        {
            return Err(InvalidActivityRecord::new(
                "Subagent id must use a non-empty bounded value",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SubagentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("SubagentId")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for SubagentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted subagent id>")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivityActor {
    Primary,
    Subagent(SubagentId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubagentParent {
    Operation,
    Subagent(SubagentId),
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SubagentStatus {
    Unknown,
    Pending,
    Running,
    Waiting,
    Completed,
    Failed,
    Interrupted,
    Shutdown,
}

#[derive(Clone, Eq, PartialEq)]
pub struct SubagentSnapshot {
    id: SubagentId,
    parent: SubagentParent,
    status: SubagentStatus,
    label: Option<ActivityLabel>,
    description: Option<ActivityContent>,
    model: Option<ModelId>,
    reasoning: Option<ReasoningMode>,
    background: Option<bool>,
    originating_activity: Option<ProviderActivityRef>,
}

impl fmt::Debug for SubagentSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SubagentSnapshot")
            .field("id", &self.id)
            .field("parent", &self.parent)
            .field("status", &self.status)
            .field("label", &self.label.as_ref().map(|_| "<redacted>"))
            .field(
                "description",
                &self.description.as_ref().map(|value| value.byte_len()),
            )
            .field("model", &self.model.as_ref().map(|_| "<redacted>"))
            .field("reasoning", &self.reasoning.as_ref().map(|_| "<redacted>"))
            .field("background", &self.background)
            .field("originating_activity", &self.originating_activity)
            .finish()
    }
}

impl SubagentSnapshot {
    #[must_use]
    pub const fn new(id: SubagentId, parent: SubagentParent, status: SubagentStatus) -> Self {
        Self {
            id,
            parent,
            status,
            label: None,
            description: None,
            model: None,
            reasoning: None,
            background: None,
            originating_activity: None,
        }
    }

    #[must_use]
    pub fn with_label(mut self, label: ActivityLabel) -> Self {
        self.label = Some(label);
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: ActivityContent) -> Self {
        self.description = Some(description);
        self
    }

    #[must_use]
    pub fn with_model(mut self, model: ModelId) -> Self {
        self.model = Some(model);
        self
    }

    #[must_use]
    pub fn with_reasoning(mut self, reasoning: ReasoningMode) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    #[must_use]
    pub const fn with_background(mut self, background: bool) -> Self {
        self.background = Some(background);
        self
    }

    #[must_use]
    pub const fn with_status(mut self, status: SubagentStatus) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn with_originating_activity(mut self, activity: ProviderActivityRef) -> Self {
        self.originating_activity = Some(activity);
        self
    }

    #[must_use]
    pub const fn id(&self) -> &SubagentId {
        &self.id
    }

    #[must_use]
    pub const fn parent(&self) -> &SubagentParent {
        &self.parent
    }

    #[must_use]
    pub const fn status(&self) -> SubagentStatus {
        self.status
    }

    #[must_use]
    pub const fn label(&self) -> Option<&ActivityLabel> {
        self.label.as_ref()
    }

    #[must_use]
    pub const fn description(&self) -> Option<&ActivityContent> {
        self.description.as_ref()
    }

    #[must_use]
    pub const fn model(&self) -> Option<&ModelId> {
        self.model.as_ref()
    }

    #[must_use]
    pub const fn reasoning(&self) -> Option<&ReasoningMode> {
        self.reasoning.as_ref()
    }

    #[must_use]
    pub const fn background(&self) -> Option<bool> {
        self.background
    }

    #[must_use]
    pub const fn originating_activity(&self) -> Option<&ProviderActivityRef> {
        self.originating_activity.as_ref()
    }
}

pub(super) fn validate_subagents(
    subagents: &[SubagentSnapshot],
) -> Result<(), InvalidActivityRecord> {
    if subagents.len() > MAX_SUBAGENTS_PER_OBSERVATION {
        return Err(InvalidActivityRecord::new(
            "Activity observation exceeded its subagent snapshot bound",
        ));
    }
    let mut ids = BTreeSet::new();
    if subagents
        .iter()
        .any(|snapshot| !ids.insert(snapshot.id().clone()))
    {
        return Err(InvalidActivityRecord::new(
            "Activity observation contains duplicate subagent identities",
        ));
    }
    Ok(())
}
