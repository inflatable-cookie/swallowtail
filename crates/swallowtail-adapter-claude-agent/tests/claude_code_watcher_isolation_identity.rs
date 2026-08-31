use serde_json::Value;
use swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_AXIS;

const WATCHER_ISOLATION: &str = include_str!("fixtures/claude-code-2.1.251/watcher-isolation.json");

fn evidence() -> Value {
    serde_json::from_str(WATCHER_ISOLATION)
        .expect("Claude Code 2.1.251 watcher isolation corpus is valid JSON")
}

fn candidate<'a>(evidence: &'a Value, name: &str) -> &'a Value {
    evidence["candidates"]
        .as_array()
        .expect("candidates are an array")
        .iter()
        .find(|candidate| candidate["name"] == name)
        .unwrap_or_else(|| panic!("candidate {name} is compared"))
}

fn counterexample<'a>(evidence: &'a Value, name: &str) -> &'a Value {
    evidence["counterexamples"]
        .as_array()
        .expect("counterexamples are an array")
        .iter()
        .find(|counterexample| counterexample["name"] == name)
        .unwrap_or_else(|| panic!("counterexample {name} is exercised"))
}

#[test]
fn watcher_isolation_evidence_stays_prompt_free_on_the_frozen_help_corpus() {
    let evidence = evidence();

    assert_eq!(evidence["version"], "2.1.251");
    assert_eq!(evidence["axis"], CLAUDE_CODE_HEADLESS_AXIS);
    assert_eq!(evidence["source"]["help_command"], "claude --help");
    assert_eq!(
        evidence["source"]["host_help_sha256"],
        "5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d"
    );
    assert_eq!(
        evidence["source"]["host_help_matches_official_extracted"],
        true
    );
    assert_eq!(evidence["source"]["provider_prompt_sent"], false);
    assert_eq!(evidence["source"]["credentials_used"], false);
    assert_eq!(evidence["source"]["model_request_sent"], false);
    assert_eq!(evidence["live_turn_authorized"], false);
}

#[test]
fn frozen_help_text_carries_the_exact_authentication_and_isolation_clauses() {
    let evidence = evidence();
    let bare = evidence["help_text"]["--bare"]
        .as_str()
        .expect("--bare help is frozen");
    let restricted = evidence["help_text"]["--restricted"]
        .as_str()
        .expect("--restricted help is frozen");

    assert!(bare.contains("keychain reads"), "{bare}");
    assert!(bare.contains("OAuth and keychain are never read"), "{bare}");
    assert!(bare.contains("skip hooks"), "{bare}");
    assert!(
        restricted.contains("ignores user, project and local settings files"),
        "{restricted}"
    );
    assert!(
        restricted.contains("managed settings and --settings still apply"),
        "{restricted}"
    );
    assert!(
        restricted.contains("add --strict-mcp-config to skip MCP servers too"),
        "{restricted}"
    );
    assert!(
        restricted.contains("confines the file tools to the working directories"),
        "{restricted}"
    );
    assert!(
        !restricted.contains("keychain") && !restricted.contains("OAuth"),
        "restricted mode states no credential change: {restricted}"
    );
}

#[test]
fn only_the_restricted_candidate_preserves_authentication_and_the_private_composition() {
    let evidence = evidence();

    let bare = candidate(&evidence, "current-bare");
    assert_eq!(bare["preserves_configured_authentication"], false);
    assert_eq!(bare["disposition"], "rejected");

    let restricted = candidate(&evidence, "watcher-only-restricted");
    assert_eq!(restricted["preserves_configured_authentication"], true);
    assert_eq!(restricted["preserves_private_watcher_composition"], true);
    assert_eq!(restricted["excludes_ambient_setting_sources"], true);
    assert_eq!(restricted["disposition"], "selected");

    let empty_sources = candidate(&evidence, "empty-setting-sources");
    assert_eq!(empty_sources["disposition"], "rejected");

    let safe_mode = candidate(&evidence, "safe-mode");
    assert_eq!(safe_mode["preserves_private_watcher_composition"], false);
    assert_eq!(safe_mode["disposition"], "rejected");

    let selected = evidence["candidates"]
        .as_array()
        .expect("candidates are an array")
        .iter()
        .filter(|candidate| candidate["disposition"] == "selected")
        .count();
    assert_eq!(selected, 1);
}

