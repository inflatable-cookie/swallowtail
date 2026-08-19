use serde_json::Value;
use swallowtail_adapter_oh_my_pi::{OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, oh_my_pi_rpc_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const CORPUS: &str = include_str!("fixtures/oh-my-pi-rpc-17.2.9-exact/compatibility.json");

#[test]
fn exact_package_identity_and_release_evidence_are_frozen() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("valid compatibility corpus");
    assert_eq!(corpus["axis"], "oh-my-pi.package");
    assert_eq!(corpus["qualified_version"], "17.2.9");
    assert_eq!(corpus["package"], "@oh-my-pi/pi-coding-agent");
    assert_eq!(corpus["launcher_shebang"], "#!/usr/bin/env bun");
    assert_eq!(corpus["version_output"], "omp/17.2.9");
    assert_eq!(corpus["commit"].as_str().expect("commit is text").len(), 40);
    assert!(
        corpus["npm_integrity"]
            .as_str()
            .expect("integrity is text")
            .starts_with("sha512-")
    );
    assert_eq!(corpus["protocol_versions"], serde_json::json!([1, 2]));
    assert_eq!(
        corpus["session_lifecycle_events"],
        serde_json::json!(["model_changed", "thinking_level_changed"])
    );
}

#[test]
fn production_claim_permits_the_qualified_window() {
    assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "17.3.8");
    let claim = oh_my_pi_rpc_claim();
    assert!(claim.supports(&version("17.2.9")));
    assert!(claim.supports(&version("17.3.7")));
    assert!(claim.supports(&version("17.3.8")));
    assert!(!claim.permits(&version("17.2.8")));
    assert!(matches!(
        claim.assess(&version("17.3.9")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version("17.3.9-rc.1")));
}

#[test]
fn selected_surface_excludes_session_mutation_and_subagent_authority() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("valid compatibility corpus");
    let selected = corpus["selected_commands"]
        .as_array()
        .expect("selected commands are an array");
    for command in [
        "negotiate_protocol",
        "prompt",
        "get_state",
        "get_available_models",
        "set_model",
        "set_thinking_level",
    ] {
        assert!(selected.iter().any(|value| value == command));
    }
    for excluded in [
        "switch_session",
        "branch",
        "set_host_tools",
        "set_subagent_subscription",
    ] {
        assert!(!selected.iter().any(|value| value == excluded));
    }
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
