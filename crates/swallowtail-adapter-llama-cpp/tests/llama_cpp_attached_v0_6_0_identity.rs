use serde_json::{Value, json};
use swallowtail_adapter_llama_cpp::{
    LLAMA_CPP_ATTACHED_RUNTIME_REVISION, llama_cpp_attached_runtime_binding,
    llama_cpp_attached_runtime_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionScheme,
};

const IDENTITY: &str = include_str!("fixtures/llama-cpp-v0.6.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/llama-cpp-v0.6.0/protocol.json");
const INVENTORY: &str = include_str!("fixtures/llama-cpp-v0.6.0/dist-inventory.json");

#[test]
fn official_identity_freezes_the_exact_runtime_bridge_and_complete_inventory() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture parses");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture parses");
    let inventory: Value = serde_json::from_str(INVENTORY).expect("tree inventory parses");

    assert_eq!(identity["route"], "llama-cpp.attached");
    assert_eq!(identity["axis"], "llama.cpp.attached-runtime");
    assert_eq!(identity["identity_decision"], "compatible-extension");
    assert_eq!(identity["baseline"]["runtime_revision"], "b9910-f5525f7e7");
    assert_eq!(identity["qualified_runtime_revision"], "b11429-d81235049");
    assert_eq!(identity["qualified_stable_release"], "v0.6.0");
    assert_eq!(
        identity["published_stables_after_qualified_point"],
        json!(["v0.2.0", "v0.3.0", "v0.4.0", "v0.4.1", "v0.5.0", "v0.6.0"])
    );
    let intermediate_hops = identity["intermediate_stable_hops"]
        .as_array()
        .expect("intermediate stable hops are present");
    let expected_intermediate_hops = [
        (
            "v0.2.0",
            "b10566-bb4caa754",
            "bb4caa7540188872173c44d161602d9271386413",
            "13a71b442e5376bb000b26f4e8b4b06565758cb5e47a573d882e9c299d8b2e04",
            "86383822e6176cca60227f593abdc61a605efae23efac02c3bfb1569077811c0",
        ),
        (
            "v0.3.0",
            "b10621-c1d0e7a00",
            "c1d0e7a004015f23bc0233470b747b596f29b264",
            "2d1bb0124f338c154d19f098ce56869a9e405d2d94d723bef7b8e575f901aeeb",
            "46637e1a90db3d9912a7722806c7657cc73a3965e21c0016cc6b087b3be84205",
        ),
        (
            "v0.4.0",
            "b10809-5266f24da",
            "5266f24da75dc449bd56cbed7addb9c8e4a6a73e",
            "1ebab599be928beaba548cc003035128e830d168bb5caed15137b2c8d58e6725",
            "bd0afc13ffe4aaa2f02eb7df8882a2617907d7f232012ae2ca0f21f6b865fbe0",
        ),
        (
            "v0.4.1",
            "b10964-b29c606e2",
            "b29c606e28a01b1bc8c1351026a0fa6e616bf6c4",
            "2d069b4acf5bbc59fefb3f38e393320e7ed77001da2c1a4075466e6cb100bd5b",
            "3d64814d6d05845a4b749392f4cb916b84709ab5b1393edcec5e680e2ddd4663",
        ),
        (
            "v0.5.0",
            "b11146-7fe450e19",
            "7fe450e19305b828c199d602c23a8337aaa1f03b",
            "3233591576fa0d6527ab92b0c40f55a5e02e1bc167a50d61bfe23c290b4be84b",
            "defb2cdfd758ca455a291fe3e37c0f9a442f5ed4fcafe4cbc179237cfb43374f",
        ),
    ];
    assert_eq!(intermediate_hops.len(), expected_intermediate_hops.len());
    for (hop, (release, runtime, commit, archive_sha, asset_sha)) in
        intermediate_hops.iter().zip(expected_intermediate_hops)
    {
        assert_eq!(hop["semantic_release"], release);
        assert_eq!(hop["source_tag_commit"], commit);
        assert_eq!(
            hop["runtime_identity_link"]["exact_runtime_revision"],
            runtime
        );
        assert_eq!(hop["runtime_identity_link"]["nightly_tag_commit"], commit);
        assert_eq!(hop["runtime_identity_link"]["semantic_tag_commit"], commit);
        assert_eq!(hop["source_archive"]["sha256"], archive_sha);
        assert_eq!(hop["nightly_tag_asset"]["sha256"], asset_sha);
        assert_eq!(hop["qualification"], "unqualified_exact_gap");
    }
    assert_eq!(
        identity["stable_channel_exclusions"]
            .as_array()
            .expect("channel exclusions are present")
            .iter()
            .map(|entry| entry["release"].as_str().expect("release tag is text"))
            .collect::<Vec<_>>(),
        ["v0.1.0", "v0.1.1", "v0.1.2"]
    );
    assert_eq!(identity["synthetic_next_stable_is_unqualified"], true);

    let official = &identity["official"];
    let runtime_link = &official["runtime_identity_link"];
    assert_eq!(official["release_tag"], "v0.6.0");
    assert_eq!(runtime_link["nightly_tag_asset"], "b11429");
    assert_eq!(
        runtime_link["nightly_tag_commit"],
        "d81235049384534c167caea52b85a694f6103d14"
    );
    assert_eq!(
        runtime_link["semantic_tag_commit"],
        runtime_link["nightly_tag_commit"]
    );
    assert_eq!(
        runtime_link["semantic_release"], "v0.6.0",
        "the semantic tag remains separate from the opaque runtime point"
    );
    assert_eq!(
        official["release_assets"][0]["sha256"],
        "5677c5a4561fad44f2a58fc14187dd9cde253874584e958b37094c723ea8e8a5"
    );
    assert_eq!(
        official["source_archive"]["sha256"],
        "a56a6273847c92c2d516b2a971343eaec05b588abe74dc361094cc28493a1fac"
    );

    let compared = json!([
        "b9910", "v0.2.0", "v0.3.0", "v0.4.0", "v0.4.1", "v0.5.0", "v0.6.0"
    ]);
    assert_eq!(inventory["compared"], compared);
    assert_eq!(
        inventory["package_file_counts"],
        json!({
            "b9910": 3031,
            "v0.2.0": 3446,
            "v0.3.0": 3494,
            "v0.4.0": 3527,
            "v0.4.1": 3588,
            "v0.5.0": 3619,
            "v0.6.0": 3687
        })
    );
    assert_eq!(inventory["package_file_counts"]["b9910"], 3031);
    assert_eq!(inventory["package_file_counts"]["v0.6.0"], 3687);
    assert_eq!(
        inventory["file_set_sha256"],
        json!({
            "added": "sha256:9860faf034a7520dcdedaa22e4a7b587f72a61125176b664541b0c54d2052caf",
            "removed": "sha256:b53befedd1da2a9ef35f0f03286260db1d424db5e42cd53112a7e3cb34931f91",
            "changed": "sha256:75e63740574dcf98e4b4ec65d93f289be95511147f473746e6d6157946a4117a",
            "identical": "sha256:4b1384ff4b3e535f25d4ee8b1b25d1ccd007d9b76a30bbc7cb286296be527016"
        })
    );

    let per_hop = inventory["per_hop"]
        .as_array()
        .expect("adjacent tree inventory is present");
    assert_eq!(per_hop.len(), 6);
    let stable_assessments = protocol["stable_hop_assessments"]
        .as_array()
        .expect("published stable assessments are present");
    assert_eq!(stable_assessments.len(), 6);
    assert_eq!(
        stable_assessments
            .iter()
            .map(|hop| hop["runtime_revision"].as_str().expect("runtime is text"))
            .collect::<Vec<_>>(),
        [
            "b10566-bb4caa754",
            "b10621-c1d0e7a00",
            "b10809-5266f24da",
            "b10964-b29c606e2",
            "b11146-7fe450e19",
            "b11429-d81235049"
        ]
    );
    assert_eq!(
        stable_assessments
            .iter()
            .map(|hop| hop["qualification"]
                .as_str()
                .expect("qualification is text"))
            .collect::<Vec<_>>(),
        [
            "unqualified_exact_gap",
            "unqualified_exact_gap",
            "unqualified_exact_gap",
            "unqualified_exact_gap",
            "unqualified_exact_gap",
            "qualified_exact_point"
        ]
    );
    let expected_tree_hops: [(&str, &str, [u64; 6], &str); 6] = [
        (
            "b9910",
            "v0.2.0",
            [3031, 3446, 546, 131, 1070, 1830],
            "sha256:4f9d886bbc4a7dbd808b55c68399604110182e4c703cffc2f7a1b5221ca69bfe",
        ),
        (
            "v0.2.0",
            "v0.3.0",
            [3446, 3494, 54, 6, 193, 3247],
            "sha256:764cb931f90ac0c9bb39521b5fbc08e1ff79b336a456469b45862c0f9ee2adb3",
        ),
        (
            "v0.3.0",
            "v0.4.0",
            [3494, 3527, 52, 19, 686, 2789],
            "sha256:79c216ae331bbeec9432de796ff6758afaeddf956bdb0c79a7e9ac30a224797c",
        ),
        (
            "v0.4.0",
            "v0.4.1",
            [3527, 3588, 66, 5, 340, 3182],
            "sha256:c13a80104bd87d52d8b71a8f5fe4b353d3d203d135350bc97ab7faafd475e79f",
        ),
        (
            "v0.4.1",
            "v0.5.0",
            [3588, 3619, 36, 5, 378, 3205],
            "sha256:56fc060c610fc228baf85bfabba40fd0f1832d9ee2b7142c3959454645310f34",
        ),
        (
            "v0.5.0",
            "v0.6.0",
            [3619, 3687, 73, 5, 524, 3090],
            "sha256:c4c98e891feb7f2f5aa8f11117f8469c17710c5059db60c92f324b8a67765e3b",
        ),
    ];
    let expected_tree_file_set_hashes = [
        json!({
            "added": "sha256:0c1fa69879faf44a461504f95732a371154237fa2cd824ab5771abf847cd1a0d",
            "removed": "sha256:27b46d5484d86be9951ec5c1dadb73c5722e448a9a359cb6f654aa51f7f1add2",
            "changed": "sha256:4f9d886bbc4a7dbd808b55c68399604110182e4c703cffc2f7a1b5221ca69bfe",
            "identical": "sha256:d397963ab9246d69cbbea461d81c783a182ab00b855518fc50b170db3c3bd26f"
        }),
        json!({
            "added": "sha256:a31ef338161bdc382689dd436a78f66d48dffb69b49d04781edfe63fed2d357d",
            "removed": "sha256:7bdc7a78ac5d52e3034757679647dd8950cbf61b1e85b8f22eacf3e6f1054c50",
            "changed": "sha256:764cb931f90ac0c9bb39521b5fbc08e1ff79b336a456469b45862c0f9ee2adb3",
            "identical": "sha256:e70581b2c9599e981752a8e24195a91aa4a14be11eb5b66e6c44dc931680bacd"
        }),
        json!({
            "added": "sha256:86233c54d6d26054d946f1b309d241cee3610bc415fe7cbd5b7acd966288bb66",
            "removed": "sha256:50ee19a3fde93b1014767665d3a8a39a22ed1b5f16742df94f3e2b56e2a4e156",
            "changed": "sha256:79c216ae331bbeec9432de796ff6758afaeddf956bdb0c79a7e9ac30a224797c",
            "identical": "sha256:276b511e56bd239f4cd57e47030d1126501116c9f0f3dccd045071f204f4dd2d"
        }),
        json!({
            "added": "sha256:49aa19f30a2f97d461a02112242496d8ca5c2578500bf5a7f23c4d6cc204a644",
            "removed": "sha256:fa0bdc252bfbb6a5ba4c85d8fd682ed60cf27655378d5741cfebb882440e8e0a",
            "changed": "sha256:c13a80104bd87d52d8b71a8f5fe4b353d3d203d135350bc97ab7faafd475e79f",
            "identical": "sha256:9176d40d74b3a60ee97adf77aed89de471cd380d2d6743e90154e60ca41bbe2b"
        }),
        json!({
            "added": "sha256:eefb36e14f5a9e04589ae504f5a032e36221377ab7f1774ac84b793a3ae1be3e",
            "removed": "sha256:b0283e2abbed821d7b10530c4926d17774b0cd70be8933f5bf265b951b181624",
            "changed": "sha256:56fc060c610fc228baf85bfabba40fd0f1832d9ee2b7142c3959454645310f34",
            "identical": "sha256:e17b65a7773fcc86acd36aa8be4dc730faebe16d2ba15c5f1707df47696b3506"
        }),
        json!({
            "added": "sha256:eb503a690cdc61f7824802752934a0086a551ecc40c80ef09c61e9973ef322ac",
            "removed": "sha256:523b31fe4359fb5fe5ebe11bf3f1eebdaa491c50275610d32cde725f9d792ad2",
            "changed": "sha256:c4c98e891feb7f2f5aa8f11117f8469c17710c5059db60c92f324b8a67765e3b",
            "identical": "sha256:0f63345ed62076862ec38df35592c332e6aa407851529a589e027287d276e99e"
        }),
    ];
    let expected_selected_paths: [&[&str]; 6] = [
        &[
            "CMakeLists.txt",
            "common/arg.cpp",
            "common/build-info.cpp.in",
            "common/build-info.h",
            "common/chat.cpp",
            "common/chat.h",
            "common/jinja/caps.cpp",
            "common/jinja/caps.h",
            "tools/server/server-chat.cpp",
            "tools/server/server-common.cpp",
            "tools/server/server-common.h",
            "tools/server/server-context.cpp",
            "tools/server/server-context.h",
            "tools/server/server-http.cpp",
            "tools/server/server-http.h",
            "tools/server/server-models.cpp",
            "tools/server/server-models.h",
            "tools/server/server-queue.cpp",
            "tools/server/server-queue.h",
            "tools/server/server-stream.cpp",
            "tools/server/server-stream.h",
            "tools/server/server-task.cpp",
            "tools/server/server-task.h",
            "tools/server/server.cpp",
            "vendor/cpp-httplib/httplib.cpp",
            "vendor/cpp-httplib/httplib.h",
        ],
        &[
            "CMakeLists.txt",
            "common/arg.cpp",
            "common/chat.cpp",
            "common/chat.h",
            "common/jinja/caps.cpp",
            "tools/server/server-chat.cpp",
            "tools/server/server-chat.h",
            "tools/server/server-common.cpp",
            "tools/server/server-common.h",
            "tools/server/server-context.cpp",
            "tools/server/server-context.h",
            "tools/server/server-models.cpp",
            "tools/server/server-task.cpp",
            "tools/server/server-task.h",
        ],
        &[
            "CMakeLists.txt",
            "common/arg.cpp",
            "common/build-info.cpp.in",
            "common/build-info.h",
            "common/chat.cpp",
            "src/llama-version.h.in",
            "tools/server/server-common.cpp",
            "tools/server/server-common.h",
            "tools/server/server-context.cpp",
            "tools/server/server-context.h",
            "tools/server/server.cpp",
            "vendor/cpp-httplib/httplib.cpp",
            "vendor/cpp-httplib/httplib.h",
        ],
        &[
            "CMakeLists.txt",
            "common/arg.cpp",
            "common/chat.cpp",
            "common/chat.h",
            "common/jinja/caps.cpp",
            "tools/server/server-common.cpp",
            "tools/server/server-common.h",
            "tools/server/server-context.cpp",
            "tools/server/server-models.cpp",
            "tools/server/server-models.h",
            "vendor/cpp-httplib/httplib.cpp",
            "vendor/cpp-httplib/httplib.h",
        ],
        &[
            "CMakeLists.txt",
            "common/arg.cpp",
            "common/chat.cpp",
            "tools/server/server-chat.cpp",
            "tools/server/server-common.cpp",
            "tools/server/server-context.cpp",
            "tools/server/server-context.h",
            "tools/server/server-http.cpp",
            "tools/server/server-http.h",
            "tools/server/server-models.cpp",
            "tools/server/server.cpp",
            "vendor/cpp-httplib/httplib.cpp",
            "vendor/cpp-httplib/httplib.h",
        ],
        &[
            "CMakeLists.txt",
            "common/arg.cpp",
            "common/chat.cpp",
            "tools/server/server-common.cpp",
            "tools/server/server-common.h",
            "tools/server/server-context.cpp",
            "tools/server/server-context.h",
            "tools/server/server-http.cpp",
            "tools/server/server-models.cpp",
            "tools/server/server-models.h",
            "tools/server/server-task.cpp",
            "tools/server/server-task.h",
            "tools/server/server.cpp",
            "vendor/cpp-httplib/httplib.cpp",
            "vendor/cpp-httplib/httplib.h",
        ],
    ];
    for (index, (hop, (from, to, counts, changed_sha))) in
        per_hop.iter().zip(expected_tree_hops).enumerate()
    {
        assert_eq!(hop["from"], from);
        assert_eq!(hop["to"], to);
        assert_eq!(
            [
                hop["package_file_counts"][from]
                    .as_u64()
                    .expect("source count"),
                hop["package_file_counts"][to]
                    .as_u64()
                    .expect("target count"),
                hop["counts"]["added"].as_u64().expect("added count"),
                hop["counts"]["removed"].as_u64().expect("removed count"),
                hop["counts"]["changed"].as_u64().expect("changed count"),
                hop["counts"]["identical"]
                    .as_u64()
                    .expect("identical count"),
            ],
            counts.map(u64::from)
        );
        assert_eq!(hop["file_set_sha256"], expected_tree_file_set_hashes[index]);
        assert_eq!(hop["file_set_sha256"]["changed"], changed_sha);
        assert_eq!(
            hop["selected_changed_files"]
                .as_array()
                .expect("hop selected file set is present")
                .iter()
                .map(|path| path.as_str().expect("file path is text"))
                .collect::<Vec<_>>(),
            expected_selected_paths[index].to_vec()
        );
        assert_eq!(
            hop["selected_changed_files"],
            protocol["stable_hop_assessments"][index]["selected_changed_files"]
        );
        let hop_classifications = hop["selected_change_classification"]
            .as_object()
            .expect("every hop has selected file classifications");
        let all_classifications =
            protocol["selected_behavior_comparison"]["source_change_classifications"]
                .as_object()
                .expect("selected source classifications are present");
        let mut flattened = Vec::new();
        let mut expected_category_count = 0;
        for (category, files) in all_classifications {
            let expected_files = files
                .as_array()
                .expect("classification file set is an array")
                .iter()
                .filter_map(|file| {
                    let path = file.as_str().expect("file path is text");
                    expected_selected_paths[index]
                        .contains(&path)
                        .then_some(path)
                })
                .collect::<Vec<_>>();
            if !expected_files.is_empty() {
                expected_category_count += 1;
                assert_eq!(
                    hop_classifications[category.as_str()],
                    json!(expected_files)
                );
                flattened.extend(expected_files);
            }
        }
        assert_eq!(hop_classifications.len(), expected_category_count);
        flattened.sort_unstable();
        assert_eq!(flattened, expected_selected_paths[index].to_vec());
    }

    let selected_files = protocol["selected_behavior_comparison"]["selected_changed_files"]
        .as_array()
        .expect("selected changed-file list is present")
        .iter()
        .map(|path| path.as_str().expect("file path is text"))
        .collect::<Vec<_>>();
    let expected_files = [
        "CMakeLists.txt",
        "common/arg.cpp",
        "common/build-info.cpp.in",
        "common/build-info.h",
        "common/chat.cpp",
        "common/chat.h",
        "common/jinja/caps.cpp",
        "common/jinja/caps.h",
        "src/llama-version.h.in",
        "tools/server/server-chat.cpp",
        "tools/server/server-chat.h",
        "tools/server/server-common.cpp",
        "tools/server/server-common.h",
        "tools/server/server-context.cpp",
        "tools/server/server-context.h",
        "tools/server/server-http.cpp",
        "tools/server/server-http.h",
        "tools/server/server-models.cpp",
        "tools/server/server-models.h",
        "tools/server/server-queue.cpp",
        "tools/server/server-queue.h",
        "tools/server/server-stream.cpp",
        "tools/server/server-stream.h",
        "tools/server/server-task.cpp",
        "tools/server/server-task.h",
        "tools/server/server.cpp",
        "vendor/cpp-httplib/httplib.cpp",
        "vendor/cpp-httplib/httplib.h",
    ];
    assert_eq!(selected_files, expected_files);
    assert_eq!(
        protocol["stable_hop_assessments"]
            .as_array()
            .expect("stable hop classifications are present")
            .len(),
        6
    );
    assert_eq!(
        inventory["hashes"]
            .as_object()
            .expect("selected file hashes are present")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        expected_files
    );
    for (file, hashes) in inventory["hashes"]
        .as_object()
        .expect("selected file hashes are present")
    {
        let hashes = hashes.as_object().expect("file hash map is an object");
        assert_eq!(
            hashes.keys().map(String::as_str).collect::<Vec<_>>(),
            [
                "b9910", "v0.2.0", "v0.3.0", "v0.4.0", "v0.4.1", "v0.5.0", "v0.6.0"
            ]
        );
        for (version, digest) in hashes {
            if file == "src/llama-version.h.in"
                && matches!(version.as_str(), "b9910" | "v0.2.0" | "v0.3.0")
            {
                assert!(digest.is_null());
            } else {
                assert!(
                    digest
                        .as_str()
                        .is_some_and(|value| value.starts_with("sha256:"))
                );
            }
        }
    }
    assert_eq!(
        protocol["selected_behavior_comparison"]["source_change_classifications"],
        json!({
            "chat_message_and_template_mapping": ["common/chat.cpp", "common/chat.h", "tools/server/server-chat.cpp", "tools/server/server-chat.h"],
            "chat_template_capability_assessment": ["common/jinja/caps.cpp", "common/jinja/caps.h"],
            "completion_usage_and_terminal_lifecycle": ["tools/server/server-queue.cpp", "tools/server/server-queue.h", "tools/server/server-task.cpp", "tools/server/server-task.h"],
            "configuration_and_permission_boundary": ["common/arg.cpp", "tools/server/server-http.cpp", "tools/server/server-http.h", "tools/server/server.cpp"],
            "errors_and_http_stream_transport": ["tools/server/server-common.cpp", "tools/server/server-common.h", "tools/server/server-stream.cpp", "tools/server/server-stream.h", "vendor/cpp-httplib/httplib.cpp", "vendor/cpp-httplib/httplib.h"],
            "health_properties_and_chat_mapping": ["tools/server/server-context.cpp", "tools/server/server-context.h"],
            "properties_and_model_catalogue_handlers": ["tools/server/server-models.cpp", "tools/server/server-models.h"],
            "runtime_identity": ["CMakeLists.txt", "common/build-info.cpp.in", "common/build-info.h", "src/llama-version.h.in"]
        })
    );
    assert_eq!(
        protocol["selected_behavior_comparison"]["selected_capability_assessment_source_changed"],
        true
    );
    let mut selected_classification_detail_paths =
        protocol["selected_behavior_comparison"]["source_change_classification_details"]
            .as_object()
            .expect("omitted source paths have details")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>();
    selected_classification_detail_paths.sort_unstable();
    assert_eq!(
        selected_classification_detail_paths,
        [
            "common/build-info.h",
            "common/jinja/caps.cpp",
            "common/jinja/caps.h",
            "tools/server/server-models.cpp",
            "tools/server/server-models.h"
        ]
    );
    assert_eq!(
        protocol["mutation_sensitive_inventory"]["exact_selected_file_list"],
        true
    );
    assert_eq!(
        protocol["mutation_sensitive_inventory"]["exact_added_removed_changed_identical_lists"],
        true
    );
    assert_eq!(
        protocol["mutation_sensitive_inventory"]["exact_stable_hop_selected_file_sets"],
        true
    );
}

