use serde_json::Value;
use sha2::{Digest, Sha256};
use swallowtail_adapter_pi::{
    PI_PACKAGE_BASELINE_VERSION, PI_PACKAGE_LATEST_QUALIFIED_VERSION, pi_rpc_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/pi-rpc-1.1.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/pi-rpc-1.1.0/protocol.json");
const INVENTORY: &str = include_str!("fixtures/pi-rpc-1.1.0/dist-inventory.json");

const VERSIONS: [&str; 12] = [
    "0.86.1", "0.87.0", "0.87.1", "0.99.0", "0.99.1", "0.99.2", "1.0.0", "1.0.1", "1.0.2", "1.0.3",
    "1.0.4", "1.1.0",
];

const ARTIFACTS: [(&str, usize, &str, &str); 12] = [
    (
        "0.86.1",
        1100,
        "38920eb7b8cf3232eae59063a4f81b1c8fd6d7a6a2600fdab6206c79e2779ce5",
        "8dff93e6fa03e0d498e72a78d2c7bb5f094f5e06ee268e6abd000ba2984a0b6a",
    ),
    (
        "0.87.0",
        1100,
        "3312cce2863588249ebbea6c4608d05a814fbb5ca2a39de86f7a655c5d8c7969",
        "9a6733c0e6a31d592b53dc60df43dd0c26fa793ccf2384ed123fc17b0448866a",
    ),
    (
        "0.87.1",
        1108,
        "d6f039d6a5ba209287537c9f2ae5d71cc00acda15060c98ad496361b6fc17217",
        "1423ee3c61e7c96464e1cbf3c8dc24d3056cb3410995c3671a98c3ecc527540f",
    ),
    (
        "0.99.0",
        1226,
        "7ddfa796b9c5b70dcbe08d60119ede96d19ba11a9e1e7860bead859f1c00c052",
        "19a8dbd8d697a599ea972a8ba307c109a814bb9122604b2553c62006b9617724",
    ),
    (
        "0.99.1",
        1227,
        "913bf8cdb5db68249233691f66b15cffb147c3db39487abd6bea3838c157596c",
        "6686592adaea19092c85c94f5d40323dbf3db141e90eb3ede9e9e87302abdd1d",
    ),
    (
        "0.99.2",
        1228,
        "4e2034d74e6d2f9ea9a49bee3b73e6b761c6dc1ce5922907eb05d79c01ef5586",
        "5bb197bed8e46b5352a7a940ddc868c358725b214f27f3ad4d33e77ee9832558",
    ),
    (
        "1.0.0",
        1243,
        "14313b8e739381719bc844df12dc648dcf531924252e10551cd969e019488134",
        "638ed3abbe54ef70cbf8673ae4bc531e791613756aac04644cfcdcc4af0fafaf",
    ),
    (
        "1.0.1",
        1243,
        "e0a3267a0269ba4a009ff7faeb42bf00e251ed45661c768e78b6ceb424dcef03",
        "99c2e1958ac6d4c6a36e7f1c3690ae38778bb397c63e9d611d2c09521be735c5",
    ),
    (
        "1.0.2",
        1243,
        "a31b4fbf5ee8aae33dd4ee46a0637e850f1f5ddf8114d02a667ed5e38453f662",
        "eda5ae7875343bd902ffe55718fb65b2406d7b03abecc5cb89d8e4bb09ceeda2",
    ),
    (
        "1.0.3",
        1248,
        "4a47c39bc26dc237e97779c44ad2f77698c255d4b4438b7fc52a8455c857180c",
        "106eadb1f823f72f012c08f23bd36e435f9e62f6c81e98a5d8f70c8a9543dd05",
    ),
    (
        "1.0.4",
        1248,
        "3d433e7858f53a48b6a693cb0791e38abe54c5e942b8c1ca1976553581403ab8",
        "04910bdae661a6529e9d6869b04006f6aad01186398b04a16c6d9793667b96c5",
    ),
    (
        "1.1.0",
        1254,
        "62fb9de5c48a64d72f8f8fb2ad7a1f7aae15ca389dc52348b7d973cc9bdf82c8",
        "09cd8a0a43dbb1d81a67346b09400b439ce71d818caba4e963ea958846a1aed4",
    ),
];

const HOP_PATH_HASHES: [(&str, &str, &str, &str, &str); 11] = [
    (
        "0.86.1->0.87.0",
        "f8b7ab7fcc800d2675a8eac19e7b0a4a427edf93f0a8c9e2eb19a2082156def0",
        "83e670f87165e65801eeed62f2df7c4728a96d06bc8025827d237eb6525d1ad5",
        "d1aae9b3bb8b868779e548777e7e4857c83c5a0f996d4e99794a504cae88adb7",
        "1d48401636f5ad64bf2177cb99e24419c10c35cb7eb8e2b6c777d9376d2dad6c",
    ),
    (
        "0.87.0->0.87.1",
        "59d88bc4946dea8149df1490df312a0c7a98f91c15ac53dcba304831b6864609",
        "b4b5f8eb184d5566bc84b76608bcebacd5d347e0003581aa60abb73e3eeff99d",
        "235bff9e91ca09fd516450278d6ea7eb5db6b56cc97532f377ab810f5b201167",
        "f36f9469fc80304311f87dae526e26f340e217b3483d2ac183c3c963c9d8d895",
    ),
    (
        "0.87.1->0.99.0",
        "96d14c08d4b1e4d97d047632063e57a52ef5184ef159c3d8ff45f0259e7d60cb",
        "8fbc8b60860e8669eaf7a0f18126504b2cb23f225adea2226266aed93d3f4971",
        "c546388a20bc4320fdc6d0d7c678f38bb4c4bbfb0dd402a3fe4cd185cf14dd28",
        "b99fff8b56b981867a2d8c44a15a9db0a30c0137853d617f7225d9c68028d6ba",
    ),
    (
        "0.99.0->0.99.1",
        "aeef6deec6d31eb33df97ce20406f1875c92be8ab234e50e5cef7785c2824a3e",
        "75af06a3bded150ca56cd586ecd13e9d9a6881fc7abff3695596be0b27dd0492",
        "105a97baf6ab64ea2a7210203d6a0bc3737a87c2b17aca7e8fda42f8b6356065",
        "74078a5434d0a149daa58aac1a3a23a15582a999c728af5cf11f85f04a46aed8",
    ),
    (
        "0.99.1->0.99.2",
        "3b252ead824c4a98f3af43ba2b48a03428f7c3ee37760285c22b872533126ef2",
        "918388bf4b32fbeef0bf1783e3fba376d12d9f045f338d77d50e79064ce4e87f",
        "93bcb98dd31ffb28e75df79852b5ecb59dac57f9d25a91a9c223e9ddafac8998",
        "2b9b4cb6d610151e844e23467bc2ef88bd3723f11b6b6cc25115f8aa5f567585",
    ),
    (
        "0.99.2->1.0.0",
        "259e2283c4a2aea1515de05409d00ed8e3dba40fc8652c1685a0bab9f60628d9",
        "c680d0928b5d892cafa0396e6ac29c58066999a75ddfd86a9cf6a62d31c7e1d8",
        "1221b2f99e21a7a73f41653064b846753c8405e47ace37514548a20a93266d6b",
        "d1ac8f110bd5bf5496e8101b59751a075169c353377c4fec73ceb094184b4671",
    ),
    (
        "1.0.0->1.0.1",
        "81cfcefeef2d31335b2633445e5b13730e7466d0262861f8dee377c465d3745b",
        "49cc6fe804ebf91bbb7d49c9d8c8c9a3a5ac28241f8e78d1ebe558bc774bf014",
        "25a1a899ded41f13037ec0a0c4d9c1c52a6998ad467a68b1561077cd858a64d6",
        "febfacbf760c4819707d00778da8d110902f76e592f684360d7c72f54c98f591",
    ),
    (
        "1.0.1->1.0.2",
        "beae8c3ed688a760344f9820c5168597dfbf7dafcef1946b3777c73bb5d95933",
        "d5494d34205f84b8488918f4452978760924b3a3c13e78533463fb4424ddcf77",
        "638bb81a8ac7629d35f4be8e0f5ce323880324785d9328998019a4cf83c67fe9",
        "24dd608543f4ba2f4113ed99d35c1cd21de99e665a8f96fb62b592c45044544d",
    ),
    (
        "1.0.2->1.0.3",
        "bdcfce0d9ef788dd6ba4397b10a156899cc0dfbefc415ffda269b42b239049c6",
        "4c4170dbcce7df639bbefc3ad32a03dfd456cc58e3809c23bc30d98404f29758",
        "3399fc859b7ed3c9fd85fda6a24b794923c9d37eee4a743e13c33556dfbf5e5f",
        "ff64bafc90b09e024a79831d36b6773b5661a7686eeb6a3782408c74e4eb9c18",
    ),
    (
        "1.0.3->1.0.4",
        "129046b77864a68bb5d0f40c6b4cd0be52c50b6ae25bc166c0202d794c16e60c",
        "ba807ef5485f93151e0eba800fee3f01dfc6d2f6d96f1db8143f7b5b8ef0740b",
        "8b14ff6a9815f053d2f36206d205c4639e0ce28995c95a97c0c89e0d429e1b8e",
        "59fae3b2457d9e72c94ce6a554f9c40863cad2a811d144c1eb7f5c27c63088de",
    ),
    (
        "1.0.4->1.1.0",
        "f28385da9db5eb4818612f8b4c9ecca65c1eea1f23603be506c583d6a9833951",
        "81cd7d35096eb23dfb85747b88fa04ef6e5cc3f54066ed2ff532c569a5c38bf8",
        "cdcacfe42624dd06d7fb13581e31444f1765a8758594de83604281cca8a037b9",
        "d0e98fb611ef46ba2f4e51d3ac7186cf46008f68f04e5945f0768dd315c3f063",
    ),
];

const HOP_COUNTS: [(&str, usize, usize, usize, usize); 11] = [
    ("0.86.1->0.87.0", 4, 4, 109, 987),
    ("0.87.0->0.87.1", 14, 6, 60, 1034),
    ("0.87.1->0.99.0", 139, 21, 570, 517),
    ("0.99.0->0.99.1", 4, 3, 17, 1206),
    ("0.99.1->0.99.2", 29, 28, 109, 1090),
    ("0.99.2->1.0.0", 30, 15, 106, 1107),
    ("1.0.0->1.0.1", 30, 30, 115, 1098),
    ("1.0.1->1.0.2", 14, 14, 24, 1205),
    ("1.0.2->1.0.3", 35, 30, 42, 1171),
    ("1.0.3->1.0.4", 14, 14, 85, 1149),
    ("1.0.4->1.1.0", 40, 34, 142, 1072),
];

const RELEASE_IDENTITIES: [(&str, &str, &str, &str, u64); 12] = [
    (
        "0.86.1",
        "sha512-vZBuNfJnruxZyemZ3O05V0S/Ylze08ahFTIQ1Mik++gVdOevPl89gt/Uv0U97BPAJaj9cj6Vf9rcIgKtUrd0BA==",
        "13cbf77df2396303013a41646bcfa77b4271ae56",
        "b8e38a10c51d28e946ac2740f13c01358f886a52",
        392403966,
    ),
    (
        "0.87.0",
        "sha512-S9JJVGHya/h0e0M+zwPTB6RkPe7PmLLqfBUTssFYW5mxAti6oZEILn4jvaUImENRC3U9RwXAB6H4gw8xj2J0GQ==",
        "16787ad5b2dc748047f314ca1bfe7708f30f54f3",
        "908417741052a4ef12d9b9a8c0acb98a51ce9d87",
        393194755,
    ),
    (
        "0.87.1",
        "sha512-m8ArJUtVcQMSe1lLE/Ei7vX/JV7O39sWmWBsXV2NOU70F0qCp8GubA24pT3LnwTmM6LL2xV80/h6sQg85n69ew==",
        "f07218c4d4bbc12bef056a7058c3dd49dfe41abe",
        "5708b9310325177d5c1b487b5c99627ffa733324",
        394053788,
    ),
    (
        "0.99.0",
        "sha512-rZ8lLzqhrXrlZSt9VffVmVOcPBzbnK53UilkwEpDqU/WfE1RKCRqzYfGll/Xsk5XumjpeBT8IWzKNP6CyiEtrg==",
        "4b060d3a98618019adb9985d517516c8e99a2bbe",
        "b0e83620c580b0e09e4d519da5f71678ece233b7",
        399359432,
    ),
    (
        "0.99.1",
        "sha512-cWUrTOqA5M73cOYMgsh9PlhDrsBhavd+n5kVY6F7BGbGl1RjqCteVCoeVMVqhngoGACVDyw1tbLjajL8l9jrHg==",
        "d86654abb8862e201933517d6f1fce9f88dd117f",
        "00e7e6c668d67d0825fc8814d80097eb07ec44e0",
        399404207,
    ),
    (
        "0.99.2",
        "sha512-6R1BZ2N77CrVcGf3eC2KovTz1Q4RYiAeydvVWQT546N2fi1nBc81aURlbOZCgruWoW9VY/UrLzDynF4YTolpoA==",
        "005af57d88ee23b33778f343a9595b32e67ff788",
        "6f95461d41661aea1243f6cc2ac0ace8497d21f7",
        400361557,
    ),
    (
        "1.0.0",
        "sha512-/FtbxoSQU/mEv1QnichJjRjqteqaIaMWxmhB4G367+MwZfX7/DI5B9YAg5lqbN7nztFskBEtUSZ+FlmMBECtMw==",
        "a13d35a742c6ef8462812a28fbe1d8c8b7431c32",
        "ae6346e0d5e2a7e2d1fdd177965cda781aa3c514",
        401260174,
    ),
    (
        "1.0.1",
        "sha512-B7FGYpHpBPvS+Ux16CbCuVnE9S4v6c2h6ykocPNarC4msD/4JM/eFrCrO2wbRkedJZ25myFozSWHvCOx4QnN+w==",
        "a7229ddc21810d6245105978033b7df645ecc2f7",
        "c43730168f5482f1c55b462ae43193cc6f7c1e13",
        402502529,
    ),
    (
        "1.0.2",
        "sha512-3ZdIghMSELMGV3sKi5iASOb1Jwb696fLjmNu0aezaqDxTLLWWoRpqBYkGxJ1CgAMCbtfqXWEF0lcRrlVXmiEGQ==",
        "cd32f7725fdbddbaecdff5b1e68491563394e0ca",
        "aba0009c736a1de4e045665acceb017cdb791371",
        402755206,
    ),
    (
        "1.0.3",
        "sha512-t2lb0dw4y/jr5a2PRo6eTHGTZOPB3/YAMVyhhYFC1W3Hl5xE+462I/gMWjF4gCLuhGipNEfuNqONFmdqLFz4SQ==",
        "d78dc83d633229d12f8b79631384c4c2717c399f",
        "de643ce8049ed7182517bbc5620dacf2b68314c6",
        403509577,
    ),
    (
        "1.0.4",
        "sha512-+956nfMFHr5lDUVY/2Q4k+YzojzBuCaBXFgj0eSlXVGr7QVliVddKdc1Pz6yVg1dOlJQmb67doOVrlMsIcIdaw==",
        "7c10bd4337495ee613f2224843ecdf349b80d1df",
        "878235d4ecc2ad3ac6fad0adfeffcf233b8159d7",
        404122010,
    ),
    (
        "1.1.0",
        "sha512-SeEi/4hdcHNgA9UWlefZl7ZZpm3dzi2OoxNjDHsBJ9o298LNOtbL4DGKgitlEj6uCTccvtw6f2hlCkTPVJ2RXg==",
        "abe508e1b89912adde45528136c3221eb69acdd7",
        "0ebd064bf1342f812f9676dde85b2f0b3e02b7e6",
        406210577,
    ),
];

#[test]
fn official_identity_preserves_exact_channels_and_artifact_digests() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "pi.package");
    assert_eq!(identity["npm_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(identity["official_channels"]["npm_latest"], "1.1.0");
    assert_eq!(identity["official_channels"]["github_latest_tag"], "v1.1.0");
    assert_eq!(
        identity["official_channels"]["github_latest_commit"],
        "abe508e1b89912adde45528136c3221eb69acdd7"
    );
    assert_eq!(identity["official_channels"]["latest_channels_agree"], true);
    assert_eq!(
        identity["previous_qualification"]["latest_qualified"],
        "0.86.1"
    );
    assert_eq!(
        identity["previous_qualification"]["claim_id"],
        "pi.rpc.package-window-2"
    );
    assert_eq!(
        identity["runtime_identity"]["npm_engine_requirement"],
        ">=22.19.0"
    );
    assert_eq!(
        identity["runtime_identity"]["official_artifact_executed"],
        false
    );
    assert_eq!(identity["execution_limits"]["provider_prompt_sent"], false);
    assert_eq!(identity["execution_limits"]["live_rpc_session"], false);
    assert_eq!(identity["execution_limits"]["credentials_used"], false);
    assert_eq!(identity["execution_limits"]["package_installation"], false);
    assert_eq!(identity["execution_limits"]["host_mutation"], false);

    assert_eq!(
        strings(&identity["official_channels"]["published_stables_after_previous_ceiling"]),
        VERSIONS[1..]
    );
    let artifacts = identity["official_artifacts"]
        .as_array()
        .expect("official artifacts");
    assert_eq!(artifacts.len(), ARTIFACTS.len());
    assert_eq!(artifacts.len(), RELEASE_IDENTITIES.len());
    for (artifact, (version, count, tree_sha256, tarball_sha256)) in artifacts.iter().zip(ARTIFACTS)
    {
        assert_exact_keys(
            artifact,
            &[
                "complete_tree_sha256",
                "github_release_id",
                "github_release_published_at",
                "github_tag",
                "github_tag_commit",
                "npm_files",
                "npm_git_head",
                "npm_git_head_matches_tag",
                "npm_integrity",
                "npm_published_at",
                "npm_shasum",
                "npm_tarball_sha256",
                "version",
            ],
        );
        assert_eq!(artifact["version"], version);
        assert_eq!(artifact["npm_files"], count);
        assert_eq!(artifact["complete_tree_sha256"], tree_sha256);
        assert_eq!(artifact["npm_tarball_sha256"], tarball_sha256);
        assert_eq!(artifact["github_tag"], format!("v{version}"));
    }
    for (artifact, (version, integrity, tag_commit, shasum, release_id)) in
        artifacts.iter().zip(RELEASE_IDENTITIES)
    {
        assert_eq!(artifact["version"], version);
        assert_eq!(artifact["npm_integrity"], integrity);
        assert_eq!(artifact["npm_shasum"], shasum);
        assert_eq!(artifact["github_release_id"], release_id);
        assert_eq!(artifact["github_tag_commit"], tag_commit);
        if version == "1.1.0" {
            assert_eq!(artifact["npm_git_head"], Value::Null);
            assert_eq!(artifact["npm_git_head_matches_tag"], Value::Null);
        } else {
            assert_eq!(artifact["npm_git_head"], tag_commit);
            assert_eq!(artifact["npm_git_head_matches_tag"], true);
        }
    }
    assert_eq!(
        identity["latest_source_correlation"]["checks"][0]["source"],
        "packages/coding-agent/src/modes/rpc/rpc-mode.ts"
    );
    assert_eq!(
        identity["latest_source_correlation"]["checks"][1]["source"],
        "packages/coding-agent/src/core/agent-session.ts"
    );
    for check in identity["latest_source_correlation"]["checks"]
        .as_array()
        .expect("latest tagged source checks")
    {
        assert_eq!(check["matches_tag"], "v1.1.0");
    }
    assert_eq!(
        identity["latest_source_correlation"]["checks"][0]["sha256"],
        "d534e0fa1484844a097963d5f4fb4df9397c5a1b5e29e42aed7575ab6f67989b"
    );
    assert_eq!(
        identity["latest_source_correlation"]["checks"][1]["sha256"],
        "19186bf8a79876131a296fe17333c6276a671f8f48401df238c9814eeba0f5df"
    );
    assert_eq!(
        identity["latest_source_correlation"]["npm_sourcemap_sources_content_matches_tagged_source"],
        true
    );
}

#[test]
fn complete_package_trees_and_each_published_hop_are_hash_checked() {
    let identity = json(IDENTITY);
    let inventory = json(INVENTORY);
    assert_eq!(inventory["package"], "@earendil-works/pi-coding-agent");
    assert_eq!(strings(&inventory["compared"]), VERSIONS);
    let hashes = inventory["hashes"]
        .as_object()
        .expect("package file hashes");
    assert_eq!(hashes.len(), 1453);

    for (version, count, expected_tree, _) in ARTIFACTS {
        let mut rows = hashes
            .iter()
            .filter_map(|(path, versions)| {
                versions
                    .get(version)
                    .and_then(Value::as_str)
                    .map(|digest| (path.as_str(), digest))
            })
            .collect::<Vec<_>>();
        rows.sort_unstable_by_key(|(path, _)| *path);
        assert_eq!(rows.len(), count, "{version} complete file count");
        let mut tree = Vec::new();
        for (path, digest) in rows {
            tree.extend_from_slice(path.as_bytes());
            tree.push(b'\t');
            tree.extend_from_slice(digest.as_bytes());
            tree.push(b'\n');
        }
        let tree_sha256 = sha256_hex(&tree);
        assert_eq!(
            tree_sha256, expected_tree,
            "{version} recursive tree digest"
        );
        assert_eq!(inventory["manifests"][version]["file_count"], count);
        assert_eq!(
            inventory["manifests"][version]["tree_sha256"],
            expected_tree
        );
        let artifact = identity["official_artifacts"]
            .as_array()
            .expect("official artifacts")
            .iter()
            .find(|artifact| artifact["version"] == version)
            .expect("matching official artifact");
        assert_eq!(artifact["complete_tree_sha256"], expected_tree);
    }

    let hop_records = inventory["hops"].as_object().expect("published hops");
    assert_eq!(hop_records.len(), HOP_PATH_HASHES.len());
    for (
        (hop_name, added_sha256, removed_sha256, changed_sha256, identical_sha256),
        (_, added_count, removed_count, changed_count, identical_count),
    ) in HOP_PATH_HASHES.into_iter().zip(HOP_COUNTS)
    {
        let hop = &inventory["hops"][hop_name];
        assert_exact_keys(
            hop,
            &[
                "added",
                "added_path_list_sha256",
                "changed",
                "changed_path_list_sha256",
                "identical_count",
                "identical_path_list_sha256",
                "removed",
                "removed_path_list_sha256",
            ],
        );
        assert_path_list_hash(&hop["added"], added_sha256);
        assert_path_list_hash(&hop["removed"], removed_sha256);
        assert_path_list_hash(&hop["changed"], changed_sha256);
        assert_eq!(hop["added_path_list_sha256"], added_sha256);
        assert_eq!(hop["removed_path_list_sha256"], removed_sha256);
        assert_eq!(hop["changed_path_list_sha256"], changed_sha256);
        assert_eq!(hop["identical_path_list_sha256"], identical_sha256);

        let (before, after) = hop_name.split_once("->").expect("hop version pair");
        let added = strings(&hop["added"]);
        let removed = strings(&hop["removed"]);
        let changed = strings(&hop["changed"]);
        assert_eq!(added.len(), added_count, "{hop_name} additions");
        assert_eq!(removed.len(), removed_count, "{hop_name} removals");
        assert_eq!(changed.len(), changed_count, "{hop_name} changes");
        let changed_paths = added
            .iter()
            .chain(removed.iter())
            .chain(changed.iter())
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let mut identical = hashes
            .iter()
            .filter_map(|(path, versions)| {
                let old = versions.get(before).and_then(Value::as_str)?;
                let new = versions.get(after).and_then(Value::as_str)?;
                (old == new && !changed_paths.contains(path.as_str())).then_some(path.as_str())
            })
            .collect::<Vec<_>>();
        identical.sort_unstable();
        assert_eq!(
            identical.len(),
            identical_count,
            "{hop_name} unchanged paths"
        );
        assert_eq!(hop["identical_count"], identical.len());
        assert_eq!(sha256_hex(&path_list_bytes(&identical)), identical_sha256);
        let before_count = inventory["manifests"][before]["file_count"]
            .as_u64()
            .expect("previous tree file count") as usize;
        assert_eq!(
            added.len() + removed.len() + changed.len() + identical.len(),
            before_count + added.len()
        );
    }
}

#[test]
fn selected_rpc_hops_and_claim_preserve_segments_and_exclusions() {
    let identity = json(IDENTITY);
    let protocol = json(PROTOCOL);
    let inventory = json(INVENTORY);
    assert_eq!(
        protocol["scope"],
        "pi.rpc only; no Pi SDK sidecar, Oh My Pi, provider API or public operation claim"
    );
    assert_eq!(protocol["previous_qualified_ceiling"], "0.86.1");
    assert_eq!(protocol["current_official_stable"], "1.1.0");
    assert_eq!(protocol["selected_invocation"]["selected_mode"], "rpc");
    assert_eq!(
        protocol["selected_invocation"]["upstream_rpc_command_count"],
        33
    );
    assert_eq!(
        protocol["selected_invocation"]["extension_paths"],
        serde_json::json!([])
    );
    assert_eq!(
        strings(&protocol["selected_invocation"]["adapter_selected_rpc_commands"]),
        [
            "prompt",
            "steer",
            "follow_up",
            "abort",
            "get_state",
            "get_available_models",
            "set_auto_compaction",
            "set_auto_retry",
            "set_steering_mode",
            "set_follow_up_mode",
        ]
    );
    assert_eq!(
        strings(&protocol["selected_invocation"]["flags_stable_across_hops"]),
        [
            "--mode",
            "--no-session",
            "--offline",
            "--provider",
            "--model",
            "--tools",
            "--no-extensions",
            "--no-skills",
            "--no-prompt-templates",
            "--no-themes",
            "--no-context-files",
        ]
    );
    assert_eq!(
        protocol["mapped_behavior"]["command_dispositions"]["introduced"],
        "0.99.0"
    );
    assert_eq!(
        protocol["mapped_behavior"]["agent_settled_aborted"]["introduced"],
        "1.1.0"
    );
    assert_eq!(
        protocol["selected_invocation"]["configuration_and_mcp"]["1.0.4"],
        "--tools allows MCP tools unless an mcp__ entry is named, and --no-mcp is available; route keeps --no-extensions and supplies no extension paths, so the built-in MCP extension is not loaded"
    );
    assert_eq!(
        identity["identity_decision"]["preserve_existing_behavior_revisions"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["new_public_operations"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["public_lifecycle_contract_changed"],
        false
    );
    assert_eq!(
        strings(&identity["official_channels"]["published_stables_after_previous_ceiling"]),
        VERSIONS[1..]
    );

    let hop_records = protocol["per_hop_classifications"]
        .as_array()
        .expect("per-hop classifications");
    assert_eq!(hop_records.len(), HOP_PATH_HASHES.len());
    for (index, hop) in hop_records.iter().enumerate() {
        let (expected_name, _, _, _, _) = HOP_PATH_HASHES[index];
        let (before, after) = expected_name.split_once("->").expect("hop pair");
        assert_eq!(hop["from"], before);
        assert_eq!(hop["to"], after);
        let runtime_from = &hop["compiled_runtime_transition"]["from"];
        let runtime_to = &hop["compiled_runtime_transition"]["to"];
        let runtime_transition = &hop["compiled_runtime_transition"];
        assert_exact_keys(runtime_transition, &["classification", "from", "to"]);
        assert_eq!(
            runtime_transition["classification"],
            "Compiled CLI launcher, runtime import, and main chunk for the selected RPC invocation; exact hashes and invocation markers are checked. Mapped semantics are classified in selected_source_files; other provider-internal implementation details are not claimed."
        );
        assert_eq!(runtime_from["version"], before);
        assert_eq!(runtime_to["version"], after);
        for runtime in [runtime_from, runtime_to] {
            assert_exact_keys(
                runtime,
                &[
                    "classification",
                    "launcher_sha256",
                    "main_chunk",
                    "main_chunk_sha256",
                    "mcp_option_present",
                    "runtime_import_sha256",
                    "selected_markers",
                    "version",
                ],
            );
            let version = runtime["version"].as_str().expect("runtime version");
            let chunk = runtime["main_chunk"].as_str().expect("runtime chunk path");
            assert_eq!(
                inventory["hashes"][chunk][version], runtime["main_chunk_sha256"],
                "{version} compiled runtime chunk"
            );
            assert_eq!(
                inventory["hashes"]["dist/bundle/cli-runtime.js"][version],
                runtime["runtime_import_sha256"]
            );
            assert_eq!(
                inventory["hashes"]["dist/bundle/cli.js"][version],
                runtime["launcher_sha256"]
            );
            assert_eq!(
                runtime["classification"],
                runtime_transition["classification"]
            );
            assert_exact_keys(
                &runtime["selected_markers"],
                &[
                    "--mode",
                    "--no-context-files",
                    "--no-extensions",
                    "--no-session",
                    "--offline",
                    "--model",
                    "--provider",
                    "--tools",
                    "agent_settled",
                ],
            );
            for marker in [
                "agent_settled",
                "--mode",
                "--no-session",
                "--offline",
                "--provider",
                "--model",
                "--tools",
                "--no-extensions",
                "--no-context-files",
            ] {
                assert_eq!(
                    runtime["selected_markers"][marker], true,
                    "{version} {marker}"
                );
            }
        }
        assert_eq!(
            runtime_from["mcp_option_present"],
            matches!(before, "1.0.4" | "1.1.0")
        );
        assert_eq!(
            runtime_to["mcp_option_present"],
            matches!(after, "1.0.4" | "1.1.0")
        );

        let recorded = hop["selected_source_files"]
            .as_array()
            .expect("selected changed source files");
        for entry in recorded {
            assert_exact_keys(entry, &["change", "classification", "path"]);
        }
        let mut recorded_paths = recorded
            .iter()
            .map(|entry| {
                (
                    entry["change"].as_str().expect("change kind"),
                    entry["path"].as_str().expect("source path"),
                )
            })
            .collect::<Vec<_>>();
        recorded_paths.sort_unstable();
        let changed = &inventory["hops"][expected_name];
        let mut inventory_paths = Vec::new();
        for change in ["added", "removed", "changed"] {
            for path in strings(&changed[change]) {
                if selected_source_path(path) {
                    inventory_paths.push((change, path));
                }
            }
        }
        inventory_paths.sort_unstable();
        assert_eq!(
            recorded_paths, inventory_paths,
            "{expected_name} selected file set"
        );
        assert!(recorded.iter().all(|entry| {
            entry["classification"]
                .as_str()
                .is_some_and(|value| !value.is_empty())
        }));
    }

    assert_eq!(PI_PACKAGE_BASELINE_VERSION, "0.80.10");
    assert_eq!(PI_PACKAGE_LATEST_QUALIFIED_VERSION, "1.1.0");
    let claim = pi_rpc_claim();
    assert_eq!(claim.id().as_str(), "pi.rpc.package-window-2");
    for (version, behavior, status) in [
        (
            "0.87.0",
            "pi.rpc.strict-lf-v0.84.0-message-update-delta",
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.99.0",
            "pi.rpc.strict-lf-v0.99.0-command-disposition",
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "1.0.4",
            "pi.rpc.strict-lf-v0.99.0-command-disposition",
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "1.1.0",
            "pi.rpc.strict-lf-v1.1.0-aborted-settled",
            InterfaceSupportStatus::Maintained,
        ),
    ] {
        assert!(matches!(
            claim.assess(&version_value(version)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.behavior_revision().as_str() == behavior
                    && matched.support_status() == status
        ));
    }
    for gap in [
        "0.83.1", "0.84.5", "0.85.2", "0.86.2", "0.87.2", "0.98.9", "0.99.3", "1.0.5",
    ] {
        assert!(
            matches!(
                claim.assess(&version_value(gap)),
                InterfaceCompatibilityAssessment::Incompatible
            ),
            "{gap}"
        );
    }
    assert!(matches!(
        claim.assess(&version_value("1.1.1")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen identity JSON is valid")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

fn assert_path_list_hash(value: &Value, expected: &str) {
    let paths = strings(value);
    assert!(paths.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(sha256_hex(&path_list_bytes(&paths)), expected);
}

fn path_list_bytes(paths: &[&str]) -> Vec<u8> {
    let mut bytes = paths.join("\n").into_bytes();
    bytes.push(b'\n');
    bytes
}

fn sha256_hex(value: &[u8]) -> String {
    Sha256::digest(value)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let mut keys = value
        .as_object()
        .expect("JSON value is an object")
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    keys.sort_unstable();
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    assert_eq!(keys, expected);
}

fn selected_source_path(path: &str) -> bool {
    path.starts_with("dist/modes/rpc/")
        || [
            "dist/core/agent-session.js",
            "dist/core/agent-session.js.map",
            "dist/core/settings-manager.js",
            "dist/core/settings-manager.js.map",
            "dist/core/resource-loader.js",
            "dist/core/resource-loader.js.map",
            "dist/core/mcp-servers.js",
            "dist/core/mcp-servers.js.map",
            "dist/cli/args.js",
            "dist/cli/args.js.map",
            "dist/cli/setup.js",
            "dist/cli/setup.js.map",
            "dist/extensions/index.js",
            "dist/extensions/index.js.map",
        ]
        .contains(&path)
        || path.starts_with("dist/extensions/mcp/")
        || path.starts_with("dist/extensions/codemode/")
        || path.starts_with("dist/extensions/tool-search/")
}

fn version_value(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("version fixture is valid")
}
