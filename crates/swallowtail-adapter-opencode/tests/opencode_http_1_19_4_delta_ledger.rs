use std::collections::BTreeSet;

use serde_json::{Map, Value};
use swallowtail_adapter_opencode::{
    OPENCODE_LATEST_QUALIFIED_VERSION, opencode_acp_claim, opencode_http_claim,
    opencode_server_binding,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus};

const IDENTITY: &str = include_str!("fixtures/opencode-1.19.4/identity.json");
const INVENTORY: &str = include_str!("fixtures/opencode-1.19.4/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.19.4/protocol.json");
const CLAIM: &str = include_str!("fixtures/opencode-1.19.4/claim.json");

const VERSIONS: [&str; 6] = ["1.18.35", "1.19.0", "1.19.1", "1.19.2", "1.19.3", "1.19.4"];

const RELEASES: [[&str; 10]; 6] = [
    [
        "1.18.35",
        "2026-10-06T20:21:30.784Z",
        "4d3408d0950d70cf870efe3f86e8bd0271d8149bea0767008d42de14f0986a04",
        "08cd9a77993031d2c66289d82a99f10227af7fac",
        "53d1eabb61e21162157817bf677da0a4ad3332e3",
        "4ac0d9c3d169bbe81d9570013effdda3fe24d36e",
        "2026-10-06T20:18:39Z",
        "3092a7b9f55d80c42a9c1bb2e2cc0a9961316673faf92b741577257a1576dd59",
        "c8f888b451f5494a18f858fffb0e0b68f4e4baa9c241761c5f206884f0fa640d",
        "60669097",
    ],
    [
        "1.19.0",
        "2026-10-11T01:38:53.143Z",
        "ebb8bf46c6a3ac363b568296dccfeda75cdb4360db097edd43e947b1c742b1bd",
        "5a655f72fc2601d39cd3373340ccb8ed1a2ace49",
        "733b2c627e5aec39cb33cc22373bdce33053ef2c",
        "add9b4ba5adfb895934cb1bf46f590342cec68d2",
        "2026-10-11T01:39:22Z",
        "5dfcfffdf1df64d4a2e8e0e034b5128eb2376cab6d6d34ed808108aac4ce0d78",
        "7bb9e61ad6feacf2e89c10a6a2c1840e3de4220bb50ba1fc7110f31806076aec",
        "60685698",
    ],
    [
        "1.19.1",
        "2026-10-15T19:20:16.648Z",
        "b64566f66b0e46eec2d54512a35ece43a8391c89dd815a0fc09f9c881590b571",
        "41494cbe3bcc68efdf210afe0ee6307710c83dd8",
        "4a929747483f9e19b05c7cf944fc0aca6e5111b0",
        "d320177aa4cb93d63b6462acd6efca0ee037dedf",
        "2026-10-15T19:19:45Z",
        "d96b32749ab8e96fa553086d9d03c0356022a3af312911ed065bd69f22c261be",
        "6f1904b860bdbd32e90e679c9e1b0e64e78269a353e1497d5ea792dfa449538c",
        "60707458",
    ],
    [
        "1.19.2",
        "2026-10-16T02:29:31.737Z",
        "4cc647226daff4d19520597f76b94ce07c63cd34bd05730597c375e3797ea440",
        "2313e411d5f44f3d33c61ad7f087ac7ee4bd9d84",
        "7901b7d325c34852caa84fc2c325f3a08dd7a473",
        "3be039593a9a2cbe44f9578e37ac2bd789768262",
        "2026-10-16T02:30:38Z",
        "d55e90544ff8392149903c33a7b059e0c748851c67043b8323915bfe1848af87",
        "4eff8d2cf8614ead97eadf1123d46033dc975eadeac3df04644a64bae0f9e819",
        "60705540",
    ],
    [
        "1.19.3",
        "2026-10-16T12:32:00.280Z",
        "0ca1a05ab99438bdfbcb40ee3f82ebd46f57679db40ab3492ce8545b106990dc",
        "f5ed6b0ea52c5be2484a41f130365bafbfc9cf2c",
        "e3c4eb242585dcd77be1a867391b7c25fd47ce52",
        "60c9be092129745688bb3c3a752affc6cbd66ede",
        "2026-10-16T12:31:52Z",
        "8395f12e26df2d16610760ae224daf87da1d253a9883bf230be2c53984fe99ea",
        "13a8fb2978302071b86f9d7e00da9c8895f9cd846978379c3bdc6a5333536c9b",
        "60716380",
    ],
    [
        "1.19.4",
        "2026-10-22T13:03:43.704Z",
        "2ceca45c7e0bbd50ddd3f4d6d651ed43645647d74279d656e7c7747a4b6a9137",
        "20d2a9916eb87ca1a70090089e67eca1b92a6b85",
        "148842ef7cb1bc34192b3fff57d0f9721ab9c801",
        "c0c7bae726798d02592f8d3305891ba2f0cd880a",
        "2026-10-22T13:03:41Z",
        "2e553ed005b2df4a7efb9edd26db8fc3caafb1b4dfc3c5c3869192334584d8bb",
        "849bd6b3d959a1d2ef200ed1634a51c79a45ca236aa26665d75debd694805cce",
        "79667571",
    ],
];

