//! Exact selected-channel identity and delta evidence for Deep Agents ACP.

use serde_json::{Value, json};

const IDENTITY: &str = include_str!("fixtures/deepagents-acp-0.1.34/identity.json");
const PROTOCOL: &str = include_str!("fixtures/deepagents-acp-0.1.34/protocol.json");
const INVENTORY: &str = include_str!("fixtures/deepagents-acp-0.1.34/dist-inventory.json");

const ACP_HOPS: [(&str, &str, &str, &str, &str); 4] = [
    (
        "0.1.31",
        "sha512-vpLuclWPLHll3xlXVMmdB63CMcMyQXmg4rlaQ4SlQNVHm8YSZBsxgv+pnIwNmu52U7QwYOkbyYPyvK8ucrtz1w==",
        "89010a8649e6de9b6c9e594206e44a9afb2e732f2e4027d9371cc84c01fe3bb3",
        "1e529af89b560268f5986a0665dd328be7989125b7d9f480033a0f0de87a3da6",
        "1.13.5",
    ),
    (
        "0.1.32",
        "sha512-Kimyrw1vqVdkHAPljOQmQGOcayqX51mdFSnXiFlKFKBDAls5GWdmBrdoWiaBzP1nkT0iF4Vt7u3KlzRbctTp4g==",
        "742d82e3306bfe283d48683bf52430abba94a2e35bb60ba045357c6a9152266a",
        "b43b1a3ecf50741b57cfbe1c17eed8f001f11413ec93e1d86c135fd89b321887",
        "1.14.0",
    ),
    (
        "0.1.33",
        "sha512-PnZe+8ei2h/cmEGSc0i4Sm1SDwdlJ3bO53TIf2Z/KqermIm8k/cER0RC0sL47SHSfp9BlcmNGOw1VYlZy2qUOQ==",
        "a00e7095a7ea9af9c915b85be3771afda9c3ba55e051e5d11e3a9dbb09a28529",
        "ce90c3da1e020d4182fadf9583edf2a7aa3c8f82a11d122b390ded7a08ffa19a",
        "1.14.1",
    ),
    (
        "0.1.34",
        "sha512-2c8wt/Na9AjiTBINRRfDXSR8sdsFEpd1ser+dJdIzZu7VvvLQdNQTLzjgTwVym7DsgDCkMjN/AEAfIJiqrr3uA==",
        "7b5faf49d58b6afede606cb45d5da70569ee85b671bf7e336eac7e79ec60963f",
        "08d311c1903e2baf42558980ddec5292f6c7abcd08fb2c2e8f4bb21636c7543e",
        "1.14.2",
    ),
];

const RUNTIME_POINTS: [(&str, &str, &str); 5] = [
    (
        "1.13.4",
        "a00d90a3041794cc4f80186b0eaf519f4fdbccea16dc8576771aeb0d8b3c800c",
        "03d74e6e1c03414ada4c27a942bdb9e94a769cbc699911921b8ca2e1f37c0d6f",
    ),
    (
        "1.13.5",
        "811a6c07fe0b5e678883b66acc51672244fd7fb9c2b89e5563fc8ebc3b2f0bba",
        "474c1922e07c4c9089bcb8aea9fd744bf7053d0d7b75410cf29c0d825e81d411",
    ),
    (
        "1.14.0",
        "34469f3b04e3ba22dcbf140d06e3ce77789394e6fc702d43d80fe3e2cb70834b",
        "6a1a5ea3a0bc60baf809f26dcdbc24da9e38f526678fa331e8987431d79ae144",
    ),
    (
        "1.14.1",
        "6f2fc692a6f9919a77d8f387a9143cbb0a1208d0b6ccc9aca0e1abab2eb8a7b5",
        "0e2b52a868c6df5f32b011b41385c0a45273fdbbc2a97210acb38564318c291c",
    ),
    (
        "1.14.2",
        "305b138366827025452f752af98c6f2f6cc9e5bdfa14aa1c371222c00cac71c4",
        "ef2801c3cdb9e7768fecbfef802477f65fcca5ec57c1ac248ad87d3ee7029447",
    ),
];

