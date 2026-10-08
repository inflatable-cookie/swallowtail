use std::collections::BTreeSet;

use serde_json::{Map, Value};
use swallowtail_adapter_opencode::{
    OPENCODE_LATEST_QUALIFIED_VERSION, opencode_http_claim, opencode_server_binding,
};
use swallowtail_core::InterfaceCompatibilityAssessment;

const IDENTITY: &str = include_str!("fixtures/opencode-1.18.35/identity.json");
const INVENTORY: &str = include_str!("fixtures/opencode-1.18.35/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.18.35/protocol.json");
const CLAIM: &str = include_str!("fixtures/opencode-1.18.35/claim.json");

const VERSIONS: [&str; 5] = ["1.18.31", "1.18.32", "1.18.33", "1.18.34", "1.18.35"];

#[test]
fn official_channels_cover_every_published_hop_and_exact_artifacts() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["selected_route_family"], "opencode.http");
    assert_eq!(identity["version"], "1.18.35");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], "1.18.35");
    assert_eq!(identity["github_latest_tag"], "v1.18.35");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["first_unpublished_later_stable"], "1.18.36");
    assert_exact_strings(
        &identity["official_stable_hops_from_previous_qualified_ceiling"],
        &["1.18.32", "1.18.33", "1.18.34", "1.18.35"],
    );
    assert_exact_strings(&identity["unpublished_or_withdrawn_hops"], &[]);
    assert_eq!(
        identity["identity_decision"]["shape"],
        "compatible-extension"
    );
    assert_eq!(
        identity["identity_decision"]["raise_latest_qualified_to"],
        "1.18.35"
    );
    assert_eq!(identity["identity_decision"]["keep_surface_19"], true);
    assert_eq!(
        identity["identity_decision"]["keep_baseline_and_historical_segments_and_gaps"],
        true
    );
    assert_eq!(identity["identity_decision"]["new_public_operation"], false);
    assert_eq!(
        identity["identity_decision"]["security_or_authority_change"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["consumer_visible_narrowing"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["claim_changed_in_identity_card"],
        false
    );

    const EXPECTED_RELEASES: [[&str; 11]; 5] = [
        [
            "1.18.31",
            "2026-09-14T17:47:43.078Z",
            "b6baa53003cd2e096981474ba9461a33aba2e6948b2b4b08c4aa1a88e6dc1b59",
            "sha512-J95feefeWwtIaw3irx76WjzWcgQXxmuHmDVphvs5ep9X30fBJ6T6bFhw50i9Kx50MG/xPn5w2pafXIfNtdry9w==",
            "af4a04d634555c02e4f410937c232c3c087c8a10",
            "014614d35b397775e5d397a490fc72368c894ec2",
            "a97622c801f4ca571530ddc51076af659a9c32cd",
            "2026-09-14T17:47:30Z",
            "76f69fe27ec2b44e23fa1749029e7c012eb7e975a0f0c7819e9458198dfd3896",
            "caf7f31fa1aec2353ea859d4ef9ab824c6273d941b016e88d51193fa3028d34e",
            "46273106",
        ],
        [
            "1.18.32",
            "2026-09-21T22:50:42.463Z",
            "454fbb032ade95a21323891138d4b425573f67631e7e888e516da893dc4be8ba",
            "sha512-SCrZWdq44y/EoH2+fE4HLcXS+DzpVqHPzmXk3p2RrufYy8LWvpfhRhKtijb5ktvx8r94ScqToqwHxR0BwS65OQ==",
            "11493b166da727403b3ba41455b5a66dbf790fe5",
            "545f51d26cc39a907d2867492d498d9607ea5fa4",
            "f5ce4f881e477c7b75421cea2d20939f0ddd71fb",
            "2026-09-21T22:51:20Z",
            "65e95c9a6666ca65bbd17de1e7cecddac1504e66eeebbcfaf5ac68f97e6f392b",
            "fa643f93401c13508d8d513780e54ce9cc01203d501114be9b88d62408b8101f",
            "46299070",
        ],
        [
            "1.18.33",
            "2026-09-28T04:23:28.113Z",
            "6fed32445d04df1d9d56aef0f15ae200618663ef532f7ec4241432024f84097c",
            "sha512-58P1ffLRXiAY4QeoABqUjqf6DEzm1ihpovvn9Ad+oOLbLiwK/rf88wruT9IATJ7AtNjmi8iA7pC8tN4o5y799Q==",
            "1195faeb9b33cb39ad58fc01be813307d1e3b935",
            "51ef4be1d3c122f18fefb510dca8d778571f4f18",
            "1eacc1bdb919bc54fc1c9f745b0da02e83ebbdf9",
            "2026-09-28T04:22:46Z",
            "34a4b810f4e839f2c4ac62206bbc64d736006f3c96712b903cda83ca60271301",
            "24b12873e605b3db3387cb355f43ba7451cd6065c180d8c188663337d2eeb553",
            "46317276",
        ],
        [
            "1.18.34",
            "2026-09-30T22:38:58.987Z",
            "279923bb5754e810b173a8a7401cf17699fccad3c1159500c10c477887f0b53b",
            "sha512-9WUS2T0t4HHDVzXvuwTHF0nvhXvZ9mQ0r+ozCvKdJu0LVoQpOzqAW4qWTC3ygNc5Oedcc7j+Oct24JYlQYnySA==",
            "d7f954b669f1cacb24637e8e302b2ec4faaef63e",
            "aec0b9a6d8898f68f923aaf08b7306d931fd9d76",
            "e9f8a210b9e2b1e13d375b84906069886eb3b767",
            "2026-09-30T22:39:45Z",
            "c2c60efde22639b64c7bfa740da39b9c8079391a7540e2f67bf91b36e5797f17",
            "8522b70f545184b3a8d97c5ca4f814093b2476d72aebfda8c48bcd072ec31d1b",
            "45538151",
        ],
        [
            "1.18.35",
            "2026-10-06T20:21:30.784Z",
            "4d3408d0950d70cf870efe3f86e8bd0271d8149bea0767008d42de14f0986a04",
            "sha512-tDQKThIZ3amp8NSn3XDP2Dj5BlhRbZC0VkIxgYuf6gvRSXkseqD31j/U4KfpPwha593i8M59hdsDfz32yJx4Og==",
            "08cd9a77993031d2c66289d82a99f10227af7fac",
            "53d1eabb61e21162157817bf677da0a4ad3332e3",
            "4ac0d9c3d169bbe81d9570013effdda3fe24d36e",
            "2026-10-06T20:18:39Z",
            "3092a7b9f55d80c42a9c1bb2e2cc0a9961316673faf92b741577257a1576dd59",
            "80b05124357a77cd57945bfde36082a028e829c198d222d5e146617f49a2c4b7",
            "45540968",
        ],
    ];
    let releases = identity["release_artifacts"]
        .as_array()
        .expect("release array");
    assert_eq!(releases.len(), EXPECTED_RELEASES.len());
    for (release, expected) in releases.iter().zip(EXPECTED_RELEASES) {
        assert_eq!(release["version"], expected[0]);
        assert_eq!(release["npm"]["published_at"], expected[1]);
        assert_eq!(release["npm"]["tarball_sha256"], expected[2]);
        assert_eq!(release["npm"]["integrity"], expected[3]);
        assert_eq!(release["npm"]["shasum"], expected[4]);
        assert_eq!(release["github"]["tag_commit"], expected[5]);
        assert_eq!(release["github"]["release_target_commit"], expected[6]);
        assert_eq!(release["github"]["published_at"], expected[7]);
        assert_eq!(release["github"]["source_archive_sha256"], expected[8]);
        assert_eq!(release["github"]["darwin_arm64_asset_sha256"], expected[9]);
        assert_eq!(
            release["github"]["darwin_arm64_asset_size"],
            expected[10].parse::<u64>().unwrap()
        );
        assert_eq!(release["npm"]["package"], "opencode-ai");
        assert_eq!(release["npm"]["gitHead"], Value::Null);
        assert_eq!(
            release["npm"]["latest_at_observation"],
            expected[0] == "1.18.35"
        );
        assert_eq!(release["github"]["tag"], format!("v{}", expected[0]));
        let package_files = release["npm"]["files"].as_object().unwrap();
        assert_exact_object_keys(
            package_files,
            &[
                "LICENSE",
                "bin/opencode.exe",
                "package.json",
                "postinstall.mjs",
            ],
        );
        assert_eq!(
            package_files["LICENSE"],
            "sha256:625f0f619133f89bbbb2abe37369613dfa1885eba1e50d02170deb62bb42cb6b"
        );
        assert_eq!(
            package_files["bin/opencode.exe"],
            "sha256:21c366f53283d5b5e1cdbbec2aafc98286b4c1075a4c3d5afd5ce1f5c9bf46dd"
        );
        assert_eq!(
            package_files["postinstall.mjs"],
            "sha256:5a7c990fe552e76b16422cdba3f4b0550590c7a487f7932c773362f74317c87b"
        );
    }

    let host = &identity["host"];
    assert_eq!(host["present"], true);
    assert_eq!(host["on_path"], true);
    assert_eq!(host["observed_version"], "1.18.32");
    assert_eq!(host["observation_only"], true);
    assert_eq!(host["installed_or_updated"], false);
    assert_eq!(host["official_asset_match"], true);
    assert_eq!(host["matched_release_asset_version"], "1.18.32");
    assert_eq!(
        host["matched_release_asset_sha256"],
        "fa643f93401c13508d8d513780e54ce9cc01203d501114be9b88d62408b8101f"
    );
    assert_eq!(
        host["host_binary_sha256"],
        "a3c45d4e1d6620b436851f1ef6b25c71befcf06a382e279a1eb1c2196424395e"
    );
    assert_eq!(host["invoked_beyond_version"], false);
}

#[test]
fn complete_tree_inventories_and_all_repository_hops_are_frozen() {
    const TREE_EXPECTATIONS: [[&str; 4]; 5] = [
        [
            "1.18.31",
            "6566",
            "408",
            "3bd9c7c0e74285434edc771eb0279e28a8fd0634c3cc8f1c804f3d50cfcff21f",
        ],
        [
            "1.18.32",
            "6572",
            "408",
            "41d939a5f15d7cc72fdc19aff5592a4ef86c341636318594f9510f9458365dff",
        ],
        [
            "1.18.33",
            "6578",
            "409",
            "d0b53d2b6099ab2a0d498c25c6efc135b483370463ab4b3f23eabce7f0c05d09",
        ],
        [
            "1.18.34",
            "6581",
            "409",
            "35d11829d8fdd6e17d2e700be58ddf48cca9afff2d5a79b1708681218425e7f0",
        ],
        [
            "1.18.35",
            "6568",
            "409",
            "616f4afe54525f9d0a7cdcbf0818f1f00c6cba722daaa7f1b4f9c2deef1216b2",
        ],
    ];
    let inventory = json(INVENTORY);
    assert_exact_strings(&inventory["compared"], &VERSIONS);
    let trees = &inventory["complete_repository_tree_inventory"]["per_version"];
    for [version, files, source_files, digest] in TREE_EXPECTATIONS {
        let tree = &trees[version];
        assert_eq!(tree["entries"], files.parse::<u64>().unwrap() + 60);
        assert_eq!(tree["files"], files.parse::<u64>().unwrap());
        assert_eq!(
            tree["opencode_src_files"],
            source_files.parse::<u64>().unwrap()
        );
        assert_eq!(tree["symlinks"], 60);
        assert_eq!(tree["canonical_sha256"], digest);
    }

    const HOP_EXPECTATIONS: [[&str; 8]; 4] = [
        [
            "1.18.31_to_1.18.32",
            "6",
            "0",
            "102",
            "6524",
            "c0ca7ddbe4041432cefb16c0fc8eea5c16f6c119d0f337ae03469aecc92e28e3",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "02e05bea8b7dbbf7c48a4e5de7d0b8dbdfa8e7822c5b9991984d0ac457b2e397",
        ],
        [
            "1.18.32_to_1.18.33",
            "6",
            "0",
            "143",
            "6489",
            "94bb20e03a8cc3a36fe334ce4b26aacd8b12f9d42a4c4b172c8ef4a5f37fa961",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "b7a3aa8dcfd5306978f8598eab1b0cf9a16f19133cd8edb7d86e782020045aa9",
        ],
        [
            "1.18.33_to_1.18.34",
            "5",
            "2",
            "128",
            "6508",
            "db215a98ca499150979e9dc76d343d9d7106a33d4cf6738e2d04deedec0bdb6e",
            "dcccc6ff64fbb7bc837109c9c449216f26c4b41c697d84a4207815a3ddf14fde",
            "a34c999d94ee0876045bd37da987a8c5258730c2dc3c319f082612018e018950",
        ],
        [
            "1.18.34_to_1.18.35",
            "15",
            "28",
            "151",
            "6462",
            "ff2656023b4bf550ce3c24294b182f29b0c5e4534e7ac6d7372295395b6f6487",
            "dee7c4daeecb1328cccffcea0490d27cedb31a32e544a23c3d0035fca5d0e336",
            "7291b86e1c31d3837a686f0bda44cd04ef54ca32100923c6dfef3c108cc1c404",
        ],
    ];
    let hops = inventory["hops"].as_array().expect("hop array");
    assert_eq!(hops.len(), HOP_EXPECTATIONS.len());
    for (hop, expected) in hops.iter().zip(HOP_EXPECTATIONS) {
        assert_eq!(hop["hop"], expected[0]);
        let counts = &hop["repository_delta_counts"];
        assert_eq!(counts["added"], expected[1].parse::<u64>().unwrap());
        assert_eq!(counts["removed"], expected[2].parse::<u64>().unwrap());
        assert_eq!(counts["changed"], expected[3].parse::<u64>().unwrap());
        assert_eq!(counts["identical"], expected[4].parse::<u64>().unwrap());
        for (action, digest) in [
            ("added", expected[5]),
            ("removed", expected[6]),
            ("changed", expected[7]),
        ] {
            let paths = hop[format!("repository_{action}")]
                .as_array()
                .expect("path array");
            assert_eq!(paths.len(), counts[action].as_u64().unwrap() as usize);
            assert_eq!(paths.as_slice(), sorted_unique(paths).as_slice());
            assert_eq!(hop["repository_path_sha256"][action], digest);
        }
        assert_eq!(hop["npm_changed_files"], json(r#"["package.json"]"#));
        assert_exact_strings(
            &hop["npm_identical_files"],
            &["LICENSE", "bin/opencode.exe", "postinstall.mjs"],
        );
    }
}

#[test]
fn selected_route_files_and_each_changed_implementation_are_classified() {
    const IMPLEMENTATION_PATHS: [&[&str]; 4] = [
        &[
            "packages/core/package.json",
            "packages/core/src/filesystem/search.ts",
            "packages/core/src/npm.ts",
            "packages/opencode/package.json",
            "packages/opencode/src/session/message-v2.ts",
            "packages/sdk/js/package.json",
        ],
        &[
            "packages/core/package.json",
            "packages/core/src/open.ts",
            "packages/opencode/package.json",
            "packages/opencode/src/cli/cmd/account.ts",
            "packages/opencode/src/cli/cmd/debug/config.ts",
            "packages/opencode/src/cli/cmd/debug/redact.ts",
            "packages/opencode/src/cli/cmd/web.ts",
            "packages/opencode/src/mcp/browser.ts",
            "packages/opencode/src/mcp/oauth-provider.ts",
            "packages/opencode/src/plugin/digitalocean.ts",
            "packages/opencode/src/plugin/openai/codex.ts",
            "packages/opencode/src/plugin/snowflake-cortex.ts",
            "packages/opencode/src/provider/provider.ts",
            "packages/opencode/src/provider/transform.ts",
            "packages/sdk/js/package.json",
        ],
        &[
            "packages/core/package.json",
            "packages/core/src/session/runner/llm.ts",
            "packages/opencode/package.json",
            "packages/opencode/src/session/llm/request.ts",
            "packages/sdk/js/package.json",
        ],
        &[
            "packages/core/package.json",
            "packages/opencode/package.json",
            "packages/opencode/src/session/message-v2.ts",
            "packages/sdk/js/package.json",
        ],
    ];
    let inventory = json(INVENTORY);
    let protocol = json(PROTOCOL);
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
    assert_eq!(
        protocol["selected_route_declaration_and_handler_files_byte_identical"],
        true
    );
    assert_eq!(
        protocol["openapi_deltas"]["selected_operation_objects_changed"],
        false
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_server_started"], false);
    assert_eq!(protocol["downloaded_artifact_executed"], false);
    assert_eq!(protocol["host_install_changed"], false);

    let classifications = protocol["classified_mapped_deltas"]
        .as_array()
        .unwrap()
        .iter()
        .chain(protocol["classified_unmapped_deltas"].as_array().unwrap())
        .map(|entry| {
            let hop = entry["hop"].as_str().unwrap();
            let file = entry["file"].as_str().unwrap();
            assert!(entry["classification"].as_str().unwrap().len() > 20);
            (hop.to_owned(), file.to_owned())
        })
        .collect::<BTreeSet<_>>();
    for (index, hop) in inventory["hops"].as_array().unwrap().iter().enumerate() {
        let hop_name = hop["hop"].as_str().unwrap();
        let implementation = hop["implementation_file_deltas"].as_object().unwrap();
        assert_exact_strings(
            &Value::Array(
                implementation
                    .keys()
                    .map(|path| Value::String(path.clone()))
                    .collect(),
            ),
            IMPLEMENTATION_PATHS[index],
        );
        for path in implementation.keys() {
            assert!(classifications.contains(&(hop_name.to_owned(), path.clone())));
            let delta = &implementation[path];
            assert!(delta["change"].as_str().is_some());
            if delta["change"] != "added" {
                assert!(delta["before_sha256"].as_str().unwrap().len() == 64);
            }
            if delta["change"] != "removed" {
                assert!(delta["after_sha256"].as_str().unwrap().len() == 64);
            }
        }
        let mapped_files = protocol["classified_mapped_deltas"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| entry["hop"] == hop_name)
            .map(|entry| entry["file"].as_str().unwrap())
            .collect::<Vec<_>>();
        for file in mapped_files {
            assert!(implementation.contains_key(file));
        }
    }
    const SELECTED_INTERNAL_HASHES: [(&str, &str, &str, &str); 14] = [
        (
            "1.18.31_to_1.18.32",
            "packages/opencode/src/session/message-v2.ts",
            "828789508afda97a1570b96413dd40ee9d9119dd25d057b5f0a985154378e619",
            "bfeb41e03e3788c83d3a031cce0aa1cf19a82c024a09e2a6aadbebb7a3d40d53",
        ),
        (
            "1.18.31_to_1.18.32",
            "packages/opencode/package.json",
            "778160ff537cfb76b0abdb160292e48268f1d900a29e8e9276863b67d0fca19d",
            "eb126c466ed6aee083bc42904c3f4c1d72e2821bde01e929e88c7727160eaa8f",
        ),
        (
            "1.18.31_to_1.18.32",
            "packages/core/package.json",
            "6f744799faf4dfd42d0f11a122fbe9071994bf3ae0841fc239bdd9e7ae756f16",
            "d01605ddad476f34f39c679becc1288c617c5c11d2bd9cf7181b4a8e9e95820e",
        ),
        (
            "1.18.32_to_1.18.33",
            "packages/opencode/src/provider/provider.ts",
            "77216c71fd0079da036ac25af516a29ee45f58b52e7c63987065ca97583ee364",
            "9fb8958a521bbc5a70e5e6d4ecb130149115fe891fb78bca0ae73a966b9f547a",
        ),
        (
            "1.18.32_to_1.18.33",
            "packages/opencode/src/provider/transform.ts",
            "c07d49e48dd2478ad2813a10805781a72551db2fd847b7df994cc854bf654c16",
            "ddfea009d85f61172d056216addeeabd920cd368d4d27b2f652629f7e3bcb0d2",
        ),
        (
            "1.18.32_to_1.18.33",
            "packages/opencode/src/plugin/openai/codex.ts",
            "6450893522f00fae9039ce6306d461b223580e0c0e4eb6fc627ea1e5795ec5ca",
            "9480ee62a3de6cfd8c49651a1651f27a7381c2fa7a183a5eedff5960d284c202",
        ),
        (
            "1.18.32_to_1.18.33",
            "packages/opencode/package.json",
            "eb126c466ed6aee083bc42904c3f4c1d72e2821bde01e929e88c7727160eaa8f",
            "18f558406b42434d91288906d130872d0c8d35f6332771e5e44110388c76061d",
        ),
        (
            "1.18.32_to_1.18.33",
            "packages/core/package.json",
            "d01605ddad476f34f39c679becc1288c617c5c11d2bd9cf7181b4a8e9e95820e",
            "8250b24fdf60564d5c8809c92275ebf21d1d370dc9b934b29547cd00f5916309",
        ),
        (
            "1.18.33_to_1.18.34",
            "packages/core/src/session/runner/llm.ts",
            "625134472e3c810fcf5b39f116950a5532bd24ae7ec0caa8b1c3198389fa5681",
            "ba0b03e2b00cfe67bc98988f348ad76ba4f73d9e36fe46883710bbdc4e48c3a3",
        ),
        (
            "1.18.33_to_1.18.34",
            "packages/opencode/src/session/llm/request.ts",
            "a92010ff1981f9bdf62c7d2f6dcbe28baea041e2cd54bf0b54fd2ea667b92cf2",
            "90077551a9a46e11a37f44279c584832918d722c0ba0407ff8cad8d3437dd0de",
        ),
        (
            "1.18.33_to_1.18.34",
            "packages/opencode/package.json",
            "18f558406b42434d91288906d130872d0c8d35f6332771e5e44110388c76061d",
            "d07f8eeb436c2f29671382cdbb4decf49f2e10e6603e91c6cda78d3ee4cbc4fb",
        ),
        (
            "1.18.34_to_1.18.35",
            "packages/opencode/src/session/message-v2.ts",
            "bfeb41e03e3788c83d3a031cce0aa1cf19a82c024a09e2a6aadbebb7a3d40d53",
            "b55648c31423ace5c730ca085ef5429a966486fcd7bf84ab25e59453a1a9417a",
        ),
        (
            "1.18.34_to_1.18.35",
            "packages/opencode/package.json",
            "d07f8eeb436c2f29671382cdbb4decf49f2e10e6603e91c6cda78d3ee4cbc4fb",
            "ef0e849f7c5ea99b89934f53ce439d4dbc74ed55b8be4a8ce9112864d7d23073",
        ),
        (
            "1.18.34_to_1.18.35",
            "packages/core/package.json",
            "63ae9051d0ba1b037fcef41e50422e24aa2cb1124ab5ab101cca80360aee263b",
            "d2237dedb791e4afafa504cae7fdf38ac6d24725cfb9641d6e9fecb7040d2560",
        ),
    ];
    for (hop_name, path, before, after) in SELECTED_INTERNAL_HASHES {
        let hop = inventory["hops"]
            .as_array()
            .unwrap()
            .iter()
            .find(|hop| hop["hop"] == hop_name)
            .expect("selected behavior hop exists");
        let delta = &hop["implementation_file_deltas"][path];
        assert_eq!(delta["before_sha256"], before, "{hop_name}: {path}");
        assert_eq!(delta["after_sha256"], after, "{hop_name}: {path}");
    }

    let mapped_hashes = &inventory["mapped_stable_file_sha256"];
    const MAPPED_FILES: [&str; 12] = [
        "packages/opencode/src/server/routes/instance/httpapi/groups/event.ts",
        "packages/opencode/src/server/routes/instance/httpapi/groups/provider.ts",
        "packages/opencode/src/server/routes/instance/httpapi/groups/session.ts",
        "packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts",
        "packages/opencode/src/server/routes/instance/httpapi/handlers/provider.ts",
        "packages/opencode/src/server/routes/instance/httpapi/handlers/session.ts",
        "packages/opencode/src/server/routes/instance/httpapi/lifecycle.ts",
        "packages/opencode/src/session/compaction.ts",
        "packages/opencode/src/session/message-v2.ts",
        "packages/opencode/src/session/session.ts",
        "packages/opencode/src/session/status.ts",
        "packages/sdk/openapi.json",
    ];
    assert_exact_object_keys(mapped_hashes.as_object().unwrap(), &MAPPED_FILES);
    assert_exact_strings(&protocol["mapped_stable_files"], &MAPPED_FILES);
    const ROUTE_FILE_HASHES: [(&str, &str); 10] = [
        (
            "packages/opencode/src/server/routes/instance/httpapi/groups/event.ts",
            "fd1ba9befff410ef6e0ac3f4fd1c2427ca721c95c31aaad174c9db6e30bdd513",
        ),
        (
            "packages/opencode/src/server/routes/instance/httpapi/groups/provider.ts",
            "9e9c382d2d7b69dda7a5bab87ed999e400534a268d3f29f7e95d5cd95032d246",
        ),
        (
            "packages/opencode/src/server/routes/instance/httpapi/groups/session.ts",
            "3fd2ae22b6bbc211adc45011243a42fa869862355ad08c086c591792d8fa91dc",
        ),
        (
            "packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts",
            "64985bf2d4d5e1002bdef7b3f64217cdd830c8b0dadfe3faf84ad0dffc5c5ba6",
        ),
        (
            "packages/opencode/src/server/routes/instance/httpapi/handlers/provider.ts",
            "0a42588ae88dea589963f41f8a65c354a05e82c0a864b31896e1e50439fee077",
        ),
        (
            "packages/opencode/src/server/routes/instance/httpapi/handlers/session.ts",
            "3ce829f386267181da442d499d841463a9e532db827f75a6f41d501de6f71cf6",
        ),
        (
            "packages/opencode/src/server/routes/instance/httpapi/lifecycle.ts",
            "88618adc27b27bebbc596207c7cf4324c112b7b49b696972e22050f8d5ca4adb",
        ),
        (
            "packages/opencode/src/session/compaction.ts",
            "8d478570a7e4ad32b746030d4f86a1c673949b1e2259bd716b3885d99283289a",
        ),
        (
            "packages/opencode/src/session/session.ts",
            "0c56ae3535e29cae0de51156eaba2842c0896309f1d6b12525566b4a8ba4c7f2",
        ),
        (
            "packages/opencode/src/session/status.ts",
            "dbbbdee83c292379c1665a1d482b810a13754b6ae2143169b24c75ced5841b42",
        ),
    ];
    for (path, expected_hash) in ROUTE_FILE_HASHES {
        let versions = &mapped_hashes[path];
        for version in VERSIONS {
            assert_eq!(
                versions[version], expected_hash,
                "route file changed: {path}"
            );
        }
    }
    const MESSAGE_CONVERSION_HASHES: [&str; 5] = [
        "828789508afda97a1570b96413dd40ee9d9119dd25d057b5f0a985154378e619",
        "bfeb41e03e3788c83d3a031cce0aa1cf19a82c024a09e2a6aadbebb7a3d40d53",
        "bfeb41e03e3788c83d3a031cce0aa1cf19a82c024a09e2a6aadbebb7a3d40d53",
        "bfeb41e03e3788c83d3a031cce0aa1cf19a82c024a09e2a6aadbebb7a3d40d53",
        "b55648c31423ace5c730ca085ef5429a966486fcd7bf84ab25e59453a1a9417a",
    ];
    for (version, expected_hash) in VERSIONS.into_iter().zip(MESSAGE_CONVERSION_HASHES) {
        assert_eq!(
            mapped_hashes["packages/opencode/src/session/message-v2.ts"][version],
            expected_hash
        );
    }
    for (path, versions) in mapped_hashes.as_object().unwrap() {
        assert_exact_object_keys(versions.as_object().unwrap(), &VERSIONS);
        if path != "packages/opencode/src/session/message-v2.ts" {
            let first = &versions["1.18.31"];
            for version in VERSIONS {
                assert_eq!(
                    &versions[version], first,
                    "unexpected mapped-file change in {path}"
                );
            }
        }
    }
    assert_eq!(
        inventory["openapi_sha256"]["1.18.31"],
        "00502bd13e9c86f3ca9e765e99a57e06fa9f434ca16f2a714766d1444f8d37f3"
    );
    for version in VERSIONS {
        assert_eq!(
            inventory["openapi_sha256"][version],
            inventory["openapi_sha256"]["1.18.31"]
        );
    }
}

#[test]
fn admitted_claim_qualifies_all_four_hops_and_keeps_the_next_patch_unverified() {
    let fixture = json(CLAIM);
    let claim = opencode_http_claim();
    assert_eq!(OPENCODE_LATEST_QUALIFIED_VERSION, "1.18.35");
    assert_eq!(claim.id().as_str(), fixture["claim_id"]);
    assert_eq!(claim.baseline().as_str(), fixture["baseline"]);
    assert_eq!(
        claim.latest_qualified().as_str(),
        fixture["latest_qualified"]
    );
    assert_eq!(fixture["behavior_revision"], "opencode.http-sse.surface-19");
    assert_eq!(fixture["newer_version_posture"], "allow_unverified");
    assert_exact_strings(
        &fixture["newly_qualified"],
        &["1.18.32", "1.18.33", "1.18.34", "1.18.35"],
    );
    assert_exact_strings(
        &fixture["historical_gaps_preserved"],
        &[
            "1.14.52", "1.15.8", "1.15.14", "1.16.1", "1.16.3", "1.17.21",
        ],
    );
    for version in fixture["newly_qualified"].as_array().unwrap() {
        let binding =
            opencode_server_binding(version.as_str().unwrap()).expect("qualified binding");
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(binding.version())
        else {
            panic!("published hop is not qualified");
        };
        assert_eq!(
            matched.behavior_revision().as_str(),
            fixture["behavior_revision"]
        );
    }
    let later = opencode_server_binding(fixture["unverified_newer"].as_str().unwrap()).unwrap();
    assert!(matches!(
        claim.assess(later.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn assert_exact_strings(actual: &Value, expected: &[&str]) {
    let actual = actual.as_array().expect("string array");
    assert_eq!(actual.len(), expected.len());
    let actual = actual
        .iter()
        .map(|value| value.as_str().expect("string"))
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_exact_object_keys(actual: &Map<String, Value>, expected: &[&str]) {
    let actual = actual.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn sorted_unique(paths: &[Value]) -> Vec<Value> {
    let mut values = paths
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let original_len = values.len();
    values.sort();
    values.dedup();
    assert_eq!(values.len(), original_len);
    values.into_iter().map(Value::String).collect()
}
