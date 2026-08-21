use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

mod lifecycle;

pub use lifecycle::codex_app_server_lifecycle_claim;
pub(crate) use lifecycle::{
    CodexLifecycleAssessment, CodexLifecycleBehavior, classify_lifecycle_version,
};

/// CLI-version interface axis shared by Codex routes.
pub const CODEX_CLI_AXIS: &str = "codex.cli";
/// Oldest qualified Codex exec version.
pub const CODEX_EXEC_BASELINE_VERSION: &str = "0.80.0";
/// Oldest qualified Codex app-server version.
pub const CODEX_APP_SERVER_BASELINE_VERSION: &str = "0.80.0";
/// Oldest app-server version qualified for thread catalogue and import.
pub const CODEX_APP_SERVER_THREAD_CATALOGUE_BASELINE_VERSION: &str = "0.105.0";
/// Most recent qualified Codex CLI version.
pub const CODEX_LATEST_QUALIFIED_VERSION: &str = "0.149.0";
pub(crate) const CODEX_APP_SERVER_WORKSPACE_ROOTS_VERSION: &str = "0.131.0";
const CODEX_EXEC_RETAINED_BOOLEAN_SEARCH_BEHAVIOR: &str =
    "codex.exec.jsonl-v1.retained-boolean-search";
const CODEX_EXEC_RETAINED_SEARCH_MODE_BEHAVIOR: &str = "codex.exec.jsonl-v1.retained-search-mode";
const CODEX_EXEC_EPHEMERAL_AMBIENT_BEHAVIOR: &str = "codex.exec.jsonl-v1.ephemeral-ambient";
pub(crate) const CODEX_EXEC_BEHAVIOR: &str = "codex.exec.jsonl-v1";
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

/// Maximum accepted observed Codex CLI-version text.
const MAX_VERSION_BYTES: usize = 64;

/// Converts an exact Codex CLI version into its interface binding.
///
/// Returns `None` for blank, oversized, control-character, or non-semantic
/// text, so observed CLI output can never panic a caller.
#[must_use]
pub fn codex_cli_binding(version: &str) -> Option<InterfaceVersionBinding> {
    if version.is_empty()
        || version.len() > MAX_VERSION_BYTES
        || version.trim() != version
        || version.chars().any(char::is_control)
        || semver::Version::parse(version).is_err()
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(CODEX_CLI_AXIS).expect("static Codex axis is valid"),
        InterfaceVersion::new(version).ok()?,
    ))
}

#[must_use]
/// Returns the qualified compatibility claim for Codex exec.
pub fn codex_exec_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("codex.exec.cli-window-2")
            .expect("static claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
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
        [
            version("0.108.0").expect("static Codex version is valid"),
            version("0.109.0").expect("static Codex version is valid"),
        ],
    )
    .expect("static Codex exec claim is valid")
}

#[must_use]
/// Returns the qualified compatibility claim for Codex app-server.
pub fn codex_app_server_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("codex.app-server.cli-window-2")
            .expect("static claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
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
                InterfaceSupportStatus::Deprecated,
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
    let assessment = classify_plan(plan, &codex_exec_claim(), "exec")?;
    match assessment
        .behavior_revision()
        .expect("permitted assessment has behavior")
        .as_str()
    {
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
    let assessment = classify_plan(plan, &codex_app_server_claim(), "app_server")?;
    match assessment
        .behavior_revision()
        .expect("permitted assessment has behavior")
        .as_str()
    {
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

pub(crate) fn supports_thread_catalogue_version(version: &InterfaceVersion) -> bool {
    let Ok(version) = semver::Version::parse(version.as_str()) else {
        return false;
    };
    let latest = semver::Version::parse(CODEX_LATEST_QUALIFIED_VERSION)
        .expect("static Codex latest qualified version is valid");
    (version >= semver::Version::new(0, 105, 0) && version <= semver::Version::new(0, 107, 0))
        || (version >= semver::Version::new(0, 110, 0) && version <= latest)
}

pub(crate) fn classify_plan(
    plan: &PreflightPlan,
    claim: &InterfaceCompatibilityClaim,
    diagnostic_prefix: &'static str,
) -> Result<InterfaceCompatibilityAssessment, RuntimeFailure> {
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
    let assessment = claim.assess(binding.version());
    if assessment.is_permitted() {
        Ok(assessment)
    } else {
        Err(super::exec::failure(
            diagnostic_code(diagnostic_prefix, "version_incompatible"),
            "Codex executable version is incompatible with this driver",
        ))
    }
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
        version(minimum).expect("static Codex version is valid"),
        version(maximum).expect("static Codex version is valid"),
        InterfaceBehaviorRevision::new(behavior).expect("static behavior is valid"),
        status,
    )
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

#[cfg(test)]
#[path = "selection_tests.rs"]
mod tests;
