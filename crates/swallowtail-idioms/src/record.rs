use crate::bounded::{
    BoundedText, DEFAULT_MAX_CORRELATION_BYTES, DEFAULT_MAX_ID_BYTES, DEFAULT_MAX_PATTERN_BYTES,
    DEFAULT_MAX_TARGET_BYTES,
};
use crate::confidence::{Confidence, DEFAULT_DECAY_HALF_LIFE_TICKS};
use crate::error::{IdiomError, require_bounded, require_confidence};
use crate::time::MonotonicInstant;

/// Stable idiom identity, opaque and consistent across scopes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdiomId(BoundedText);

impl IdiomId {
    /// Creates an idiom id after rejecting blank or overlong text.
    pub fn new(value: impl Into<String>) -> Result<Self, IdiomError> {
        require_bounded("idiom id", value, DEFAULT_MAX_ID_BYTES).map(Self)
    }

    /// Returns the stable idiom id text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Where an idiom applies: one user, one project, or one named package.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum IdiomScope {
    /// Applies to every project of one user.
    User,
    /// Applies to one project.
    Project,
    /// Applies to one named package.
    Package(BoundedText),
}

impl IdiomScope {
    /// Creates a package scope after rejecting blank or overlong names.
    pub fn package(name: impl Into<String>) -> Result<Self, IdiomError> {
        require_bounded("package name", name, DEFAULT_MAX_ID_BYTES).map(Self::Package)
    }
}

/// Typed symbolic constraint an idiom enforces (Contract 055).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum IdiomConstraint {
    /// Free-text preference (for example "use named exports").
    Text(BoundedText),
    /// File-path pattern using gitignore-style matching.
    File(BoundedText),
    /// Tool-name pattern (for example `edit_*`).
    Tool(BoundedText),
    /// Command pattern (for example `npm run *`).
    Command(BoundedText),
}

impl IdiomConstraint {
    fn bounded(field: &'static str, value: impl Into<String>) -> Result<BoundedText, IdiomError> {
        require_bounded(field, value, DEFAULT_MAX_PATTERN_BYTES)
    }

    /// Creates a text constraint after rejecting blank or overlong patterns.
    pub fn text(value: impl Into<String>) -> Result<Self, IdiomError> {
        Self::bounded("text pattern", value).map(Self::Text)
    }

    /// Creates a file-path constraint after rejecting blank or overlong
    /// patterns.
    pub fn file(value: impl Into<String>) -> Result<Self, IdiomError> {
        Self::bounded("file pattern", value).map(Self::File)
    }

    /// Creates a tool-name constraint after rejecting blank or overlong
    /// patterns.
    pub fn tool(value: impl Into<String>) -> Result<Self, IdiomError> {
        Self::bounded("tool pattern", value).map(Self::Tool)
    }

    /// Creates a command constraint after rejecting blank or overlong
    /// patterns.
    pub fn command(value: impl Into<String>) -> Result<Self, IdiomError> {
        Self::bounded("command pattern", value).map(Self::Command)
    }
}

/// Where an idiom record came from (Contract 055).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Provenance {
    /// A written rule with a source reference.
    Static(BoundedText),
    /// Learned from signals with the count and latest signal instant.
    Learned {
        /// Number of signals that shaped the record.
        signal_count: u64,
        /// Latest signal instant for the record.
        last_signal: MonotonicInstant,
    },
    /// Pulled from a registry package with its merge base.
    Imported {
        /// Package reference the record came from.
        package_ref: BoundedText,
        /// Merge base instant of the pulled package.
        merge_base: MonotonicInstant,
    },
}

/// One learned-preference record: constraint, confidence, provenance, scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Idiom {
    id: IdiomId,
    scope: IdiomScope,
    constraint: IdiomConstraint,
    confidence: Confidence,
    provenance: Provenance,
}

impl Idiom {
    /// Creates an idiom record, applying default decay to the confidence.
    pub fn new(
        id: IdiomId,
        scope: IdiomScope,
        constraint: IdiomConstraint,
        confidence_value: u8,
        as_of: MonotonicInstant,
        provenance: Provenance,
    ) -> Result<Self, IdiomError> {
        Self::with_half_life(
            id,
            scope,
            constraint,
            confidence_value,
            as_of,
            DEFAULT_DECAY_HALF_LIFE_TICKS,
            provenance,
        )
    }

