use super::AcpBoundedText;

/// One entry in a replacement ACP plan snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpPlanEntry {
    /// Human-readable task content.
    pub content: AcpBoundedText,
    /// Provider-assigned task priority.
    pub priority: AcpPlanEntryPriority,
    /// Current task lifecycle status.
    pub status: AcpPlanEntryStatus,
}

/// Priority assigned to an ACP plan entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpPlanEntryPriority {
    /// High priority.
    High,
    /// Medium priority.
    Medium,
    /// Low priority.
    Low,
}

/// Lifecycle state of an ACP plan entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpPlanEntryStatus {
    /// Work has not started.
    Pending,
    /// Work is active.
    InProgress,
    /// Work finished.
    Completed,
}

/// One command advertised by an ACP agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpCommand {
    /// Command invocation name.
    pub name: AcpBoundedText,
    /// Human-readable command description.
    pub description: AcpBoundedText,
    /// Optional hint describing accepted input.
    pub input_hint: Option<AcpBoundedText>,
}

/// One configurable option advertised for an ACP session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpConfigOption {
    /// Stable provider option identity.
    pub id: AcpBoundedText,
    /// Human-readable option name.
    pub name: AcpBoundedText,
    /// Optional option description.
    pub description: Option<AcpBoundedText>,
    /// Optional portable or provider-defined category.
    pub category: Option<AcpConfigCategory>,
    /// Option shape and current value.
    pub kind: AcpConfigKind,
}

/// Portable category of an ACP configuration option.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpConfigCategory {
    /// Agent or harness operating mode.
    Mode,
    /// Model selection.
    Model,
    /// Model-specific configuration.
    ModelConfig,
    /// Reasoning or thought intensity.
    ThoughtLevel,
    /// Provider-defined category retained verbatim.
    Other(AcpBoundedText),
}

/// Supported ACP configuration option shapes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpConfigKind {
    /// Selection from one bounded choice collection.
    Select {
        /// Currently selected provider value.
        current_value: AcpBoundedText,
        /// Available values, optionally grouped for display.
        options: AcpConfigChoices,
    },
    /// Boolean toggle.
    Boolean {
        /// Current toggle value.
        current_value: bool,
    },
}

/// Ungrouped or display-grouped ACP selection choices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpConfigChoices {
    /// Flat choice list.
    Ungrouped(Vec<AcpConfigChoice>),
    /// Choice groups with provider display labels.
    Grouped(Vec<AcpConfigGroup>),
}

/// One selectable ACP configuration value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpConfigChoice {
    /// Provider value sent when selected.
    pub value: AcpBoundedText,
    /// Human-readable choice name.
    pub name: AcpBoundedText,
    /// Optional choice description.
    pub description: Option<AcpBoundedText>,
}

/// One display group of ACP configuration choices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpConfigGroup {
    /// Stable provider group identity.
    pub group: AcpBoundedText,
    /// Human-readable group name.
    pub name: AcpBoundedText,
    /// Choices within the group.
    pub options: Vec<AcpConfigChoice>,
}

/// Three-state partial-update field preserving omission and explicit clearing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpOptionalUpdate<T> {
    /// The provider omitted the field; retain the prior value.
    Unchanged,
    /// The provider explicitly cleared the field.
    Cleared,
    /// The provider supplied a replacement value.
    Set(T),
}

/// ACP context-window usage and optional monetary cost snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct AcpUsage {
    /// Context units currently used.
    pub used: u64,
    /// Total context capacity in the same units.
    pub size: u64,
    /// Optional cost snapshot.
    pub cost: Option<AcpCost>,
}

/// Provider-reported monetary cost.
#[derive(Clone, Debug, PartialEq)]
pub struct AcpCost {
    /// Numeric amount in the reported currency.
    pub amount: f64,
    /// Provider-supplied currency identifier.
    pub currency: AcpBoundedText,
}
