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
    assert_eq!(routes.len(), 13);

    let ids = routes
        .iter()
        .map(|route| route["id"].as_str().expect("route id is text"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        ids,
        BTreeSet::from([
            "anthropic.managed-agent",
            "claude-agent.acp",
            "claude-code.headless",
            "codex.app-server",
            "codex.exec",
            "gemini-cli.acp",
            "gemini-cli.headless",
            "kimi-code.acp",
            "kimi-code.headless",
            "kimi-code.local-server",
            "opencode.http",
            "pi.rpc",
            "qwen.headless",
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
        assert_eq!(route["unknown_posture"], "preserve-namespaced");
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

    assert_eq!(profiles.len(), 18);
    for expected in [
        ("claude-agent.acp", "structured-run"),
        ("claude-agent.acp", "interactive-session"),
        ("kimi-code.local-server", "structured-run"),
        ("kimi-code.local-server", "interactive-session"),
        ("opencode.http", "structured-run"),
        ("opencode.http", "interactive-session"),
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
            ("claude-agent.acp", "provider-session-management"),
            ("codex.app-server", "model-catalogue"),
            ("codex.app-server", "provider-session-management"),
            ("gemini-cli.headless", "provider-session-management"),
            ("kimi-code.local-server", "model-catalogue"),
            ("kimi-code.local-server", "provider-session-management"),
            ("opencode.http", "model-catalogue"),
            ("opencode.http", "provider-session-management"),
            ("pi.rpc", "model-catalogue"),
            ("qwen.headless", "model-catalogue"),
        ])
    );
}
