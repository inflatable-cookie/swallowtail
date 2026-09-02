use crate::exec::failure;
use crate::selection::{CODEX_EXEC_BEHAVIOR, CodexExecBehavior};
use swallowtail_core::{
    Diagnostic, InstalledExecutableCompatibility, InterfaceVersion, ModelId, PreflightPlan,
    SafeDiagnostic,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

const ADMITTED_VERSIONS: &[&str] = &["0.147.0", "0.148.0", "0.149.0", "0.149.1"];

const ADMITTED_MODELS: &[&str] = &[
    "gpt-5.6-sol",
    "gpt-5.6-terra",
    "gpt-5.6-luna",
    "gpt-5.5",
    "gpt-5.4",
    "gpt-5.4-mini",
    "gpt-5.2",
];

/// Closed adapter-local Codex Exec `model_verbosity` selection.
///
/// This is not a portable capability, reasoning mode, or generic config map.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodexModelVerbosity {
    /// Responses `text.verbosity=low`.
    Low,
    /// Responses `text.verbosity=medium`.
    Medium,
    /// Responses `text.verbosity=high`.
    High,
}

impl CodexModelVerbosity {
    /// Returns the exact CLI/config token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

pub(crate) fn admits(
    behavior: CodexExecBehavior,
    version: &InterfaceVersion,
    model: &ModelId,
) -> bool {
    behavior == CodexExecBehavior::EphemeralSuppressed
        && version_admitted(version.as_str())
        && ADMITTED_MODELS.contains(&model.as_str())
}

pub(crate) fn maintained_exec_behavior(
    compatibility: &InstalledExecutableCompatibility,
) -> Option<CodexExecBehavior> {
    let revision = match compatibility {
        InstalledExecutableCompatibility::Qualified(matched) => {
            matched.behavior_revision().as_str()
        }
        InstalledExecutableCompatibility::UnverifiedNewer(unverified) => {
            unverified.behavior_revision().as_str()
        }
        InstalledExecutableCompatibility::Incompatible => return None,
    };
    (revision == CODEX_EXEC_BEHAVIOR).then_some(CodexExecBehavior::EphemeralSuppressed)
}

pub(crate) fn validate_runtime(
    plan: &PreflightPlan,
    behavior: CodexExecBehavior,
    model: &ModelId,
    model_verbosity: Option<CodexModelVerbosity>,
) -> Result<(), RuntimeFailure> {
    let Some(_verbosity) = model_verbosity else {
        return Ok(());
    };
    let version = plan.interface_versions().next().ok_or_else(|| {
        failure(
            "swallowtail.codex.exec.version_missing",
            "Codex exec requires a preflight-bound executable version",
        )
    })?;
    if admits(behavior, version.version(), model) {
        Ok(())
    } else {
        Err(reject_runtime())
    }
}

pub(crate) fn reject_runtime() -> RuntimeFailure {
    failure(
        "swallowtail.codex.exec.model_verbosity_unsupported",
        "Codex exec model verbosity is not supported for this version, model, or behavior",
    )
}

pub(crate) fn reject_preparation() -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(
            "swallowtail.codex.preparation.model_verbosity_unsupported",
            "Codex exec model verbosity is not supported for this version, model, or behavior",
        )),
    )
}

fn version_admitted(version: &str) -> bool {
    ADMITTED_VERSIONS.contains(&version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_core::{InterfaceVersion, ModelId};

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }

    fn model(value: &str) -> ModelId {
        ModelId::new(value).expect("fixture model is valid")
    }

    #[test]
    fn admits_every_exact_research_version_and_model() {
        for version_value in ADMITTED_VERSIONS {
            for model_value in ADMITTED_MODELS {
                assert!(
                    admits(
                        CodexExecBehavior::EphemeralSuppressed,
                        &version(version_value),
                        &model(model_value),
                    ),
                    "expected {version_value}/{model_value} to be admitted",
                );
            }
        }
    }

    #[test]
    fn rejects_unretrieved_release_identities() {
        for version_value in [
            "0.146.99",
            "0.147.0-alpha.1",
            "0.147.0+build.1",
            "0.147.1",
            "0.148.1",
            "0.149.1-alpha.1",
            "0.149.1+build.1",
            "0.149.2",
            "0.150.0",
            "0.150.1",
            "0.151.0",
            "0.151.1",
            "0.152.0",
            "0.152.1",
            "0.152.2",
        ] {
            assert!(
                !admits(
                    CodexExecBehavior::EphemeralSuppressed,
                    &version(version_value),
                    &model("gpt-5.4-mini"),
                ),
                "expected {version_value} to be rejected",
            );
        }
    }

    #[test]
    fn rejects_prefix_foreign_and_ambient_rows() {
        for model_value in ["gpt-5.4-mini-preview", "codex-auto-review"] {
            assert!(!admits(
                CodexExecBehavior::EphemeralSuppressed,
                &version("0.149.1"),
                &model(model_value),
            ));
        }
        assert!(!admits(
            CodexExecBehavior::EphemeralAmbient,
            &version("0.149.1"),
            &model("gpt-5.4-mini"),
        ));
    }

    #[test]
    fn verbosity_tokens_are_closed_and_exact() {
        assert_eq!(CodexModelVerbosity::Low.as_str(), "low");
        assert_eq!(CodexModelVerbosity::Medium.as_str(), "medium");
        assert_eq!(CodexModelVerbosity::High.as_str(), "high");
    }
}
