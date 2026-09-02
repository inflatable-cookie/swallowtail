use serde_json::Value;
use std::collections::BTreeSet;

const INVENTORY: &str = include_str!("fixtures/provider-wide-harness-activity.json");

#[test]
fn every_production_harness_route_has_exact_prepared_activity_truth() {
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory is JSON");
    assert_eq!(
        inventory["contract"],
        "044-observable-agent-activity-and-disclosure"
    );
    let routes = inventory["routes"].as_array().expect("routes are an array");
    assert_eq!(routes.len(), 36);

    let ids = routes
        .iter()
        .map(|route| route["id"].as_str().expect("route id is text"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        ids,
        BTreeSet::from([
            "antigravity.catalogue",
            "antigravity.headless",
            "anthropic.managed-agent",
            "claude-agent.acp",
            "claude-agent.sdk",
            "claude-code.headless",
            "claude-code.response-only",
            "cline.acp",
            "cline.headless",
            "codex.app-server",
            "codex.exec",
            "command-code.headless",
            "copilot-cli.acp",
            "cursor-agent.acp",
            "cursor-agent.catalogue",
            "cursor-agent.headless",
            "deepagents.acp",
            "deepseek-harness.jsonrpc",
            "deepseek-harness.local-server",
            "gemini-cli.acp",
            "gemini-cli.headless",
            "goose.acp",
            "grok-build.acp",
            "kimi-code.acp",
            "kimi-code.headless",
            "kimi-code.local-server",
            "kiro.acp",
            "mistral-vibe.headless",
            "muse-code.headless",
            "oh-my-pi.rpc",
            "opencode.http",
            "pi.rpc",
            "pi.sdk-sidecar",
            "qoder.headless",
            "qwen.headless",
            "zcode.app-server",
        ])
    );

    let mut profiles = BTreeSet::new();
    for route in routes {
        for field in ["driver", "transport", "qualified", "unknown_posture"] {
            assert!(
                route[field].as_str().is_some_and(|value| !value.is_empty()),
                "{} is missing {field}",
                route["id"]
            );
        }
        let expected_unknown_posture = if matches!(
            route["id"].as_str(),
            Some(
                "claude-agent.sdk"
                    | "claude-code.response-only"
                    | "qoder.headless"
                    | "pi.sdk-sidecar"
            )
        ) {
            "fail-closed"
        } else {
            "preserve-namespaced"
        };
        assert_eq!(route["unknown_posture"], expected_unknown_posture);
        assert!(
            route["unavailable_operations"]
                .as_array()
                .expect("unavailable operations are an array")
                .is_empty(),
            "{} retains an unexplained unavailable ordinary operation",
            route["id"]
        );
        assert!(
            !route["exact_absences"]
                .as_array()
                .expect("exact absences are an array")
                .is_empty(),
            "{} omitted its exact activity gaps",
            route["id"]
        );
        for profile in route["prepared_profiles"]
            .as_array()
            .expect("prepared profiles are an array")
        {
            assert_eq!(profile["availability"], "available");
            let key = (
                route["id"].as_str().expect("route id is text"),
                profile["operation"]
                    .as_str()
                    .expect("operation shape is text"),
            );
            assert!(profiles.insert(key), "duplicate prepared profile {key:?}");
        }
    }

    assert_eq!(profiles.len(), 43);
    for expected in [
        ("antigravity.headless", "structured-run"),
        ("antigravity.headless", "interactive-session"),
        ("claude-agent.acp", "structured-run"),
        ("claude-agent.acp", "interactive-session"),
        ("command-code.headless", "structured-run"),
        ("command-code.headless", "interactive-session"),
        ("grok-build.acp", "structured-run"),
        ("grok-build.acp", "interactive-session"),
        ("kimi-code.local-server", "structured-run"),
        ("kimi-code.local-server", "interactive-session"),
        ("opencode.http", "structured-run"),
        ("opencode.http", "interactive-session"),
        ("oh-my-pi.rpc", "structured-run"),
        ("oh-my-pi.rpc", "interactive-session"),
        ("pi.rpc", "structured-run"),
        ("pi.rpc", "interactive-session"),
        ("qwen.headless", "structured-run"),
        ("qwen.headless", "interactive-session"),
    ] {
        assert!(
            profiles.contains(&expected),
            "missing dual-role profile {expected:?}"
        );
    }
}

#[test]
fn catalogue_and_management_roles_are_not_misreported_as_ordinary_activity() {
    let inventory: Value = serde_json::from_str(INVENTORY).expect("inventory is JSON");
    let routes = inventory["routes"].as_array().expect("routes are an array");
    let classified = routes
        .iter()
        .flat_map(|route| {
            route["not_applicable_operations"]
                .as_array()
                .expect("not-applicable operations are an array")
                .iter()
                .map(move |operation| {
                    (
                        route["id"].as_str().expect("route id is text"),
                        operation.as_str().expect("operation is text"),
                    )
                })
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(
        classified,
        BTreeSet::from([
            ("antigravity.catalogue", "model-catalogue"),
            ("claude-agent.acp", "provider-session-management"),
            ("codex.app-server", "model-catalogue"),
            ("codex.app-server", "provider-session-management"),
            ("cursor-agent.catalogue", "model-catalogue"),
            ("deepseek-harness.local-server", "interactive-session"),
            ("deepseek-harness.local-server", "model-catalogue"),
            (
                "deepseek-harness.local-server",
                "provider-session-management",
            ),
            ("gemini-cli.headless", "provider-session-management"),
            ("kimi-code.local-server", "model-catalogue"),
            ("kimi-code.local-server", "provider-session-management"),
            ("opencode.http", "model-catalogue"),
            ("opencode.http", "provider-session-management"),
            ("oh-my-pi.rpc", "model-catalogue"),
            ("pi.rpc", "model-catalogue"),
            ("pi.sdk-sidecar", "model-catalogue"),
            ("qwen.headless", "model-catalogue"),
        ])
    );
}
