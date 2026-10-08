use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const CURRENTNESS_EVIDENCE: &str =
    include_str!("fixtures/deepseek-harness-web-0.2.0rc2-stop/dist-inventory.json");
const CURRENTNESS_EVIDENCE_SHA256: &str =
    "fd69cf09bee0cbc918298b0cdcc695367feee9612a5373079ba2719e80133cd7";

#[test]
fn published_rc_inventory_preserves_qualified_points_and_the_exact_auth_stop() {
    let evidence: Value =
        serde_json::from_str(CURRENTNESS_EVIDENCE).expect("currentness inventory is valid JSON");

    assert_eq!(evidence["fixture_schema"], 1);
    assert_eq!(evidence["route_id"], "deepseek-harness.local-server");
    assert_eq!(evidence["version_axis"], "deepseek-harness.web");
    assert_eq!(evidence["official_source"]["package"], "@deepseek-ai/dsh");
    // Pin the full artifact ledger so changed path/key sets cannot drift while
    // preserving only the headline package hashes below.
    assert_eq!(
        format!("{:x}", Sha256::digest(CURRENTNESS_EVIDENCE.as_bytes())),
        CURRENTNESS_EVIDENCE_SHA256
    );
    assert_eq!(
        evidence["official_source"]["dist_tags"]["latest"],
        "0.2.0-rc.2"
    );
    assert_eq!(
        evidence["official_source"]["dist_tags"]["next"],
        "0.2.0-rc.2"
    );
    assert_eq!(
        evidence["official_source"]["dist_tags"]["alpha"],
        "0.2.1-alpha.1"
    );
    assert_eq!(
        evidence["current_qualified_claim"]["qualified_versions"],
        json!([
            "0.1.0-rc.6",
            "0.1.0-rc.7",
            "0.1.0-rc.8",
            "0.1.1-rc.1",
            "0.1.1-rc.2"
        ])
    );
    assert_eq!(
        evidence["current_qualified_claim"]["claim_id"],
        "deepseek-harness.web-rc6-1"
    );
    assert_eq!(
        evidence["current_qualified_claim"]["protocol_facade_revision"],
        "deepseek-harness.apiproxy-v1"
    );
    assert_eq!(evidence["stop"]["from"], "0.1.1-rc.2");
    assert_eq!(evidence["stop"]["to"], "0.1.2-rc.1");
    assert_eq!(
        evidence["stop"]["package"],
        "@deepseek-ai/dsh-client-connection"
    );
    assert_eq!(evidence["stop"]["file"], "package/lib/index.js");
    let observed_hops = evidence["pre_stop_hop_ledger"]
        .as_array()
        .expect("every selected-channel hop is inventoried")
        .iter()
        .map(|hop| {
            (
                hop["from"].as_str().expect("hop has a source version"),
                hop["to"].as_str().expect("hop has a target version"),
                hop["status"].as_str().expect("hop has a classification"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed_hops,
        [
            (
                "0.1.0-rc.6",
                "0.1.0-rc.7",
                "screened-through-next-selected-rc"
            ),
            (
                "0.1.0-rc.7",
                "0.1.0-rc.8",
                "screened-through-next-selected-rc"
            ),
            (
                "0.1.0-rc.8",
                "0.1.1-rc.1",
                "screened-through-next-selected-rc"
            ),
            (
                "0.1.1-rc.1",
                "0.1.1-rc.2",
                "screened-through-next-selected-rc"
            ),
            ("0.1.1-rc.2", "0.1.2-rc.1", "first-security-authority-stop"),
        ]
    );

    let artifacts = evidence["artifacts"]
        .as_array()
        .expect("complete artifact inventories are present");
    assert!(!artifacts.is_empty());
    for artifact in artifacts {
        assert_eq!(artifact["source_git_head"], Value::Null);
        assert_eq!(artifact["file_count"], artifact["npm_file_count"]);
        assert_eq!(
            artifact["local_integrity_sha512"],
            artifact["npm_integrity"]
        );
        assert_eq!(artifact["tarball_sha256"].as_str().unwrap().len(), 64);

        let files = artifact["complete_file_tree"]
            .as_array()
            .expect("each tarball has its complete sorted file tree");
        assert_eq!(artifact["file_count"].as_u64(), Some(files.len() as u64));
        assert!(
            files
                .windows(2)
                .all(|pair| { pair[0]["path"].as_str() < pair[1]["path"].as_str() })
        );
        assert!(files.iter().all(|file| {
            file["sha256"].as_str().is_some_and(|digest| {
                digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit())
            })
        }));
    }

    let client_connection = |version: &str| {
        artifacts
            .iter()
            .find(|artifact| {
                artifact["name"] == "@deepseek-ai/dsh-client-connection"
                    && artifact["version"] == version
            })
            .expect("exact client-connection npm artifact is inventoried")
    };
    let stop_artifact = client_connection("0.1.2-rc.1");
    assert_eq!(
        stop_artifact["tarball_sha256"],
        "bf2663cb66c0fd07987dbdc5d197327e2452dc5634f816b4e2dae03146e1149f"
    );
    assert_eq!(
        stop_artifact["complete_file_tree"]
            .as_array()
            .unwrap()
            .iter()
            .find(|file| file["path"] == "package/lib/index.js")
            .unwrap()["sha256"],
        "91b8e7b90c5189a5dc0f4eafc4a62494eef9b5a28ded0715146ddf1a31126c8e"
    );
    let current_artifact = client_connection("0.2.0-rc.2");
    assert_eq!(
        current_artifact["tarball_sha256"],
        "536ec548c42af452b5f5d518f4ae0f1c0c992d5f183a460544bf3329f7b2e70b"
    );

    let stop_app = artifacts
        .iter()
        .find(|artifact| {
            artifact["name"] == "@deepseek-ai/dsh-web-app" && artifact["version"] == "0.1.2-rc.1"
        })
        .expect("the first app composition using the new controllers is inventoried");
    assert!(stop_app["dependencies"]["@deepseek-ai/dsh-api-session-controller"].is_string());
    assert!(stop_app["dependencies"]["@deepseek-ai/dsh-api-workspace-controller"].is_string());
    assert!(stop_app["dependencies"]["@deepseek-ai/dsh-host-apiproxy"].is_null());
}