#[test]
fn every_authority_delta_between_bare_and_restricted_is_explicit() {
    let evidence = evidence();
    let bare = &evidence["authority"]["--bare"];
    let restricted = &evidence["authority"]["--restricted"];

    assert_eq!(bare["configured_authentication"], "removed");
    assert_eq!(bare["oauth_and_keychain"], "never-read");
    assert_eq!(restricted["configured_authentication"], "preserved");
    assert_eq!(restricted["oauth_and_keychain"], "not-restricted");

    for mode in [bare, restricted] {
        assert_eq!(mode["ambient_settings"], "excluded");
        assert_eq!(mode["ambient_hooks"], "excluded");
        assert_eq!(mode["private_mcp_with_strict_mcp_config"], "admitted");
    }
    assert_eq!(restricted["explicit_settings_hooks"], "still-apply");
    assert_eq!(
        restricted["working_directory_confinement"],
        "file-tools-confined-to-working-and-add-dir"
    );
}

#[test]
fn the_prompt_free_parser_probe_separates_acceptance_from_rejection() {
    let evidence = evidence();
    let probes = evidence["parser_probe"]
        .as_array()
        .expect("parser probes are an array");
    let result = |variant: &str| {
        probes
            .iter()
            .find(|probe| probe["variant"] == variant)
            .unwrap_or_else(|| panic!("probe {variant} ran"))["result"]
            .as_str()
            .expect("every probe records a result")
            .to_owned()
    };

    assert_eq!(
        result("watcher-candidate-restricted"),
        "parse-accepted-print-input-validation"
    );
    assert_eq!(
        result("empty-setting-sources"),
        "parse-accepted-print-input-validation"
    );
    assert_eq!(
        result("unknown-option-negative-control"),
        "parse-rejected-unknown-option"
    );
    assert_eq!(
        result("invalid-setting-source-value"),
        "parse-rejected-invalid-setting-source"
    );
}

#[test]
fn the_selected_candidate_exhibits_no_review_oracle_counterexample() {
    let evidence = evidence();

    for name in [
        "ambient-setting-source-reopened",
        "every-configured-authentication-path-removed",
        "omission-or-normal-argv-changed",
    ] {
        let counterexample = counterexample(&evidence, name);
        assert_eq!(
            counterexample["selected_candidate_exhibits"], false,
            "{name}"
        );
        assert_eq!(counterexample["disposition"], "rejected", "{name}");
    }

    let selection = &evidence["selection"];
    assert_eq!(selection["selected"], "watcher-only-restricted");
    assert_eq!(selection["argv_removed"][0], "--bare");
    assert_eq!(selection["argv_added"][0], "--restricted");
    assert_eq!(selection["watcher_argv_token_delta"], 1);
    assert_eq!(selection["watcher_omission_argv_changed"], false);
    assert_eq!(selection["normal_non_watcher_argv_changed"], false);
    assert_eq!(selection["contract_059_060_changed"], false);
    assert_eq!(selection["watcher_version_claim_changed"], false);
}

#[test]
fn the_unclosed_isolation_residual_stays_recorded() {
    let evidence = evidence();
    let residual = evidence["residual"]
        .as_array()
        .expect("residuals are an array");
    assert!(!residual.is_empty());
    assert!(
        residual
            .iter()
            .any(|entry| entry.as_str().is_some_and(|entry| entry.contains("skills")))
    );
    assert!(residual.iter().any(|entry| {
        entry
            .as_str()
            .is_some_and(|entry| entry.contains("CLAUDE.md auto-discovery"))
    }));
}
