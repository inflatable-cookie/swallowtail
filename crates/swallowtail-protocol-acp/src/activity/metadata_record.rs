use super::AcpBoundedText;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpPlanEntry {
    pub content: AcpBoundedText,
    pub priority: AcpPlanEntryPriority,
    pub status: AcpPlanEntryStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpPlanEntryPriority {
    High,
    Medium,
    Low,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpPlanEntryStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpCommand {
    pub name: AcpBoundedText,
    pub description: AcpBoundedText,
    pub input_hint: Option<AcpBoundedText>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpConfigOption {
    pub id: AcpBoundedText,
    pub name: AcpBoundedText,
    pub description: Option<AcpBoundedText>,
    pub category: Option<AcpConfigCategory>,
    pub kind: AcpConfigKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpConfigCategory {
    Mode,
    Model,
    ModelConfig,
    ThoughtLevel,
    Other(AcpBoundedText),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpConfigKind {
    Select {
        current_value: AcpBoundedText,
        options: AcpConfigChoices,
    },
    Boolean {
        current_value: bool,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpConfigChoices {
    Ungrouped(Vec<AcpConfigChoice>),
    Grouped(Vec<AcpConfigGroup>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpConfigChoice {
    pub value: AcpBoundedText,
    pub name: AcpBoundedText,
    pub description: Option<AcpBoundedText>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpConfigGroup {
    pub group: AcpBoundedText,
    pub name: AcpBoundedText,
    pub options: Vec<AcpConfigChoice>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpOptionalUpdate<T> {
    Unchanged,
    Cleared,
    Set(T),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcpUsage {
    pub used: u64,
    pub size: u64,
    pub cost: Option<AcpCost>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcpCost {
    pub amount: f64,
    pub currency: AcpBoundedText,
}
