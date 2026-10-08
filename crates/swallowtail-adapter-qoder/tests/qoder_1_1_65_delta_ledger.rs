//! Complete artifact and selected-surface evidence for Qoder CLI `1.1.54..=1.1.65`.

use std::collections::BTreeSet;

use serde_json::Value;
use swallowtail_adapter_qoder::{
    QODER_PACKAGE_VERSION, qoder_headless_claim, qoder_package_binding,
};
use swallowtail_core::{InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/qoder-headless-1.1.65/identity.json");
const INVENTORY: &str = include_str!("fixtures/qoder-headless-1.1.65/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/qoder-headless-1.1.65/protocol.json");

const VERSIONS: [&str; 12] = [
    "1.1.54", "1.1.55", "1.1.56", "1.1.57", "1.1.58", "1.1.59", "1.1.60", "1.1.61", "1.1.62",
    "1.1.63", "1.1.64", "1.1.65",
];

const PACKAGE_FILES: [&str; 30] = [
    "package/LICENSE",
    "package/README.md",
    "package/bundle/builtin/agent-creator/SKILL.md",
    "package/bundle/builtin/hook-config/SKILL.md",
    "package/bundle/builtin/sdk/SKILL.md",
    "package/bundle/builtin/skill-creator/SKILL.md",
    "package/bundle/proto/chat.proto",
    "package/bundle/qoder-npm-dispatcher.cjs",
    "package/bundle/qoder-worker-runtime.mjs",
    "package/bundle/qodercli.js",
    "package/bundle/vendor/qoder-security/.qoder-plugin/plugin.json",
    "package/bundle/vendor/qoder-security/.qoder-plugin/qoder-hooks.json",
    "package/bundle/vendor/qoder-security/assets/logo.svg",
    "package/bundle/vendor/qoder-security/bin/bootstrap.cmd",
    "package/bundle/vendor/qoder-security/bin/bootstrap.sh",
    "package/bundle/vendor/qoder-security/bin/qodersec-launch.cmd",
    "package/bundle/vendor/qoder-security/bin/qodersec-launch.sh",
    "package/bundle/vendor/qoder-security/bin/qodersec-update.cmd",
    "package/bundle/vendor/qoder-security/bin/qodersec-update.sh",
    "package/bundle/vendor/qoder-security/bin/security-scan-settings.cmd",
    "package/bundle/vendor/qoder-security/bin/security-scan-settings.sh",
    "package/bundle/vendor/qoder-security/config.yaml.example",
    "package/bundle/vendor/qoder-security/package.json",
    "package/bundle/vendor/qoder-security/scripts/postinstall.sh",
    "package/bundle/vendor/qoder-security/security-patterns.yaml.example",
    "package/bundle/vendor/qoder-security/skills/security-scan/SKILL.md",
    "package/bundle/vendor/sites/artifact.json",
    "package/bundle/vendor/sites/sites.zip",
    "package/package.json",
    "package/postinstall.cjs",
];

const TARBALL_SHA256: [(&str, &str); 12] = [
    (
        "1.1.54",
        "a2c52c5d1edbf9ebb70af6c47a052ce705ba55b0c4e6221eddc1ce2563c61a9b",
    ),
    (
        "1.1.55",
        "ccb7f9e2a5c3e6bd7f3a853d34dce039f6ea827d9e1673149fb31ac53fc6f86a",
    ),
    (
        "1.1.56",
        "3266dba5ca02ed6e0c3366ee1b976bad95aedbfb1fd7c9867708f612a369d301",
    ),
    (
        "1.1.57",
        "d7b52026cc969353b59c77a19fcdb42e5bdd6da939299a9f0a84c03f7ecc2f24",
    ),
    (
        "1.1.58",
        "b3239185f2c9a64c7fe9e0661204de889fc008f49f955ef50ca37943e7348b85",
    ),
    (
        "1.1.59",
        "55b5fb6beeee33c03bc56c791d9455af01d22c369765fde944430dd369202d49",
    ),
    (
        "1.1.60",
        "41cae030690b8a4e8d9e097bf62d3ce5af4ca4fe63e60f90cfb93ec4229d08a5",
    ),
    (
        "1.1.61",
        "633df764e53e20afa881c8805858daf0df7d9476ac0ed3cda67f048418e86265",
    ),
    (
        "1.1.62",
        "60988bd5d1ea9bdf4944f9dd156b1b845ab0fc0047e866bc6032f72b5e3cf9c2",
    ),
    (
        "1.1.63",
        "d7f7eaa64f0d162ebfa1de2df66010714efae61778446bf867032815ff6fdd81",
    ),
    (
        "1.1.64",
        "ba61cfcfede367b7514c0de096f605fc98337d3fa5946aebe0d73cca190a79dd",
    ),
    (
        "1.1.65",
        "04e996fb9ed3b718098b477f269a32ef8aa00017b8e2c533eca3a653cd7ed824",
    ),
];

const QODERCLI_SHA256: [(&str, &str); 12] = [
    (
        "1.1.54",
        "d4fa03671e987d0d17cafc726e7e265fd9682a3fbe90c0e1ac0632c34891d417",
    ),
    (
        "1.1.55",
        "221c925e3eb940fe293c9bfe713061d76452c8ac6a8335fe9fd712148ded8950",
    ),
    (
        "1.1.56",
        "09dc6320672c20b9367359fe476ae446d18dc498ef75fca9b30394d1966f162f",
    ),
    (
        "1.1.57",
        "6faccad52f0a5fe3cf8a98ddaa795e32fc5963ad1e50c1592ed6d9f6dc4daea7",
    ),
    (
        "1.1.58",
        "3e0dbf4ee646db23931c88b5f799cf7362708cbe35f163aaf8265da3c2e9c970",
    ),
    (
        "1.1.59",
        "1e92acfed1f0029218a576d4977bcd1f0b3d654233a11b995e46ff584634d8b6",
    ),
    (
        "1.1.60",
        "142517b77df692a2920f40ebfd60c200493a8c0034e862eed24f7f1e9e7bf5d4",
    ),
    (
        "1.1.61",
        "34875088fd8af9c97e88446cbbfa11bca24c1b6558cec048ac1152bd5486f4e9",
    ),
    (
        "1.1.62",
        "29b27d5b440a71a57d42561355306e3a6f5b0196a046b1f57a09b67adbbc00ff",
    ),
    (
        "1.1.63",
        "c4604452562aa1b9a315d0da07d89faee540f4ddf87168f6b8943b2be1c1559a",
    ),
    (
        "1.1.64",
        "7568b1fadf9e514b5d084178810aeed16e6947ffc7adab94691dc5f4f975df1f",
    ),
    (
        "1.1.65",
        "fbb5ea617e687cfa6387ad80aeb897917545e3f7e0ba23de3577184f8834a330",
    ),
];

const WORKER_RUNTIME_SHA256: [(&str, &str); 12] = [
    (
        "1.1.54",
        "f0ca3b8b6d986b0a1edea55fd6556c74b872b5a95800733fe05850d50796def7",
    ),
    (
        "1.1.55",
        "aa990d3d5c3031e2b2d2b861b02cddbd94d2d7d3b594d49726f41ed8442995fa",
    ),
    (
        "1.1.56",
        "26b014cc855930ce0f9c6ae55cfa840db9c766eabb576f415573cbd6a7ff492c",
    ),
    (
        "1.1.57",
        "f05bb4e0ffed669952a6934a7d4356d438bdf654145a42964dceb1e38a9e033e",
    ),
    (
        "1.1.58",
        "2f1ba0309b65544e68653dd5b2152896f7dd47e23a36061be9f2b1653ae31cd1",
    ),
    (
        "1.1.59",
        "d0126cadcbe404bf2be9a1fc77dd1f2889cedfc98db5af0fc93b0731209fe042",
    ),
    (
        "1.1.60",
        "bbe396aa27be8ca0def180d944297e01716f82fc9e6613e16b3dc941dd4752db",
    ),
    (
        "1.1.61",
        "fa037b6b217815ff1c97ead255098de7119c7537df232eb01e3128128aa6bfcd",
    ),
    (
        "1.1.62",
        "8fedec957db81da0ba1c58f076c737ec6d959351cb523798834a31e5c1da98d9",
    ),
    (
        "1.1.63",
        "957d727ffc9c5cdd4caf77dfa697cb270e6eb5b5b883cb1df91472ad737c3480",
    ),
    (
        "1.1.64",
        "293e64d1e6bdc88b459d503bcfa25aef7e6e77d244793bf143f948f1e52581c5",
    ),
    (
        "1.1.65",
        "ccec40397597c4c1a17d38981c8066dd5b8cec6fc5f7cb8361221311680c710d",
    ),
];

const PACKAGE_JSON_SHA256: [(&str, &str); 12] = [
    (
        "1.1.54",
        "549e49bf4a97414b7d67d5fe3968c6e556a8691a36ee48c0d4b26ea6a2ab0f78",
    ),
    (
        "1.1.55",
        "32dddd6b0e8fa2bd34d8465137b426a4d0d3dd40b6cdbb45241cba5c788eef10",
    ),
    (
        "1.1.56",
        "1a059ed3c7671c66a121dcbaf152e9be9f0befd0f61ec38b4b5d859457979f94",
    ),
    (
        "1.1.57",
        "c80ce94f1622b7d15b60e8365bb6473de380b7bc122c97fc376d26f76282b310",
    ),
    (
        "1.1.58",
        "95ccd41b74a526dd817dbb28092981573ccbe0b82172518ace80a339556b20da",
    ),
    (
        "1.1.59",
        "3d36aaef11abdf7973ade6924744adc17c018cd4671609aca2d942cc80860ac2",
    ),
    (
        "1.1.60",
        "69b6af2afd8c9b0f373b78f11ac598da4d21a031b1609383a8b98e080cdd54d9",
    ),
    (
        "1.1.61",
        "42b5434198de0a8efb1e317540d4b4ef1c375143e5b30bb2f18661ce8ab5411e",
    ),
    (
        "1.1.62",
        "c53f3bd49882b19dccdbcc9d6f35981f36c630ce50f415b174998236d138f8cb",
    ),
    (
        "1.1.63",
        "bc1a2c3b61295edcc0584767b83799cbc2f333a93870fdc12589bbcdda4f192e",
    ),
    (
        "1.1.64",
        "77478eb78ee0784af93848f552ea19096b75c45e7872614ffe7bc0f18e836576",
    ),
    (
        "1.1.65",
        "9bdce1e1442d6c919ab9daae1a974dcd264176e38feaef42da6c42d6186d2018",
    ),
];

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn exact_keys(value: &Value, expected: &[&str], name: &str) {
    let actual = value
        .as_object()
        .unwrap_or_else(|| panic!("{name} must be an object"))
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "{name} keys changed");
}

