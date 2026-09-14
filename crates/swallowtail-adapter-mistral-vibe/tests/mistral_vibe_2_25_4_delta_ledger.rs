//! Identity and compatible-extension evidence for official Mistral Vibe
//! `2.25.4` (g05.073).
//!
//! This corpus freezes the exact `2.24.2` baseline plus all eight published
//! GitHub stable successors through `2.25.4`, retrieved as exact source
//! tarballs and PyPI distributions into `/tmp`, hashed against registry
//! digests, and statically inspected without executing a downloaded
//! artifact. The assertions are mutation-sensitive: they fail if the tag or
//! distribution identities, the packaging gap, the selected-surface
//! classification, or the harness-pin decision drifts.

use serde_json::Value;

const IDENTITY: &str = include_str!("fixtures/mistral-vibe-headless-2.25.4/identity.json");
const SURFACE: &str = include_str!("fixtures/mistral-vibe-headless-2.25.4/surface-ledger.json");

const BASELINE: &str = "2.24.2";
const OFFICIAL: &str = "2.25.4";
const STREAMING_PROGRAMMATIC: &str =
    "36a5008914136714a851880b3c6921f6ba196371e28422f58a45c8a728b25b34";
const TURN_LIMIT_MIDDLEWARE: &str =
    "ede809d72ced3328986a3a7c93d4793f66506467182e45f89168fd8ae8343a7a";
const PROGRAMMATIC_TESTS: &str = "42cb58a1210e398fd8e33991307d43822d8da32f4e74797017d276dd859337b4";

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn tags(ledger: &Value) -> Vec<(&'static str, &str)> {
    vec![
        ("v2.24.2", "5e6aa0f6beb3454454f4c1de74a7652ba577ab05"),
        ("v2.24.3", "a84be0391bf93e93a4025a5e08e8032ecb587123"),
        ("v2.24.4", "dcb1c7d44e3e59c94f1216f77190a02292212753"),
        ("v2.24.5", "50d99cf79b55ba92e43fa1ffdb0fc9c7bee83533"),
        ("v2.25.0", "6c79ef0e1ee484d7069bc38590d5917d3914cd48"),
        ("v2.25.1", "2817f3df81ae05d49ba9538262edb1d5a18fa006"),
        ("v2.25.2", "96836b551e36c987656625011e4ffa39806645b6"),
        ("v2.25.3", "b166c3a39eaac6ece96ca31840a3278c636616b5"),
        ("v2.25.4", "19b5b74faa78d0816b8d4d4c7d7543fc3520678c"),
    ]
    .into_iter()
    .map(|(tag, commit)| {
        assert_eq!(ledger["github_tags"][tag], commit, "{tag} identity drifted");
        (tag, commit)
    })
    .collect()
}

#[test]
fn identity_freezes_the_baseline_and_every_published_successor() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(identity["axis"], "mistral-vibe.release");
    assert_eq!(identity["route"], "mistral-vibe.headless");
    assert_eq!(identity["official"]["version"], OFFICIAL);
    assert_eq!(
        identity["official"]["github_commit"],
        "19b5b74faa78d0816b8d4d4c7d7543fc3520678c"
    );
    assert_eq!(
        identity["official"]["pypi_sdist_sha256"],
        "9e3ecadfa8d9be4a2693b19d5c2cf71ec914bc623649463b93b478be423f7fb8"
    );
    assert_eq!(
        identity["official"]["pypi_wheel_sha256"],
        "d43aa028e1931f3d05d735111384eadc937b4d47b7a05eb2ee04313fd860662f"
    );
    assert_eq!(
        identity["baseline_reproduction"]["pypi_sdist_sha256"],
        "be62b3148a9640ab2d72ab9849a40499d1680aa59589b01deb62c5eb08df269d"
    );
    assert_eq!(
        identity["baseline_reproduction"]["matches_research_150"],
        true
    );
    assert_eq!(identity["host"]["present"], false);

    let claim = &identity["claim_at_observation"];
    assert_eq!(claim["point"], BASELINE);
    assert_eq!(claim["posture"], "QualifiedOnly");
    assert_eq!(
        claim["behavior_revision"],
        "mistral-vibe.headless.stdio-streaming-v1"
    );
    assert_eq!(claim["claim_id"], "mistral-vibe.headless.release-window-1");

    let ledger = fixture(SURFACE, "surface-ledger");
    assert_eq!(tags(&ledger).len(), 9);
}

#[test]
fn pypi_gap_stays_named_and_never_qualified_by_adjacency() {
    let ledger = fixture(SURFACE, "surface-ledger");
    let distributions = &ledger["pypi_distributions"];
    assert_eq!(distributions.as_object().expect("points").len(), 9);
    assert_eq!(distributions["2.24.4"]["pypi"], "absent");
    assert_eq!(distributions["2.24.4"]["sdist_sha256"], Value::Null);
    assert_eq!(distributions["2.24.4"]["wheel_sha256"], Value::Null);
    for point in [
        "2.24.2", "2.24.3", "2.24.5", "2.25.0", "2.25.1", "2.25.2", "2.25.3", "2.25.4",
    ] {
        let sdist = distributions[point]["sdist_sha256"]
            .as_str()
            .unwrap_or_else(|| panic!("{point} sdist digest"));
        let wheel = distributions[point]["wheel_sha256"]
            .as_str()
            .unwrap_or_else(|| panic!("{point} wheel digest"));
        assert_eq!(sdist.len(), 64);
        assert_eq!(wheel.len(), 64);
    }

    let identity = fixture(IDENTITY, "identity");
    let gap = &identity["github_only_pypi_gap"];
    assert_eq!(gap["version"], "2.24.4");
    assert_eq!(gap["pypi"], "absent");
    assert_eq!(
        gap["disposition"],
        "named packaging gap; never qualified by adjacency"
    );
}

