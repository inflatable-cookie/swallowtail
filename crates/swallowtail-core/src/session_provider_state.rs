/// Provider-owned state permitted for one interactive session.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SessionProviderStatePolicy {
    /// The session may not create durable provider-owned conversation state.
    #[default]
    Prohibited,
    /// One driver-owned conversation may live until item-and-conversation deletion on close.
    DurableConversationDeleteOnClose,
}

impl SessionProviderStatePolicy {
    #[must_use]
    pub const fn permits_durable_conversation(self) -> bool {
        matches!(self, Self::DurableConversationDeleteOnClose)
    }

    #[must_use]
    pub const fn requires_delete_on_close(self) -> bool {
        matches!(self, Self::DurableConversationDeleteOnClose)
    }
}

#[cfg(test)]
mod tests {
    use super::SessionProviderStatePolicy;

    #[test]
    fn durable_conversation_is_an_explicit_delete_on_close_opt_in() {
        assert!(!SessionProviderStatePolicy::default().permits_durable_conversation());
        let policy = SessionProviderStatePolicy::DurableConversationDeleteOnClose;
        assert!(policy.permits_durable_conversation());
        assert!(policy.requires_delete_on_close());
    }
}
