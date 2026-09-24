//! Frozen npm, platform, and executable identity for the Grok Build ACP 1.0.41 run.

use super::support::{IDENTITY, assert_exact_string_set, json};
use std::collections::BTreeMap;

pub(super) const PREVIOUS_CEILING: &str = "1.0.40";
pub(super) const OFFICIAL_STABLE: &str = "1.0.41";
pub(super) const COMPARED: &[&str] = &["1.0.40", "1.0.41"];
pub(super) const HOPS: &[&str] = &["1.0.41"];

const MODEL_DOCUMENT: &str = "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a";

const LINUX_EXECUTABLE_DIGESTS: &[(&str, &str)] = &[
    (
        "1.0.40",
        "92c997dfd109c0672d40d5ae6fbd15835d53ffaf12cf9ea124d22aaef3ff23fc",
    ),
    (
        "1.0.41",
        "9ce03ed23e16ea01072b4496263d6213a27899e1e3e107f008d36edf82e70407",
    ),
];

const DARWIN_EXECUTABLE_DIGESTS: &[(&str, &str)] = &[
    (
        "1.0.40",
        "3f2aef9618191a2c60d18a5044fa462c9c77bdc4187b02ed716b0394e8d4fef2",
    ),
    (
        "1.0.41",
        "9c844eb13365180787d9ad22b2b3748a024be8e1ed845253cc114781b31c591d",
    ),
];

const GIT_HEADS: &[(&str, &str)] = &[
    ("1.0.40", "eb1a2256660dda9ade5d2919fa8542283f608abf"),
    ("1.0.41", "4220f3b224a672ff2641e35ba78ef6b0c6fd7069"),
];

fn table<'a>(pairs: &'a [(&'a str, &'a str)]) -> BTreeMap<&'a str, &'a str> {
    pairs.iter().copied().collect()
}

#[test]
fn official_channel_freezes_the_published_hop_after_the_previous_ceiling() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["npm_package"], "@xai-official/grok");
    assert_eq!(identity["npm_dist_tags"]["latest"], OFFICIAL_STABLE);
    assert_eq!(identity["npm_dist_tags"]["alpha"], OFFICIAL_STABLE);
    assert_eq!(identity["previous_ceiling"], PREVIOUS_CEILING);
    assert_eq!(identity["official_stable"], OFFICIAL_STABLE);
    assert!(
        identity["first_unpublished_stable_after_latest"].is_null(),
        "1.0.41 is the published latest; no later stable exists"
    );
    assert_eq!(identity["observed_platform"], "linux-x64");
    assert_eq!(
        identity["platform_package_compared"],
        "@xai-official/grok-linux-x64"
    );
    assert_exact_string_set(&identity["published_stables_after_ceiling"], HOPS);
    assert_exact_string_set(&identity["alpha_after_ceiling"], &[]);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified_before"],
        PREVIOUS_CEILING
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_1_0_41"],
        "unverified_newer"
    );
    assert_eq!(
        identity["claim_at_observation"]["posture"],
        "allow_unverified"
    );
    assert_eq!(identity["host"]["installed"], false);
}

