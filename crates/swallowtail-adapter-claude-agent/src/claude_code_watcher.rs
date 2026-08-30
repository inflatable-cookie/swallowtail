//! Exact Claude Code `2.1.251` watcher opt-in composition.
//!
//! Omission keeps the current empty strict MCP command. Opt-in opens the
//! existing Contract 060 bridge and materializes operation-private files.

mod activity;
mod binding;
mod material;

pub(crate) use activity::WatcherActivityFeed;
pub(crate) use binding::{WatcherBinding, WatcherCommandFiles, open_binding};

use swallowtail_core::InterfaceVersionBinding;

/// Exact Claude Code version that may opt into the watcher candidate.
pub(crate) const CLAUDE_CODE_WATCHER_VERSION: &str = "2.1.251";

/// Reports whether one observed interface version may opt into watchers.
///
/// Only exact `2.1.251` on the headless axis is admitted. The route's
/// qualified window and `AllowUnverified` later stables are not a watcher
/// range.
pub(crate) fn admits(binding: &InterfaceVersionBinding) -> bool {
    binding.axis().as_str() == crate::CLAUDE_CODE_HEADLESS_AXIS
        && binding.version().as_str() == CLAUDE_CODE_WATCHER_VERSION
}

#[cfg(test)]
mod tests {
    use super::{CLAUDE_CODE_WATCHER_VERSION, admits};
    use crate::claude_code_headless_binding;

    #[test]
    fn only_exact_2_1_251_admits_watcher_opt_in() {
        assert!(admits(
            &claude_code_headless_binding(CLAUDE_CODE_WATCHER_VERSION).expect("binding")
        ));
        for version in ["2.1.241", "2.1.250", "2.1.252", "2.1.220"] {
            assert!(
                !admits(&claude_code_headless_binding(version).expect("binding")),
                "{version}"
            );
        }
    }
}
