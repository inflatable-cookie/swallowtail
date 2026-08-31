#[test]
fn thread_catalogue_corpus_freezes_the_complete_import_floor() {
    let corpus = json(APP_SERVER_THREAD_CATALOGUE);
    assert_eq!(corpus["claim_status"], "evidence-only");
    assert_eq!(corpus["qualified_range"]["baseline"], "0.80.0");
    assert_eq!(corpus["qualified_range"]["latest"], "0.151.0");
    assert_eq!(
        strings(&corpus["qualified_range"]["excluded_gaps"]),
        ["0.82.0", "0.83.0", "0.108.0", "0.109.0", "0.149.2", "0.150.2"]
    );

    let segments = corpus["segments"]
        .as_array()
        .expect("segments are an array");
    let complete: Vec<_> = segments
        .iter()
        .filter(|segment| segment["catalogue_import"] == true)
        .map(|segment| segment["range"].as_str().expect("range is text"))
        .collect();
    assert_eq!(
        complete,
        [
            "0.105.0..=0.107.0",
            "0.110.0..=0.128.0",
            "0.129.0..=0.130.0",
            "0.131.0..=0.151.0"
        ]
    );
    for legacy in &segments[..4] {
        assert_eq!(legacy["catalogue_import"], false);
        assert!(
            !legacy["missing"]
                .as_array()
                .expect("missing features are an array")
                .is_empty()
        );
    }

    assert_eq!(
        corpus["method_boundaries"]["thread.status"]["absent"],
        "0.104.0"
    );
    assert_eq!(
        corpus["method_boundaries"]["thread.status"]["present"],
        "0.105.0"
    );
    assert_unverified_newer(&corpus);
}

#[test]
fn thread_catalogue_corpus_keeps_selected_wire_shape_narrow() {
    let corpus = json(APP_SERVER_THREAD_CATALOGUE);
    let profile = &corpus["selected_profile"];
    assert_eq!(profile["list_request"]["method"], "thread/list");
    assert_eq!(profile["list_request"]["params"]["archived"], false);
    assert_eq!(
        strings(&profile["list_request"]["params"]["sourceKinds"]),
        ["cli", "vscode", "appServer"]
    );
    assert_eq!(profile["read_request"]["method"], "thread/read");
    assert_eq!(profile["read_request"]["params"]["includeTurns"], true);
    assert_eq!(profile["resume_request"]["method"], "thread/resume");

    let excluded_sources = string_set(&profile["excluded_sources"]);
    for source in ["exec", "subAgent", "unknown"] {
        assert!(excluded_sources.contains(source));
    }
    let candidate_fields = string_set(&profile["list_response"]["candidate_fields"]);
    for field in ["id", "preview", "updatedAt", "status", "cwd", "source"] {
        assert!(candidate_fields.contains(field));
    }
    for private in ["path", "gitInfo", "historyMetadata", "isPinned"] {
        assert!(
            string_set(&profile["excluded_fields"]).contains(private),
            "provider-private field {private} cannot become portable content"
        );
    }
}

#[test]
fn thread_reconciliation_corpus_separates_exact_and_session_scoped_truth() {
    let corpus = json(APP_SERVER_THREAD_RECONCILIATION);
    assert_eq!(corpus["minimum"], "0.105.0");
    assert_eq!(corpus["latest_qualified"], "0.151.0");
    assert_eq!(corpus["request"]["method"], "thread/read");
    assert_eq!(corpus["request"]["params"]["includeTurns"], true);
    assert_eq!(corpus["exact_turn"]["status"]["completed"], "completed");
    assert_eq!(corpus["exact_turn"]["missing"], "fail-closed");
    assert_eq!(corpus["session_scoped"]["terminal_states"], false);
    let forbidden = string_set(&corpus["forbidden_methods"]);
    for method in [
        "turn/start",
        "turn/interrupt",
        "thread/resume",
        "thread/delete",
    ] {
        assert!(forbidden.contains(method));
    }
}

#[test]
fn thread_catalogue_milestones_have_exact_release_and_source_identity() {
    let corpus = json(APP_SERVER_THREAD_CATALOGUE);
    let milestones = corpus["milestones"]
        .as_array()
        .expect("milestones are an array");
    assert_eq!(
        milestones
            .iter()
            .map(|milestone| milestone["version"].as_str().expect("version is text"))
            .collect::<Vec<_>>(),
        [
            "0.80.0", "0.91.0", "0.92.0", "0.103.0", "0.104.0", "0.105.0", "0.107.0", "0.110.0",
            "0.123.0", "0.129.0", "0.130.0", "0.131.0", "0.139.0", "0.144.6", "0.145.0", "0.146.0",
            "0.147.0", "0.148.0", "0.149.0", "0.149.1", "0.150.0", "0.150.1", "0.151.0"
        ]
    );

    for milestone in milestones {
        assert_eq!(
            milestone["tag_commit"]
                .as_str()
                .expect("tag commit is text")
                .len(),
            40
        );
        assert!(
            milestone["published_at"]
                .as_str()
                .expect("published time is text")
                .ends_with('Z')
        );
        assert_eq!(milestone["list"], true);
        if milestone["version"] == "0.80.0" {
            assert_eq!(milestone["read"], false);
            assert_eq!(milestone["history_read"], false);
        }
    }

    let first_complete = milestones
        .iter()
        .find(|milestone| milestone["version"] == "0.105.0")
        .expect("first complete milestone is frozen");
    for field in [
        "list",
        "read",
        "source_filter",
        "cwd_filter",
        "status",
        "history_read",
    ] {
        assert_eq!(
            first_complete[field], true,
            "missing complete feature {field}"
        );
    }
    assert_eq!(
        corpus["sources"]["current_documentation_role"],
        "corroboration-only"
    );
}