const TREES: [(&str, u64, u64, u64, u64, &str, &str); 6] = [
    (
        "1.18.35",
        6628,
        6568,
        60,
        409,
        "616f4afe54525f9d0a7cdcbf0818f1f00c6cba722daaa7f1b4f9c2deef1216b2",
        "3092a7b9f55d80c42a9c1bb2e2cc0a9961316673faf92b741577257a1576dd59",
    ),
    (
        "1.19.0",
        6634,
        6574,
        60,
        409,
        "82ca5e56310fd21977a588911f6708ecba8396f1bb52b9cf5d7e8ed0eb2f0cda",
        "5dfcfffdf1df64d4a2e8e0e034b5128eb2376cab6d6d34ed808108aac4ce0d78",
    ),
    (
        "1.19.1",
        6632,
        6572,
        60,
        408,
        "989fa05f7347be8aacdb6bb4c2725116f55ae8e29b02d75cd71314511be83148",
        "d96b32749ab8e96fa553086d9d03c0356022a3af312911ed065bd69f22c261be",
    ),
    (
        "1.19.2",
        6634,
        6574,
        60,
        408,
        "942e41509a9cf958a79089ff3072838ead3b1a6ab6a8ab8866a4520c4f986d57",
        "d55e90544ff8392149903c33a7b059e0c748851c67043b8323915bfe1848af87",
    ),
    (
        "1.19.3",
        6634,
        6574,
        60,
        408,
        "0fcdf1dfa5dec44ad05a2f14e264fe0dd37cbea395d04ae217c84b071e2f7aca",
        "8395f12e26df2d16610760ae224daf87da1d253a9883bf230be2c53984fe99ea",
    ),
    (
        "1.19.4",
        6636,
        6576,
        60,
        408,
        "ae0d6884639fda9deb79082e5b27483c9b6a6f45fb8868dadd89714722cbf287",
        "2e553ed005b2df4a7efb9edd26db8fc3caafb1b4dfc3c5c3869192334584d8bb",
    ),
];

type Hop = (
    &'static str,
    u64,
    u64,
    u64,
    u64,
    &'static str,
    &'static str,
    &'static str,
);

const HOPS: [Hop; 5] = [
    (
        "1.18.35_to_1.19.0",
        6,
        0,
        132,
        6496,
        "6e2952483d2a3ea4c0c42a35a49d97ec8cad5f8ddbac313e7cb5501ed3c24912",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "f8e4b4bcc49d049e9ca902922c6849fa14a06cc4f194d68865d94be76f16312d",
    ),
    (
        "1.19.0_to_1.19.1",
        0,
        2,
        66,
        6566,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "48ac85681e814d6834baf01b23e3a7468c20a28e32c371d04d0992aceb71ffaf",
        "dbda759d04dfe29ed66ce8a705bf0167d91e0d821e7666a4e7a9f534ad88a461",
    ),
    (
        "1.19.1_to_1.19.2",
        2,
        0,
        41,
        6591,
        "98ffaff23d851bdaf783266b51c93d3ecf2477202c93e21b033c9b653066b82e",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "1a62c90b489ccd1b2f5dcb11898572c8113656d8dc19912f74b993fcdf2ba542",
    ),
    (
        "1.19.2_to_1.19.3",
        0,
        0,
        36,
        6598,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "0bfa26392c13bbec41d0f8ae6eeeff75d855ec60b6744bc8cf77c39ef0844816",
    ),
    (
        "1.19.3_to_1.19.4",
        2,
        0,
        46,
        6588,
        "139e7cec0a30bd65dbdf806a803ba7462672317eae3522470c09411c21130023",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "4d2fd9cf80c08b925f2a0878a9b843f060f3f84740104e2e64985f132db584a0",
    ),
];