fn string_set(value: &Value, name: &str) -> BTreeSet<String> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{name} must be an array"))
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .unwrap_or_else(|| panic!("{name} entries must be strings"))
                .to_owned()
        })
        .collect()
}

fn version_set() -> BTreeSet<String> {
    VERSIONS
        .iter()
        .map(|version| (*version).to_owned())
        .collect()
}

fn pair_map(entries: &[(&str, &str)]) -> serde_json::Map<String, Value> {
    entries
        .iter()
        .map(|(key, value)| ((*key).to_owned(), Value::String((*value).to_owned())))
        .collect()
}

#[test]
fn npm_identity_freezes_every_stable_hop_and_the_exact_current_artifact() {
    let identity = fixture(IDENTITY, "identity");
    exact_keys(
        &identity,
        &[
            "fixture_schema",
            "evidence_checked",
            "axis",
            "route",
            "npm_package",
            "registry",
            "source_correlation",
            "host",
            "npm_channel",
            "previous_ceiling",
            "published_stables_since_previous_ceiling",
            "published_stable_points",
            "first_unpublished_later_stable",
            "first_unpublished_verified",
            "missing_published_hops",
            "withdrawn_published_hops",
            "compared",
            "official",
            "claim_at_observation",
            "identity_decision",
            "artifact_verification",
            "live_evidence_disposition",
        ],
        "identity",
    );
    assert_eq!(identity["fixture_schema"], 1);
    assert_eq!(identity["axis"], "qoder.package");
    assert_eq!(identity["route"], "qoder.headless");
    assert_eq!(identity["npm_package"], "@qoder-ai/qodercli");
    assert_eq!(
        identity["registry"],
        "https://registry.npmjs.org/@qoder-ai/qodercli"
    );
    exact_keys(
        &identity["source_correlation"],
        &["repository", "commit", "authority"],
        "source correlation",
    );
    exact_keys(
        &identity["host"],
        &[
            "qoder_present",
            "qodercli_present",
            "node",
            "node_satisfies_engine",
        ],
        "host observation",
    );
    exact_keys(
        &identity["npm_channel"],
        &[
            "dist_tags",
            "latest",
            "beta",
            "beta_is_not_stable_authority",
        ],
        "npm channel",
    );
    exact_keys(
        &identity["npm_channel"]["dist_tags"],
        &["latest", "beta"],
        "dist tags",
    );
    assert_eq!(identity["previous_ceiling"], "1.1.54");
    assert_eq!(identity["npm_channel"]["latest"], "1.1.65");
    assert_eq!(identity["npm_channel"]["beta"], "1.1.54-beta.1");
    assert_eq!(identity["first_unpublished_later_stable"], "1.1.66");
    assert_eq!(identity["first_unpublished_verified"], true);
    assert_eq!(identity["missing_published_hops"], serde_json::json!([]));
    assert_eq!(identity["withdrawn_published_hops"], serde_json::json!([]));
    assert_eq!(identity["source_correlation"]["repository"], Value::Null);
    assert_eq!(identity["source_correlation"]["commit"], Value::Null);
    assert_eq!(identity["host"]["qoder_present"], false);
    assert_eq!(identity["host"]["qodercli_present"], false);
    assert_eq!(identity["host"]["node"], "22.23.2");
    assert_eq!(identity["host"]["node_satisfies_engine"], true);
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["segment"],
        serde_json::json!({
            "minimum":"1.1.54",
            "maximum":"1.1.65",
            "support_status":"Maintained"
        })
    );
    assert_eq!(
        identity["identity_decision"]["posture_retained"],
        "QualifiedOnly"
    );
    assert_eq!(identity["identity_decision"]["baseline_retained"], true);
    assert_eq!(
        identity["identity_decision"]["previous_point_retained"],
        true
    );
    assert_eq!(identity["identity_decision"]["claim_id_retained"], true);
    assert_eq!(
        identity["claim_at_observation"]["id"],
        "qoder.headless.package-window-2"
    );
    assert_eq!(
        identity["identity_decision"]["behavior_revision_retained"],
        true
    );
    assert_eq!(
        identity["claim_at_observation"]["behavior_revision"],
        "qoder.headless.stdio-stream-json-v2"
    );
    assert_eq!(
        identity["identity_decision"]["exclusions_retained"],
        serde_json::json!([])
    );
    assert_eq!(identity["identity_decision"]["new_operations"], false);
    assert_eq!(
        identity["identity_decision"]["selected_skill_visibility_promoted"],
        false
    );
    exact_keys(
        &identity["identity_decision"],
        &[
            "shape",
            "baseline_retained",
            "previous_point_retained",
            "claim_id_retained",
            "behavior_revision_retained",
            "posture_retained",
            "segment",
            "exclusions_retained",
            "new_operations",
            "selected_skill_visibility_promoted",
        ],
        "identity decision",
    );
    assert_eq!(
        identity["published_stables_since_previous_ceiling"],
        serde_json::json!([
            "1.1.55", "1.1.56", "1.1.57", "1.1.58", "1.1.59", "1.1.60", "1.1.61", "1.1.62",
            "1.1.63", "1.1.64", "1.1.65"
        ])
    );
    assert_eq!(string_set(&identity["compared"], "compared"), version_set());
    let points = identity["published_stable_points"]
        .as_object()
        .expect("published points object");
    assert_eq!(
        points.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        VERSIONS.iter().copied().collect()
    );
    for (version, sha256) in TARBALL_SHA256 {
        let point = &identity["published_stable_points"][version];
        exact_keys(
            point,
            &[
                "published_at",
                "tarball",
                "tarball_sha256",
                "shasum",
                "integrity",
                "file_count",
                "unpacked_size",
                "source_repository",
                "deprecated",
            ],
            version,
        );
        assert_eq!(point["tarball_sha256"], sha256, "{version} tarball digest");
        assert_eq!(point["source_repository"], Value::Null, "{version} source");
        assert_eq!(point["deprecated"], false, "{version} npm status");
        assert_eq!(point["file_count"], 30, "{version} file count");
    }
    exact_keys(
        &identity["claim_at_observation"],
        &[
            "id",
            "baseline",
            "latest_qualified",
            "scheme",
            "newer_version_posture",
            "behavior_revision",
            "segments",
            "exclusions",
        ],
        "claim at observation",
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.1.54"
    );
    assert_eq!(
        identity["claim_at_observation"]["newer_version_posture"],
        "QualifiedOnly"
    );
    assert_eq!(
        identity["claim_at_observation"]["segments"],
        serde_json::json!([{
            "minimum":"1.1.54",
            "maximum":"1.1.54",
            "support_status":"Maintained"
        }])
    );
    exact_keys(
        &identity["artifact_verification"],
        &[
            "registry_shasum_verified",
            "registry_integrity_verified",
            "tarball_sha256_recorded",
            "complete_tree_inventory",
            "artifact_execution",
            "npm_install",
            "postinstall_execution",
        ],
        "artifact verification",
    );
    exact_keys(
        &identity["live_evidence_disposition"],
        &[
            "provider_operation",
            "provider_prompt",
            "credentials_used",
            "host_install_changed",
            "downloaded_artifact_executed",
        ],
        "live evidence disposition",
    );
    assert_eq!(
        identity["official"]["tarball_sha256"],
        "04e996fb9ed3b718098b477f269a32ef8aa00017b8e2c533eca3a653cd7ed824"
    );
    assert_eq!(
        identity["official"]["integrity"],
        "sha512-dwJXur4lFovjNFGEjoSG04HhrrC+Ai/ENzBLkSlgk4xKEjEnsAO1wgIoufFBLAox8xlm6c+29LxKNT3RjDD2wA=="
    );
    assert_eq!(identity["official"]["file_count"], 30);
    assert_eq!(identity["official"]["unpacked_size"], 68_028_943);
    assert_eq!(
        identity["artifact_verification"]["registry_shasum_verified"],
        true
    );
    assert_eq!(
        identity["artifact_verification"]["registry_integrity_verified"],
        true
    );
    assert_eq!(
        identity["artifact_verification"]["artifact_execution"],
        false
    );
    assert_eq!(identity["artifact_verification"]["npm_install"], false);
    assert_eq!(
        identity["artifact_verification"]["postinstall_execution"],
        false
    );
}