const RUNTIME_BASELINE_FILES: [&str; 25] = [
    "dist/agent-CVBXKZAu.d.ts",
    "dist/agent-D7nWARfg.d.cts",
    "dist/browser.cjs",
    "dist/browser.d.cts",
    "dist/browser.d.ts",
    "dist/browser.js",
    "dist/index.cjs",
    "dist/index.d.cts",
    "dist/index.d.ts",
    "dist/index.js",
    "dist/langsmith-BAO_h4J6.js",
    "dist/langsmith-BAO_h4J6.js.map",
    "dist/langsmith-yA3yhBRa.cjs",
    "dist/langsmith-yA3yhBRa.cjs.map",
    "dist/node.cjs",
    "dist/node.d.cts",
    "dist/node.d.ts",
    "dist/node.js",
    "dist/src-CcY34Ckx.js",
    "dist/src-CcY34Ckx.js.map",
    "dist/src-D_XxmEak.cjs",
    "dist/src-D_XxmEak.cjs.map",
    "LICENSE",
    "package.json",
    "README.md",
];

const RUNTIME_HOPS: [(&str, &[&str], &[&str], &[&str]); 4] = [
    (
        "1.13.4..1.13.5",
        &[
            "dist/langsmith-CG8px69t.cjs",
            "dist/langsmith-CG8px69t.cjs.map",
            "dist/langsmith-CY7MawjU.js",
            "dist/langsmith-CY7MawjU.js.map",
            "dist/src-Bos_UCsK.js",
            "dist/src-Bos_UCsK.js.map",
            "dist/src-C2ZohcHT.cjs",
            "dist/src-C2ZohcHT.cjs.map",
        ],
        &[
            "dist/langsmith-BAO_h4J6.js",
            "dist/langsmith-BAO_h4J6.js.map",
            "dist/langsmith-yA3yhBRa.cjs",
            "dist/langsmith-yA3yhBRa.cjs.map",
            "dist/src-CcY34Ckx.js",
            "dist/src-CcY34Ckx.js.map",
            "dist/src-D_XxmEak.cjs",
            "dist/src-D_XxmEak.cjs.map",
        ],
        &[
            "dist/browser.cjs",
            "dist/browser.js",
            "dist/index.cjs",
            "dist/index.js",
            "dist/node.cjs",
            "dist/node.js",
            "package.json",
        ],
    ),
    (
        "1.13.5..1.14.0",
        &[
            "dist/agent-CrQgOsbl.d.cts",
            "dist/agent-DGkf2K7U.d.ts",
            "dist/langsmith-3LzYb-m7.js",
            "dist/langsmith-3LzYb-m7.js.map",
            "dist/langsmith-C9zNPKIz.cjs",
            "dist/langsmith-C9zNPKIz.cjs.map",
            "dist/src-Nhlq1O0Y.cjs",
            "dist/src-Nhlq1O0Y.cjs.map",
            "dist/src-gIoHrhh3.js",
            "dist/src-gIoHrhh3.js.map",
        ],
        &[
            "dist/agent-CVBXKZAu.d.ts",
            "dist/agent-D7nWARfg.d.cts",
            "dist/langsmith-CG8px69t.cjs",
            "dist/langsmith-CG8px69t.cjs.map",
            "dist/langsmith-CY7MawjU.js",
            "dist/langsmith-CY7MawjU.js.map",
            "dist/src-Bos_UCsK.js",
            "dist/src-Bos_UCsK.js.map",
            "dist/src-C2ZohcHT.cjs",
            "dist/src-C2ZohcHT.cjs.map",
        ],
        &[
            "dist/browser.cjs",
            "dist/browser.d.cts",
            "dist/browser.d.ts",
            "dist/browser.js",
            "dist/index.cjs",
            "dist/index.d.cts",
            "dist/index.d.ts",
            "dist/index.js",
            "dist/node.cjs",
            "dist/node.d.cts",
            "dist/node.d.ts",
            "dist/node.js",
            "package.json",
        ],
    ),
    (
        "1.14.0..1.14.1",
        &[
            "dist/agent-B05mw_iK.d.cts",
            "dist/agent-CxEdojMv.d.ts",
            "dist/langsmith-Bgi_Ta-k.js",
            "dist/langsmith-Bgi_Ta-k.js.map",
            "dist/langsmith-Bhg_Q4ZO.cjs",
            "dist/langsmith-Bhg_Q4ZO.cjs.map",
            "dist/src-BGwHTd6z.js",
            "dist/src-BGwHTd6z.js.map",
            "dist/src-CB_Eac87.cjs",
            "dist/src-CB_Eac87.cjs.map",
        ],
        &[
            "dist/agent-CrQgOsbl.d.cts",
            "dist/agent-DGkf2K7U.d.ts",
            "dist/langsmith-3LzYb-m7.js",
            "dist/langsmith-3LzYb-m7.js.map",
            "dist/langsmith-C9zNPKIz.cjs",
            "dist/langsmith-C9zNPKIz.cjs.map",
            "dist/src-Nhlq1O0Y.cjs",
            "dist/src-Nhlq1O0Y.cjs.map",
            "dist/src-gIoHrhh3.js",
            "dist/src-gIoHrhh3.js.map",
        ],
        &[
            "dist/browser.cjs",
            "dist/browser.d.cts",
            "dist/browser.d.ts",
            "dist/browser.js",
            "dist/index.cjs",
            "dist/index.d.cts",
            "dist/index.d.ts",
            "dist/index.js",
            "dist/node.cjs",
            "dist/node.d.cts",
            "dist/node.d.ts",
            "dist/node.js",
            "package.json",
        ],
    ),
    (
        "1.14.1..1.14.2",
        &[
            "dist/agent-3XynVB9Z.d.cts",
            "dist/agent-BKShWAoi.d.ts",
            "dist/langsmith--RbtjNdQ.js",
            "dist/langsmith--RbtjNdQ.js.map",
            "dist/langsmith-CjOWZAS2.cjs",
            "dist/langsmith-CjOWZAS2.cjs.map",
            "dist/src-BuYhtaIW.cjs",
            "dist/src-BuYhtaIW.cjs.map",
            "dist/src-YqBMARBL.js",
            "dist/src-YqBMARBL.js.map",
        ],
        &[
            "dist/agent-B05mw_iK.d.cts",
            "dist/agent-CxEdojMv.d.ts",
            "dist/langsmith-Bgi_Ta-k.js",
            "dist/langsmith-Bgi_Ta-k.js.map",
            "dist/langsmith-Bhg_Q4ZO.cjs",
            "dist/langsmith-Bhg_Q4ZO.cjs.map",
            "dist/src-BGwHTd6z.js",
            "dist/src-BGwHTd6z.js.map",
            "dist/src-CB_Eac87.cjs",
            "dist/src-CB_Eac87.cjs.map",
        ],
        &[
            "dist/browser.cjs",
            "dist/browser.d.cts",
            "dist/browser.d.ts",
            "dist/browser.js",
            "dist/index.cjs",
            "dist/index.d.cts",
            "dist/index.d.ts",
            "dist/index.js",
            "dist/node.cjs",
            "dist/node.d.cts",
            "dist/node.d.ts",
            "dist/node.js",
            "package.json",
        ],
    ),
];

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn strings(value: &Value, context: &str) -> Vec<String> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{context} must be an array"))
        .iter()
        .map(|item| {
            item.as_str()
                .unwrap_or_else(|| panic!("{context} entries must be text"))
                .to_owned()
        })
        .collect()
}

