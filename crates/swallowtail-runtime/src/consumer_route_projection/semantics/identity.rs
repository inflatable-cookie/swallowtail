use super::extension::ConsumerRouteNamespacedExtension;

/// Closed portable feature identity shared by every projection view.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteFeatureId {
    /// Enumerating models through the exact route.
    ModelCatalogue,
    /// A bounded structured run.
    StructuredRun,
    /// A multi-turn interactive session.
    InteractiveSession,
    /// A realtime media session.
    RealtimeMediaSession,
    /// Incremental operation events.
    StreamingEvents,
    /// Provider usage measurement evidence.
    UsageEvidence,
    /// Portable activity observation.
    ActivityObservation,
    /// Explicit reasoning selection.
    ReasoningSelection,
    /// Enforced structured output.
    StructuredOutput,
    /// Non-text request attachments.
    Attachments,
    /// Consumer-mediated tool exchange.
    ConsumerToolExchange,
    /// Consumer-mediated question exchange.
    QuestionExchange,
    /// Cancellation or interruption of active work.
    CancellationOrInterruption,
    /// Loading retained provider-session state.
    LoadSession,
    /// Resuming a loaded or retained provider session.
    ResumeSession,
    /// Listing bounded provider-session candidates.
    ProviderSessionCatalogue,
    /// Importing an exact provider-session candidate.
    ProviderSessionImport,
    /// Archiving a retained provider session.
    ProviderSessionArchive,
    /// Restoring an archived provider session.
    ProviderSessionRestore,
    /// Deleting retained provider-session data.
    ProviderSessionDelete,
    /// Reconciling retained provider-session truth.
    ProviderSessionReconciliation,
    /// Reading newest-first provider-session history.
    ProviderSessionHistory,
    /// Route persistence posture as lifecycle evidence only.
    PersistentSessionPosture,
    /// Binding the operation to a working resource.
    WorkingResource,
    /// Writing bounded workspace text through the host service.
    BoundedWorkspaceTextWrite,
    /// Requesting external search through the route.
    ExternalSearch,
    /// Applying an output-token ceiling.
    OutputTokenLimit,
    /// The exact prepared-operation facade itself.
    PreparedFacade,
    /// Exact active-session reasoning acknowledgement truth.
    ActiveSessionReasoningAcknowledgement,
    /// Bounded provider-native feature identity.
    Namespaced(ConsumerRouteNamespacedExtension),
}

/// Closed portable control identity shared by every projection view.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteControlId {
    /// Exact model-route selection.
    ModelSelection,
    /// Exact reasoning selection.
    ReasoningSelection,
    /// The exact session-option bundle.
    SessionOptions,
    /// Declared consumer tools.
    ToolDeclarations,
    /// Developer instruction content.
    DeveloperInstructions,
    /// Selected idiom values.
    Idioms,
    /// Per-turn consumer-mediated user-input exchange.
    UserInputExchange,
    /// Loading an exact retained provider session.
    LoadSession,
    /// Resuming an exact retained provider session.
    ResumeSession,
    /// Bounded provider-session catalogue query.
    SessionCatalogueBounds,
    /// Bounded provider-session history query.
    SessionHistoryBounds,
    /// Bounded provider-session reconciliation query.
    SessionReconciliation,
    /// Provider output-token maximum.
    MaximumOutputTokens,
    /// Fixed realtime media configuration.
    RealtimeMediaConfig,
    /// Planned connection-rollover policy.
    PlannedConnectionRollover,
    /// Bounded provider-native control identity.
    Namespaced(ConsumerRouteNamespacedExtension),
}

/// Shared semantic identity of one projected row.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteRowIdentity {
    /// A descriptive feature row.
    Feature(ConsumerRouteFeatureId),
    /// A control row with a value domain and omission semantics.
    Control(ConsumerRouteControlId),
}

impl ConsumerRouteRowIdentity {
    #[must_use]
    /// Returns the bounded namespaced extension when the row uses one.
    pub const fn namespaced_extension(&self) -> Option<&ConsumerRouteNamespacedExtension> {
        match self {
            Self::Feature(ConsumerRouteFeatureId::Namespaced(extension))
            | Self::Control(ConsumerRouteControlId::Namespaced(extension)) => Some(extension),
            _ => None,
        }
    }
}
