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
    assert_eq!(deltas.len(), 13);
    let mut saw_thread_processor = false;
    let mut saw_message_processor = false;
    let mut saw_bwrap = false;
    let mut saw_linux_run_main = false;
    for delta in deltas {
        let class = delta["class"].as_str().expect("class");
        assert!(
            matches!(
                class,
                "unselected"
                    | "additive-optional"
                    | "selected-lifecycle-internal-hardening"
                    | "bounded-new-dispatch"
                    | "selected-linux-sandbox-mount-protection-preserved"
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
        if path == "codex-rs/app-server/src/message_processor.rs" {
            saw_message_processor = true;
            assert_eq!(class, "bounded-new-dispatch");
            assert!(
                delta["reason"]
                    .as_str()
                    .unwrap()
                    .contains("Every selected method arm remains")
            );
        }
        if path == "codex-rs/linux-sandbox/src/bwrap.rs" {
            saw_bwrap = true;
            assert_eq!(class, "selected-linux-sandbox-mount-protection-preserved");
            assert!(
                delta["reason"]
                    .as_str()
                    .unwrap()
                    .contains("create-monitor was never armed")
            );
        }
        if path == "codex-rs/linux-sandbox/src/linux_run_main.rs" {
            saw_linux_run_main = true;
            assert_eq!(class, "selected-linux-sandbox-mount-protection-preserved");
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
    assert!(
        saw_message_processor,
        "message_processor.rs must be classified on the dispatch path"
    );
    assert!(saw_bwrap, "linux-sandbox bwrap.rs must be classified");
    assert!(
        saw_linux_run_main,
        "linux-sandbox linux_run_main.rs must be classified"
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
fn per_tag_source_digests_bind_fetched_hops() {
    let sha = &json(PROTOCOL)["contentSha256"];
    let message_processor = &sha["codex-rs/app-server/src/message_processor.rs"];
    assert_eq!(
        message_processor["0.161.0"],
        "f9cf285fc6a3c7037510f507e225e80305c21eefd0d85267d44075e7f7ba5511"
    );
    assert_eq!(
        message_processor["0.162.0"],
        "5e17343f7c7129dc09a77300771dcd93119192c5cb39a3f4d1e29a547fec34ed"
    );
    assert_eq!(
        message_processor["0.162.1"],
        "5e17343f7c7129dc09a77300771dcd93119192c5cb39a3f4d1e29a547fec34ed"
    );
    assert_eq!(
        message_processor["0.163.0"],
        "96d57b0cb554802211460637993c2e57e27440b1052ca5f12ac79e1bbaf54449"
    );
    assert_eq!(
        message_processor["0.164.0"],
        "e8e5fcccc04e699c0f9394d4586e25e9560b3f1a4a1ee066692adf93139443c2"
    );

    let model = &sha["codex-rs/app-server-protocol/src/protocol/v2/model.rs"];
    assert_eq!(
        model["0.161.0"],
        "1cdd2d3e0d4840dd58974d3960485b6f77e649b226e0656d84890b29a92c1a81"
    );
    assert_eq!(
        model["0.162.0"],
        "a25e1cf1aaf5350b593ebc0ecc811e4f24fa5da3f73abb29ce380336bff599d9"
    );
    assert_eq!(
        model["0.162.1"],
        "a25e1cf1aaf5350b593ebc0ecc811e4f24fa5da3f73abb29ce380336bff599d9"
    );
    assert_eq!(
        model["0.163.0"],
        "a25e1cf1aaf5350b593ebc0ecc811e4f24fa5da3f73abb29ce380336bff599d9"
    );
    assert_eq!(
        model["0.164.0"],
        "a25e1cf1aaf5350b593ebc0ecc811e4f24fa5da3f73abb29ce380336bff599d9"
    );

    let bwrap = &sha["codex-rs/linux-sandbox/src/bwrap.rs"];
    assert_eq!(
        bwrap["0.161.0"],
        "1d3dfe853f9b5f1cafd7283cf760e38ff739eb7c1ac54650ae2ed510b92b870a"
    );
    assert_eq!(
        bwrap["0.163.0"],
        "4249718b4b50fa385ce27c539961d68c46a59f63fce4428a6380e43aa02c7b89"
    );
    assert_eq!(
        bwrap["0.164.0"],
        "69a23ea16d525de5938c1fd47561c90d6047eb58f9864ca47749d696e5d9fa76"
    );
    assert_ne!(bwrap["0.163.0"], bwrap["0.164.0"]);
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

    let linux = &reach["linuxSandboxHelper"];
    assert_eq!(linux["bwrapFile"], "codex-rs/linux-sandbox/src/bwrap.rs");
    assert_eq!(
        linux["runMainFile"],
        "codex-rs/linux-sandbox/src/linux_run_main.rs"
    );
    assert_eq!(
        linux["protectedCreateMonitorArmedOnSelectedExecReadOnly"],
        false
    );
    assert_eq!(
        linux["protectedCreateMonitorUsedOnSelectedMissingMetadataAt0_163_0"],
        false
    );
    assert_eq!(linux["protectedCreateMonitorRemovedAt"], "0.164.0");
    assert_eq!(
        linux["protectedMetadataNames"],
        serde_json::json!([".git", ".agents", ".codex", ".aws"])
    );
    assert_eq!(linux["awsProtectionRemains"], true);
    assert!(
        linux["selectedExecReadOnlyWritableRoots"]
            .as_str()
            .unwrap()
            .contains("none on the workspace")
    );
    assert!(
        linux["missingMetadataEnforcement"]
            .as_str()
            .unwrap()
            .contains("synthetic empty RO tmpfs")
    );
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

    let dispatch = &traces["messageProcessorDispatch"];
    assert_eq!(
        dispatch["file"],
        "codex-rs/app-server/src/message_processor.rs"
    );
    assert_eq!(
        dispatch["archiveDeleteUnarchiveStillCall"],
        "thread_processor"
    );
    for method in [
        "Initialize",
        "ConfigRead",
        "ModelList",
        "ThreadStart",
        "TurnStart",
        "ThreadArchive",
        "ThreadDelete",
        "ThreadUnarchive",
    ] {
        assert!(
            dispatch["selectedMethodArmsRemain"]
                .as_array()
                .unwrap()
                .iter()
                .any(|arm| arm.as_str() == Some(method)),
            "{method}"
        );
    }
    assert_eq!(
        dispatch["newUnmappedArms"]["0.162.0"],
        serde_json::json!(["ThreadAttachmentOwnerList"])
    );
    assert_eq!(
        dispatch["newUnmappedArms"]["0.163.0"],
        serde_json::json!(["ThreadReadStateUpdate", "ThreadItemsRead"])
    );
    assert_eq!(
        dispatch["newUnmappedArms"]["0.164.0"],
        serde_json::json!(["ThreadSample", "ThreadSampleCancel", "FeedbackSave"])
    );
}
