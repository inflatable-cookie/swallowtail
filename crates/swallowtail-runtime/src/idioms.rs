use swallowtail_idioms::{IdiomContext, IdiomScope, IdiomSet, Provenance};

use crate::content::OperationContent;
use crate::host_registry::HostServices;
use crate::session_options::SessionOptions;

/// Default maximum bytes for one folded idioms block.
pub const DEFAULT_MAX_FOLD_BYTES: usize = 8192;

/// Bounded idioms selection option bound into a prepared plan (Contract
/// 056).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdiomSessionOption {
    scope: IdiomScope,
    maximum: usize,
}

impl IdiomSessionOption {
    /// Creates an idioms option after rejecting a zero maximum.
    pub fn new(scope: IdiomScope, maximum: usize) -> Result<Self, ZeroIdiomMaximum> {
        if maximum == 0 {
            return Err(ZeroIdiomMaximum);
        }
        Ok(Self { scope, maximum })
    }

    /// Returns the session scope used for selection.
    #[must_use]
    pub const fn scope(&self) -> &IdiomScope {
        &self.scope
    }

    /// Returns the maximum number of idioms the session may receive.
    #[must_use]
    pub const fn maximum(&self) -> usize {
        self.maximum
    }
}

/// An idioms session option requested a zero maximum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZeroIdiomMaximum;

impl std::fmt::Display for ZeroIdiomMaximum {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("idiom session option maximum must be greater than zero")
    }
}

impl std::error::Error for ZeroIdiomMaximum {}

/// The idioms opt-in was requested but no source is registered on the host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdiomSourceUnavailable;

impl std::fmt::Display for IdiomSourceUnavailable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("idioms session option requires a registered idiom source")
    }
}

impl std::error::Error for IdiomSourceUnavailable {}

fn scope_label(scope: &IdiomScope) -> String {
    match scope {
        IdiomScope::User => "user".to_owned(),
        IdiomScope::Project => "project".to_owned(),
        IdiomScope::Package(name) => format!("package:{}", name.as_str()),
    }
}

fn provenance_label(provenance: &Provenance) -> &'static str {
    match provenance {
        Provenance::Static(_) => "static",
        Provenance::Learned { .. } => "learned",
        Provenance::Imported { .. } => "imported",
    }
}

fn constraint_line(constraint: &swallowtail_idioms::IdiomConstraint) -> String {
    use swallowtail_idioms::IdiomConstraint;
    match constraint {
        IdiomConstraint::Text(text) => text.as_str().to_owned(),
        IdiomConstraint::File(pattern) => format!("file: {}", pattern.as_str()),
        IdiomConstraint::Tool(pattern) => format!("tool: {}", pattern.as_str()),
        IdiomConstraint::Command(pattern) => format!("command: {}", pattern.as_str()),
    }
}

/// Renders one idiom as a bounded labeled line.
fn render_line(idiom: &swallowtail_idioms::Idiom) -> String {
    format!(
        "[{} {}] {}",
        scope_label(idiom.scope()),
        provenance_label(idiom.provenance()),
        constraint_line(idiom.constraint()),
    )
}

/// Renders a selected set into one bounded idioms block.
///
/// Each idiom becomes one labeled line; overflow truncates with an explicit
/// marker. Pure and deterministic (Contract 056).
#[must_use]
pub fn fold_idioms(set: &IdiomSet) -> String {
    fold_idioms_with_bound(set, DEFAULT_MAX_FOLD_BYTES)
}

/// Explicit marker appended when a folded block overflows its byte bound.
const TRUNCATION_MARKER: &str = "… [idioms truncated]";

