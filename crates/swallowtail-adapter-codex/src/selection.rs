use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceCompatibilityMatch, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

pub const CODEX_CLI_AXIS: &str = "codex.cli";
pub const CODEX_EXEC_BASELINE_VERSION: &str = "0.80.0";
pub const CODEX_APP_SERVER_BASELINE_VERSION: &str = "0.80.0";
pub const CODEX_LATEST_QUALIFIED_VERSION: &str = "0.145.0";
pub(crate) const CODEX_APP_SERVER_WORKSPACE_ROOTS_VERSION: &str = "0.131.0";
const CODEX_EXEC_RETAINED_BOOLEAN_SEARCH_BEHAVIOR: &str =
    "codex.exec.jsonl-v1.retained-boolean-search";
const CODEX_EXEC_RETAINED_SEARCH_MODE_BEHAVIOR: &str = "codex.exec.jsonl-v1.retained-search-mode";
const CODEX_EXEC_EPHEMERAL_AMBIENT_BEHAVIOR: &str = "codex.exec.jsonl-v1.ephemeral-ambient";
const CODEX_EXEC_BEHAVIOR: &str = "codex.exec.jsonl-v1";
const CODEX_APP_SERVER_LEGACY_DEFAULT_BEHAVIOR: &str = "codex.app-server.v2.legacy-default-stdio";
const CODEX_APP_SERVER_LEGACY_EXPLICIT_BEHAVIOR: &str = "codex.app-server.v2.legacy-explicit-stdio";
pub(crate) const CODEX_APP_SERVER_BASE_BEHAVIOR: &str = "codex.app-server.v2.base";
pub(crate) const CODEX_APP_SERVER_WORKSPACE_BEHAVIOR: &str = "codex.app-server.v2.workspace-roots";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CodexExecBehavior {
    RetainedBooleanSearch,
    RetainedSearchMode,
    EphemeralAmbient,
    EphemeralSuppressed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CodexAppServerBehavior {
    LegacyDefaultStdio,
    LegacyExplicitStdio,
    CurrentBase,
    CurrentWorkspaceRoots,
}

impl CodexAppServerBehavior {
    pub(crate) const fn is_legacy(self) -> bool {
        matches!(self, Self::LegacyDefaultStdio | Self::LegacyExplicitStdio)
    }

    pub(crate) const fn supports_workspace_roots(self) -> bool {
        matches!(self, Self::CurrentWorkspaceRoots)
    }

    pub(crate) fn invocation(self) -> Vec<String> {
        if self == Self::LegacyDefaultStdio {
            vec!["app-server".to_owned()]
        } else {
            vec![
                "app-server".to_owned(),
                "--listen".to_owned(),
                "stdio://".to_owned(),
            ]
        }
    }
}

#[must_use]
pub fn codex_cli_binding(version: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(CODEX_CLI_AXIS).expect("static Codex axis is valid"),
        InterfaceVersion::new(version).expect("Codex version is required"),
    )
}

#[must_use]
pub fn codex_exec_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("codex.exec.cli-window-2")
            .expect("static claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        [
            segment(
                "0.80.0",
                "0.81.0",
                CODEX_EXEC_RETAINED_BOOLEAN_SEARCH_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.84.0",
                "0.98.0",
                CODEX_EXEC_RETAINED_SEARCH_MODE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.99.0",
                "0.121.0",
                CODEX_EXEC_EPHEMERAL_AMBIENT_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.122.0",
                CODEX_LATEST_QUALIFIED_VERSION,
                CODEX_EXEC_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [version("0.108.0"), version("0.109.0")],
    )
    .expect("static Codex exec claim is valid")
}