#[test]
fn identity_freezes_every_stable_hop_and_exact_source_artifact() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(identity["axis"], "deepagents-acp.package");
    assert_eq!(identity["route"], "deepagents.acp");
    assert_eq!(identity["official_channel"]["name"], "npm latest");
    assert_eq!(identity["official_channel"]["latest"], "0.1.34");
    assert_eq!(identity["official"]["version"], "0.1.34");
    assert_eq!(
        identity["official"]["published"],
        "2026-10-05T14:30:28.049Z"
    );
    assert_eq!(identity["official"]["gitHead"], Value::Null);
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(
        identity["first_unpublished_after_official"]["version"],
        "0.1.35"
    );
    assert_eq!(
        identity["first_unpublished_after_official"]["published"],
        false
    );
    assert_eq!(
        identity["first_unpublished_after_official"]["claim_status"],
        "incompatible under QualifiedOnly"
    );

    let previous = &identity["qualified_points_in_identity"];
    assert_eq!(previous["baseline"], "0.1.30");
    assert_eq!(previous["ceiling_at_observation"], "0.1.30");
    assert_eq!(previous["claim_id"], "deepagents.acp.package-window-1");
    assert_eq!(previous["behavior_revision"], "deepagents.acp.stdio-v1");
    assert_eq!(previous["posture"], "QualifiedOnly");
    assert_eq!(
        previous["segments"],
        json!([{"kind":"exact","version":"0.1.30","support_status":"Maintained"}])
    );
    assert_eq!(previous["excluded_versions"], json!([]));

    let hops = identity["published_stable_points_after_previous_ceiling"]
        .as_array()
        .expect("published stable hops");
    assert_eq!(hops.len(), ACP_HOPS.len());
    for (hop, (version, integrity, tarball_sha, package_sha, runtime)) in hops.iter().zip(ACP_HOPS)
    {
        assert_eq!(hop["version"], version);
        assert_eq!(hop["integrity"], integrity, "{version}");
        assert_eq!(hop["tarball_sha256"], tarball_sha, "{version}");
        assert_eq!(hop["package_json_sha256"], package_sha, "{version}");
        assert_eq!(hop["dependencies"]["deepagents"], runtime, "{version}");
        assert_eq!(hop["dependencies"]["@agentclientprotocol/sdk"], "^1.1.0");
        assert_eq!(hop["peerDependencies"]["@langchain/core"], "^1.1.40");
        assert_eq!(hop["peerDependencies"]["@langchain/langgraph"], "^1.4.10");
        assert_eq!(hop["gitHead"], Value::Null);
    }

    let runtime_points = identity["runtime_dependency"]["points"]
        .as_array()
        .expect("exact deepagents dependency points");
    assert_eq!(runtime_points.len(), RUNTIME_POINTS.len());
    for (point, (version, tarball_sha, package_sha)) in runtime_points.iter().zip(RUNTIME_POINTS) {
        assert_eq!(point["version"], version);
        assert_eq!(point["tarball_sha256"], tarball_sha, "deepagents {version}");
        assert_eq!(
            point["package_json_sha256"], package_sha,
            "deepagents {version}"
        );
        assert_eq!(point["gitHead"], Value::Null);
    }
    for point in &runtime_points[..4] {
        assert_eq!(point["peerDependencies"]["@langchain/core"], "^1.2.9");
        assert_eq!(point["peerDependencies"]["@langchain/langgraph"], "^1.4.10");
    }
    assert_eq!(
        runtime_points[4]["peerDependencies"]["@langchain/core"],
        "^1.2.14"
    );
    assert_eq!(
        runtime_points[4]["peerDependencies"]["@langchain/langgraph"],
        "^1.4.10"
    );
}