/// Renders a selected set into one idioms block with an explicit byte bound.
///
/// The returned block never exceeds `maximum_bytes`, including its truncation
/// marker: when no room remains for the marker, the block is cut short rather
/// than overshooting the bound. A first-line overflow adds no leading newline.
#[must_use]
pub fn fold_idioms_with_bound(set: &IdiomSet, maximum_bytes: usize) -> String {
    let mut output = String::new();
    for idiom in set.idioms() {
        let line = render_line(idiom);
        let separator = usize::from(!output.is_empty());
        if output.len() + separator + line.len() <= maximum_bytes {
            if separator == 1 {
                output.push('\n');
            }
            output.push_str(&line);
            continue;
        }
        push_truncation_marker(&mut output, maximum_bytes);
        break;
    }
    output
}

/// Appends the truncation marker inside the remaining byte budget.
///
/// The marker is only appended when it fits whole; a marker that would exceed
/// the bound is omitted, so the bound always wins over the diagnostic.
fn push_truncation_marker(output: &mut String, maximum_bytes: usize) {
    let separator = usize::from(!output.is_empty());
    let needed = output.len() + separator + TRUNCATION_MARKER.len();
    if needed <= maximum_bytes {
        if separator == 1 {
            output.push('\n');
        }
        output.push_str(TRUNCATION_MARKER);
    }
}

/// Appends a folded idioms block after consumer-supplied developer
/// instructions under one labeled block.
///
/// Consumer instructions stay first and unchanged; the idioms block is
/// appended only when it is non-empty (Contract 056).
#[must_use]
pub fn append_folded_idioms(
    instructions: Option<OperationContent>,
    folded: String,
) -> Option<OperationContent> {
    if folded.is_empty() {
        return instructions;
    }
    let block = format!("\n\n[idioms]\n{folded}");
    match instructions {
        Some(instructions) => {
            let mut combined = instructions.into_string();
            combined.push_str(&block);
            OperationContent::new(combined).ok()
        }
        None => OperationContent::new(block.trim_start().to_owned()).ok(),
    }
}

