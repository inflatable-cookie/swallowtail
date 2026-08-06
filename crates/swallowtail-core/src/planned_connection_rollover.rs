use std::num::NonZeroU32;

/// An operation-scoped, planned replacement of an active provider connection.
///
/// This is not retry, reconnect, session resume, or stream reattachment. The
/// provider session remains active while one connection generation replaces
/// another at an idle turn boundary.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PlannedConnectionRolloverPolicy {
    /// Do not replace the active provider connection.
    #[default]
    Disabled,
    /// Allow at most the given number of planned replacements.
    Bounded(NonZeroU32),
}

impl PlannedConnectionRolloverPolicy {
    #[must_use]
    /// Returns the rollover bound, or `None` when disabled.
    pub const fn maximum_count(self) -> Option<NonZeroU32> {
        match self {
            Self::Disabled => None,
            Self::Bounded(maximum) => Some(maximum),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlannedConnectionRolloverPolicy;
    use std::num::NonZeroU32;

    #[test]
    fn disabled_is_default_and_a_bound_cannot_be_zero() {
        assert_eq!(
            PlannedConnectionRolloverPolicy::default(),
            PlannedConnectionRolloverPolicy::Disabled
        );
        assert!(NonZeroU32::new(0).is_none());
        assert_eq!(
            PlannedConnectionRolloverPolicy::Bounded(NonZeroU32::new(1).unwrap()).maximum_count(),
            NonZeroU32::new(1)
        );
    }
}
