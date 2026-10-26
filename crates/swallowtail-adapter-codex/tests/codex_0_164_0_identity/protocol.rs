use super::support::{HOPS, PROTOCOL, assert_sha256, json};

#[test]
fn selected_mapped_files_stay_byte_identical() {
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["selectedMappedSubsetUnchanged"], true);
    assert_eq!(protocol["noPrompt"], true);
    assert_eq!(protocol["noLiveSession"], true);
    let identical = protocol["byteIdenticalThroughAllHops"]
        .as_object()
        .expect("identical files");
    assert_eq!(identical.len(), 9);
    for (path, digest) in identical {
        assert_sha256(digest);
        assert_eq!(digest.as_str().unwrap().len(), 64, "{path}");
    }
    assert_eq!(
        identical["codex-rs/exec/src/cli.rs"],
        "66643e8b3d5964b70cb8f231fb309312376272fb3ea87bcb6849f44cf112f8f6"
    );
    assert_eq!(
        identical["codex-rs/app-server/src/request_processors/thread_delete.rs"],
        "44391901784847c1b62463739df39fdac408d78e300c0a58f39c2f7804400751"
    );
}

#[test]
fn classified_deltas_include_thread_processor_and_every_digest() {
    let protocol = json(PROTOCOL);
    let deltas = protocol["classifiedDeltas"].as_array().expect("deltas");
    assert_eq!(deltas.len(), 10);
    let mut saw_thread_processor = false;
    for delta in deltas {
        let class = delta["class"].as_str().expect("class");
        assert!(
            matches!(
                class,
                "unselected" | "additive-optional" | "selected-lifecycle-internal-hardening"
            ),
            "{class}"
        );
        assert!(!delta["reason"].as_str().unwrap().is_empty());
        let path = delta["path"].as_str().expect("path");
        if path == "codex-rs/app-server/src/request_processors/thread_processor.rs" {
            saw_thread_processor = true;
            assert_eq!(class, "selected-lifecycle-internal-hardening");
            assert!(
                delta["reason"]
                    .as_str()
                    .unwrap()
                    .contains("finalize_thread_teardown")
            );
            assert!(
                delta["reason"]
                    .as_str()
                    .unwrap()
                    .contains("wait_for_thread_shutdown")
            );
        }
        let sha = &protocol["contentSha256"][path];
        let hop = delta["hop"].as_str().expect("hop");
        let (from, to) = hop.split_once("->").expect("hop range");
        assert_sha256(&sha[from]);
        assert_sha256(&sha[to]);
        if hop == "0.161.0->0.164.0" {
            for point in HOPS {
                assert_sha256(&sha[*point]);
            }
        }
    }
    assert!(
        saw_thread_processor,
        "thread_processor.rs must be classified"
    );
    assert_eq!(
        protocol["contentSha256"]["codex-rs/app-server/src/request_processors/thread_processor.rs"]
            ["0.161.0"],
        "2c4786cb9b9fc2a4401d4bdd6f4af53fb18c8481cdb0277ba26c719d5f0a91e4"
    );
    assert_eq!(
        protocol["contentSha256"]["codex-rs/app-server/src/request_processors/thread_processor.rs"]
            ["0.164.0"],
        "0c806ff3aaf348f7373032171d55ff110dfd30bcbb5cfabded89330b5b4375b6"
    );
}

#[test]
fn exec_deny_globs_are_traced_off_the_selected_route() {
    let reach = &json(PROTOCOL)["execReachability"];
    assert_eq!(
        reach["selectedArgv"],
        serde_json::json!(["exec", "--json", "--sandbox", "read-only"])
    );
    assert_eq!(reach["selectedArgvFilesByteIdentical"], true);
    let deny = &reach["withExpandedDenyGlobs"];
    assert_eq!(
        deny["definedAt"],
        "codex-rs/protocol/src/permissions.rs:1843"
    );
    assert_eq!(deny["introducedHop"], "0.162.1->0.163.0");
    let callers = deny["callers"].as_array().expect("callers");
    assert_eq!(callers.len(), 1);
    assert_eq!(
        callers[0]["path"],
        "codex-rs/exec-server/src/sandbox_integrity/bubblewrap.rs"
    );
    assert_eq!(callers[0]["line"], 119);
    assert_eq!(
        deny["selectedExecCallers"]
            .as_array()
            .expect("selected callers")
            .len(),
        0
    );
    assert_eq!(deny["execLibExecServerRefs"], 0);
    assert_eq!(deny["jsonModeBwrapWarning"]["reachedOnSelected"], false);
    assert_eq!(deny["jsonModeBwrapWarning"]["selectedJsonMode"], true);
    assert_eq!(deny["awsProtectionRemains"], true);
}

#[test]
fn archive_delete_unarchive_traces_use_handler_paths() {
    let traces = &json(PROTOCOL)["handlerTraces"];
    let delete = &traces["threadDelete"];
    assert_eq!(
        delete["file"],
        "codex-rs/app-server/src/request_processors/thread_delete.rs"
    );
    assert_eq!(delete["byteIdenticalThroughAllHops"], true);
    assert!(
        delete["path"]
            .as_array()
            .unwrap()
            .iter()
            .any(|step| step.as_str() == Some("descendants-then-root delete_threads"))
    );
    assert_eq!(
        delete["prepareThreadForRemoval"],
        serde_json::json!([
            "remove_thread_for_client",
            "wait_for_thread_shutdown",
            "finalize_thread_teardown"
        ])
    );

    let archive = &traces["threadArchive"];
    assert_eq!(
        archive["file"],
        "codex-rs/app-server/src/request_processors/thread_processor.rs"
    );
    assert!(archive["path"].as_array().unwrap().iter().any(|step| {
        step.as_str()
            .unwrap()
            .contains("prepare_thread_for_archive")
    }));
    assert_eq!(
        traces["threadUnarchive"]["file"],
        "codex-rs/app-server/src/request_processors/thread_processor.rs"
    );

    let teardown = &traces["finalizeThreadTeardown"];
    assert_eq!(
        teardown["0.161.0"],
        serde_json::json!([
            "pending_unloads.remove",
            "cancel",
            "remove_thread_state/watch"
        ])
    );
    assert_eq!(
        teardown["0.164.0"],
        serde_json::json!([
            "cancel",
            "remove_thread_state/watch",
            "pending_unloads.remove"
        ])
    );
    assert_eq!(teardown["selectedSuccessPathPreserved"], true);
    assert_eq!(teardown["joinCancelPersistPreserved"], true);
    assert_eq!(
        teardown["classification"],
        "selected-lifecycle-internal-hardening"
    );

    let eviction = &traces["tryEvictV2Thread"];
    assert_eq!(eviction["path"], "idle-unload only");
    assert_eq!(eviction["onExplicitArchiveDeleteRpc"], false);
    assert_eq!(
        eviction["file"],
        "codex-rs/app-server/src/request_processors/thread_lifecycle.rs"
    );
}