/// Resolves the complete developer-instructions value for one session.
///
/// Without an idioms option this passes the consumer-supplied instructions
/// through unchanged. With the opt-in, the registered source must exist and
/// the selected set is folded after the consumer text under the fixed rule.
/// Missing source fails closed before any provider work (Contract 056).
pub fn resolve_idiom_instructions(
    services: &HostServices,
    options: &SessionOptions,
) -> Result<Option<OperationContent>, IdiomSourceUnavailable> {
    let Some(idiom_option) = options.idioms() else {
        return Ok(options.developer_instructions().cloned());
    };
    let Some(source) = services.idiom_source() else {
        return Err(IdiomSourceUnavailable);
    };
    let now_ticks = services.time().map(|time| time.now().ticks()).unwrap_or(0);
    let now = swallowtail_idioms::MonotonicInstant::from_ticks(now_ticks);
    let context = IdiomContext::new(idiom_option.scope().clone(), now, idiom_option.maximum());
    let folded = fold_idioms(&source.select(&context));
    Ok(append_folded_idioms(
        options.developer_instructions().cloned(),
        folded,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        IdiomSessionOption, ZeroIdiomMaximum, append_folded_idioms, fold_idioms,
        fold_idioms_with_bound,
    };
    use crate::content::OperationContent;
    use crate::host_registry::HostServices;
    use swallowtail_idioms::{
        BoundedText, Idiom, IdiomConstraint, IdiomId, IdiomScope, IdiomSet, MonotonicInstant,
        Provenance,
    };

    fn at(ticks: u64) -> MonotonicInstant {
        MonotonicInstant::from_ticks(ticks)
    }

    fn idiom(id: &str, scope: IdiomScope, constraint: IdiomConstraint, confidence: u8) -> Idiom {
        Idiom::new(
            IdiomId::new(id).expect("id"),
            scope,
            constraint,
            confidence,
            at(0),
            Provenance::Static(BoundedText::new("team rules", 256).expect("bounded source")),
        )
        .expect("idiom")
    }

    #[test]
    fn session_option_rejects_zero_maximum() {
        assert_eq!(
            IdiomSessionOption::new(IdiomScope::Project, 0).expect_err("zero maximum"),
            ZeroIdiomMaximum
        );
        assert_eq!(
            IdiomSessionOption::new(IdiomScope::Project, 8)
                .expect("option")
                .maximum(),
            8
        );
    }

    #[test]
    fn fold_renders_labeled_lines() {
        let set = IdiomSet::new(
            vec![
                idiom(
                    "a",
                    IdiomScope::Project,
                    IdiomConstraint::text("use named exports").expect("constraint"),
                    90,
                ),
                idiom(
                    "b",
                    IdiomScope::User,
                    IdiomConstraint::tool("edit_*").expect("constraint"),
                    80,
                ),
            ],
            false,
        );
        let folded = fold_idioms(&set);
        assert_eq!(
            folded,
            "[project static] use named exports\n[user static] tool: edit_*"
        );
    }

    #[test]
    fn fold_truncates_with_marker() {
        let set = IdiomSet::new(
            vec![
                idiom(
                    "a",
                    IdiomScope::Project,
                    IdiomConstraint::text("x".repeat(100)).expect("constraint"),
                    90,
                ),
                idiom(
                    "b",
                    IdiomScope::Project,
                    IdiomConstraint::text("y".repeat(100)).expect("constraint"),
                    80,
                ),
            ],
            false,
        );
        let folded = fold_idioms_with_bound(&set, 64);
        assert!(folded.ends_with("… [idioms truncated]"));
        assert!(folded.len() <= 64 + "… [idioms truncated]".len() + 1);
    }

    #[test]
    fn a_first_line_overflow_adds_no_leading_newline_and_keeps_the_bound() {
        let set = IdiomSet::new(
            vec![idiom(
                "a",
                IdiomScope::Project,
                IdiomConstraint::text("use named exports").expect("constraint"),
                90,
            )],
            false,
        );

        let marker_only = fold_idioms_with_bound(&set, 30);
        assert_eq!(marker_only, "… [idioms truncated]");
        assert!(!marker_only.starts_with('\n'));

        let no_room = fold_idioms_with_bound(&set, 8);
        assert!(no_room.is_empty(), "no room means no overshooting marker");
        assert!(no_room.len() <= 8);
    }

    #[test]
    fn every_fold_stays_within_its_byte_bound() {
        let set = IdiomSet::new(
            vec![
                idiom(
                    "a",
                    IdiomScope::Project,
                    IdiomConstraint::text("use named exports").expect("constraint"),
                    90,
                ),
                idiom(
                    "b",
                    IdiomScope::User,
                    IdiomConstraint::tool("edit_*").expect("constraint"),
                    80,
                ),
            ],
            false,
        );
        for maximum_bytes in 0..64 {
            let folded = fold_idioms_with_bound(&set, maximum_bytes);
            assert!(
                folded.len() <= maximum_bytes,
                "bound {maximum_bytes} was exceeded by {} bytes",
                folded.len()
            );
            assert!(!folded.starts_with('\n'));
        }
    }

    #[test]
    fn a_later_line_overflow_keeps_the_marker_inside_the_bound() {
        let set = IdiomSet::new(
            vec![
                idiom(
                    "a",
                    IdiomScope::Project,
                    IdiomConstraint::text("first").expect("constraint"),
                    90,
                ),
                idiom(
                    "b",
                    IdiomScope::Project,
                    IdiomConstraint::text("y".repeat(100)).expect("constraint"),
                    80,
                ),
            ],
            false,
        );
        let folded = fold_idioms_with_bound(&set, 45);
        assert_eq!(folded, "[project static] first\n… [idioms truncated]");
        assert!(folded.len() <= 45);

        // A marker that cannot fit whole is omitted, so the bound still wins.
        let cut_short = fold_idioms_with_bound(&set, 40);
        assert_eq!(cut_short, "[project static] first");
        assert!(cut_short.len() <= 40);
    }

    #[test]
    fn resolve_without_the_opt_in_passes_consumer_instructions_through() {
        let services = HostServices::new(
            swallowtail_core::ExecutionHostId::new("fixture.host").expect("host id"),
        );
        let instructions = OperationContent::new("consumer guidance".to_owned()).expect("content");
        let options = crate::SessionOptions::default().with_developer_instructions(instructions);
        let resolved = super::resolve_idiom_instructions(&services, &options)
            .expect("no opt-in needs no source");
        assert_eq!(
            resolved.map(|content| content.as_str().to_owned()),
            Some("consumer guidance".to_owned())
        );
    }

    #[test]
    fn resolve_fails_closed_when_the_opt_in_has_no_source() {
        let services = HostServices::new(
            swallowtail_core::ExecutionHostId::new("fixture.host").expect("host id"),
        );
        let options = crate::SessionOptions::default()
            .with_idioms(IdiomSessionOption::new(IdiomScope::Project, 8).expect("option"));
        assert_eq!(
            super::resolve_idiom_instructions(&services, &options).expect_err("no source"),
            super::IdiomSourceUnavailable
        );
    }

    #[test]
    fn resolve_appends_the_folded_block_after_consumer_text() {
        let set = IdiomSet::new(
            vec![idiom(
                "a",
                IdiomScope::Project,
                IdiomConstraint::text("use named exports").expect("constraint"),
                90,
            )],
            false,
        );
        let services = HostServices::new(
            swallowtail_core::ExecutionHostId::new("fixture.host").expect("host id"),
        )
        .with_idiom_source(std::sync::Arc::new(FixedSource(set)));
        let instructions = OperationContent::new("consumer guidance".to_owned()).expect("content");
        let options = crate::SessionOptions::default()
            .with_developer_instructions(instructions)
            .with_idioms(IdiomSessionOption::new(IdiomScope::Project, 8).expect("option"));
        let resolved = super::resolve_idiom_instructions(&services, &options)
            .expect("a registered source resolves");
        assert_eq!(
            resolved.map(|content| content.as_str().to_owned()),
            Some("consumer guidance\n\n[idioms]\n[project static] use named exports".to_owned())
        );
    }

    struct FixedSource(IdiomSet);

    impl swallowtail_idioms::IdiomSource for FixedSource {
        fn select(&self, _context: &swallowtail_idioms::IdiomContext) -> IdiomSet {
            self.0.clone()
        }
    }

    #[test]
    fn append_keeps_consumer_instructions_first() {
        let instructions = OperationContent::new("consumer guidance".to_owned()).expect("content");
        let set = IdiomSet::new(
            vec![idiom(
                "a",
                IdiomScope::Project,
                IdiomConstraint::text("use named exports").expect("constraint"),
                90,
            )],
            false,
        );
        let combined =
            append_folded_idioms(Some(instructions), fold_idioms(&set)).expect("combined content");
        assert_eq!(
            combined.as_str(),
            "consumer guidance\n\n[idioms]\n[project static] use named exports"
        );
    }

    #[test]
    fn append_skips_empty_fold_and_absent_instructions() {
        let empty = append_folded_idioms(None, String::new());
        assert!(empty.is_none());

        let folded = fold_idioms(&IdiomSet::default());
        assert!(folded.is_empty());
        let none = append_folded_idioms(None, folded);
        assert!(none.is_none());

        let only_idioms = IdiomSet::new(
            vec![idiom(
                "a",
                IdiomScope::Project,
                IdiomConstraint::text("use named exports").expect("constraint"),
                90,
            )],
            false,
        );
        let combined = append_folded_idioms(None, fold_idioms(&only_idioms)).expect("content");
        assert_eq!(
            combined.as_str(),
            "[idioms]\n[project static] use named exports"
        );
    }
}
