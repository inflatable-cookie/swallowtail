use super::support::{IDENTITY_0_38_0_HEADLESS_V2, PROTOCOL_0_38_0_HEADLESS_V2, version};
use serde_json::Value;
use swallowtail_adapter_kimi::{KIMI_CODE_AXIS, kimi_headless_claim};
use swallowtail_core::InterfaceCompatibilityAssessment;

#[test]
fn headless_v2_corpus_admits_adapter_private_milestone_at_0_38_0() {
    let identity: Value = serde_json::from_str(IDENTITY_0_38_0_HEADLESS_V2)
        .expect("Kimi 0.38.0 headless v2 identity corpus is valid JSON");
    let protocol: Value = serde_json::from_str(PROTOCOL_0_38_0_HEADLESS_V2)
        .expect("Kimi 0.38.0 headless v2 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], KIMI_CODE_AXIS);
    assert_eq!(identity["official_version"], "0.38.0");
    assert_eq!(identity["default_dispatch"], "agent-core-v2-run-v2-print");
    assert_eq!(
        identity["identity_decision"]["shape"],
        "adapter-private-milestone"
    );
    assert_eq!(
        identity["identity_decision"]["behavior_revision"],
        "kimi.headless.stream-json.v2"
    );
    assert_eq!(identity["identity_decision"]["qualified_exact"], "0.38.0");
    assert_eq!(
        identity["identity_decision"]["preserve_v1_through"],
        "0.37.2"
    );
    assert_eq!(
        protocol["selected_headless_v2"]["public_facade_id"],
        "kimi-headless-stream-json-v1"
    );
    assert_eq!(
        protocol["selected_headless_v2"]["preamble_meta"],
        "system.version"
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["host_install_changed"], false);

    let headless_claim = kimi_headless_claim();
    assert!(matches!(
        headless_claim.assess(&version("0.38.0")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.behavior_revision().as_str() == "kimi.headless.stream-json.v2"
    ));
}
