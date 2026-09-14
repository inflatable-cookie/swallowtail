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
    ("2.1.258", &["package.json"]),
    ("2.1.259", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.260", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.261", &["package.json"]),
    ("2.1.263", &["package.json"]),
    ("2.1.265", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.266", &["package.json"]),
    ("2.1.267", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.268", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.269", &["package.json", "sdk-tools.d.ts"]),
    ("2.1.270", &["package.json"]),
];

const DARWIN_BINARIES: &[(&str, &str)] = &[
    (
        "2.1.257",
        "64590d7d9d9c189d33fb3dfa58c5408eaf2a10fe556bd84155d95efaab46b60e",
    ),
    (
        "2.1.258",
        "b63136194160791c27cfa7b0403060d85eb0752991625fde8c09f9acacb17c78",
    ),
    (
        "2.1.259",
        "884baa38fe1a624be25c4a91568bf5a08b5cf4e7d7acf29b7760e3525d964898",
    ),
    (
        "2.1.260",
        "3c269f66801028823e24a63ced9fdd3988cb86cf85fccd9f03f87e463b9d3e3c",
    ),
    (
        "2.1.261",
        "5efecaff231b798be3c66def9be54183623b328b80eaef17f93c43987024e82a",
    ),
    (
        "2.1.263",
        "ef5d2909c8af49f31ab6d5487e90316777bc2fac170adfe8160716caa8aaf4f9",
    ),
    (
        "2.1.265",
        "164b09eb800dedb9bb06304129fbf07743ab9db972ae07333ef4f18cde0cb8d5",
    ),
    (
        "2.1.266",
        "553d1b9e9e7068b275c0a783c7e139ff6503096f286e674c8c919379fb0eca62",
    ),
    (
        "2.1.267",
        "a681f3008f0050029aeebcab3af51bb6a55ddeb625a3af3141a4416d43cd2558",
    ),
    (
        "2.1.268",
        "06a96d5423f83770f120859f1c58e60d7252cc4c122aa13043b7e7cd716bc76a",
    ),
    (
        "2.1.269",
        "c942e1228b93cb4d52183b3dfbc77f28264f35aa947acd9c0853d029164cf450",
    ),
    (
        "2.1.270",
        "a506b6d970a4cf44f6abdb53a81ddcd5d3b0ce042a95c502fe9d1f946bdb8807",
    ),
];

const LINUX_BINARIES: &[(&str, &str)] = &[
    (
        "2.1.257",
        "9a64bda9d8722a1fa05bef9a5961d07e0331b99597eda9e2f6a732f3a0ff7f05",
    ),
    (
        "2.1.258",
        "704f1334ac65d3e89e1c6c1d7663293ad786a6166afdb71b5075337df630f976",
    ),
    (
        "2.1.259",
        "f7dd62ae415378018cd21dd950eb3bac174ab085830304d3b8b098146bfd47b6",
    ),
    (
        "2.1.260",
        "7a2fdc74b6836ea3d183f665b869f0ee3baebc9713cbebffe5838da4ea7bd82e",
    ),
    (
        "2.1.261",
        "4ae40dd1784e85753e742e09f267d29ecbb82890361ad3817d27560866d364a6",
    ),
    (
        "2.1.263",
        "26d020351e8112f4006790f3cfce43b4c9df0c1bb1d0e542364d64151b81d5ba",
    ),
    (
        "2.1.265",
        "e14738e3a58d1fc6ccc23b9c919451b4846bc27074a3fb48db976a7d595bdeeb",
    ),
    (
        "2.1.266",
        "19842705e989393fce936804df6d2ab034860e24b8f8880357981d87ffd83fac",
    ),
    (
        "2.1.267",
        "0399c793ff571d5946ef923d80b4f330d05ac4b6842a6b0775468f5d389403c0",
    ),
    (
        "2.1.268",
        "9691a2b7bd796712ca8cffb8e32e54ff7fc45b662540233171a16a94a0425653",
    ),
    (
        "2.1.269",
        "25e44883f54419569a3d739f38cbbdaebe83b09895da0f343e1b003710a4775b",
    ),
    (
        "2.1.270",
        "3a624a5a7cd79bbad4d32bd7db36f1197ecf458bc5bf1e2aed81834a01ad3ef0",
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
        &inventory["hashes"]["wrapper.cli-wrapper.cjs"]["2.1.270"],
        "61ad63033d9c8155d5e60a29f45dc4665afa07631c0b108e62cc83bf45ba490e",
    );
    assert_sha256(
        &inventory["hashes"]["wrapper.install.cjs"]["2.1.270"],
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
    assert_eq!(darwin_unique.len(), 12);
    assert_eq!(linux_unique.len(), 12);
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
            vec!["2.1.257", "2.1.258"],
            vec!["2.1.259"],
            vec!["2.1.260", "2.1.261", "2.1.263"],
            vec!["2.1.265", "2.1.266"],
            vec!["2.1.267"],
            vec!["2.1.268"],
            vec!["2.1.269", "2.1.270"],
        ]
    );
    assert!(
        json(IDENTITY)["sdk_tools_d_ts_delta_summary"]
            .as_str()
            .expect("summary is text")
            .contains("Not selected stream-JSON")
    );
}