#[test]
fn selected_wire_and_lifecycle_surfaces_are_byte_stable_across_every_hop() {
    let ledger = fixture(SURFACE, "surface-ledger");
    assert_eq!(
        ledger["selected_surfaces_stable_at_every_hop"]["vibe/cli/programmatic.py"],
        STREAMING_PROGRAMMATIC
    );
    assert_eq!(
        ledger["selected_surfaces_stable_at_every_hop"]["vibe/core/middleware.py"],
        TURN_LIMIT_MIDDLEWARE
    );
    assert_eq!(
        ledger["selected_surfaces_stable_at_every_hop"]["tests/cli/test_programmatic.py"],
        PROGRAMMATIC_TESTS
    );
    assert_eq!(
        ledger["selected_surfaces_stable_at_every_hop"]["console_script"],
        "vibe = vibe.cli.entrypoint:main"
    );
    assert_eq!(
        ledger["selected_surfaces_stable_at_every_hop"]["public_turn_stop_reason"],
        "LIMIT-only unchanged"
    );
    assert_eq!(
        ledger["selected_surfaces_stable_at_every_hop"]["public_history_union"],
        "message|reasoning|effect|callback|checkpoint|notice discriminator unchanged"
    );

    let identity = fixture(IDENTITY, "identity");
    assert_eq!(
        identity["selected_source_digests_2_25_4"]["vibe/cli/programmatic.py"],
        STREAMING_PROGRAMMATIC
    );
    assert_eq!(
        identity["selected_source_digests_2_25_4"]["vibe/core/middleware.py"],
        TURN_LIMIT_MIDDLEWARE
    );
    assert_eq!(
        identity["selected_source_digests_2_25_4"]["tests/cli/test_programmatic.py"],
        PROGRAMMATIC_TESTS
    );
}

#[test]
fn every_changed_selected_input_hop_is_classified() {
    let ledger = fixture(SURFACE, "surface-ledger");
    let hops = ledger["per_hop_changed_selected_inputs"]
        .as_object()
        .expect("per-hop classification");
    let expected = [
        "2.24.2..2.24.3",
        "2.24.3..2.24.4",
        "2.24.4..2.24.5",
        "2.24.5..2.25.0",
        "2.25.0..2.25.1",
        "2.25.1..2.25.2",
        "2.25.2..2.25.3",
        "2.25.3..2.25.4",
    ];
    assert_eq!(
        hops.keys().map(String::as_str).collect::<Vec<_>>(),
        expected,
        "every published hop needs a classification entry"
    );
    for (hop, entry) in hops {
        assert!(
            !entry.as_object().expect(hop).is_empty(),
            "{hop} classification is empty"
        );
    }

    let invariants = &ledger["classification_invariants"];
    for name in [
        "argument_parsing",
        "plan_profile",
        "headless_callback_denial",
        "streaming_ndjson",
        "provider_limit_failure_mapping",
        "process_exit",
        "working_resource_authority",
        "deadline",
        "task_ownership",
        "session_close",
        "joined_cleanup",
        "authentication",
    ] {
        assert!(
            invariants[name].as_str().is_some(),
            "missing classification invariant {name}"
        );
    }
}

#[test]
fn harness_rollout_is_pinned_by_the_adapter_private_legacy_flag() {
    let ledger = fixture(SURFACE, "surface-ledger");
    let rollout = &ledger["harness_rollout_evidence"];
    assert_eq!(
        rollout["pypi_channel"],
        "mistralai_vibe_local_harness is absent from every official wheel and sdist, so experimental_harness_available() is false and the rollout branch can never fire"
    );
    let bundling = &rollout["darwin_aarch64_zip_bundles_native_harness"];
    for point in ["2.24.2", "2.24.3", "2.24.4", "2.24.5", "2.25.0"] {
        assert_eq!(
            bundling[point], false,
            "{point} must not bundle the internal harness"
        );
    }
    for point in ["2.25.1", "2.25.2", "2.25.3", "2.25.4"] {
        assert_eq!(
            bundling[point], true,
            "{point} bundles the internal harness"
        );
    }
    assert_eq!(
        rollout["resolution"],
        "the adapter-privately selected --legacy-harness flag has first-precedence in resolve_harness_selection and pins the legacy Python harness deterministically on both channels; the unified backend stays unmapped"
    );
    assert_eq!(
        rollout["native_backend_authority"],
        "the bundled backend is a native module whose tool-authority behavior is not statically classifiable; execution remains forbidden, so it is excluded on the selected route rather than mapped"
    );

    let identity = fixture(IDENTITY, "identity");
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-exact-point-rebind");
    assert_eq!(decision["move_exact_point_to_2_25_4"], true);
    assert_eq!(decision["behavior_revision_change"], "none");
    assert_eq!(decision["add_adapter_private_legacy_harness_pin"], true);
    assert_eq!(decision["map_experimental_unified_harness"], false);
    assert_eq!(decision["map_smart_approve"], false);
    assert_eq!(decision["pass_auto_approve_or_yolo"], false);
    assert_eq!(decision["second_exact_point_retained"], false);
    assert_eq!(decision["range_inferred"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
    assert_eq!(
        decision["entrypoint"],
        serde_json::json!([
            "vibe",
            "--prompt",
            "--output",
            "streaming",
            "--max-turns",
            "--trust",
            "--agent",
            "plan",
            "--workdir",
            "--legacy-harness"
        ])
    );
}