#[test]
fn attached_claim_is_a_finite_exact_set_and_keeps_the_released_binding() {
    let claim = llama_cpp_attached_runtime_claim();
    assert_eq!(claim.id().as_str(), "llama-cpp.attached-runtime-window-1");
    assert_eq!(claim.axis().as_str(), "llama.cpp.attached-runtime");
    assert_eq!(claim.scheme(), InterfaceVersionScheme::Opaque);
    assert_eq!(
        claim.newer_version_posture(),
        InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert!(!claim.has_version_interval());
    assert_eq!(
        llama_cpp_attached_runtime_binding().version().as_str(),
        LLAMA_CPP_ATTACHED_RUNTIME_REVISION
    );
    assert_eq!(LLAMA_CPP_ATTACHED_RUNTIME_REVISION, "b9910-f5525f7e7");
    assert!(claim.exclusions().next().is_none());

    let members = claim.milestones().collect::<Vec<_>>();
    assert_eq!(
        members
            .iter()
            .map(|member| member.minimum().as_str())
            .collect::<Vec<_>>(),
        ["b11429-d81235049", "b9910-f5525f7e7"]
    );
    assert!(members.iter().all(|member| {
        member.minimum() == member.maximum()
            && member.support_status() == InterfaceSupportStatus::Maintained
            && member.behavior_revision().as_str() == "llama-cpp.attached-openai-chat-b9910"
    }));

    for point in ["b9910-f5525f7e7", "b11429-d81235049"] {
        assert!(matches!(
            claim.assess(&version(point)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str() == "llama-cpp.attached-openai-chat-b9910"
        ));
    }
    for nonmember in [
        "b11430-d81235049",
        "b10069-178a6c449",
        "b10566-bb4caa754",
        "b10621-c1d0e7a00",
        "b10809-5266f24da",
        "b10964-b29c606e2",
        "b11146-7fe450e19",
        "b11429-d81235048",
        "v0.6.0",
    ] {
        assert_eq!(
            claim.assess(&version(nonmember)),
            InterfaceCompatibilityAssessment::Incompatible,
            "non-member {nonmember} must not inherit support"
        );
    }
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture runtime revision is valid")
}
