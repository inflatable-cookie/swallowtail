use super::{
    DIGEST_BYTES, MAGIC, PersistedProviderRunCheckpoint, ProviderRunCheckpoint,
    ProviderRunCheckpointFailureKind, VERSION,
};
use sha2::{Digest, Sha256};
use swallowtail_core::RunRef;

use crate::{EventDelivery, RuntimeEvent, RuntimeEventKind, RuntimeRunId};

#[test]
fn persisted_run_checkpoint_rejects_malformed_oversized_version_and_corruption() {
    assert_eq!(
        PersistedProviderRunCheckpoint::from_bytes(b"not-a-checkpoint")
            .expect_err("malformed record rejects")
            .kind(),
        ProviderRunCheckpointFailureKind::InvalidEncoding
    );
    assert_eq!(
        PersistedProviderRunCheckpoint::from_bytes(vec![0; 16 * 1024 + 1])
            .expect_err("oversized record rejects")
            .kind(),
        ProviderRunCheckpointFailureKind::Oversized
    );

    let mut unsupported = fixture_record();
    unsupported[MAGIC.len()..MAGIC.len() + 2].copy_from_slice(&2u16.to_be_bytes());
    resign(&mut unsupported);
    assert_eq!(
        PersistedProviderRunCheckpoint::from_bytes(unsupported)
            .expect_err("unsupported version rejects")
            .kind(),
        ProviderRunCheckpointFailureKind::UnsupportedVersion
    );

    let mut corrupted = fixture_record();
    corrupted[24] ^= 1;
    assert_eq!(
        PersistedProviderRunCheckpoint::from_bytes(corrupted)
            .expect_err("corruption rejects")
            .kind(),
        ProviderRunCheckpointFailureKind::IntegrityMismatch
    );
}

#[test]
fn persisted_run_checkpoint_default_formatting_is_opaque() {
    let record = PersistedProviderRunCheckpoint::from_bytes(fixture_record())
        .expect("fixture record is valid");
    assert_eq!(
        format!("{record:?}"),
        "PersistedProviderRunCheckpoint(<opaque>)"
    );
}

#[test]
fn run_checkpoint_makes_even_a_coalescible_event_semantic() {
    let checkpoint = ProviderRunCheckpoint {
        runtime_run_id: RuntimeRunId::new("runtime-run").expect("runtime run is valid"),
        provider_run_ref: RunRef::new("provider-run").expect("provider run is valid"),
        cursor: b"cursor".to_vec(),
        route_fingerprint: [7; 32],
    };
    let event = RuntimeEvent::new(1, RuntimeEventKind::ProgressSnapshot)
        .with_run_reconciliation_checkpoint(checkpoint);
    assert_eq!(event.delivery(), EventDelivery::Semantic);
}

fn fixture_record() -> Vec<u8> {
    let fields: [&[u8]; 3] = [b"runtime-run", b"provider-run", b"cursor"];
    let mut payload = Vec::new();
    payload.extend_from_slice(MAGIC);
    payload.extend_from_slice(&VERSION.to_be_bytes());
    for field in fields {
        payload.extend_from_slice(&(field.len() as u16).to_be_bytes());
        payload.extend_from_slice(field);
    }
    payload.extend_from_slice(&[7; 32]);
    let digest = Sha256::digest(&payload);
    payload.extend_from_slice(&digest);
    payload
}

fn resign(record: &mut [u8]) {
    let payload_end = record.len() - DIGEST_BYTES;
    let digest = Sha256::digest(&record[..payload_end]);
    record[payload_end..].copy_from_slice(&digest);
}
