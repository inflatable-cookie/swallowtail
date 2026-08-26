/// Exact admitted `--reasoning` selection for owned llama.cpp serving.
///
/// This is adapter-local serving configuration on exact owned `b10069`. It is
/// not a portable reasoning capability, a model reasoning claim, an inference
/// control, or an `llama-cpp.attached` request option.
///
/// Research 225 admits one value. `on` and `auto` are withheld: exact tagged
/// source stores `auto` as the parser default, and the only `on` behavior that
/// differs from the default lives inside a per-request chat-template render
/// this serving route can neither predict nor observe. `--reasoning-budget` is
/// withheld entirely because exact source discards it without a template
/// thinking end tag, and no prompt-free channel reports that tag.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LlamaCppReasoningSelection {
    /// Dispatches `--reasoning off`.
    ///
    /// Exact `b10069` stores `enable_reasoning = 0`, which short-circuits
    /// `enable_thinking` to `false` before the chat template is probed, and
    /// sets the template argument `enable_thinking = false`. That applied
    /// server state is template-independent, so no model or chat-template fact
    /// has to be known before process work.
    ///
    /// Effective and observed reasoning behavior stay withheld. A chat
    /// template need not honor the render variable, and a consumer request may
    /// override it through `chat_template_kwargs`.
    Disabled,
}

impl LlamaCppReasoningSelection {
    /// Returns the exact canonical literal emitted after `--reasoning`.
    ///
    /// Exact `b10069` also accepts `disabled`, `false`, and `0` as falsey
    /// aliases. Canonical argv emits `off` only.
    #[must_use]
    pub const fn as_argument_value(self) -> &'static str {
        match self {
            Self::Disabled => "off",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_dispatches_the_canonical_falsey_literal() {
        assert_eq!(
            LlamaCppReasoningSelection::Disabled.as_argument_value(),
            "off"
        );
    }
}