const IMPLEMENTATION_DELTAS: [(&str, &[&str]); 5] = [
    (
        "1.18.35_to_1.19.0",
        &[
            "packages/core/package.json",
            "packages/core/src/github-copilot/chat/convert-to-openai-compatible-chat-messages.ts",
            "packages/core/src/github-copilot/chat/openai-compatible-chat-language-model.ts",
            "packages/core/src/v1/config/config.ts",
            "packages/opencode/package.json",
            "packages/opencode/src/cli/cmd/tui.ts",
            "packages/opencode/src/cli/cmd/upgrade.ts",
            "packages/opencode/src/cli/tui/worker.ts",
            "packages/opencode/src/cli/upgrade.ts",
            "packages/opencode/src/installation/index.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/pty.ts",
            "packages/sdk/js/package.json",
            "packages/sdk/js/src/v2/gen/types.gen.ts",
            "packages/sdk/openapi.json",
        ],
    ),
    (
        "1.19.0_to_1.19.1",
        &[
            "packages/core/package.json",
            "packages/opencode/package.json",
            "packages/opencode/src/cli/cmd/github.handler.ts",
            "packages/opencode/src/cli/cmd/import.ts",
            "packages/opencode/src/cli/cmd/pr.ts",
            "packages/opencode/src/cli/cmd/run.ts",
            "packages/opencode/src/effect/app-runtime.ts",
            "packages/opencode/src/effect/bootstrap-runtime.ts",
            "packages/opencode/src/project/bootstrap.ts",
            "packages/opencode/src/server/routes/instance/httpapi/server.ts",
            "packages/opencode/src/share/session.ts",
            "packages/opencode/src/share/share-next.ts",
            "packages/sdk/js/package.json",
        ],
    ),
    (
        "1.19.1_to_1.19.2",
        &[
            "packages/core/package.json",
            "packages/opencode/package.json",
            "packages/opencode/src/cli/cmd/tui.ts",
            "packages/opencode/src/cli/cmd/upgrade.ts",
            "packages/opencode/src/cli/upgrade.ts",
            "packages/opencode/src/installation/index.ts",
            "packages/sdk/js/package.json",
        ],
    ),
    (
        "1.19.2_to_1.19.3",
        &[
            "packages/core/package.json",
            "packages/core/src/v1/config/provider.ts",
            "packages/opencode/package.json",
            "packages/opencode/src/provider/provider.ts",
            "packages/opencode/src/provider/transform.ts",
            "packages/sdk/js/package.json",
            "packages/sdk/js/src/v2/gen/types.gen.ts",
            "packages/sdk/openapi.json",
        ],
    ),
    (
        "1.19.3_to_1.19.4",
        &[
            "packages/core/package.json",
            "packages/opencode/package.json",
            "packages/sdk/js/package.json",
        ],
    ),
];

