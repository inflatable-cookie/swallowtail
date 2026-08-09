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

/// Renders a selected set into one idioms block with an explicit byte bound.
#[must_use]
pub fn fold_idioms_with_bound(set: &IdiomSet, maximum_bytes: usize) -> String {
    let mut output = String::new();
    for idiom in set.idioms() {
        let line = render_line(idiom);
        let needed = if output.is_empty() {
            line.len()
        } else {
            output.len() + 1 + line.len()
        };
        if needed > maximum_bytes {
            output.push_str("\n… [idioms truncated]");
            break;
        }
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(&line);
    }
    output
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
