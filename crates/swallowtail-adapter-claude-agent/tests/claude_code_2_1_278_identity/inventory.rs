use super::support::{
    COMPARED_VERSIONS, DIST_INVENTORY, IDENTITY, assert_exact_string_set, assert_sha256, json,
};

const WRAPPER_FILES: &[&str] = &[
    "LICENSE.md",
    "README.md",
    "bin/claude.exe",
    "cli-wrapper.cjs",
    "install.cjs",
    "package.json",
    "sdk-tools.d.ts",
];
const WRAPPER_IDENTICAL_WITHOUT_SDK_TOOLS: &[&str] = &[
    "LICENSE.md",
    "README.md",
    "bin/claude.exe",
    "cli-wrapper.cjs",
    "install.cjs",
];
const WRAPPER_IDENTICAL_INCLUDING_SDK_TOOLS: &[&str] = &[
    "LICENSE.md",
    "README.md",
    "bin/claude.exe",
    "cli-wrapper.cjs",
    "install.cjs",
    "sdk-tools.d.ts",
];
const PLATFORM_IDENTICAL: &[&str] = &["LICENSE.md", "README.md"];

/// `version`, wrapper files changed from the previous hop.
const HOP_WRAPPER_CHANGED: &[(&str, &[&str])] = &[
    ("2.1.271", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.272", &["package.json"]),
    ("2.1.273", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.274", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.275", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.276", &["package.json"]),
    ("2.1.277", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.278", &["package.json"]),
];

const DARWIN_BINARIES: &[(&str, &str)] = &[
    (
        "2.1.270",
        "a506b6d970a4cf44f6abdb53a81ddcd5d3b0ce042a95c502fe9d1f946bdb8807",
    ),
    (
        "2.1.271",
        "87d119eb46782a1369d79d6e8cc00557f1b51bc9db7a51bd01fa2f909d8fe3dc",
    ),
    (
        "2.1.272",
        "195e24e8e1f9bf46f1eaee72d434a33e18f9f5796f29a6348a00d16c5f8aee75",
    ),
    (
        "2.1.273",
        "953e9880dbcb0b70f31c1f508de6a3fd389753d131688557fd992da9184693fb",
    ),
    (
        "2.1.274",
        "3509913f9d1576316c8845b88837f8fd3bbbcf26625833ac82cfb6b8985da94a",
    ),
    (
        "2.1.275",
        "1b8177fe49f2be5bacc75e89b5f88fa7454791283113ace16a453fe9171d179b",
    ),
    (
        "2.1.276",
        "9de364db11a410d53cbbb0f6b1f18c66c90053efc9a63370072856d10db66329",
    ),
    (
        "2.1.277",
        "73d6a2a55c46907e49bd8bb7608e134333bd71173351ee16ddce7d7db9914b9c",
    ),
    (
        "2.1.278",
        "bd245662fb8a0e321b3bf133e930371d6563c387527885f30b2613aef3ba14d6",
    ),
];

const LINUX_BINARIES: &[(&str, &str)] = &[
    (
        "2.1.270",
        "3a624a5a7cd79bbad4d32bd7db36f1197ecf458bc5bf1e2aed81834a01ad3ef0",
    ),
    (
        "2.1.271",
        "5e7b6fc24d0e124f68e99a0641c47b662945c6d197d768d7ad7a72cbf1897f58",
    ),
    (
        "2.1.272",
        "d81396a668eb76fbddb49a2a5841f1b5d7af96b4c1f6500ced92f2c988f5bcd4",
    ),
    (
        "2.1.273",
        "6c752e2cc7c110c9df15f26d8d134d438c5ae95dbd610efc1a308bf7f9c5f6c1",
    ),
    (
        "2.1.274",
        "15e2d05148f801b5774032faad87e624ecd172e9903288bda448b892eb58fa07",
    ),
    (
        "2.1.275",
        "13586f3150a7ca1655f36e1dba759fb404e0f7cf7021d4a3dcd5e6f604e56156",
    ),
    (
        "2.1.276",
        "8a56c8a14bd3cb246e2bdb7e60aefe0f609bff78c8bbcc5ea6b1817c111c6145",
    ),
    (
        "2.1.277",
        "722210f05ba494d8f6df69423c4d4f2960900f7a007d0532851c7a36e375cab7",
    ),
    (
        "2.1.278",
        "5c4735937844e84f8a93306e841a5b0e12252909b07870f789b190468da147ab",
    ),
];

#[test]
fn wrapper_and_platform_inventories_are_exact_for_every_hop() {
    let inventory = json(DIST_INVENTORY);
    assert_exact_string_set(&inventory["compared"], COMPARED_VERSIONS);
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    for version in COMPARED_VERSIONS {
        assert_eq!(
            inventory["package_file_counts"][format!("wrapper-{version}")],
            7
        );
        assert_eq!(
            inventory["package_file_counts"][format!("darwin-arm64-{version}")],
            4
        );
        assert_eq!(
            inventory["package_file_counts"][format!("linux-x64-{version}")],
            4
        );
    }
    for (version, wrapper_changed) in HOP_WRAPPER_CHANGED {
        let index = COMPARED_VERSIONS
            .iter()
            .position(|candidate| candidate == version)
            .expect("hop is compared");
        let previous = COMPARED_VERSIONS[index - 1];
        let from_to = |suffix: &str| {
            format!(
                "from_{}_to_{}_{}",
                previous.replace('.', "_"),
                version.replace('.', "_"),
                suffix
            )
        };
        let wrapper = &inventory[from_to("wrapper")];
        assert_exact_string_set(&wrapper["added"], &[]);
        assert_exact_string_set(&wrapper["removed"], &[]);
        assert_exact_string_set(&wrapper["changed"], wrapper_changed);
        if wrapper_changed.contains(&"sdk-tools.d.ts") {
            assert_exact_string_set(&wrapper["identical"], WRAPPER_IDENTICAL_WITHOUT_SDK_TOOLS);
        } else {
            assert_exact_string_set(&wrapper["identical"], WRAPPER_IDENTICAL_INCLUDING_SDK_TOOLS);
        }
        for suffix in ["darwin_arm64", "linux_x64"] {
            let platform = &inventory[from_to(suffix)];
            assert_exact_string_set(&platform["added"], &[]);
            assert_exact_string_set(&platform["removed"], &[]);
            assert_exact_string_set(&platform["changed"], &["claude", "package.json"]);
            assert_exact_string_set(&platform["identical"], PLATFORM_IDENTICAL);
        }
    }
}

#[test]
fn mapped_feeding_wrapper_files_stay_byte_identical_across_the_window() {
    let inventory = json(DIST_INVENTORY);
    assert_exact_string_set(
        &inventory["wrapper_byte_identical_across_all_compared"],
        &[
            "LICENSE.md",
            "README.md",
            "bin/claude.exe",
            "cli-wrapper.cjs",
            "install.cjs",
        ],
    );
    assert_exact_string_set(
        &inventory["platform_files_byte_identical_across_all_compared"],
        PLATFORM_IDENTICAL,
    );
    for file in [
        "wrapper.cli-wrapper.cjs",
        "wrapper.install.cjs",
        "wrapper.bin.claude.exe",
        "wrapper.README.md",
        "wrapper.LICENSE.md",
    ] {
        let digests = &inventory["hashes"][file];
        let unique: std::collections::BTreeSet<&str> = COMPARED_VERSIONS
            .iter()
            .map(|version| digests[version].as_str().expect("digest is text"))
            .collect();
        assert_eq!(unique.len(), 1, "{file} must be byte-identical across hops");
    }
    assert_sha256(
        &inventory["hashes"]["wrapper.cli-wrapper.cjs"]["2.1.278"],
        "61ad63033d9c8155d5e60a29f45dc4665afa07631c0b108e62cc83bf45ba490e",
    );
    assert_sha256(
        &inventory["hashes"]["wrapper.install.cjs"]["2.1.278"],
        "5cbab1670597f492cd4eeb946f3c344ebcb1fbd43c623ba192c9b33744461b85",
    );
}

#[test]
fn platform_binary_digests_move_once_per_published_hop() {
    let inventory = json(DIST_INVENTORY);
    for (darwin, linux) in DARWIN_BINARIES.iter().zip(LINUX_BINARIES) {
        assert_eq!(darwin.0, linux.0);
        assert_sha256(
            &inventory["hashes"]["darwin-arm64.claude"][darwin.0],
            darwin.1,
        );
        assert_sha256(&inventory["hashes"]["linux-x64.claude"][linux.0], linux.1);
    }
    let darwin_unique: std::collections::BTreeSet<&str> =
        DARWIN_BINARIES.iter().map(|(_, digest)| *digest).collect();
    let linux_unique: std::collections::BTreeSet<&str> =
        LINUX_BINARIES.iter().map(|(_, digest)| *digest).collect();
    assert_eq!(darwin_unique.len(), 9);
    assert_eq!(linux_unique.len(), 9);
    assert_eq!(
        inventory["hashes"]
            .as_object()
            .expect("hashes are an object")
            .len(),
        9
    );
    assert_eq!(WRAPPER_FILES.len(), 7);
}

#[test]
fn sdk_tools_declaration_deltas_are_sdk_only() {
    let inventory = json(DIST_INVENTORY);
    let groups = inventory["sdk_tools_d_ts_distinct_groups"]
        .as_object()
        .expect("groups are an object");
    let mut grouped: Vec<(String, Vec<&str>)> = groups
        .iter()
        .map(|(digest, versions)| {
            (
                digest.clone(),
                versions
                    .as_array()
                    .expect("versions are an array")
                    .iter()
                    .map(|version| version.as_str().expect("version is text"))
                    .collect(),
            )
        })
        .collect();
    grouped.sort_by_key(|(_, versions)| {
        COMPARED_VERSIONS
            .iter()
            .position(|candidate| *candidate == versions[0])
            .expect("group starts at a compared version")
    });
    let collapsed: Vec<Vec<&str>> = grouped.into_iter().map(|(_, versions)| versions).collect();
    assert_eq!(
        collapsed,
        vec![
            vec!["2.1.270"],
            vec!["2.1.271", "2.1.272"],
            vec!["2.1.273"],
            vec!["2.1.274"],
            vec!["2.1.275", "2.1.276"],
            vec!["2.1.277", "2.1.278"],
        ]
    );
    assert!(
        json(IDENTITY)["sdk_tools_d_ts_delta_summary"]
            .as_str()
            .expect("summary is text")
            .contains("Not selected stream-JSON")
    );
}
