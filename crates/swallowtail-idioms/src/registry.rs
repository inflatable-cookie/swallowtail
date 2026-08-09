use crate::bounded::{BoundedText, DEFAULT_MAX_ID_BYTES};
use crate::error::{IdiomError, require_bounded};
use crate::lint::{LintIssue, lint_idiom};
use crate::merge::MergeOutcome;
use crate::record::{Idiom, IdiomScope};
use crate::time::MonotonicInstant;

/// Registry namespace owning one set of packages.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegistryNamespace(BoundedText);

impl RegistryNamespace {
    /// Creates a namespace after rejecting blank or overlong text.
    pub fn new(value: impl Into<String>) -> Result<Self, IdiomError> {
        require_bounded("registry namespace", value, DEFAULT_MAX_ID_BYTES).map(Self)
    }

    /// Returns the namespace text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// One named registry package inside a namespace.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegistryPackageRef {
    namespace: RegistryNamespace,
    name: BoundedText,
}

impl RegistryPackageRef {
    /// Creates a package reference after rejecting blank or overlong names.
    pub fn new(namespace: RegistryNamespace, name: impl Into<String>) -> Result<Self, IdiomError> {
        let name = require_bounded("registry package name", name, DEFAULT_MAX_ID_BYTES)?;
        Ok(Self { namespace, name })
    }

    /// Returns the owning namespace.
    #[must_use]
    pub const fn namespace(&self) -> &RegistryNamespace {
        &self.namespace
    }

    /// Returns the package name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl std::fmt::Display for RegistryPackageRef {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.namespace.as_str())?;
        formatter.write_str("/")?;
        formatter.write_str(self.name.as_str())
    }
}

/// One portable registry package: a reference and its idiom records.
///
/// Construction fails closed when any record is malformed; a registry
/// payload never bypasses lint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryPackage {
    reference: RegistryPackageRef,
    records: Vec<Idiom>,
}

impl RegistryPackage {
    /// Creates a package after lint-validating every record.
    pub fn new(reference: RegistryPackageRef, records: Vec<Idiom>) -> Result<Self, LintIssue> {
        for (index, record) in records.iter().enumerate() {
            if let Some(reason) = lint_idiom(record).into_iter().next() {
                return Err(LintIssue::new(index, reason));
            }
        }
        Ok(Self { reference, records })
    }

    /// Returns the package reference.
    #[must_use]
    pub const fn reference(&self) -> &RegistryPackageRef {
        &self.reference
    }

    /// Returns the lint-clean records in the package.
    #[must_use]
    pub fn records(&self) -> &[Idiom] {
        &self.records
    }
}

/// Deterministic per-id merge summary for one store merge.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MergeSummary {
    /// Records added to the store.
    pub new: usize,
    /// Records whose effective confidence rose.
    pub raised: usize,
    /// Records whose effective confidence fell.
    pub lowered: usize,
    /// Records whose effective confidence stayed the same.
    pub unchanged: usize,
}

impl MergeSummary {
    /// Creates an empty summary.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            new: 0,
            raised: 0,
            lowered: 0,
            unchanged: 0,
        }
    }
}

/// Outcome of one registry merge: the resulting store and its summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryMergeOutcome {
    store: Vec<Idiom>,
    summary: MergeSummary,
}

impl RegistryMergeOutcome {
    /// Creates an outcome from a store and its merge summary.
    #[must_use]
    pub const fn new(store: Vec<Idiom>, summary: MergeSummary) -> Self {
        Self { store, summary }
    }

    /// Returns the resulting store.
    #[must_use]
    pub fn store(&self) -> &[Idiom] {
        &self.store
    }

    /// Returns the merge summary for the store.
    #[must_use]
    pub const fn summary(&self) -> MergeSummary {
        self.summary
    }
}

fn store_key(record: &Idiom) -> (IdiomScope, String) {
    (record.scope().clone(), record.id().as_str().to_owned())
}

fn merge_store(
    local: &[Idiom],
    incoming: &[Idiom],
    at: MonotonicInstant,
) -> (Vec<Idiom>, MergeSummary) {
    // Incoming records apply in package order: each merges against the
    // store state after the previous incoming record.
    let mut store: Vec<Idiom> = local.to_vec();
    let mut summary = MergeSummary::empty();
    for incoming_record in incoming {
        let key = store_key(incoming_record);
        let existing = store.iter().find(|record| store_key(record) == key);
        let (merged, outcome) = crate::merge::merge(existing, incoming_record.clone(), at);
        match outcome {
            MergeOutcome::New => {
                store.push(merged);
                summary.new += 1;
            }
            MergeOutcome::Raised | MergeOutcome::Lowered => {
                if let Some(slot) = store.iter_mut().find(|record| store_key(record) == key) {
                    *slot = merged;
                }
                if outcome == MergeOutcome::Raised {
                    summary.raised += 1;
                } else {
                    summary.lowered += 1;
                }
            }
            MergeOutcome::Unchanged => summary.unchanged += 1,
        }
    }
    (store, summary)
}

/// Pulls one registry package into a local store (Contract 055).
///
/// Remote learnings merge into the local store keyed by scope and idiom id,
/// following the confidence merge outcomes. Pure: transport, auth, and wire
/// bounds remain host-owned.
#[must_use]
pub fn pull_package(
    local: &[Idiom],
    package: &RegistryPackage,
    at: MonotonicInstant,
) -> RegistryMergeOutcome {
    let (store, summary) = merge_store(local, package.records(), at);
    RegistryMergeOutcome::new(store, summary)
}

