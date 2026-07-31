use serde_json::Value;
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionScheme, InterfaceVersionSegment,
};

pub fn compatibility_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        valid(
            InterfaceCompatibilityClaimId::new,
            "claude-agent.acp.range-v2",
        ),
        valid(InterfaceVersionAxis::new, "claude-agent.acp-adapter"),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            segment("0.53.0", "0.53.0", "claude-agent.acp.baseline-v1"),
            segment("0.54.0", "0.59.0", "claude-agent.acp.session-config-v2"),
            segment(
                "0.60.0",
                "0.60.0",
                "claude-agent.acp.provider-capability-v3",
            ),
            segment("0.61.0", "0.62.0", "claude-agent.acp.steering-metadata-v4"),
            segment(
                "0.63.0",
                "0.63.0",
                "claude-agent.acp.tool-subagent-correlation-v5",
            ),
            segment(
                "0.64.0",
                "0.64.0",
                "claude-agent.acp.host-steering-form-marker-v6",
            ),
        ],
        [version("0.52.0"), version("0.58.0")],
    )
    .expect("fixture compatibility claim is valid")
}

pub fn version(value: &str) -> InterfaceVersion {
    valid(InterfaceVersion::new, value)
}

pub fn current_model(config_options: &Value) -> Option<&str> {
    config_options.as_array()?.iter().find_map(|option| {
        (option["id"] == "model")
            .then(|| option["currentValue"].as_str())
            .flatten()
    })
}

fn segment(minimum: &str, maximum: &str, behavior: &str) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        version(minimum),
        version(maximum),
        valid(InterfaceBehaviorRevision::new, behavior),
        InterfaceSupportStatus::Maintained,
    )
}

fn valid<T>(
    constructor: impl FnOnce(String) -> Result<T, swallowtail_core::ValueRequired>,
    value: &str,
) -> T {
    constructor(value.to_owned()).expect("fixture value is valid")
}
