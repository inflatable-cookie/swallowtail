use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Adapter-version interface axis used by Claude Agent ACP.
pub const CLAUDE_AGENT_ACP_AXIS: &str = "claude-agent.acp-adapter";
/// Oldest qualified Claude Agent ACP version.
pub const CLAUDE_AGENT_ACP_BASELINE_VERSION: &str = "0.53.0";
/// Most recent qualified Claude Agent ACP version.
pub const CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION: &str = "0.64.0";

const BASELINE_BEHAVIOR: &str = "claude-agent.acp.baseline-v1";
const SESSION_CONFIG_BEHAVIOR: &str = "claude-agent.acp.session-config-v2";
const PROVIDER_CAPABILITY_BEHAVIOR: &str = "claude-agent.acp.provider-capability-v3";
const STEERING_METADATA_BEHAVIOR: &str = "claude-agent.acp.steering-metadata-v4";
const TOOL_SUBAGENT_CORRELATION_BEHAVIOR: &str = "claude-agent.acp.tool-subagent-correlation-v5";
const HOST_STEERING_FORM_MARKER_BEHAVIOR: &str = "claude-agent.acp.host-steering-form-marker-v6";
const MAX_VERSION_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentBehavior {
    Baseline,
    SessionConfig,
    ProviderCapability,
    SteeringMetadata,
    ToolSubagentCorrelation,
    HostSteeringFormMarker,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ClaudeAgentPlanSelection {
    behavior: ClaudeAgentBehavior,
    version: InterfaceVersion,
    qualified: bool,
}

impl ClaudeAgentPlanSelection {
    pub(crate) const fn behavior(&self) -> ClaudeAgentBehavior {
        self.behavior
    }

    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }

    pub(crate) const fn is_qualified(&self) -> bool {
        self.qualified
    }
}

impl ClaudeAgentBehavior {
    pub(crate) const fn supports_config_options(self) -> bool {
        !matches!(self, Self::Baseline)
    }
}

pub(crate) fn version_supports_config_options(version: &InterfaceVersion) -> bool {
    claude_agent_acp_claim()
        .assess(version)
        .behavior_revision()
        .and_then(behavior)
        .is_some_and(ClaudeAgentBehavior::supports_config_options)
}

#[must_use]
/// Parses a Claude Agent ACP semantic version into its interface binding.
pub fn claude_agent_acp_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty()
        || value.len() > MAX_VERSION_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
        || semver::Version::parse(value).is_err()
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

#[must_use]
/// Returns the qualified compatibility claim for Claude Agent ACP.
pub fn claude_agent_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("claude-agent.acp.window-2")
            .expect("static Claude Agent claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            segment(
                "0.53.0",
                "0.53.0",
                BASELINE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.54.0",
                "0.59.0",
                SESSION_CONFIG_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.60.0",
                "0.60.0",
                PROVIDER_CAPABILITY_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.61.0",
                "0.62.0",
                STEERING_METADATA_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.63.0",
                "0.63.0",
                TOOL_SUBAGENT_CORRELATION_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.64.0",
                "0.64.0",
                HOST_STEERING_FORM_MARKER_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [
            version("0.52.0").expect("static Claude Agent version is valid"),
            version("0.58.0").expect("static Claude Agent version is valid"),
        ],
    )
    .expect("static Claude Agent compatibility claim is valid")
}

