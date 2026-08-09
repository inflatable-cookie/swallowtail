//! Provider-neutral idiom records, confidence, merge, and lint functions.
//!
//! Idioms are learned behavioral preferences (the substrate Command Code's
//! taste exposes) kept portable: typed constraints with confidence and
//! provenance, scoped to user, project, or package. This crate owns records
//! and pure functions only — no prompt composition, no permission authority,
//! and no learned-model dependency (Contract 055).

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod bounded;
mod confidence;
mod engine;
mod error;
mod lint;
mod merge;
mod record;
mod registry;
mod static_rules;
mod time;

pub use bounded::{
    BoundedText, DEFAULT_MAX_CORRELATION_BYTES, DEFAULT_MAX_ID_BYTES, DEFAULT_MAX_PATTERN_BYTES,
    DEFAULT_MAX_SOURCE_BYTES, DEFAULT_MAX_TARGET_BYTES,
};
pub use confidence::{Confidence, DEFAULT_DECAY_HALF_LIFE_TICKS, MAX_DECAY_HALVINGS};
pub use engine::{IdiomContext, IdiomRecorder, IdiomSet, IdiomSink, IdiomSource, select_bounded};
pub use error::{IdiomError, IdiomErrorKind};
pub use lint::{LintIssue, LintReason, lint_idiom, lint_store};
pub use merge::{MergeOutcome, merge};
pub use record::{
    Idiom, IdiomConstraint, IdiomId, IdiomScope, IdiomSignal, Provenance, SignalKind,
};
pub use registry::{
    MergeSummary, RegistryMergeOutcome, RegistryNamespace, RegistryPackage, RegistryPackageRef,
    pull_package, push_merge,
};
pub use static_rules::{StaticRulesSource, prepare_session_idioms};
pub use time::MonotonicInstant;
