//! Trusted consumer admission binding and its live revocation port.

use super::failure::RegisteredToolFailure;
use super::identity::{
    ConsumerTaskGeneration, ConsumerWorkspaceGeneration, RegisteredToolReasonCode, admit_identity,
};
use crate::{BoxFuture, RuntimeFailure};
use std::fmt;
use std::sync::Arc;

macro_rules! opaque_admitted_identity {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Admits bounded, non-blank, control-free opaque identity text.
            pub fn new(value: impl Into<String>) -> Result<Self, RegisteredToolFailure> {
                admit_identity(value.into()).map(Self)
            }

            /// Returns the opaque value for exact host comparison only.
            #[must_use]
            pub fn as_host_value(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"<opaque>")
                    .finish()
            }
        }
    };
}

opaque_admitted_identity!(
    ConsumerProcessIncarnation,
    "Opaque consumer process incarnation created fresh at every consumer start."
);
opaque_admitted_identity!(AdmittedTaskId, "Opaque consumer-issued task identity.");
opaque_admitted_identity!(
    AdmittedSessionId,
    "Opaque consumer-issued session identity."
);
opaque_admitted_identity!(
    AdmittedAttemptId,
    "Opaque consumer-issued attempt identity for one dispatch."
);

/// Phase at which the kernel revalidates a live admission verdict.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AdmissionPhase {
    /// Immediately before the kernel commits a call to dispatch.
    BeforeDispatch,
    /// Immediately before the kernel delivers a settled result or progress.
    BeforeDelivery,
}

impl AdmissionPhase {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BeforeDispatch => "before-dispatch",
            Self::BeforeDelivery => "before-delivery",
        }
    }
}

/// Live verdict returned by the consumer admission port.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdmissionVerdict {
    /// The admitted binding is still current.
    Current,
    /// The admitted binding is revoked, with a bounded safe reason code.
    Revoked(RegisteredToolReasonCode),
}

impl AdmissionVerdict {
    /// Reports whether the binding may still admit work.
    #[must_use]
    pub const fn is_current(&self) -> bool {
        matches!(self, Self::Current)
    }

    /// Returns the bounded revocation reason code, when revoked.
    #[must_use]
    pub const fn reason(&self) -> Option<&RegisteredToolReasonCode> {
        match self {
            Self::Current => None,
            Self::Revoked(reason) => Some(reason),
        }
    }
}

/// Trusted consumer-supplied port that answers live admission questions.
///
/// The consumer does not reissue identity during validation. A new generation
/// needs a new admitted attempt and a new binding.
pub trait ConsumerAdmissionHostService: Send + Sync {
    /// Answers whether the fixed binding is still current at one exact phase.
    fn validate(
        &self,
        binding: &ConsumerAdmissionBinding,
        phase: AdmissionPhase,
    ) -> BoxFuture<'_, Result<AdmissionVerdict, RuntimeFailure>>;
}

/// Immutable trusted admission binding for one consumer attempt.
///
/// The binding carries a private port reference to its live revocation source.
/// It holds no serialized bearer and cannot be created from provider input.
#[derive(Clone)]
pub struct ConsumerAdmissionBinding {
    incarnation: ConsumerProcessIncarnation,
    workspace_generation: ConsumerWorkspaceGeneration,
    task_generation: ConsumerTaskGeneration,
    task: AdmittedTaskId,
    session: AdmittedSessionId,
    attempt: AdmittedAttemptId,
    source: Arc<dyn ConsumerAdmissionHostService>,
}

impl ConsumerAdmissionBinding {
    /// Binds fixed admitted identity to one live revocation source.
    #[must_use]
    pub fn new(
        incarnation: ConsumerProcessIncarnation,
        workspace_generation: ConsumerWorkspaceGeneration,
        task_generation: ConsumerTaskGeneration,
        task: AdmittedTaskId,
        session: AdmittedSessionId,
        attempt: AdmittedAttemptId,
        source: Arc<dyn ConsumerAdmissionHostService>,
    ) -> Self {
        Self {
            incarnation,
            workspace_generation,
            task_generation,
            task,
            session,
            attempt,
            source,
        }
    }

    /// Returns the opaque consumer process incarnation.
    #[must_use]
    pub const fn incarnation(&self) -> &ConsumerProcessIncarnation {
        &self.incarnation
    }

    /// Returns the admitted workspace generation.
    #[must_use]
    pub const fn workspace_generation(&self) -> ConsumerWorkspaceGeneration {
        self.workspace_generation
    }

    /// Returns the admitted task generation.
    #[must_use]
    pub const fn task_generation(&self) -> ConsumerTaskGeneration {
        self.task_generation
    }

    /// Returns the admitted task identity.
    #[must_use]
    pub const fn task(&self) -> &AdmittedTaskId {
        &self.task
    }

    /// Returns the admitted session identity.
    #[must_use]
    pub const fn session(&self) -> &AdmittedSessionId {
        &self.session
    }

    /// Returns the admitted attempt identity.
    #[must_use]
    pub const fn attempt(&self) -> &AdmittedAttemptId {
        &self.attempt
    }

    /// Reports whether two bindings carry the same fixed admitted identity.
    #[must_use]
    pub fn identity_matches(&self, other: &Self) -> bool {
        self.incarnation == other.incarnation
            && self.workspace_generation == other.workspace_generation
            && self.task_generation == other.task_generation
            && self.task == other.task
            && self.session == other.session
            && self.attempt == other.attempt
    }

    /// Asks the bound consumer port for a live verdict at one exact phase.
    pub fn validate(
        &self,
        phase: AdmissionPhase,
    ) -> BoxFuture<'_, Result<AdmissionVerdict, RuntimeFailure>> {
        self.source.validate(self, phase)
    }
}

impl PartialEq for ConsumerAdmissionBinding {
    fn eq(&self, other: &Self) -> bool {
        self.identity_matches(other)
    }
}

impl Eq for ConsumerAdmissionBinding {}

impl fmt::Debug for ConsumerAdmissionBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConsumerAdmissionBinding")
            .field("incarnation", &self.incarnation)
            .field("workspace_generation", &self.workspace_generation)
            .field("task_generation", &self.task_generation)
            .field("task", &self.task)
            .field("session", &self.session)
            .field("attempt", &self.attempt)
            .field("source", &"<private consumer admission port>")
            .finish()
    }
}