    /// Creates an idiom record with an explicit confidence half-life.
    pub fn with_half_life(
        id: IdiomId,
        scope: IdiomScope,
        constraint: IdiomConstraint,
        confidence_value: u8,
        as_of: MonotonicInstant,
        half_life_ticks: u64,
        provenance: Provenance,
    ) -> Result<Self, IdiomError> {
        let confidence =
            require_confidence("confidence", confidence_value, as_of, half_life_ticks)?;
        Ok(Self {
            id,
            scope,
            constraint,
            confidence,
            provenance,
        })
    }

    /// Returns the stable idiom identity.
    #[must_use]
    pub const fn id(&self) -> &IdiomId {
        &self.id
    }

    /// Returns the scope this idiom applies to.
    #[must_use]
    pub const fn scope(&self) -> &IdiomScope {
        &self.scope
    }

    /// Returns the typed constraint this idiom enforces.
    #[must_use]
    pub const fn constraint(&self) -> &IdiomConstraint {
        &self.constraint
    }

    /// Returns the stored confidence record.
    #[must_use]
    pub const fn confidence(&self) -> Confidence {
        self.confidence
    }

    /// Returns the effective confidence at an evaluation instant.
    #[must_use]
    pub fn effective_confidence(&self, at: MonotonicInstant) -> u8 {
        self.confidence.effective(at)
    }

    /// Returns the provenance of this record.
    #[must_use]
    pub const fn provenance(&self) -> &Provenance {
        &self.provenance
    }
}

#[cfg(test)]
impl Idiom {
    /// Builds a record without validation so lint fixtures can exercise
    /// shapes constructors reject.
    pub(crate) fn unchecked(
        id: IdiomId,
        scope: IdiomScope,
        constraint: IdiomConstraint,
        confidence: Confidence,
        provenance: Provenance,
    ) -> Self {
        Self {
            id,
            scope,
            constraint,
            confidence,
            provenance,
        }
    }
}

/// What one interaction signal reported about a suggestion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalKind {
    /// The suggestion was accepted.
    Accept,
    /// The suggestion was rejected.
    Reject,
    /// The suggestion was edited after acceptance.
    Edit,
}

/// One bounded interaction signal for idiom learning (Contract 055).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdiomSignal {
    kind: SignalKind,
    target: BoundedText,
    correlation: BoundedText,
    scope: IdiomScope,
    sequence: u64,
    observed_at: MonotonicInstant,
}

impl IdiomSignal {
    /// Creates a bounded signal record.
    pub fn new(
        kind: SignalKind,
        target: impl Into<String>,
        correlation: impl Into<String>,
        scope: IdiomScope,
        sequence: u64,
        observed_at: MonotonicInstant,
    ) -> Result<Self, IdiomError> {
        Ok(Self {
            kind,
            target: require_bounded("signal target", target, DEFAULT_MAX_TARGET_BYTES)?,
            correlation: require_bounded(
                "signal correlation",
                correlation,
                DEFAULT_MAX_CORRELATION_BYTES,
            )?,
            scope,
            sequence,
            observed_at,
        })
    }

    /// Returns the signal kind.
    #[must_use]
    pub const fn kind(&self) -> SignalKind {
        self.kind
    }

    /// Returns the bounded redacted target of the signal.
    #[must_use]
    pub fn target(&self) -> &str {
        self.target.as_str()
    }

    /// Returns the opaque session correlation for the signal.
    #[must_use]
    pub fn correlation(&self) -> &str {
        self.correlation.as_str()
    }

    /// Returns the scope the signal belongs to.
    #[must_use]
    pub const fn scope(&self) -> &IdiomScope {
        &self.scope
    }

    /// Returns the per-store signal sequence number.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the instant the signal was observed.
    #[must_use]
    pub const fn observed_at(&self) -> MonotonicInstant {
        self.observed_at
    }
}
