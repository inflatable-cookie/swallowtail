//! Stream-JSON invocation prefix shared by the two Claude Code routes.
//!
//! Both routes run the same executable with `-p` over a text prompt and read
//! `stream-json` back, and both express an explicit reasoning effort the same
//! way. Only the tool and isolation flags that follow differ, so the shared
//! prefix and the effort rule live here once.

use swallowtail_core::{ModelId, ReasoningMode};

/// Builds the response-shape and model prefix every Claude Code route uses.
pub(crate) fn invocation_prefix(model: &ModelId, reasoning: Option<&ReasoningMode>) -> Vec<String> {
    let mut arguments = [
        "-p",
        "--input-format",
        "text",
        "--output-format",
        "stream-json",
        "--verbose",
        "--no-session-persistence",
        "--model",
        model.as_str(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if let Some(reasoning) = reasoning.filter(|mode| mode.as_str() != "default") {
        arguments.extend(["--effort".to_owned(), reasoning.as_str().to_owned()]);
    }
    arguments
}