#[test]
fn official_channels_and_artifact_identities_cover_every_hop() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["selected_route_family"], "opencode.http");
    assert_eq!(identity["companion_route_family"], "opencode.acp");
    assert_eq!(identity["version"], "1.19.4");
    assert_eq!(identity["research"], 453);
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], "1.19.4");
    assert_eq!(identity["github_latest_tag"], "v1.19.4");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["unpublished_gap_before_new_minor"], "1.18.36");
    assert_eq!(identity["first_unpublished_later_stable"], "1.19.5");
    assert_eq!(identity["prereleases_ignored"], true);
    assert_exact_strings(
        &identity["official_stable_hops_from_previous_qualified_ceiling"],
        &["1.19.0", "1.19.1", "1.19.2", "1.19.3", "1.19.4"],
    );
    assert_exact_strings(&identity["unpublished_or_withdrawn_hops"], &[]);
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["raise_http_latest_qualified_to"],
        "1.19.4"
    );
    assert_eq!(
        identity["identity_decision"]["keep_claims_unflattened"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["claim_changed_in_identity_card"],
        false
    );
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(identity["host"]["on_path"], false);
    assert_eq!(identity["host"]["installed_or_updated"], false);

    let releases = identity["release_artifacts"].as_array().expect("releases");
    assert_eq!(releases.len(), RELEASES.len());
    let mut previous_package_json = None::<String>;
    let mut previous_linux_binary = None::<String>;
    let mut previous_linux_package = None::<String>;
    for (release, expected) in releases.iter().zip(RELEASES) {
        assert_eq!(release["version"], expected[0]);
        assert_eq!(release["npm"]["published_at"], expected[1]);
        assert_eq!(release["npm"]["tarball_sha256"], expected[2]);
        assert_eq!(release["npm"]["shasum"], expected[3]);
        assert_eq!(release["npm"]["gitHead"], Value::Null);
        assert_eq!(release["npm"]["package"], "opencode-ai");
        assert_eq!(
            release["npm"]["latest_at_observation"],
            expected[0] == "1.19.4"
        );
        assert_eq!(release["github"]["tag_commit"], expected[4]);
        assert_eq!(release["github"]["release_target_commit"], expected[5]);
        assert_eq!(release["github"]["published_at"], expected[6]);
        assert_eq!(release["github"]["source_archive_sha256"], expected[7]);
        assert_eq!(release["github"]["linux_x64_asset_sha256"], expected[8]);
        assert_eq!(
            release["github"]["linux_x64_asset_size"],
            expected[9].parse::<u64>().unwrap()
        );
        assert_eq!(
            release["github"]["linux_x64_asset_digest_source"],
            "github-release-api"
        );
        let files = release["npm"]["files"].as_object().unwrap();
        assert_exact_object_keys(
            files,
            &[
                "LICENSE",
                "bin/opencode.exe",
                "package.json",
                "postinstall.mjs",
            ],
        );
        assert_eq!(
            files["LICENSE"],
            "sha256:625f0f619133f89bbbb2abe37369613dfa1885eba1e50d02170deb62bb42cb6b"
        );
        assert_eq!(
            files["bin/opencode.exe"],
            "sha256:21c366f53283d5b5e1cdbbec2aafc98286b4c1075a4c3d5afd5ce1f5c9bf46dd"
        );
        assert_eq!(
            files["postinstall.mjs"],
            "sha256:5a7c990fe552e76b16422cdba3f4b0550590c7a487f7932c773362f74317c87b"
        );
        let package_json = files["package.json"].as_str().unwrap().to_owned();
        assert_ne!(
            Some(package_json.as_str()),
            previous_package_json.as_deref()
        );
        previous_package_json = Some(package_json);

        let linux = release["npm_linux_x64"]["files"].as_object().unwrap();
        assert_exact_object_keys(linux, &["bin/opencode", "package.json"]);
        let linux_binary = linux["bin/opencode"].as_str().unwrap().to_owned();
        let linux_package = linux["package.json"].as_str().unwrap().to_owned();
        assert_ne!(
            Some(linux_binary.as_str()),
            previous_linux_binary.as_deref()
        );
        assert_ne!(
            Some(linux_package.as_str()),
            previous_linux_package.as_deref()
        );
        previous_linux_binary = Some(linux_binary);
        previous_linux_package = Some(linux_package);
        assert_eq!(release["npm_linux_x64"]["gitHead"], Value::Null);
    }
}

