//! Catalogue-specific identity assertions for Grok Build `1.0.30`.
//!
//! Every value below is re-derived from the frozen
//! `fixtures/grok-1.0.30-catalogue/identity.json` ledger, which was produced
//! by downloading the official `darwin-arm64` platform packages for each hop,
//! verifying the published tarball digest, brotli-decompressing `bin/grok.br`,
//! verifying the decompressed executable digest, and carving the
//! catalogue-specific literals and embedded default-model document.

use super::support::{IDENTITY, assert_exact_string_set, json};
use std::collections::{BTreeMap, BTreeSet};

/// Previous exact catalogue point; the run starts from it.
const PREVIOUS_EXACT_POINT: &str = "1.0.25";
/// Official npm stable and installed host the run targets.
const OFFICIAL_STABLE: &str = "1.0.30";
/// Every compared hop, the previous exact point plus every later stable.
const COMPARED: &[&str] = &["1.0.25", "1.0.26", "1.0.27", "1.0.28", "1.0.29", "1.0.30"];

const EXECUTABLES: &[(&str, &str, u64)] = &[
    (
        "1.0.25",
        "9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c",
        138080336,
    ),
    (
        "1.0.26",
        "081f1d861e99bd738e2939ccc7d447d113f0e05fa253c6489a8918adaaf7d8b2",
        141421808,
    ),
    (
        "1.0.27",
        "6e85277b0432c894abcb325d9132bdc88c9715fa3089f65b7589963d5522ec80",
        141421536,
    ),
    (
        "1.0.28",
        "bfaa983f8dd4d300ce8277f52ec38257e42a6105d4df75492f6059a28e55cc5c",
        141587632,
    ),
    (
        "1.0.29",
        "2b6b44a2e7e72348c4fc65f3c427b1c47fe95cbcf9f43f204f826d382b7d85db",
        141553456,
    ),
    (
        "1.0.30",
        "d53b6e543e482716236748914331db50145c696ac7af91f1ebdedcf5654cfecb",
        141869568,
    ),
];

const TARBALLS: &[(&str, &str)] = &[
    (
        "1.0.25",
        "c1548be1210511909bc74e2f4264feb569166ffcfecf15123fda8ead250e0cc2",
    ),
    (
        "1.0.26",
        "f237c01a41f2993fa93a1cef1715ddbea16f571d2f8cc5293a17e2801bcc2964",
    ),
    (
        "1.0.27",
        "6c12734cadf73812a3a40dc10e9f8fda53d22f2f7311d224b87dde7119343e4e",
    ),
    (
        "1.0.28",
        "0782e98799dc03049aac7513fc70261dc2c82d0b237e1cc47ebcf76565650e4d",
    ),
    (
        "1.0.29",
        "da4a3756a367d5d342bd0885f6bbf73a497539ed2f810cc6ba6e914d345a35bf",
    ),
    (
        "1.0.30",
        "7e522683b99268fb44b5834909739bdea743fb8ed9fe9fa3186b6be56938b3ed",
    ),
];

const BROTLI: &[(&str, &str, u64)] = &[
    (
        "1.0.25",
        "7d13cf31755d1dc4918a3bd5bd1bed2917fcaa06d965602357759178f375fcb0",
        40481176,
    ),
    (
        "1.0.26",
        "879d6b1bc67624676c67711f3a5d4177b64a1469ac0f0c166e36f8e8c93f9141",
        41294250,
    ),
    (
        "1.0.27",
        "9077bfa8ff155bc109bf35f495b61342b4304505a1c6be9eb198ef12544248a2",
        41300597,
    ),
    (
        "1.0.28",
        "ba5ad4781216ad0858f60d9d322f754e50614d5f5b0513d65c2d7aa1996afd29",
        41352031,
    ),
    (
        "1.0.29",
        "a6729424609ea58e6317f9ff046b6203c9530973ecc45e4164468178e4d90da6",
        41327540,
    ),
    (
        "1.0.30",
        "13f3c6cd5145c5d1e39a6ab9ab51bc429655d6f7410938b295011a367db17d1e",
        41436148,
    ),
];

