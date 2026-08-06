/// Operation-scoped remote resource whose deletion is owned by a driver.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OwnedRemoteResourceKind {
    /// Provider-owned execution environment.
    Environment,
    /// Provider-owned session.
    Session,
    /// Provider-owned response or run object.
    Response,
    /// Provider-owned conversation.
    Conversation,
    /// Items contained by a provider conversation.
    ConversationItems,
}
