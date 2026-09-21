//! Frozen npm, platform, and executable identity for the Grok Build ACP 1.0.40 run.

use super::support::{IDENTITY, assert_exact_string_set, json};
use std::collections::BTreeMap;

pub(super) const PREVIOUS_CEILING: &str = "1.0.30";
pub(super) const OFFICIAL_STABLE: &str = "1.0.40";
pub(super) const COMPARED: &[&str] = &[
    "1.0.30", "1.0.31", "1.0.32", "1.0.33", "1.0.34", "1.0.35", "1.0.36", "1.0.37", "1.0.38",
    "1.0.39", "1.0.40",
];
pub(super) const HOPS: &[&str] = &[
    "1.0.31", "1.0.32", "1.0.33", "1.0.34", "1.0.35", "1.0.36", "1.0.37", "1.0.38", "1.0.39",
    "1.0.40",
];

const MODEL_DOCUMENT: &str = "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a";

const LINUX_EXECUTABLE_DIGESTS: &[(&str, &str)] = &[
    (
        "1.0.30",
        "504dd6546ab991b75d36698242875ce461489cd1f8cd84285873cb55bd5c7d54",
    ),
    (
        "1.0.31",
        "d37fa3e50c5509a12af4bb4fc4f3c6bb2eadfc28f8f3e88dc21935b47e4e342f",
    ),
    (
        "1.0.32",
        "519493ba078dc280be954ed6c94e356bdedf51e053a98d48ccd779ee0446905b",
    ),
    (
        "1.0.33",
        "47d3c69f93013a12669641f69caf385cebe385c591c5cfb5d5486ce5656f88ba",
    ),
    (
        "1.0.34",
        "be5905e107d2b8b5f3c142d21ecfe4c8fd32a913d2fd551b788707930c4dc80d",
    ),
    (
        "1.0.35",
        "f57df130e95a230d444933596f6530738d891ae470b7b7a45e304919d71b1cc4",
    ),
    (
        "1.0.36",
        "90e373f48b0fd5b6fb2af4e92a6e59ff4fb03e97628715db74bb57dcf89c2f01",
    ),
    (
        "1.0.37",
        "5b18c917d4e3ab41d23dde88d46bc3cb22488de042697d26ee4975811bd92b5c",
    ),
    (
        "1.0.38",
        "d09092c50f1bf1cd686a0ecd3489aedd83ec2d16b27a821ca9f0994a4b605ef9",
    ),
    (
        "1.0.39",
        "576cd799f754643d1bf807086849eb4a90aed41a223e7cac72a1cb909f7c40e7",
    ),
    (
        "1.0.40",
        "92c997dfd109c0672d40d5ae6fbd15835d53ffaf12cf9ea124d22aaef3ff23fc",
    ),
];

const DARWIN_EXECUTABLE_DIGESTS: &[(&str, &str)] = &[
    (
        "1.0.30",
        "d53b6e543e482716236748914331db50145c696ac7af91f1ebdedcf5654cfecb",
    ),
    (
        "1.0.31",
        "6444143255f8c5095d26b40ebd362bbedee76bddb98fa8f634efe871fde8ba23",
    ),
    (
        "1.0.32",
        "7bfcd0938367a3696dcde7b53f66549e7f274918a67887500dda06272c7d810f",
    ),
    (
        "1.0.33",
        "43a51b2c913683b6fe56dcba561a1c7df412bd9afa848e592922dfda3a1bee5d",
    ),
    (
        "1.0.34",
        "9cd26b579840f0f5c9148a8059ad651904c08b41b7f2ef0b4ec04b9ba898844e",
    ),
    (
        "1.0.35",
        "948d44c6491c389d65338f2bc7cf4faf34baea787a7ed35f9e31f925abe8e41c",
    ),
    (
        "1.0.36",
        "14a3dda68b368f933a7eaa2e970a30b4ff2149ba42be7dd90eb81c99b6ae3c9a",
    ),
    (
        "1.0.37",
        "0fdb8a2e05cbe5ba623558b076f08475d5b4de257549dfc632eee7967d6d6f68",
    ),
    (
        "1.0.38",
        "a3c5c279339a1294cc99b4d105fe7c67a9f64d20647f2edf131ee72d70f94ed1",
    ),
    (
        "1.0.39",
        "da2011c9a4e011e61d59651bb243b30304727eda8b987bdddd5836c279609453",
    ),
    (
        "1.0.40",
        "3f2aef9618191a2c60d18a5044fa462c9c77bdc4187b02ed716b0394e8d4fef2",
    ),
];