#[test]
fn complete_package_trees_and_per_hop_file_sets_are_closed() {
    let inventory = fixture(INVENTORY, "dist-inventory");
    let acp_versions = json!(["0.1.30", "0.1.31", "0.1.32", "0.1.33", "0.1.34"]);
    let runtime_versions = json!(["1.13.4", "1.13.5", "1.14.0", "1.14.1", "1.14.2"]);
    assert_eq!(inventory["compared"]["deepagents-acp"], acp_versions);
    assert_eq!(inventory["compared"]["deepagents"], runtime_versions);
    assert_eq!(
        inventory["package_file_counts"]["deepagents-acp"],
        json!({"0.1.30":11,"0.1.31":11,"0.1.32":11,"0.1.33":11,"0.1.34":11})
    );
    assert_eq!(
        inventory["package_file_counts"]["deepagents"],
        json!({"1.13.4":25,"1.13.5":25,"1.14.0":25,"1.14.1":25,"1.14.2":25})
    );

    let selected_acp_files = json!([
        "dist/cli.js",
        "dist/cli.js.map",
        "dist/index.cjs",
        "dist/index.cjs.map",
        "dist/index.d.cts",
        "dist/index.d.ts",
        "dist/index.js",
        "dist/index.js.map",
        "LICENSE",
        "package.json",
        "README.md"
    ]);
    let identity = fixture(IDENTITY, "identity");
    for version in ["0.1.30", "0.1.31", "0.1.32", "0.1.33", "0.1.34"] {
        let files = inventory["package_files"]["deepagents-acp"][version]["files"]
            .as_array()
            .expect("complete ACP tree");
        let paths = files
            .iter()
            .map(|file| file["path"].clone())
            .collect::<Vec<_>>();
        assert_eq!(
            Value::Array(paths),
            selected_acp_files,
            "deepagents-acp@{version}"
        );
        assert!(files.iter().all(|file| {
            file["type"] == "file" && file["sha256"].as_str().is_some_and(|hash| hash.len() == 64)
        }));
        let package_json_sha = files
            .iter()
            .find(|file| file["path"] == "package.json")
            .expect("package metadata file")
            .get("sha256")
            .expect("package metadata digest");
        let identity_sha = if version == "0.1.30" {
            "1f7f4b7af7d87dff92bcd612ca019524aee4be8eb59df373c9397780243311b8"
        } else {
            identity["published_stable_points_after_previous_ceiling"]
                .as_array()
                .expect("published hops")
                .iter()
                .find(|point| point["version"] == version)
                .expect("identity hop")["package_json_sha256"]
                .as_str()
                .expect("package metadata hash")
        };
        assert_eq!(package_json_sha, identity_sha, "{version}");
        let identity_tarball = if version == "0.1.30" {
            "c9bc8b95d779fc120d0614bbc37f8d6d35361376b19a5c77d0ae4e870823a661"
        } else {
            identity["published_stable_points_after_previous_ceiling"]
                .as_array()
                .expect("published hops")
                .iter()
                .find(|point| point["version"] == version)
                .expect("identity hop")["tarball_sha256"]
                .as_str()
                .expect("tarball hash")
        };
        assert_eq!(
            inventory["package_files"]["deepagents-acp"][version]["tarball_sha256"],
            identity_tarball,
            "{version} tarball"
        );
    }

    let acp_hops = inventory["per_hop"]["deepagents-acp"]
        .as_object()
        .expect("all ACP package hops");
    assert_eq!(
        acp_hops.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "0.1.30..0.1.31",
            "0.1.31..0.1.32",
            "0.1.32..0.1.33",
            "0.1.33..0.1.34",
        ]
    );
    for (hop, entry) in acp_hops {
        assert_eq!(
            strings(&entry["changed"], hop),
            vec!["package.json".to_owned()]
        );
        assert_eq!(entry["added"], json!([]), "{hop}");
        assert_eq!(entry["removed"], json!([]), "{hop}");
        assert_eq!(
            entry["identical"]
                .as_array()
                .expect("identical files")
                .len(),
            10
        );
    }

    let runtime_hops = inventory["per_hop"]["deepagents"]
        .as_object()
        .expect("all exact runtime hops");
    assert_eq!(
        runtime_hops.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "1.13.4..1.13.5",
            "1.13.5..1.14.0",
            "1.14.0..1.14.1",
            "1.14.1..1.14.2",
        ]
    );
    let mut expected_files = RUNTIME_BASELINE_FILES
        .into_iter()
        .map(str::to_owned)
        .collect::<std::collections::BTreeSet<_>>();
    let baseline_files = inventory["package_files"]["deepagents"]["1.13.4"]["files"]
        .as_array()
        .expect("complete baseline runtime tree");
    assert_eq!(baseline_files.len(), 25);
    assert_eq!(
        baseline_files
            .iter()
            .map(|file| file["path"].as_str().expect("path").to_owned())
            .collect::<std::collections::BTreeSet<_>>(),
        expected_files,
        "baseline runtime paths are exact"
    );
    let identity_runtime = identity
        .get("runtime_dependency")
        .and_then(|runtime| runtime.get("points"))
        .and_then(Value::as_array)
        .expect("runtime artifact identities");
    let identity_baseline = identity_runtime
        .iter()
        .find(|point| point["version"] == "1.13.4")
        .expect("baseline runtime identity");
    assert_eq!(
        inventory["package_files"]["deepagents"]["1.13.4"]["tarball_sha256"],
        identity_baseline["tarball_sha256"]
    );
    assert_eq!(
        baseline_files
            .iter()
            .find(|file| file["path"] == "package.json")
            .expect("baseline runtime package metadata")["sha256"],
        identity_baseline["package_json_sha256"]
    );
    for (hop, expected_added, expected_removed, expected_changed) in RUNTIME_HOPS {
        let entry = &runtime_hops[hop];
        let before_files = expected_files.clone();
        assert_eq!(
            strings(&entry["added"], hop),
            expected_added
                .iter()
                .map(|path| (*path).to_owned())
                .collect::<Vec<_>>(),
            "{hop} added paths"
        );
        assert_eq!(
            strings(&entry["removed"], hop),
            expected_removed
                .iter()
                .map(|path| (*path).to_owned())
                .collect::<Vec<_>>(),
            "{hop} removed paths"
        );
        assert_eq!(
            strings(&entry["changed"], hop),
            expected_changed
                .iter()
                .map(|path| (*path).to_owned())
                .collect::<Vec<_>>(),
            "{hop} changed paths"
        );
        for path in expected_removed {
            assert!(
                expected_files.remove(*path),
                "{hop} removes existing {path}"
            );
        }
        for path in expected_added {
            assert!(
                expected_files.insert((*path).to_owned()),
                "{hop} adds new {path}"
            );
        }
        for path in expected_changed {
            assert!(
                before_files.contains(*path),
                "{hop} changes existing {path}"
            );
            assert!(
                expected_files.contains(*path),
                "{hop} retains changed {path}"
            );
        }
        let after = hop.split("..").nth(1).expect("to version");
        let files = inventory["package_files"]["deepagents"][after]["files"]
            .as_array()
            .expect("complete runtime tree");
        assert_eq!(files.len(), 25, "deepagents@{after}");
        let identity_point = identity_runtime
            .iter()
            .find(|point| point["version"] == after)
            .expect("runtime identity point");
        assert_eq!(
            inventory["package_files"]["deepagents"][after]["tarball_sha256"],
            identity_point["tarball_sha256"],
            "deepagents@{after} tarball"
        );
        let package_json_sha = files
            .iter()
            .find(|file| file["path"] == "package.json")
            .expect("runtime package metadata file")
            .get("sha256")
            .expect("runtime package metadata digest");
        assert_eq!(
            package_json_sha, &identity_point["package_json_sha256"],
            "deepagents@{after} package.json"
        );
        let actual_paths = files
            .iter()
            .map(|file| file["path"].as_str().expect("path").to_owned())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            actual_paths, expected_files,
            "deepagents@{after} full path set"
        );
        assert!(files.iter().all(|file| {
            file["type"] == "file" && file["sha256"].as_str().is_some_and(|hash| hash.len() == 64)
        }));
        let expected_identical = before_files
            .intersection(&expected_files)
            .filter(|path| !expected_changed.contains(&path.as_str()))
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let actual_identical = strings(&entry["identical"], hop)
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            actual_identical, expected_identical,
            "{hop} unchanged paths"
        );
    }
}