#[test]
fn complete_tree_ledger_binds_exact_files_and_every_changed_hop() {
    let identity = fixture(IDENTITY, "identity");
    let inventory = fixture(INVENTORY, "dist inventory");
    exact_keys(
        &inventory,
        &[
            "fixture_schema",
            "compared",
            "package_file_counts",
            "package_unpacked_sizes",
            "files",
            "identical_through_1_1_54_to_1_1_65",
            "from_hop_to_hop",
            "hashes",
            "changed_file_classifications",
        ],
        "dist inventory",
    );
    assert_eq!(inventory["fixture_schema"], 1);
    assert_eq!(
        string_set(&inventory["compared"], "inventory compared"),
        version_set()
    );
    let expected_paths = PACKAGE_FILES
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    let classified_paths = [
        "package/bundle/qodercli.js",
        "package/bundle/qoder-worker-runtime.mjs",
        "package/package.json",
        "package/bundle/vendor/qoder-security/.qoder-plugin/plugin.json",
        "package/bundle/vendor/qoder-security/.qoder-plugin/qoder-hooks.json",
        "package/bundle/vendor/qoder-security/bin/qodersec-launch.cmd",
        "package/bundle/vendor/qoder-security/bin/qodersec-launch.sh",
        "package/bundle/vendor/qoder-security/config.yaml.example",
        "package/bundle/vendor/qoder-security/package.json",
        "package/bundle/vendor/qoder-security/skills/security-scan/SKILL.md",
    ];
    exact_keys(
        &inventory["changed_file_classifications"],
        &classified_paths,
        "changed file classifications",
    );
    assert_eq!(
        inventory["changed_file_classifications"],
        serde_json::json!({
            "package/bundle/qodercli.js": "selected CLI bundle; the npm artifact includes the chosen print, stream-json, permission, persistence and AgentLoop-bound implementation; private internals remain outside Swallowtail mapping; the 1.1.61 repeated-tool-denial normalization still leaves provider error details outside the selected route mapping",
            "package/bundle/qoder-worker-runtime.mjs": "provider runtime bundle; versioned internal tool and stream implementation; only the frozen headless event fields and fail-closed decoder boundary are claimed; from 1.1.61 it classifies terminal_reason repeated_tool_call_denied into a private error_code while preserving other result fields",
            "package/package.json": "release metadata; version and optional platform ripgrep package versions move with the release; package name, entrypoints, runtime floor, dependencies and install script remain stable",
            "package/bundle/vendor/qoder-security/.qoder-plugin/plugin.json": "vendored provider plugin metadata; plugin capability and selected-skill visibility are outside qoder.headless",
            "package/bundle/vendor/qoder-security/.qoder-plugin/qoder-hooks.json": "vendored plugin hooks; hooks are not selected or asserted by the headless route",
            "package/bundle/vendor/qoder-security/bin/qodersec-launch.cmd": "vendored plugin launcher; provider-internal and not invoked by the selected Swallowtail argv",
            "package/bundle/vendor/qoder-security/bin/qodersec-launch.sh": "vendored plugin launcher; provider-internal and not invoked by the selected Swallowtail argv",
            "package/bundle/vendor/qoder-security/config.yaml.example": "vendored plugin example configuration; no Swallowtail configuration mapping",
            "package/bundle/vendor/qoder-security/package.json": "vendored plugin package metadata; plugin release remains outside the qoder.package headless mapping",
            "package/bundle/vendor/qoder-security/skills/security-scan/SKILL.md": "vendored plugin skill text; selected-run skills visibility remains unqualified under Research 256 and its independent gate"
        }),
        "exact changed-file classifications"
    );
    let files = inventory["files"]
        .as_object()
        .expect("complete file manifests");
    assert_eq!(
        files.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        VERSIONS.iter().copied().collect()
    );
    for version in VERSIONS {
        let manifest = files[version].as_array().expect("version file manifest");
        assert_eq!(manifest.len(), 30, "{version} full tree count");
        let actual_paths = manifest
            .iter()
            .map(|entry| {
                exact_keys(entry, &["path", "sha256", "size"], "file entry");
                let digest = entry["sha256"].as_str().expect("SHA-256 digest");
                assert_eq!(digest.len(), 64, "{version} SHA-256 length");
                entry["path"]
                    .as_str()
                    .expect("relative file path")
                    .to_owned()
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(
            actual_paths, expected_paths,
            "{version} exact package files"
        );
        assert_eq!(inventory["package_file_counts"][version], 30);
        assert!(
            inventory["package_unpacked_sizes"][version]
                .as_u64()
                .is_some()
        );
        assert_eq!(
            identity["published_stable_points"][version]["file_count"],
            30
        );
    }

    let hops = inventory["from_hop_to_hop"]
        .as_object()
        .expect("hop ledger");
    assert_eq!(hops.len(), VERSIONS.len() - 1);
    let common_changes = BTreeSet::from([
        "package/bundle/qoder-worker-runtime.mjs".to_owned(),
        "package/bundle/qodercli.js".to_owned(),
        "package/package.json".to_owned(),
    ]);
    let security_changes = BTreeSet::from([
        "package/bundle/vendor/qoder-security/.qoder-plugin/plugin.json".to_owned(),
        "package/bundle/vendor/qoder-security/bin/qodersec-launch.cmd".to_owned(),
        "package/bundle/vendor/qoder-security/bin/qodersec-launch.sh".to_owned(),
        "package/bundle/vendor/qoder-security/package.json".to_owned(),
    ]);
    let skill_and_config_changes = BTreeSet::from([
        "package/bundle/vendor/qoder-security/.qoder-plugin/plugin.json".to_owned(),
        "package/bundle/vendor/qoder-security/.qoder-plugin/qoder-hooks.json".to_owned(),
        "package/bundle/vendor/qoder-security/bin/qodersec-launch.cmd".to_owned(),
        "package/bundle/vendor/qoder-security/bin/qodersec-launch.sh".to_owned(),
        "package/bundle/vendor/qoder-security/config.yaml.example".to_owned(),
        "package/bundle/vendor/qoder-security/package.json".to_owned(),
        "package/bundle/vendor/qoder-security/skills/security-scan/SKILL.md".to_owned(),
    ]);
    let plugin_and_skill_changes = BTreeSet::from([
        "package/bundle/vendor/qoder-security/.qoder-plugin/plugin.json".to_owned(),
        "package/bundle/vendor/qoder-security/bin/qodersec-launch.cmd".to_owned(),
        "package/bundle/vendor/qoder-security/bin/qodersec-launch.sh".to_owned(),
        "package/bundle/vendor/qoder-security/config.yaml.example".to_owned(),
        "package/bundle/vendor/qoder-security/package.json".to_owned(),
        "package/bundle/vendor/qoder-security/skills/security-scan/SKILL.md".to_owned(),
    ]);
    for (older, newer) in VERSIONS.iter().zip(VERSIONS.iter().skip(1)) {
        let hop_name = format!("{older}..{newer}");
        let hop = &hops[&hop_name];
        exact_keys(
            hop,
            &["added", "removed", "changed", "identical"],
            &hop_name,
        );
        assert_eq!(hop["added"], serde_json::json!([]), "{hop_name} additions");
        assert_eq!(hop["removed"], serde_json::json!([]), "{hop_name} removals");
        let mut expected_changes = common_changes.clone();
        match hop_name.as_str() {
            "1.1.58..1.1.59" => expected_changes.extend(skill_and_config_changes.clone()),
            "1.1.59..1.1.60" => expected_changes.extend(security_changes.clone()),
            "1.1.64..1.1.65" => expected_changes.extend(plugin_and_skill_changes.clone()),
            _ => {}
        }
        assert_eq!(
            string_set(&hop["changed"], &hop_name),
            expected_changes,
            "{hop_name} deltas"
        );
        let expected_identical = expected_paths
            .difference(&expected_changes)
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_eq!(
            string_set(&hop["identical"], &hop_name),
            expected_identical,
            "{hop_name} byte-identical files"
        );
    }

    let hashes = inventory["hashes"].as_object().expect("key file hashes");
    let expected_hash_paths = [
        "package/package.json",
        "package/bundle/qodercli.js",
        "package/bundle/qoder-worker-runtime.mjs",
        "package/bundle/qoder-npm-dispatcher.cjs",
        "package/bundle/proto/chat.proto",
        "package/postinstall.cjs",
        "package/bundle/vendor/qoder-security/.qoder-plugin/plugin.json",
        "package/bundle/vendor/qoder-security/.qoder-plugin/qoder-hooks.json",
        "package/bundle/vendor/qoder-security/package.json",
        "package/bundle/vendor/qoder-security/skills/security-scan/SKILL.md",
        "package/bundle/vendor/sites/artifact.json",
        "package/bundle/vendor/sites/sites.zip",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(
        hashes.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        expected_hash_paths
    );
    for (path, expected) in [
        ("package/bundle/qodercli.js", &QODERCLI_SHA256[..]),
        (
            "package/bundle/qoder-worker-runtime.mjs",
            &WORKER_RUNTIME_SHA256[..],
        ),
        ("package/package.json", &PACKAGE_JSON_SHA256[..]),
    ] {
        let expected = pair_map(expected);
        assert_eq!(
            hashes[path],
            Value::Object(expected),
            "{path} digest ledger"
        );
    }
    let expected_unchanged = expected_paths
        .difference(&common_changes)
        .filter(|path| !classified_paths.contains(&path.as_str()))
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        string_set(
            &inventory["identical_through_1_1_54_to_1_1_65"],
            "byte-identical through all hops"
        ),
        expected_unchanged
    );
    for (path, expected) in [
        (
            "package/bundle/qoder-npm-dispatcher.cjs",
            "37cc389f07b046d78a2c80ff8d9ab54f433d38f94bf1f221a9e7e1dea297d120",
        ),
        (
            "package/bundle/proto/chat.proto",
            "852c39ce8447854dc1c452e44304db81978595b4961ec4beaef4762ee5141372",
        ),
        (
            "package/postinstall.cjs",
            "5b9995a17600678f17b4226582ed45dce097c6f79249d9a546c7453fc5f8f220",
        ),
        (
            "package/bundle/vendor/sites/artifact.json",
            "a24223e746eb2d082374f06c1623da0b4d98c6e7a34826a83e3ae46fee2b8c46",
        ),
        (
            "package/bundle/vendor/sites/sites.zip",
            "5751bf07de10c632834c2a96d5a6817d9d65c815d1e6bcadcf5877d6d5e6a51c",
        ),
    ] {
        let values = hashes[path].as_object().expect("per-version file hashes");
        assert_eq!(values.len(), VERSIONS.len(), "{path} version coverage");
        assert!(
            values.values().all(|digest| digest == expected),
            "{path} identity"
        );
    }
}

#[test]
fn selected_stream_contract_and_skill_visibility_limit_stay_unchanged() {
    let protocol = fixture(PROTOCOL, "protocol");
    exact_keys(
        &protocol,
        &[
            "fixture_schema",
            "artifact_revision",
            "compared",
            "route_id",
            "axis",
            "protocol_facade_revision",
            "selected_invocation",
            "selected_presence_all_hops",
            "selected_behaviour",
            "mode_dispatch",
            "selected_failure",
            "versioned_selected_failure_markers",
            "classified_deltas",
            "unmapped_boundaries",
            "live_evidence_disposition",
        ],
        "protocol",
    );
    assert_eq!(protocol["fixture_schema"], 1);
    assert_eq!(protocol["artifact_revision"], "1.1.65");
    assert_eq!(protocol["route_id"], "qoder.headless");
    assert_eq!(protocol["axis"], "qoder.package");
    assert_eq!(
        protocol["protocol_facade_revision"],
        "qoder.headless.stdio-stream-json-v1"
    );
    assert_eq!(
        protocol["selected_invocation"]["argv"],
        serde_json::json!([
            "qodercli",
            "--print",
            "--output-format",
            "stream-json",
            "--permission-mode",
            "dont_ask",
            "--max-turns",
            "8",
            "--no-session-persistence",
            "--cwd",
            "<cwd>",
            "<prompt>"
        ])
    );
    exact_keys(
        &protocol["selected_presence_all_hops"],
        &[
            "--print",
            "--output-format",
            "stream-json",
            "--permission-mode",
            "dont_ask",
            "--max-turns",
            "--no-session-persistence",
            "--cwd",
            "error_max_turns",
            "Maximum turns exceeded",
            "num_turns",
            "error_during_execution",
            "Operation aborted",
            "aborted_streaming",
            "protocol_version",
            "stream_event",
        ],
        "selected protocol markers",
    );
    assert!(
        protocol["selected_presence_all_hops"]
            .as_object()
            .expect("selected marker map")
            .values()
            .all(|value| value == true)
    );
    exact_keys(
        &protocol["selected_behaviour"],
        &[
            "stream_json_ndjson",
            "assistant_text_and_result_decoder",
            "permission_mode_dont_ask",
            "no_session_persistence",
            "adapter_owned_max_turns_8",
            "one_owned_stdio_child",
            "abort_and_join_cleanup",
            "host_process_deadline",
        ],
        "selected behaviour",
    );
    assert!(
        protocol["selected_behaviour"]
            .as_object()
            .expect("selected behaviour map")
            .values()
            .all(|value| value == true)
    );
    assert_eq!(
        protocol["selected_failure"]["max_turns_subtype"],
        "error_max_turns"
    );
    assert_eq!(protocol["selected_failure"]["max_turns_is_error"], true);
    assert_eq!(protocol["selected_failure"]["max_turns_num_turns"], 8);
    assert_eq!(
        protocol["selected_failure"]["other_error_terminal_projection"],
        "provider_failed"
    );
    exact_keys(
        &protocol["versioned_selected_failure_markers"]["repeated_tool_call_denied"],
        &[
            "first_seen",
            "present_on",
            "runtime_action",
            "adapter_projection",
            "dedicated_adapter_mapping",
        ],
        "versioned failure marker",
    );
    let repeated_denial =
        &protocol["versioned_selected_failure_markers"]["repeated_tool_call_denied"];
    assert_eq!(repeated_denial["first_seen"], "1.1.61");
    assert_eq!(
        repeated_denial["present_on"],
        serde_json::json!(["1.1.61", "1.1.62", "1.1.63", "1.1.64", "1.1.65"])
    );
    assert_eq!(repeated_denial["dedicated_adapter_mapping"], false);
    let classified_hops = protocol["classified_deltas"]
        .as_object()
        .expect("classified hops");
    assert_eq!(classified_hops.len(), VERSIONS.len() - 1);
    let generic_hop = "qodercli.js and qoder-worker-runtime.mjs changed; selected invocation and terminal markers remain present. package.json changes only release and optional ripgrep versions.";
    let versioned_failure_hop = "qodercli.js and qoder-worker-runtime.mjs changed; the runtime adds private repeated_tool_call_denied error-code normalization while preserving the result envelope for the adapter generic failure projection. Selected invocation and terminal markers remain present. package.json changes only release and optional ripgrep versions.";
    for (older, newer) in VERSIONS.iter().zip(VERSIONS.iter().skip(1)) {
        let hop_name = format!("{older}..{newer}");
        let expected = if hop_name == "1.1.60..1.1.61" {
            versioned_failure_hop
        } else {
            generic_hop
        };
        assert_eq!(
            classified_hops[&hop_name], expected,
            "{hop_name} classification"
        );
    }
    assert_eq!(classified_hops["1.1.60..1.1.61"], versioned_failure_hop);
    exact_keys(
        &protocol["unmapped_boundaries"]["skills_and_plugins"],
        &[
            "selected_skill_bundle",
            "init_skills_field_mapped",
            "init_plugins_field_mapped",
            "research",
            "independent_gate_remains_open",
        ],
        "skills and plugins boundary",
    );
    assert_eq!(
        protocol["unmapped_boundaries"]["skills_and_plugins"]["selected_skill_bundle"],
        false
    );
    assert_eq!(
        protocol["unmapped_boundaries"]["skills_and_plugins"]["init_skills_field_mapped"],
        false
    );
    assert_eq!(
        protocol["unmapped_boundaries"]["skills_and_plugins"]["init_plugins_field_mapped"],
        false
    );
    assert_eq!(
        protocol["unmapped_boundaries"]["skills_and_plugins"]["research"],
        256
    );
    assert_eq!(
        protocol["unmapped_boundaries"]["skills_and_plugins"]["independent_gate_remains_open"],
        true
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["provider_operation"],
        false
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["provider_prompt"],
        false
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["credentials_used"],
        false
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["downloaded_artifact_executed"],
        false
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["host_install_changed"],
        false
    );
}

#[test]
fn production_claim_extends_the_same_qualified_only_behavior_window() {
    let identity = fixture(IDENTITY, "identity");
    let claim = qoder_headless_claim();
    assert_eq!(QODER_PACKAGE_VERSION, "1.1.65");
    assert_eq!(
        claim.id().as_str(),
        identity["claim_at_observation"]["id"]
            .as_str()
            .expect("claim id")
    );
    assert_eq!(claim.axis().as_str(), "qoder.package");
    assert_eq!(claim.baseline().as_str(), "1.1.54");
    assert_eq!(claim.latest_qualified().as_str(), "1.1.65");
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert!(claim.exclusions().next().is_none());
    assert_eq!(claim.milestones().len(), 1);
    let segment = claim.milestones().next().expect("maintained segment");
    assert_eq!(segment.minimum().as_str(), "1.1.54");
    assert_eq!(segment.maximum().as_str(), "1.1.65");
    assert_eq!(
        segment.behavior_revision().as_str(),
        "qoder.headless.stdio-stream-json-v2"
    );
    assert_eq!(segment.support_status(), InterfaceSupportStatus::Maintained);
    assert_eq!(
        identity["identity_decision"]["segment"],
        serde_json::json!({
            "minimum":"1.1.54",
            "maximum":"1.1.65",
            "support_status":"Maintained"
        })
    );

    for version in VERSIONS {
        let binding = qoder_package_binding(version).expect("qualified stable version");
        assert_eq!(binding.axis().as_str(), claim.axis().as_str());
        assert_eq!(binding.version().as_str(), version);
        assert!(claim.assess(binding.version()).is_permitted(), "{version}");
    }
    for version in ["1.1.53", "1.1.66", "1.1.65-beta.1"] {
        assert!(qoder_package_binding(version).is_none(), "{version}");
        assert!(
            !claim
                .assess(&InterfaceVersion::new(version).expect("valid semver point"))
                .is_permitted(),
            "{version}"
        );
    }
    assert!(qoder_package_binding("1.1.65+build.1").is_none());
}