const GIT_HEADS: &[(&str, &str)] = &[
    ("1.0.30", "04b7ffed98c6943e3c736e6e8cfb7f979560638e"),
    ("1.0.31", "b44e0bc0a8e0900364a3ec1a032b2a5f22f2b8f3"),
    ("1.0.32", "e21ee47a3bbfdf20ab3de715f7f031fd63185c43"),
    ("1.0.33", "ff317a6753d14013d79e894b53328151431ab32f"),
    ("1.0.34", "3736acbc8658f753bad926dca0dac7e904da56ff"),
    ("1.0.35", "d949da3917a8dcf9e7614df5ab743dd89f028244"),
    ("1.0.36", "9ecab0bec61e9dd2a85e65be278908ce1da656d8"),
    ("1.0.37", "7bb320867c4faa090fbc3a1ad60cc16ae350213d"),
    ("1.0.38", "41b9d57a3b9a38e6969e4d56f2a77ec66107b563"),
    ("1.0.39", "0b340e9ac868865f320b02b1d9fd799d1bf314ee"),
    ("1.0.40", "eb1a2256660dda9ade5d2919fa8542283f608abf"),
];

fn table<'a>(pairs: &'a [(&'a str, &'a str)]) -> BTreeMap<&'a str, &'a str> {
    pairs.iter().copied().collect()
}

#[test]
fn official_channel_freezes_every_published_stable_hop() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["npm_package"], "@xai-official/grok");
    assert_eq!(identity["npm_dist_tags"]["latest"], OFFICIAL_STABLE);
    assert_eq!(identity["npm_dist_tags"]["alpha"], OFFICIAL_STABLE);
    assert_eq!(identity["previous_ceiling"], PREVIOUS_CEILING);
    assert_eq!(identity["official_stable"], OFFICIAL_STABLE);
    assert_eq!(identity["first_unpublished_stable_after_latest"], "1.0.41");
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
fn ceiling_reproduces_research_314_wrapper_and_darwin_arm64() {
    let identity = json(IDENTITY);
    let ceiling = &identity["hops"][0];
    assert_eq!(ceiling["version"], PREVIOUS_CEILING);
    assert_eq!(
        ceiling["wrapper"]["tarball_sha256"],
        "c57e7106e1f18e9d41677d06836a0abb3a498ae353763dacab3a1da473351628"
    );
    assert_eq!(
        ceiling["wrapper"]["shasum"],
        "dd6bbb51ddbfd6a07143e2bef4afb83d02c7beaa"
    );
    assert_eq!(
        ceiling["darwin_arm64_crosscheck"]["executable_sha256"],
        "d53b6e543e482716236748914331db50145c696ac7af91f1ebdedcf5654cfecb"
    );
    assert_eq!(
        ceiling["darwin_arm64_crosscheck"]["tarball_sha256"],
        "7e522683b99268fb44b5834909739bdea743fb8ed9fe9fa3186b6be56938b3ed"
    );
    assert_eq!(
        ceiling["darwin_arm64_crosscheck"]["brotli_sha256"],
        "13f3c6cd5145c5d1e39a6ab9ab51bc429655d6f7410938b295011a367db17d1e"
    );
    assert_eq!(
        identity["darwin_arm64_1_0_30_reproduces_research_314"],
        true
    );
    assert_eq!(
        identity["research_314_darwin_arm64_1_0_30_executable"],
        "d53b6e543e482716236748914331db50145c696ac7af91f1ebdedcf5654cfecb"
    );
    assert_eq!(
        ceiling["platform"]["other_platform_integrities"]["grok-linux-x64"],
        "sha512-yM1q8mRZeAUUhEJn0SY256D2ucz+jvmK8tKo/Ivcr+7EUouy/VtS9bRfrhTfj5i6IemYxH9NIoV4wtJjSzh90w=="
    );
}

#[test]
fn identity_decision_names_a_compatible_extension_before_the_claim_card() {
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
    assert_eq!(decision["later_unverified_after_qualification"], "1.0.41");
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