#[test]
fn selected_route_hop_classes_preserve_the_existing_acp_contract() {
    let protocol = fixture(PROTOCOL, "protocol");
    assert_eq!(protocol["route"], "deepagents.acp");
    assert_eq!(
        protocol["selected_route"]["entrypoint"],
        json!(["deepagents-acp"])
    );
    assert_eq!(protocol["selected_route"]["extra_argv"], json!([]));
    assert_eq!(protocol["selected_route"]["transport"], "ACP v1 stdio");
    assert_eq!(
        protocol["selected_route"]["interactive_operations"],
        json!([
            "initialize",
            "session/new",
            "session/prompt",
            "session/cancel"
        ])
    );
    assert_eq!(
        protocol["selected_route"]["unselected_operations"],
        json!([
            "session/load",
            "session/set_mode",
            "session/close",
            "slash commands"
        ])
    );
    assert_eq!(protocol["selected_route"]["client_mcp_servers"], json!([]));
    assert_eq!(
        protocol["unchanged_acp_artifacts"]["versions"],
        json!(["0.1.30", "0.1.31", "0.1.32", "0.1.33", "0.1.34"])
    );
    assert_eq!(
        protocol["unchanged_acp_artifacts"]["files"],
        json!([
            "dist/cli.js",
            "dist/index.js",
            "dist/index.cjs",
            "dist/index.d.ts",
            "dist/index.d.cts"
        ])
    );
    let unchanged_hashes = &protocol["unchanged_acp_artifacts"]["hashes"];
    let baseline_hashes = &unchanged_hashes["0.1.30"];
    assert_eq!(
        baseline_hashes["cli_js_sha256"],
        "508603abc388abac1dbe7b1d57e45ebf611d2ec6644fe4e3349c5b335064d830"
    );
    assert_eq!(
        baseline_hashes["index_js_sha256"],
        "6ce345349857c6f74965512cfa9fe5baaa4ae42f0401cbbdd5643c724d161358"
    );
    assert_eq!(
        baseline_hashes["index_cjs_sha256"],
        "742f5b4e60da0d11db04aeb54da2e18d87e2babc821a03b8f7b51e4fa5ddc3fe"
    );
    assert_eq!(
        baseline_hashes["index_d_ts_sha256"],
        "21449c09ce02096a073e4c4f77233a6d0326b48092aa54a44ae519b0e87de20b"
    );
    assert_eq!(
        baseline_hashes["index_d_cts_sha256"],
        "c34b3dac4d673233670cbb2163ad85bdcb0d1e5a22eb30b2f9c43e0d8ba6aed8"
    );
    for version in ["0.1.30", "0.1.31", "0.1.32", "0.1.33", "0.1.34"] {
        assert_eq!(unchanged_hashes[version], *baseline_hashes, "{version}");
    }

    let package_hops = protocol["per_hop_package_changes"]
        .as_object()
        .expect("package hop classifications");
    assert_eq!(
        package_hops.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "0.1.30..0.1.31",
            "0.1.31..0.1.32",
            "0.1.32..0.1.33",
            "0.1.33..0.1.34",
        ]
    );
    for (hop, entry) in package_hops {
        assert_eq!(entry["changed_files"], json!(["package.json"]), "{hop}");
        assert_eq!(entry["disposition"], "compatible-extension", "{hop}");
    }

    let runtime_hops = protocol["per_hop_runtime_changes"]
        .as_object()
        .expect("runtime hop classifications");
    let inventory = fixture(INVENTORY, "dist-inventory");
    assert_eq!(
        runtime_hops.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "1.13.4..1.13.5",
            "1.13.5..1.14.0",
            "1.14.0..1.14.1",
            "1.14.1..1.14.2",
        ]
    );
    let expected_source_paths = [
        (
            "1.13.4..1.13.5",
            json!([
                "src/backends/utils.ts",
                "src/middleware/fs.ts",
                "src/middleware/skills.ts"
            ]),
        ),
        (
            "1.13.5..1.14.0",
            json!([
                "src/agent.ts",
                "src/middleware/skills.ts",
                "src/middleware/subagents.ts"
            ]),
        ),
        (
            "1.14.0..1.14.1",
            json!(["src/backends/filesystem.ts", "src/middleware/fs.ts"]),
        ),
        (
            "1.14.1..1.14.2",
            json!([
                "src/agent.ts",
                "src/backends/composite.ts",
                "src/backends/state.ts",
                "src/backends/utils.ts",
                "src/middleware/blobOffload.ts",
                "src/middleware/fs.ts",
                "src/middleware/subagents.ts",
                "src/middleware/unsupportedContent.ts"
            ]),
        ),
    ];
    for (hop, expected) in expected_source_paths {
        let entry = &runtime_hops[hop];
        assert_eq!(entry["changed_sources"], expected, "{hop}");
        assert_eq!(entry["disposition"], "compatible-extension", "{hop}");
        let expected_keys = strings(&expected, "expected selected source paths");
        assert_eq!(
            inventory["selected_source_changes"][hop], expected,
            "{hop} inventory source set"
        );
        let actual_keys = entry["source_classification"]
            .as_object()
            .expect("per-file classification")
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(actual_keys, expected_keys, "{hop} classified source set");
        for source in &expected_keys {
            assert!(
                entry["source_classification"][source]
                    .as_str()
                    .is_some_and(|description| !description.is_empty()),
                "{hop} needs an explanation for {source}"
            );
        }
        assert_eq!(
            entry["selected_wire_result"],
            if hop == "1.13.4..1.13.5" {
                "tool_result_text_only"
            } else {
                "no ACP wire schema, operation, capability, permission, usage, or lifecycle change"
            },
            "{hop}"
        );
    }

    let read_text_change =
        runtime_hops["1.13.4..1.13.5"]["source_classification"]["src/middleware/fs.ts"]
            .as_str()
            .expect("read_file classification");
    assert!(read_text_change.contains("@@ range header"));
    assert!(read_text_change.contains("ACP tool_call_update text"));
    assert!(
        runtime_hops["1.13.4..1.13.5"]["source_classification"]["src/middleware/skills.ts"]
            .as_str()
            .expect("skills classification")
            .contains("skills: []")
    );
    assert!(
        runtime_hops["1.14.0..1.14.1"]["source_classification"]["src/backends/filesystem.ts"]
            .as_str()
            .expect("filesystem classification")
            .contains("virtualMode=false")
    );
    let internal_retry =
        runtime_hops["1.14.1..1.14.2"]["source_classification"]["src/middleware/fs.ts"]
            .as_str()
            .expect("internal retry classification");
    assert!(internal_retry.contains("may alter model outcomes"));
    assert!(internal_retry.contains("does not alter the tool result already sent to ACP"));
    assert!(
        runtime_hops["1.14.1..1.14.2"]["source_classification"]["src/middleware/blobOffload.ts"]
            .as_str()
            .expect("blob-offload classification")
            .contains("defaults false")
    );

    let invariants = &protocol["mapped_invariants"];
    for name in [
        "initialize",
        "session/new",
        "session/prompt",
        "tools",
        "permissions",
        "lifecycle",
        "usage",
    ] {
        assert!(
            invariants[name].as_str().is_some(),
            "missing {name} invariant"
        );
    }
    assert!(
        protocol["bounded_external_changes"]["dependency-resolution"]
            .as_str()
            .expect("dependency boundary")
            .contains("^1.2.14")
    );
    assert!(
        protocol["bounded_external_changes"]["source provenance"]
            .as_str()
            .expect("source provenance")
            .contains("no registry gitHead")
    );
}