/// The embedded default-model document is byte-identical at every hop.
const MODEL_DOCUMENT: &str = "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a";
const MODEL_DOCUMENT_SIZE: u64 = 2323;
const MODEL_DOCUMENT_COPIES: u64 = 2;

/// Exact shipped catalogue literals and the count observed in every hop.
const LITERALS: &[(&str, &str, u64)] = &[
    ("root_flag", "--no-auto-update", 4),
    ("format_default_header", "Default model: ", 3),
    ("format_available_header", "Available models:", 1),
    ("format_bullet_other", "  - ", 158),
    ("format_bullet_default", "  * ", 3),
    ("format_default_suffix", " (default)\n", 5),
    ("auth_logged_in", "You are logged in with ", 1),
    ("auth_api_key", "You are using XAI_API_KEY.", 1),
    ("auth_own_key_suffix", "' is using its own API key.", 1),
    (
        "auth_deployment_key",
        "You are authenticated via deployment key.",
        1,
    ),
    ("auth_none", "You are not authenticated.", 1),
    ("models_module", "xai-grok-pager/src/models.rs", 1),
    (
        "models_help_description",
        "List available models and exit",
        1,
    ),
];

const SURFACE_DIGEST: &str = "aa3ad436d5a6c8eb893d1091c02f0cadbdabe9338bce38a4205d9a04768d3e02";

#[test]
fn channel_host_and_compared_hops_are_exact() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["route"], "grok-build.catalogue");
    assert_eq!(identity["target_exact_point"], OFFICIAL_STABLE);
    let channel = &identity["channel"];
    assert_eq!(channel["package"], "@xai-official/grok");
    assert_eq!(channel["latest"], OFFICIAL_STABLE);
    assert_eq!(channel["alpha"], "1.0.31");
    assert_eq!(channel["first_unpublished_stable_after_latest"], "1.0.32");
    assert_eq!(channel["previous_exact_point"], PREVIOUS_EXACT_POINT);
    assert_exact_string_set(&channel["published_stables_compared"], COMPARED);

    let host = &identity["host"];
    assert_eq!(host["version"], OFFICIAL_STABLE);
    assert_eq!(host["source_revision"], "04b7ffed98c6");
    assert_eq!(host["output"], "grok 1.0.30 (04b7ffed98c6) [stable]");
    assert_eq!(host["executable_sha256"], EXECUTABLES[5].1);
    assert_eq!(host["executable_size"], EXECUTABLES[5].2);
    assert_eq!(host["official_darwin_arm64_payload_equal"], true);
    assert_eq!(host["host_install_changed"], false);
}

