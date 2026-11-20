//! Axis-aware lookup of the preflight-bound Codex CLI version.
//!
//! A plan may bind interface versions for several axes. Taking the first
//! binding regardless of its axis reads the wrong version as soon as another
//! axis is present, so every Codex lookup goes through this helper.

use crate::CODEX_CLI_AXIS;
use swallowtail_core::{InterfaceVersionBinding, PreflightPlan};

/// Returns the Codex CLI version binding, ignoring bindings on other axes.
pub(crate) fn bound_codex_cli_version(plan: &PreflightPlan) -> Option<&InterfaceVersionBinding> {
    plan.interface_versions()
        .find(|binding| binding.axis().as_str() == CODEX_CLI_AXIS)
}