pub(crate) fn select_claude_agent_plan(
    plan: &PreflightPlan,
) -> Result<ClaudeAgentPlanSelection, RuntimeFailure> {
    let claim = claude_agent_acp_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.claude_agent.acp.version_missing",
            "Claude Agent ACP plan is missing its exact adapter version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.claude_agent.acp.version_ambiguous",
            "Claude Agent ACP plan contains more than one adapter version",
        ));
    }

    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding) || !assessment.is_permitted() {
        return Err(failure(
            "swallowtail.claude_agent.acp.version_incompatible",
            "Claude Agent ACP adapter version is incompatible with this driver",
        ));
    }
    let qualified = matches!(assessment, InterfaceCompatibilityAssessment::Qualified(_));
    let revision = assessment
        .behavior_revision()
        .expect("permitted assessment has a behavior revision");
    let behavior = match behavior(revision) {
        Some(behavior) => behavior,
        None => {
            return Err(failure(
                "swallowtail.claude_agent.acp.behavior_incompatible",
                "Claude Agent ACP behavior is not mapped by this driver",
            ));
        }
    };
    Ok(ClaudeAgentPlanSelection {
        behavior,
        version: binding.version().clone(),
        qualified,
    })
}

fn behavior(revision: &InterfaceBehaviorRevision) -> Option<ClaudeAgentBehavior> {
    match revision.as_str() {
        BASELINE_BEHAVIOR => Some(ClaudeAgentBehavior::Baseline),
        SESSION_CONFIG_BEHAVIOR => Some(ClaudeAgentBehavior::SessionConfig),
        PROVIDER_CAPABILITY_BEHAVIOR => Some(ClaudeAgentBehavior::ProviderCapability),
        STEERING_METADATA_BEHAVIOR => Some(ClaudeAgentBehavior::SteeringMetadata),
        TOOL_SUBAGENT_CORRELATION_BEHAVIOR => Some(ClaudeAgentBehavior::ToolSubagentCorrelation),
        HOST_STEERING_FORM_MARKER_BEHAVIOR => Some(ClaudeAgentBehavior::HostSteeringFormMarker),
        _ => None,
    }
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("static Claude Agent axis is valid")
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

fn segment(
    minimum: &str,
    maximum: &str,
    revision: &str,
    status: InterfaceSupportStatus,
) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        version(minimum).expect("static Claude Agent version is valid"),
        version(maximum).expect("static Claude Agent version is valid"),
        InterfaceBehaviorRevision::new(revision).expect("static behavior revision is valid"),
        status,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        CLAUDE_AGENT_ACP_AXIS, HOST_STEERING_FORM_MARKER_BEHAVIOR, claude_agent_acp_binding,
        claude_agent_acp_claim, version_supports_config_options,
    };
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn claim_preserves_six_milestones_exclusions_and_visible_newer_execution() {
        let claim = claude_agent_acp_claim();
        assert_eq!(claim.baseline().as_str(), "0.53.0");
        assert_eq!(claim.latest_qualified().as_str(), "0.64.0");
        assert_eq!(claim.milestones().len(), 6);
        for qualified in [
            "0.53.0", "0.54.1", "0.58.1", "0.59.0", "0.61.0", "0.62.0", "0.63.0", "0.64.0",
        ] {
            assert!(claim.supports(&version(qualified)));
        }
        for incompatible in ["0.52.0", "0.58.0", "0.61.0-rc.1", "invalid"] {
            assert!(!claim.permits(&version(incompatible)));
        }
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("0.65.0"))
        else {
            panic!("newer stable version remains unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            HOST_STEERING_FORM_MARKER_BEHAVIOR
        );
    }

    #[test]
    fn binding_accepts_only_one_exact_semantic_version() {
        assert_eq!(
            claude_agent_acp_binding("0.61.0")
                .expect("version binds")
                .axis()
                .as_str(),
            CLAUDE_AGENT_ACP_AXIS
        );
        for rejected in [
            "",
            " 0.61.0",
            "claude-agent 0.61.0",
            "0.61.0 extra",
            "latest",
        ] {
            assert!(claude_agent_acp_binding(rejected).is_none());
        }
    }

    #[test]
    fn session_config_gate_preserves_legacy_baseline_and_newer_inheritance() {
        assert!(!version_supports_config_options(&version("0.53.0")));
        assert!(version_supports_config_options(&version("0.54.0")));
        assert!(version_supports_config_options(&version("0.61.0")));
        assert!(version_supports_config_options(&version("0.63.0")));
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is non-empty")
    }
}