#[test]
fn every_hop_reproduces_platform_and_executable_identity() {
    let identity = json(IDENTITY);
    let hops = identity["hops"].as_array().expect("hops");
    assert_eq!(hops.len(), COMPARED.len());
    let mut seen_executables = BTreeSet::new();
    for (index, hop) in hops.iter().enumerate() {
        let version = COMPARED[index];
        assert_eq!(hop["version"], version);
        assert_eq!(hop["platform_tarball_sha256"], TARBALLS[index].1);
        assert_eq!(hop["brotli_sha256"], BROTLI[index].1);
        assert_eq!(hop["brotli_size"], BROTLI[index].2);
        assert_eq!(hop["executable_sha256"], EXECUTABLES[index].1);
        assert_eq!(hop["executable_size"], EXECUTABLES[index].2);
        assert_eq!(hop["model_document_sha256"], MODEL_DOCUMENT);
        assert_eq!(hop["model_document_size"], MODEL_DOCUMENT_SIZE);
        assert_eq!(hop["model_document_copies"], MODEL_DOCUMENT_COPIES);
        let sha = hop["executable_sha256"].as_str().expect("digest");
        assert_eq!(sha.len(), 64);
        assert!(sha.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert!(
            seen_executables.insert(sha.to_owned()),
            "every hop ships a distinct executable"
        );
        let head = hop["git_head"].as_str().expect("git head");
        assert_eq!(head.len(), 40);
    }
    assert_eq!(seen_executables.len(), COMPARED.len());
}

#[test]
fn catalogue_literal_counts_are_identical_across_every_hop() {
    let identity = json(IDENTITY);
    let surface = &identity["catalogue_surface"];
    assert_eq!(surface["literal_counts_identical_across_hops"], true);
    assert_eq!(surface["model_document_identical_across_hops"], true);
    assert_eq!(surface["surface_digest"], SURFACE_DIGEST);
    assert_eq!(surface["membership_source"], "live listing");
    assert_eq!(
        surface["embedded_metadata_role"],
        "supplements matching exact ids only"
    );

    let expected: BTreeMap<&str, (&str, u64)> = LITERALS
        .iter()
        .map(|(name, literal, count)| (*name, (*literal, *count)))
        .collect();
    let literals = surface["literals"].as_array().expect("literals");
    assert_eq!(literals.len(), LITERALS.len());
    for entry in literals {
        let name = entry["name"].as_str().expect("literal name");
        let (literal, count) = expected
            .get(name)
            .unwrap_or_else(|| panic!("unknown catalogue literal {name}"));
        assert_eq!(entry["literal"], *literal, "{name}");
        assert_eq!(entry["counts"], *count, "{name}");
    }

    for hop in identity["hops"].as_array().expect("hops") {
        let counts = hop["catalogue_literal_counts"]
            .as_object()
            .expect("literal counts");
        for (name, literal, count) in LITERALS {
            assert_eq!(
                counts.get(*name),
                Some(&serde_json::json!(*count)),
                "{literal}"
            );
        }
    }
}

#[test]
fn argv_grammar_matches_the_exact_1_0_30_help_surface() {
    let grammar = &json(IDENTITY)["argv_grammar"];
    assert_exact_string_set(&grammar["accepted"], &["--no-auto-update", "models"]);
    assert_eq!(grammar["root_flag_before_subcommand_accepted"], true);
    assert_eq!(grammar["root_flag_after_subcommand_rejected"], true);
    assert_eq!(
        grammar["root_flag_after_subcommand_error"],
        "unexpected argument '--no-auto-update' found"
    );
    assert_exact_string_set(
        &grammar["models_subcommand_options"],
        &["--debug", "--debug-file", "-h", "--help", "--leader-socket"],
    );
    assert_eq!(grammar["prompt_argument_accepted"], false);
}

#[test]
fn identity_ran_without_provider_work_and_names_the_exact_point_advance() {
    let identity = json(IDENTITY);
    for flag in [
        "prompt_sent",
        "model_session_opened",
        "catalogue_command_executed",
        "credential_used",
        "download_executed",
        "host_install_changed",
    ] {
        assert_eq!(
            identity["no_provider_work_during_identity"][flag], false,
            "{flag} must stay false"
        );
    }
    let claim = &identity["claim_at_observation"];
    assert_eq!(claim["claim_id"], "grok-build.catalogue.executable-1-0-25");
    assert_eq!(claim["exact_point"], PREVIOUS_EXACT_POINT);
    assert_eq!(claim["posture"], "qualified_only");
    assert_eq!(
        claim["behavior_revision"],
        "grok-build.catalogue.models-text-v1"
    );

    let decision = &identity["decision"];
    assert_eq!(
        decision["segment_shape"],
        "compatible extension of the exact QualifiedOnly catalogue point"
    );
    assert_eq!(
        decision["claim_id_after"],
        "grok-build.catalogue.executable-1-0-30"
    );
    assert_eq!(decision["exact_point_after"], OFFICIAL_STABLE);
    assert_eq!(decision["posture_after"], "qualified_only");
    assert_eq!(
        decision["behavior_revision_after"],
        "grok-build.catalogue.models-text-v1"
    );
    assert!(
        !decision["reason"]
            .as_str()
            .expect("decision reason")
            .is_empty(),
        "the decision names why the point advances"
    );
}