#[test]
fn repository_hops_and_selected_http_files_stay_classified() {
    let inventory = json(INVENTORY);
    let protocol = json(PROTOCOL);
    assert_exact_strings(&inventory["compared"], &VERSIONS);
    assert_eq!(
        protocol["selected_openapi_path_objects_byte_identical"],
        true
    );
    assert_eq!(
        protocol["selected_route_declaration_and_handler_files_byte_identical"],
        true
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_server_started"], false);
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_exact_strings(
        &protocol["selected_routes"],
        &[
            "global.health",
            "provider.list",
            "session.create",
            "session.prompt_async",
            "event.subscribe",
            "session.abort",
            "session.delete",
            "session.list",
            "session.status",
            "session.get",
            "session.messages",
        ],
    );
    let trees = inventory["complete_repository_tree_inventory"]["per_version"]
        .as_object()
        .unwrap();
    assert_exact_object_keys(trees, &VERSIONS);
    for (version, entries, files, symlinks, sources, canonical, archive) in TREES {
        let tree = &trees[version];
        assert_eq!(tree["entries"], entries);
        assert_eq!(tree["files"], files);
        assert_eq!(tree["symlinks"], symlinks);
        assert_eq!(tree["opencode_src_files"], sources);
        assert_eq!(tree["canonical_sha256"], canonical);
        assert_eq!(tree["source_archive_sha256"], archive);
    }

    let hops = inventory["hops"].as_array().unwrap();
    assert_eq!(hops.len(), HOPS.len());
    let mut classified: BTreeSet<(String, String)> = BTreeSet::new();
    for item in protocol["classified_mapped_deltas"]
        .as_array()
        .unwrap()
        .iter()
        .chain(protocol["classified_unmapped_deltas"].as_array().unwrap())
    {
        let text = item["classification"].as_str().unwrap();
        assert!(text.len() > 20, "{}", item["file"]);
        assert!(classified.insert((
            item["hop"].as_str().unwrap().to_owned(),
            item["file"].as_str().unwrap().to_owned()
        )));
    }
    for (hop, expected) in hops.iter().zip(HOPS) {
        assert_eq!(hop["hop"], expected.0);
        assert_eq!(hop["repository_delta_counts"]["added"], expected.1);
        assert_eq!(hop["repository_delta_counts"]["removed"], expected.2);
        assert_eq!(hop["repository_delta_counts"]["changed"], expected.3);
        assert_eq!(hop["repository_delta_counts"]["identical"], expected.4);
        assert_eq!(hop["repository_path_sha256"]["added"], expected.5);
        assert_eq!(hop["repository_path_sha256"]["removed"], expected.6);
        assert_eq!(hop["repository_path_sha256"]["changed"], expected.7);
        let actual = hop["implementation_file_deltas"].as_object().unwrap();
        let expected_paths = IMPLEMENTATION_DELTAS
            .iter()
            .find(|(name, _)| *name == expected.0)
            .unwrap()
            .1;
        assert_exact_object_keys(actual, expected_paths);
        let classified_paths = classified
            .iter()
            .filter(|(name, _)| name == expected.0)
            .map(|(_, path)| path.as_str())
            .collect::<BTreeSet<_>>();
        let expected_paths = expected_paths.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(classified_paths, expected_paths);
    }

    let stable = inventory["mapped_stable_file_sha256"].as_object().unwrap();
    for (path, versions) in stable {
        assert_exact_object_keys(versions.as_object().unwrap(), &VERSIONS);
        if path == "packages/sdk/openapi.json" {
            continue;
        }
        let first = &versions["1.18.35"];
        for version in VERSIONS {
            assert_eq!(&versions[version], first, "{path} changed at {version}");
        }
    }
    assert_eq!(
        inventory["openapi_sha256"]["1.18.35"],
        "00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3"
    );
    for version in ["1.19.0", "1.19.1", "1.19.2"] {
        assert_eq!(
            inventory["openapi_sha256"][version],
            "3e27efb6e2bcfee9030df64c7be486c32f250866d6a84b88970dd83d5d9788f2"
        );
    }
    for version in ["1.19.3", "1.19.4"] {
        assert_eq!(
            inventory["openapi_sha256"][version],
            "d224c1a7506ae981c8d4fc1a8d1f58ac185e27324f94d4daf1b8f84fc2552d26"
        );
    }
}

#[test]
fn admitted_claim_extends_surface_19_without_flattening_or_covering_the_gap() {
    let fixture = json(CLAIM);
    let claim = opencode_http_claim();
    let acp = opencode_acp_claim();
    assert_eq!(OPENCODE_LATEST_QUALIFIED_VERSION, "1.19.4");
    assert_eq!(claim.latest_qualified().as_str(), "1.19.4");
    assert_eq!(claim.id().as_str(), fixture["claim_id"]);
    assert_eq!(claim.id().as_str(), "opencode.http.server-window-1");
    assert_eq!(claim.axis().as_str(), "opencode.server");
    assert_ne!(claim.axis().as_str(), acp.axis().as_str());
    assert_ne!(claim.id().as_str(), acp.id().as_str());
    assert_eq!(fixture["preserved_segment"], "1.18.11..=1.18.35");
    assert_eq!(fixture["new_segment"], "1.19.0..=1.19.4");
    assert_eq!(fixture["behavior_revision"], "opencode.http-sse.surface-19");
    assert_eq!(fixture["unpublished_gap"], "1.18.36");
    assert_eq!(fixture["unverified_newer"], "1.19.5");
    assert_exact_strings(
        &fixture["historical_gaps_preserved"],
        &[
            "1.14.52", "1.15.8", "1.15.14", "1.16.1", "1.16.3", "1.17.21",
        ],
    );
    for version in ["1.18.35", "1.19.0", "1.19.1", "1.19.2", "1.19.3", "1.19.4"] {
        let binding = opencode_server_binding(version).unwrap();
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(binding.version())
        else {
            panic!("{version} is not qualified");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            "opencode.http-sse.surface-19"
        );
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
    }
    let gap = opencode_server_binding("1.18.36").unwrap();
    assert!(matches!(
        claim.assess(gap.version()),
        InterfaceCompatibilityAssessment::Incompatible
    ));
    let later = opencode_server_binding("1.19.5").unwrap();
    assert!(matches!(
        claim.assess(later.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn assert_exact_strings(actual: &Value, expected: &[&str]) {
    let actual = actual
        .as_array()
        .expect("string array")
        .iter()
        .map(|value| value.as_str().expect("string"))
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected.iter().copied().collect());
}

fn assert_exact_object_keys(actual: &Map<String, Value>, expected: &[&str]) {
    let actual = actual.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected.iter().copied().collect());
}
