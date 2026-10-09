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
        json!(["v0.6.0"])
    );
    assert_eq!(identity["intermediate_stable_hops"], json!([]));
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

    assert_eq!(inventory["compared"], json!(["b9910", "v0.6.0"]));
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
        "common/chat.cpp",
        "common/chat.h",
        "src/llama-version.h.in",
        "tools/server/server-chat.cpp",
        "tools/server/server-chat.h",
        "tools/server/server-common.cpp",
        "tools/server/server-common.h",
        "tools/server/server-context.cpp",
        "tools/server/server-context.h",
        "tools/server/server-http.cpp",
        "tools/server/server-http.h",
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
        inventory["hashes"]
            .as_object()
            .expect("selected file hashes are present")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        expected_files
    );
    assert_eq!(
        protocol["mutation_sensitive_inventory"]["exact_added_removed_changed_identical_lists"],
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
