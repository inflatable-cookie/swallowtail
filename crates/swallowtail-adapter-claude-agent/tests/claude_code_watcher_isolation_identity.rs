use serde_json::Value;
use swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_AXIS;

const WATCHER_ISOLATION: &str = include_str!("fixtures/claude-code-2.1.251/watcher-isolation.json");

fn evidence() -> Value {
    serde_json::from_str(WATCHER_ISOLATION)
        .expect("Claude Code 2.1.251 watcher isolation corpus is valid JSON")
}

fn candidates(evidence: &Value) -> &Vec<Value> {
    evidence["candidates"]
        .as_array()
        .expect("candidates are an array")
}

fn candidate<'a>(evidence: &'a Value, name: &str) -> &'a Value {
    candidates(evidence)
        .iter()
        .find(|candidate| candidate["name"] == name)
        .unwrap_or_else(|| panic!("candidate {name} is compared"))
}

/// Re-derive the card 029 verdict from the recorded authority instead of
/// trusting the recorded `disposition`. A candidate is admissible only when it
/// preserves configured authentication and the whole private composition and
/// excludes every named ambient axis.
fn admissible(candidate: &Value, axes: &[&str]) -> bool {
    let ambient = &candidate["ambient_authority"];
    let excludes_ambient = axes
        .iter()
        .all(|axis| ambient[*axis].as_str() == Some("excluded"));
    let preserves_composition = ["private_mcp", "stop_hook", "injected_skill"]
        .iter()
        .all(|part| candidate["private_composition"][*part].as_str() == Some("preserved"));
    excludes_ambient
        && preserves_composition
        && candidate["configured_authentication"].as_str() == Some("preserved")
}

fn named_axes(evidence: &Value) -> Vec<&str> {
    evidence["named_ambient_authority"]
        .as_array()
        .expect("the invariant names its ambient axes")
        .iter()
        .map(|axis| axis.as_str().expect("every axis is a name"))
        .collect()
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
    let clause = |flag: &str| {
        evidence["help_text"][flag]
            .as_str()
            .unwrap_or_else(|| panic!("{flag} help is frozen"))
            .to_owned()
    };
    let bare = clause("--bare");
    let restricted = clause("--restricted");
    let safe_mode = clause("--safe-mode");

    assert!(bare.contains("OAuth and keychain are never read"), "{bare}");
    assert!(bare.contains("skip hooks"), "{bare}");
    assert!(
        bare.contains("Skills still resolve via /skill-name"),
        "{bare}"
    );
    assert!(
        restricted.contains("ignores user, project and local settings files"),
        "{restricted}"
    );
    assert!(
        restricted.contains("managed settings and --settings still apply"),
        "{restricted}"
    );
    assert!(
        !restricted.contains("skills") && !restricted.contains("CLAUDE.md"),
        "restricted mode excludes no ambient skill or memory authority: {restricted}"
    );
    assert!(safe_mode.contains("skills"), "{safe_mode}");
    assert!(
        safe_mode.contains("Auth, model selection, built-in tools, and permissions work normally"),
        "{safe_mode}"
    );
}

#[test]
fn every_recorded_disposition_matches_the_re_derived_card_029_verdict() {
    let evidence = evidence();
    let axes = named_axes(&evidence);
    assert!(
        axes.contains(&"skills"),
        "the invariant names ambient skills"
    );
    assert!(axes.contains(&"memory_claude_md"));

    for candidate in candidates(&evidence) {
        let name = candidate["name"].as_str().expect("candidates are named");
        let expected = if admissible(candidate, &axes) {
            "selected"
        } else {
            "rejected"
        };
        assert_eq!(candidate["disposition"], expected, "{name}");
    }
}

#[test]
fn the_lane_stops_with_no_selected_candidate_and_no_production_command_change() {
    let evidence = evidence();
    let axes = named_axes(&evidence);

    assert_eq!(
        candidates(&evidence)
            .iter()
            .filter(|candidate| admissible(candidate, &axes))
            .count(),
        0
    );
    let outcome = &evidence["outcome"];
    assert_eq!(outcome["kind"], "evidence-stop");
    assert_eq!(outcome["selected"], Value::Null);
    assert_eq!(outcome["production_command_changed"], false);
    assert_eq!(outcome["watcher_argv_unchanged"], true);
    assert_eq!(outcome["omission_argv_unchanged"], true);
    assert_eq!(outcome["normal_non_watcher_argv_unchanged"], true);
    assert_eq!(outcome["contract_059_060_changed"], false);
    assert_eq!(outcome["watcher_version_claim_changed"], false);
    assert_eq!(outcome["invariant_weakened"], false);
    assert!(
        evidence["stop_reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("skill"))
    );
}

#[test]
fn each_candidate_records_the_exact_axis_or_part_that_rejects_it() {
    let evidence = evidence();

    let bare = candidate(&evidence, "current-bare");
    assert_eq!(bare["configured_authentication"], "removed");
    assert_eq!(bare["ambient_authority"]["skills"], "admitted");
    assert_eq!(bare["private_composition"]["stop_hook"], "unstated");

    let restricted = candidate(&evidence, "watcher-only-restricted");
    assert_eq!(restricted["configured_authentication"], "preserved");
    assert_eq!(restricted["preserves_private_watcher_composition"], true);
    assert_eq!(restricted["ambient_authority"]["settings"], "excluded");
    for reopened in ["skills", "memory_claude_md", "plugins"] {
        assert_eq!(
            restricted["ambient_authority"][reopened], "admitted",
            "{reopened}"
        );
    }
    assert_eq!(restricted["excludes_all_named_ambient_authority"], false);

    let safe_mode = candidate(&evidence, "safe-mode");
    assert_eq!(safe_mode["excludes_all_named_ambient_authority"], true);
    assert_eq!(safe_mode["preserves_private_watcher_composition"], false);
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
        result("unknown-option-negative-control"),
        "parse-rejected-unknown-option"
    );
    assert_eq!(
        result("invalid-setting-source-value"),
        "parse-rejected-invalid-setting-source"
    );
}

#[test]
fn every_review_oracle_counterexample_names_the_candidates_that_exhibit_it() {
    let evidence = evidence();
    let counterexamples = evidence["counterexamples"]
        .as_array()
        .expect("counterexamples are an array");
    let exhibited_by = |name: &str| {
        counterexamples
            .iter()
            .find(|counterexample| counterexample["name"] == name)
            .unwrap_or_else(|| panic!("counterexample {name} is exercised"))["exhibited_by"]
            .as_array()
            .expect("every counterexample lists its candidates")
            .iter()
            .map(|candidate| candidate.as_str().expect("named candidate").to_owned())
            .collect::<Vec<_>>()
    };

    assert!(
        exhibited_by("ambient-authority-reopened").contains(&"watcher-only-restricted".to_owned())
    );
    assert!(
        exhibited_by("every-configured-authentication-path-removed")
            .contains(&"current-bare".to_owned())
    );
    assert!(exhibited_by("private-watcher-composition-lost").contains(&"safe-mode".to_owned()));
    assert!(exhibited_by("omission-or-normal-argv-changed").is_empty());
}

#[test]
fn the_unclosed_isolation_residual_stays_recorded() {
    let evidence = evidence();
    let residual = evidence["residual"]
        .as_array()
        .expect("residuals are an array");
    let mentions = |needle: &str| {
        residual
            .iter()
            .any(|entry| entry.as_str().is_some_and(|entry| entry.contains(needle)))
    };

    assert!(
        mentions("--bare"),
        "the blocked production command stays named"
    );
    assert!(mentions("skills"));
    assert!(mentions("--safe-mode"));
}