#[must_use]
pub fn codex_app_server_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("codex.app-server.cli-window-2")
            .expect("static claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        [
            segment(
                "0.80.0",
                "0.81.0",
                CODEX_APP_SERVER_LEGACY_DEFAULT_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.84.0",
                "0.99.0",
                CODEX_APP_SERVER_LEGACY_DEFAULT_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.100.0",
                "0.107.0",
                CODEX_APP_SERVER_LEGACY_EXPLICIT_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.110.0",
                "0.130.0",
                CODEX_APP_SERVER_BASE_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
            segment(
                CODEX_APP_SERVER_WORKSPACE_ROOTS_VERSION,
                CODEX_LATEST_QUALIFIED_VERSION,
                CODEX_APP_SERVER_WORKSPACE_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .expect("static Codex app-server claim is valid")
}

pub(crate) fn classify_exec_plan(
    plan: &PreflightPlan,
) -> Result<CodexExecBehavior, RuntimeFailure> {
    let matched = classify_plan(plan, &codex_exec_claim(), "exec")?;
    match matched.behavior_revision().as_str() {
        CODEX_EXEC_RETAINED_BOOLEAN_SEARCH_BEHAVIOR => Ok(CodexExecBehavior::RetainedBooleanSearch),
        CODEX_EXEC_RETAINED_SEARCH_MODE_BEHAVIOR => Ok(CodexExecBehavior::RetainedSearchMode),
        CODEX_EXEC_EPHEMERAL_AMBIENT_BEHAVIOR => Ok(CodexExecBehavior::EphemeralAmbient),
        CODEX_EXEC_BEHAVIOR => Ok(CodexExecBehavior::EphemeralSuppressed),
        _ => Err(super::exec::failure(
            "swallowtail.codex.exec.behavior_incompatible",
            "Codex executable behavior is not mapped by this driver",
        )),
    }
}

pub(crate) fn classify_app_server_plan(
    plan: &PreflightPlan,
) -> Result<CodexAppServerBehavior, RuntimeFailure> {
    let matched = classify_plan(plan, &codex_app_server_claim(), "app_server")?;
    match matched.behavior_revision().as_str() {
        CODEX_APP_SERVER_LEGACY_DEFAULT_BEHAVIOR => Ok(CodexAppServerBehavior::LegacyDefaultStdio),
        CODEX_APP_SERVER_LEGACY_EXPLICIT_BEHAVIOR => {
            Ok(CodexAppServerBehavior::LegacyExplicitStdio)
        }
        CODEX_APP_SERVER_BASE_BEHAVIOR => Ok(CodexAppServerBehavior::CurrentBase),
        CODEX_APP_SERVER_WORKSPACE_BEHAVIOR => Ok(CodexAppServerBehavior::CurrentWorkspaceRoots),
        _ => Err(super::exec::failure(
            "swallowtail.codex.app_server.behavior_incompatible",
            "Codex app-server behavior is not mapped by this driver",
        )),
    }
}

pub(crate) fn classify_plan(
    plan: &PreflightPlan,
    claim: &InterfaceCompatibilityClaim,
    diagnostic_prefix: &'static str,
) -> Result<InterfaceCompatibilityMatch, RuntimeFailure> {
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        super::exec::failure(
            diagnostic_code(diagnostic_prefix, "version_missing"),
            "Codex plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(super::exec::failure(
            diagnostic_code(diagnostic_prefix, "version_ambiguous"),
            "Codex plan contains more than one executable version",
        ));
    }
    claim.classify(binding.version()).ok_or_else(|| {
        super::exec::failure(
            diagnostic_code(diagnostic_prefix, "version_incompatible"),
            "Codex executable version is outside the qualified window",
        )
    })
}

fn diagnostic_code(prefix: &'static str, suffix: &'static str) -> &'static str {
    match (prefix, suffix) {
        ("exec", "version_missing") => "swallowtail.codex.exec.version_missing",
        ("exec", "version_ambiguous") => "swallowtail.codex.exec.version_ambiguous",
        ("exec", "version_incompatible") => "swallowtail.codex.exec.version_incompatible",
        ("app_server", "version_missing") => "swallowtail.codex.app_server.version_missing",
        ("app_server", "version_ambiguous") => "swallowtail.codex.app_server.version_ambiguous",
        ("app_server", "version_incompatible") => {
            "swallowtail.codex.app_server.version_incompatible"
        }
        _ => "swallowtail.codex.version_invalid",
    }
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(CODEX_CLI_AXIS).expect("static Codex axis is valid")
}

fn segment(
    minimum: &str,
    maximum: &str,
    behavior: &str,
    status: InterfaceSupportStatus,
) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        version(minimum),
        version(maximum),
        InterfaceBehaviorRevision::new(behavior).expect("static behavior is valid"),
        status,
    )
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("static Codex version is valid")
}

#[cfg(test)]
#[path = "selection_tests.rs"]
mod tests;