/// Pushes a local store into a remote store (Contract 055).
///
/// Local learnings merge into the remote store keyed by scope and idiom id,
/// following the confidence merge outcomes; the caller transports the
/// resulting store. Pure: transport, auth, and wire bounds remain
/// host-owned.
#[must_use]
pub fn push_merge(local: &[Idiom], remote: &[Idiom], at: MonotonicInstant) -> RegistryMergeOutcome {
    let (store, summary) = merge_store(remote, local, at);
    RegistryMergeOutcome::new(store, summary)
}

#[cfg(test)]
mod tests {
    use super::{
        MergeSummary, RegistryNamespace, RegistryPackage, RegistryPackageRef, pull_package,
        push_merge,
    };
    use crate::bounded::{BoundedText, DEFAULT_MAX_PATTERN_BYTES};
    use crate::record::{Idiom, IdiomConstraint, IdiomId, IdiomScope, Provenance};
    use crate::time::MonotonicInstant;

    fn at(ticks: u64) -> MonotonicInstant {
        MonotonicInstant::from_ticks(ticks)
    }

    fn static_source() -> Provenance {
        Provenance::Static(BoundedText::new("team rules", 256).expect("bounded source"))
    }

    fn idiom(id: &str, scope: IdiomScope, confidence: u8) -> Idiom {
        Idiom::new(
            IdiomId::new(id).expect("id"),
            scope,
            IdiomConstraint::text("use named exports").expect("constraint"),
            confidence,
            at(0),
            static_source(),
        )
        .expect("idiom")
    }

    fn reference(name: &str) -> RegistryPackageRef {
        RegistryPackageRef::new(RegistryNamespace::new("myorg").expect("namespace"), name)
            .expect("reference")
    }

    #[test]
    fn package_rejects_malformed_records() {
        let mut malformed = idiom("a", IdiomScope::Project, 80);
        malformed = Idiom::unchecked(
            malformed.id().clone(),
            malformed.scope().clone(),
            malformed.constraint().clone(),
            crate::confidence::Confidence::unchecked(101, at(0), 1),
            malformed.provenance().clone(),
        );
        let error = RegistryPackage::new(reference("broken"), vec![malformed])
            .expect_err("malformed package must fail closed");
        assert_eq!(error.index(), 0);
        assert_eq!(
            error.reason(),
            crate::lint::LintReason::ConfidenceOutOfRange
        );
    }

    #[test]
    fn pull_adds_new_records_and_reports_new() {
        let local = vec![idiom("a", IdiomScope::Project, 80)];
        let package =
            RegistryPackage::new(reference("cli"), vec![idiom("b", IdiomScope::Project, 60)])
                .expect("package");
        let outcome = pull_package(&local, &package, at(0));
        assert_eq!(outcome.store().len(), 2);
        assert_eq!(
            outcome.summary(),
            MergeSummary {
                new: 1,
                ..MergeSummary::empty()
            }
        );
    }

    #[test]
    fn pull_applies_incoming_records_in_package_order() {
        let local = vec![idiom("a", IdiomScope::Project, 80)];
        let package = RegistryPackage::new(
            reference("cli"),
            vec![
                idiom("a", IdiomScope::Project, 95),
                idiom("a", IdiomScope::Project, 40),
                idiom("a", IdiomScope::Project, 80),
            ],
        )
        .expect("package");
        let outcome = pull_package(&local, &package, at(0));
        assert_eq!(outcome.store().len(), 1);
        assert_eq!(
            outcome.summary(),
            MergeSummary {
                raised: 2,
                lowered: 1,
                unchanged: 0,
                ..MergeSummary::empty()
            }
        );
        assert_eq!(outcome.store()[0].confidence().value(), 80);
    }

    #[test]
    fn same_id_in_different_scopes_stays_distinct() {
        let local = vec![idiom("a", IdiomScope::Project, 80)];
        let package =
            RegistryPackage::new(reference("cli"), vec![idiom("a", IdiomScope::User, 90)])
                .expect("package");
        let outcome = pull_package(&local, &package, at(0));
        assert_eq!(outcome.store().len(), 2, "scope and id key the store");
        assert_eq!(outcome.summary().new, 1);
    }

    #[test]
    fn push_merges_local_into_remote() {
        let local = vec![
            idiom("a", IdiomScope::Project, 95),
            idiom("b", IdiomScope::User, 10),
        ];
        let remote = vec![idiom("a", IdiomScope::Project, 80)];
        let outcome = push_merge(&local, &remote, at(0));
        assert_eq!(outcome.store().len(), 2);
        assert_eq!(outcome.summary().raised, 1);
        assert_eq!(outcome.summary().new, 1);
    }

    #[test]
    fn package_reference_formats_and_bounds() {
        let package_ref = reference("cli");
        assert_eq!(package_ref.to_string(), "myorg/cli");
        assert!(RegistryNamespace::new("  ").is_err());
        assert!(
            RegistryPackageRef::new(
                RegistryNamespace::new("myorg").expect("namespace"),
                "x".repeat(DEFAULT_MAX_PATTERN_BYTES + 1),
            )
            .is_err()
        );
    }
}
