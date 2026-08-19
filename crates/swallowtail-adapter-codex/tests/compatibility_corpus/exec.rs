#[test]
fn exec_corpus_freezes_baseline_checkpoints_and_rejections() {
    let corpus = json(EXEC_RELEASES);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(
        strings(&corpus["candidate_versions"]),
        [
            "0.122.0", "0.130.0", "0.140.0", "0.144.6", "0.145.0", "0.146.0", "0.147.0", "0.148.0"
        ]
    );
    let required_argv = string_set(&corpus["required_argv"]);
    for required in [
        "exec",
        "--json",
        "--ephemeral",
        "--ignore-user-config",
        "--ignore-rules",
        "--skip-git-repo-check",
        "--sandbox",
        "read-only",
    ] {
        assert!(required_argv.contains(required), "missing {required}");
    }

    let releases = corpus["releases"]
        .as_array()
        .expect("release corpus is an array");
    for release in releases {
        assert_exact_evidence(release);
        assert_eq!(release["json"], true);
        assert_eq!(release["ephemeral"], true);
        match release["classification"]
            .as_str()
            .expect("classification is text")
        {
            "candidate" => {
                assert_eq!(release["ignore_user_config"], true);
                assert_eq!(release["ignore_rules"], true);
            }
            "below_baseline" => {
                assert_eq!(release["version"], "0.121.0");
                assert_eq!(release["ignore_user_config"], false);
                assert_eq!(release["ignore_rules"], false);
            }
            "prerelease" => assert_eq!(release["version"], "0.146.0-alpha.4"),
            other => panic!("unexpected classification {other}"),
        }
    }
    assert_eq!(strings(&corpus["synthetic_rejections"]), ["not-a-version"]);
    assert_unverified_newer(&corpus);
}

