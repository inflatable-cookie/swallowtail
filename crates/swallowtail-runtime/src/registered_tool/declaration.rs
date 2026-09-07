//! One immutable consumer-declared tool inside a registration snapshot.

use super::failure::RegisteredToolFailure;
use super::identity::{RegisteredToolExecutionKind, RegisteredToolId};
use super::limits::RegisteredToolBounds;
use super::schema::RegisteredToolSchema;

/// Consumer-declared effect posture for one registered tool.
///
/// Swallowtail never replays a mutating or indeterminate call; this declaration
/// records the consumer's truth, not a Swallowtail retry permission.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolEffectPosture {
    /// The consumer declares the call has no external effect.
    ReadOnly,
    /// The consumer declares the call mutates state.
    Mutating,
    /// The consumer cannot classify the effect.
    Indeterminate,
}

impl RegisteredToolEffectPosture {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::Mutating => "mutating",
            Self::Indeterminate => "indeterminate",
        }
    }
}

/// Consumer-declared retry posture for one registered tool.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolRetryPosture {
    /// The consumer may explicitly retry with a fresh attempt and binding.
    ConsumerRetryable,
    /// The consumer declares this call must never be retried.
    NeverRetry,
}

impl RegisteredToolRetryPosture {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerRetryable => "consumer-retryable",
            Self::NeverRetry => "never-retry",
        }
    }
}

/// Swallowtail-enforced posture that no declaration can widen.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegisteredToolEnforcedPosture;

impl RegisteredToolEnforcedPosture {
    /// Reports that Swallowtail never replays a call automatically.
    #[must_use]
    pub const fn automatic_replay_allowed() -> bool {
        false
    }

    /// Reports that reconnect retains the same binding and never replays work.
    #[must_use]
    pub const fn reconnect_replays_calls() -> bool {
        false
    }

    /// Reports that teardown always joins issued work before reporting cleanup.
    #[must_use]
    pub const fn teardown_joins_issued_work() -> bool {
        true
    }
}

/// Immutable declaration of exactly one namespaced registered tool.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolDeclaration {
    id: RegisteredToolId,
    kind: RegisteredToolExecutionKind,
    input_schema: RegisteredToolSchema,
    output_schema: RegisteredToolSchema,
    effect: RegisteredToolEffectPosture,
    retry: RegisteredToolRetryPosture,
    bounds: RegisteredToolBounds,
}

impl RegisteredToolDeclaration {
    /// Binds one namespaced identity to exactly one execution kind.
    pub fn new(
        id: RegisteredToolId,
        kind: RegisteredToolExecutionKind,
        input_schema: RegisteredToolSchema,
        output_schema: RegisteredToolSchema,
        effect: RegisteredToolEffectPosture,
        retry: RegisteredToolRetryPosture,
        bounds: RegisteredToolBounds,
    ) -> Result<Self, RegisteredToolFailure> {
        Ok(Self {
            id,
            kind,
            input_schema,
            output_schema,
            effect,
            retry,
            bounds,
        })
    }

    /// Returns the namespaced tool identity.
    #[must_use]
    pub const fn id(&self) -> &RegisteredToolId {
        &self.id
    }

    /// Returns the single execution kind bound to this identity.
    #[must_use]
    pub const fn kind(&self) -> RegisteredToolExecutionKind {
        self.kind
    }

    /// Returns the declared input schema.
    #[must_use]
    pub const fn input_schema(&self) -> &RegisteredToolSchema {
        &self.input_schema
    }

    /// Returns the declared output schema.
    #[must_use]
    pub const fn output_schema(&self) -> &RegisteredToolSchema {
        &self.output_schema
    }

    /// Returns the consumer-declared effect posture.
    #[must_use]
    pub const fn effect(&self) -> RegisteredToolEffectPosture {
        self.effect
    }

    /// Returns the consumer-declared retry posture.
    #[must_use]
    pub const fn retry(&self) -> RegisteredToolRetryPosture {
        self.retry
    }

    /// Returns the declared per-tool bounds.
    #[must_use]
    pub const fn bounds(&self) -> RegisteredToolBounds {
        self.bounds
    }

    /// Returns the declared schema bytes contributed by this tool.
    #[must_use]
    pub fn schema_byte_len(&self) -> usize {
        self.input_schema
            .byte_len()
            .saturating_add(self.output_schema.byte_len())
    }
}
