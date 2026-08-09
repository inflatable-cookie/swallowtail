use crate::confidence::Confidence;
use crate::record::{Idiom, IdiomConstraint, IdiomScope, Provenance};

/// One lint finding on an idiom record or store (Contract 055).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LintReason {
    /// A stored confidence value is outside `0..=100`.
    ConfidenceOutOfRange,
    /// A stored decay half-life is zero.
    ZeroHalfLife,
    /// An id is blank.
    IdBlank,
    /// A constraint pattern is blank.
    PatternBlank,
    /// A package scope name is blank.
    PackageNameBlank,
    /// A static provenance source reference is blank.
    SourceBlank,
    /// An imported provenance package reference is blank.
    PackageRefBlank,
}

/// A lint issue located by record index in a store.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LintIssue {
    index: usize,
    reason: LintReason,
}

impl LintIssue {
    /// Creates an issue for one store index and reason.
    pub(crate) const fn new(index: usize, reason: LintReason) -> Self {
        Self { index, reason }
    }

    /// Returns the record index the issue refers to.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the reason for the issue.
    #[must_use]
    pub const fn reason(&self) -> LintReason {
        self.reason
    }
}

fn lint_confidence(confidence: Confidence, issues: &mut Vec<LintReason>) {
    if confidence.value() > 100 {
        issues.push(LintReason::ConfidenceOutOfRange);
    }
    if confidence.half_life_ticks() == 0 {
        issues.push(LintReason::ZeroHalfLife);
    }
}

/// Lints one idiom record without a store position.
///
/// Constructors reject these shapes, so a non-empty result means the record
/// bypassed validation (for example a parsed registry payload).
#[must_use]
pub fn lint_idiom(record: &Idiom) -> Vec<LintReason> {
    let mut issues = Vec::new();
    if record.id().as_str().trim().is_empty() {
        issues.push(LintReason::IdBlank);
    }
    match record.constraint() {
        IdiomConstraint::Text(pattern)
        | IdiomConstraint::File(pattern)
        | IdiomConstraint::Tool(pattern)
        | IdiomConstraint::Command(pattern) => {
            if pattern.as_str().trim().is_empty() {
                issues.push(LintReason::PatternBlank);
            }
        }
    }
    if let IdiomScope::Package(name) = record.scope()
        && name.as_str().trim().is_empty()
    {
        issues.push(LintReason::PackageNameBlank);
    }
    match record.provenance() {
        Provenance::Static(source) => {
            if source.as_str().trim().is_empty() {
                issues.push(LintReason::SourceBlank);
            }
        }
        Provenance::Imported { package_ref, .. } => {
            if package_ref.as_str().trim().is_empty() {
                issues.push(LintReason::PackageRefBlank);
            }
        }
        Provenance::Learned { .. } => {}
    }
    lint_confidence(record.confidence(), &mut issues);
    issues
}

/// Lints a whole store and returns issues with their record indexes.
///
/// A malformed record is reported, never silently accepted. A record with
/// no issues is valid for the store.
#[must_use]
pub fn lint_store(records: &[Idiom]) -> Vec<LintIssue> {
    records
        .iter()
        .enumerate()
        .flat_map(|(index, record)| {
            lint_idiom(record)
                .into_iter()
                .map(move |reason| LintIssue { index, reason })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{LintReason, lint_idiom, lint_store};
    use crate::bounded::BoundedText;
    use crate::confidence::Confidence;
    use crate::record::{Idiom, IdiomConstraint, IdiomId, IdiomScope, Provenance};
    use crate::time::MonotonicInstant;

    fn at(ticks: u64) -> MonotonicInstant {
        MonotonicInstant::from_ticks(ticks)
    }

    fn static_source() -> Provenance {
        Provenance::Static(BoundedText::new("team rules", 256).expect("bounded source"))
    }

    fn clean_idiom() -> Idiom {
        Idiom::new(
            IdiomId::new("a").expect("id"),
            IdiomScope::Project,
            IdiomConstraint::text("use named exports").expect("constraint"),
            80,
            at(0),
            static_source(),
        )
        .expect("idiom")
    }

    fn unchecked_idiom(confidence_value: u8) -> Idiom {
        let confidence = Confidence::unchecked(confidence_value, at(0), 1);
        Idiom::unchecked(
            IdiomId::new("a").expect("id"),
            IdiomScope::Project,
            IdiomConstraint::text("use named exports").expect("constraint"),
            confidence,
            static_source(),
        )
    }

    #[test]
    fn clean_record_lints_empty() {
        assert!(lint_idiom(&clean_idiom()).is_empty());
        assert!(lint_store(&[clean_idiom()]).is_empty());
    }

    #[test]
    fn malformed_confidence_is_reported() {
        let record = unchecked_idiom(101);
        let issues = lint_idiom(&record);
        assert!(issues.contains(&LintReason::ConfidenceOutOfRange));
    }

    #[test]
    fn zero_half_life_is_reported() {
        let mut record = clean_idiom();
        record = Idiom::unchecked(
            record.id().clone(),
            record.scope().clone(),
            record.constraint().clone(),
            Confidence::unchecked(80, at(0), 0),
            record.provenance().clone(),
        );
        let issues = lint_idiom(&record);
        assert!(issues.contains(&LintReason::ZeroHalfLife));
    }

    #[test]
    fn store_reports_indexed_issues() {
        let mut records = vec![clean_idiom()];
        records.push(unchecked_idiom(101));
        let issues = lint_store(&records);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].index(), 1);
        assert_eq!(issues[0].reason(), LintReason::ConfidenceOutOfRange);
    }
}