#[test]
fn every_hop_pins_exact_wrapper_platform_and_executable_identity() {
    let identity = json(IDENTITY);
    let hops = identity["hops"].as_array().expect("hops array");
    assert_eq!(hops.len(), COMPARED.len());
    let linux = table(LINUX_EXECUTABLE_DIGESTS);
    let darwin = table(DARWIN_EXECUTABLE_DIGESTS);
    let git_heads = table(GIT_HEADS);
    let mut seen_linux = BTreeMap::new();
    let mut seen_darwin = BTreeMap::new();
    for (index, entry) in hops.iter().enumerate() {
        let version = entry["version"].as_str().expect("version");
        assert_eq!(version, COMPARED[index]);
        assert_eq!(entry["git_head"], git_heads[version]);
        assert_eq!(entry["source_revision"], &git_heads[version][..12]);
        assert_eq!(entry["wrapper"]["file_count"], 5);
        assert!(
            entry["wrapper"]["integrity"]
                .as_str()
                .unwrap()
                .starts_with("sha512-")
        );
        assert_eq!(entry["wrapper"]["shasum"].as_str().unwrap().len(), 40);
        assert_eq!(
            entry["wrapper"]["tarball_sha256"].as_str().unwrap().len(),
            64
        );
        assert_eq!(entry["platform"]["name"], "@xai-official/grok-linux-x64");
        assert_eq!(entry["platform"]["executable_sha256"], linux[version]);
        assert_eq!(
            entry["darwin_arm64_crosscheck"]["executable_sha256"],
            darwin[version]
        );
        let linux_sha = entry["platform"]["executable_sha256"].as_str().unwrap();
        let darwin_sha = entry["darwin_arm64_crosscheck"]["executable_sha256"]
            .as_str()
            .unwrap();
        assert_ne!(
            linux_sha, darwin_sha,
            "{version} platforms ship distinct executables"
        );
        assert!(
            seen_linux.insert(linux_sha.to_owned(), version).is_none(),
            "each hop ships a distinct linux executable"
        );
        assert!(
            seen_darwin.insert(darwin_sha.to_owned(), version).is_none(),
            "each hop ships a distinct darwin executable"
        );
        let platforms = entry["platform"]["other_platform_integrities"]
            .as_object()
            .expect("platform integrities");
        assert_eq!(platforms.len(), 6);
        for (name, value) in platforms {
            assert!(value.as_str().unwrap().starts_with("sha512-"), "{name}");
        }
        assert_eq!(entry["model_document_sha256"][0], MODEL_DOCUMENT);
    }
}

#[test]
fn the_baseline_hop_reproduces_the_frozen_1_0_40_corpus() {
    let identity = json(IDENTITY);
    assert_eq!(identity["hop"]["version"], OFFICIAL_STABLE);
    assert_eq!(identity["previous_hop_1_0_40"]["version"], PREVIOUS_CEILING);
    assert_eq!(
        identity["hops"][0]["platform"]["executable_sha256"],
        "92c997dfd109c0672d40d5ae6fbd15835d53ffaf12cf9ea124d22aaef3ff23fc"
    );
    assert_eq!(
        identity["hops"][0]["platform"]["tarball_sha256"],
        "8a66a13330c1abedde2c2217bbb972e0c3e04ac874063ca850a8f817e4815c50"
    );
    assert_eq!(
        identity["hops"][0]["platform"]["brotli_sha256"],
        "fe98209666ad4caf3f1f9de18b33e565569867ce5befeb88a636c5a880c13097"
    );
    assert_eq!(
        identity["hops"][0]["wrapper"]["tarball_sha256"],
        "43e432f0554bb97796d1f16aba75cf9545d5895e91caf34c2ff9e89ab28bfad0"
    );
    assert_eq!(
        identity["hops"][0]["darwin_arm64_crosscheck"]["executable_sha256"],
        "3f2aef9618191a2c60d18a5044fa462c9c77bdc4187b02ed716b0394e8d4fef2"
    );
}

#[test]
fn identity_decision_names_a_compatible_extension_at_the_current_release() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "grok-build.acp-v1.cached-token-model-4-6-v3"
    );
    assert_eq!(decision["raise_latest_qualified_to"], OFFICIAL_STABLE);
    assert_eq!(decision["keep_baseline"], "0.2.114");
    assert_eq!(decision["keep_0_2_window"], true);
    assert_eq!(decision["keep_1_0_4_floor"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["model_binding"], "grok-4.6");
    assert_eq!(decision["mid_gap_0_2_118_through_0_2_121"], "incompatible");
    assert_eq!(decision["unprobed_1_0_0_through_1_0_3"], "incompatible");
    assert_eq!(
        decision["catalogue_claim_unchanged"],
        "exact 1.0.30 under QualifiedOnly"
    );
    assert_eq!(
        decision["registered_tool_claim_unchanged"],
        "exact 1.0.4 and 1.0.5 only"
    );
    assert!(
        decision["later_unverified_after_qualification"].is_null(),
        "1.0.41 is the current release; no later stable is unverified"
    );
    for flag in [
        "provider_prompt_sent",
        "live_acp_session",
        "downloaded_artifacts_executed",
        "host_install_changed",
    ] {
        assert_eq!(decision[flag], false, "{flag} must stay false");
    }
    assert_exact_string_set(&decision["qualify_intermediates"], HOPS);
}
