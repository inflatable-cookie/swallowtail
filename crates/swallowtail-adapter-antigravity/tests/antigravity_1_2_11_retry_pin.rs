use serde_json::Value;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_BASELINE_VERSION, ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION,
    ANTIGRAVITY_HEADLESS_RETRY_PIN_NAME, ANTIGRAVITY_HEADLESS_RETRY_PIN_VALUE,
    ANTIGRAVITY_RELEASE_AXIS, antigravity_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const PIN_EVIDENCE: &str =
    include_str!("fixtures/antigravity-cli-1.2.11-retry-pin/pin-evidence.json");

const RETRY_DISABLED_BEHAVIOR: &str =
    "antigravity.stream-json.cli-1.1.8-artifact-1.2.11-retry-disabled-v1";
const KEPT_BEHAVIOR: &str = "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1";

#[test]
fn retry_pin_evidence_freezes_the_1_2_11_control_semantics() {
    let evidence: Value =
        serde_json::from_str(PIN_EVIDENCE).expect("retry-pin evidence is valid JSON");

    assert_eq!(
        evidence["record"],
        "docs/research/359-antigravity-headless-retry-pin-evidence.md"
    );
    assert_eq!(evidence["card"], "swallowtail#069");
    assert_eq!(evidence["artifact"], "antigravity-cli-1.2.11");

    let official = &evidence["official"];
    assert_eq!(official["version"], "1.2.11");
    assert_eq!(
        official["github_commit"],
        "6dadd6227a49905f475d22b7f0afe59493229595"
    );
    assert_sha256(
        &official["mac_arm64_tarball_sha256"],
        "437a813cd7c606ccbb3180886887fc69361c28fe8e880327b3b82201afa900cc",
    );
    assert_sha256(
        &official["mac_arm64_extracted_cli_sha256"],
        "42e76bedafb5896bc6a6eefb61902162f6ba08ddf59efd3357767102e3a59a0c",
    );
    assert_sha256(
        &official["linux_x64_tarball_sha256"],
        "c91c62c5e6fa954f5a7e1d7b9ad417d749db4aa60a4ba0b3d604dec1b645d190",
    );
    assert_sha256(
        &official["linux_x64_extracted_cli_sha256"],
        "ec7cf797ecb0e1d91ddf3b6d9d6c1d616bb89f78a5b0e43536b72a7fce695f56",
    );

    let control = &evidence["control"];
    assert_eq!(control["name"], ANTIGRAVITY_HEADLESS_RETRY_PIN_NAME);
    assert_eq!(control["name"], "AGY_CLI_MODEL_API_MAX_RETRIES");
    assert_eq!(control["parser"], "strconv.ParseUint(value, 10, 32)");
    assert_eq!(control["target_field"], "ModelAPIRetryConfig.max_retries");
    assert_eq!(control["unset_attempts"], 9);
    assert_eq!(control["empty_attempts"], 9);
    assert_eq!(control["empty_warns"], false);
    assert_eq!(control["invalid_falls_back_to_default"], true);
    assert_eq!(control["invalid_attempts"], 9);
    assert_eq!(control["attempts_for_pin"]["0"], 1);
    assert_eq!(control["attempts_for_pin"]["1"], 2);
    assert_eq!(control["attempts_for_pin"]["2"], 3);
    assert_exact_string_array(
        &control["pin_classes_bounded"],
        &["502", "503", "504", "429", "mid-stream-EOF"],
    );
    assert_exact_string_array(&control["terminal_class_single_attempt"], &["400"]);
    assert_eq!(control["model_output_retry_has_env_override"], false);

    let effort = &evidence["effort_mapping_1_2_11"];
    assert_eq!(effort["explicit_model_requires_effort"], true);
    assert_exact_string_array(&effort["admitted"], &["low", "medium", "high"]);
    assert_eq!(effort["max_rejected_for_gemini_3_8_flash"], true);
    assert_eq!(effort["adapter_enforces_effort_in_dispatch"], true);

    let warning = control["invalid_warning_format"]
        .as_str()
        .expect("warning format is text");
    assert!(
        warning.contains("must be a non-negative integer within uint32"),
        "unexpected warning format"
    );
    assert!(
        warning.contains("falling back to the default retry budget"),
        "unexpected warning format"
    );

    let decision = &evidence["decision"];
    assert_eq!(decision["pin_name"], ANTIGRAVITY_HEADLESS_RETRY_PIN_NAME);
    assert_eq!(decision["pin_value"], ANTIGRAVITY_HEADLESS_RETRY_PIN_VALUE);
    assert_eq!(decision["pin_value"], "0");
    assert_eq!(decision["headless_behavior"], RETRY_DISABLED_BEHAVIOR);
    assert_eq!(decision["kept_behavior"], KEPT_BEHAVIOR);
    assert_eq!(decision["kept_segment"], "1.1.9..=1.1.17");
    assert_eq!(decision["pinned_point"], "1.2.11");
    assert_eq!(decision["unqualified_gap"], "1.1.18..=1.2.10");
    assert_eq!(decision["contract_023_exception"], false);

    let method = &evidence["method"];
    assert_eq!(method["provider_call"], false);
    assert_eq!(method["login_or_credential"], false);
    assert_eq!(method["host_install"], false);
}

#[test]
fn headless_claim_pins_exact_1_2_11_on_the_retry_disabled_revision() {
    assert_eq!(ANTIGRAVITY_BASELINE_VERSION, "1.1.9");
    assert_eq!(ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, "1.2.11");

    let claim = antigravity_headless_claim();
    assert_eq!(claim.axis().as_str(), ANTIGRAVITY_RELEASE_AXIS);

    assert_eq!(claim.id().as_str(), "antigravity.headless.release-window-2");
    for kept in ["1.1.9", "1.1.15", "1.1.17"] {
        assert!(
            matches!(
                claim.assess(&version(kept)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str() == KEPT_BEHAVIOR
                        && matched.support_status() == InterfaceSupportStatus::Deprecated
            ),
            "{kept} stays deprecated on the original headless revision"
        );
    }
    assert!(
        matches!(
            claim.assess(&version("1.2.11")),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.behavior_revision().as_str() == RETRY_DISABLED_BEHAVIOR
                    && matched.support_status() == InterfaceSupportStatus::Maintained
        ),
        "exact 1.2.11 qualifies maintained on the retry-disabled revision"
    );
    // Research 359 proves the pin on 1.2.11 only: the interior gap is
    // incompatible rather than unverified newer.
    for gap in [
        "1.1.18", "1.1.22", "1.2.0", "1.2.7", "1.2.8", "1.2.9", "1.2.10",
    ] {
        assert!(
            matches!(
                claim.assess(&version(gap)),
                InterfaceCompatibilityAssessment::Incompatible
            ),
            "{gap} stays unqualified until per-point pin evidence lands"
        );
    }
    assert!(matches!(
        claim.assess(&version("1.2.12")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version("1.1.8")));
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

fn assert_sha256(value: &Value, expected: &str) {
    assert_eq!(
        value.as_str().expect("digest is text"),
        expected,
        "digest mismatch"
    );
}

fn assert_exact_string_array(value: &Value, expected: &[&str]) {
    let actual = value.as_array().expect("value is an array");
    assert_eq!(actual.len(), expected.len(), "array length mismatch");
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual.as_str().expect("item is text"), *expected);
    }
}
