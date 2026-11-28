use std::fmt::Write;

use serde_json::Value;
use sha2::{Digest, Sha256};

const IDENTITY: &str = include_str!("fixtures/pi-rpc-1.6.0/identity.json");
const INVENTORY: &str = include_str!("fixtures/pi-rpc-1.6.0/dist-inventory.tsv");
const SOURCES: &str = include_str!("fixtures/pi-rpc-1.6.0/source-correlation.tsv");
const VERSIONS: [&str; 5] = ["1.4.0", "1.5.0", "1.5.1", "1.5.2", "1.6.0"];

#[test]
fn hop_identity_and_classified_file_sets_match_frozen_evidence() {
    assert_eq!(
        sha256(IDENTITY),
        "e07f309f07214de6a4be8ceee582d7c1bb1bf1246b7338393a5e8212923cf814"
    );
    assert_eq!(
        sha256(INVENTORY),
        "936eb5ea50933b014ca0a30af020189481715c0edca93852d821ea57c155f258"
    );
    assert_eq!(
        sha256(SOURCES),
        "b1102c06cd71db27990052353baff323609bdaa95f35b1dbbc6ad27b7472e61f"
    );

    let identity: Value = serde_json::from_str(IDENTITY).expect("frozen Pi identity is JSON");
    let mut rows = INVENTORY.lines();
    let header = rows.next().expect("inventory has a header");
    assert!(header.split('\t').skip(2).eq(VERSIONS));

    let mut trees: [String; VERSIONS.len()] = std::array::from_fn(|_| String::new());
    let mut counts = [0; VERSIONS.len()];
    for row in rows {
        let mut columns = row.split('\t');
        let path = columns.next().expect("inventory row has a path");
        let _class = columns.next().expect("inventory row has a class");
        assert_eq!(columns.clone().count(), VERSIONS.len());
        for (i, digest) in columns.enumerate() {
            if digest != "-" {
                writeln!(trees[i], "{path}\t{digest}").expect("write to string");
                counts[i] += 1;
            }
        }
    }

    for (i, version) in VERSIONS.iter().enumerate() {
        let artifact = &identity["artifacts"][version];
        assert_eq!(artifact["npm_file_count"], counts[i], "{version}");
        assert_eq!(artifact["npm_tree_sha256"], sha256(&trees[i]), "{version}");
        assert_eq!(artifact["provenance_commit"], artifact["github_commit"]);
    }
}

fn sha256(bytes: &str) -> String {
    hex::encode(Sha256::digest(bytes.as_bytes()))
}
