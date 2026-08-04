use super::{
    PersistedProviderOperationCheckpoint, ProviderOperationCheckpoint,
    ProviderOperationCheckpointFailureKind,
};
use crate::RuntimeTurnId;
use sha2::{Digest, Sha256};
use swallowtail_core::{SessionRef, TurnRef};

#[test]
fn checkpoint_identity_and_cursor_stay_opaque_by_default() {
    let checkpoint = ProviderOperationCheckpoint::new(
        SessionRef::new("provider/private/session").expect("session is valid"),
        RuntimeTurnId::new("consumer/private/turn").expect("turn is valid"),
        TurnRef::new("provider/private/turn").expect("provider turn is valid"),
        b"provider-private-cursor",
    )
    .expect("checkpoint is valid");

    assert_eq!(
        format!("{checkpoint:?}"),
        "ProviderOperationCheckpoint(<opaque>)"
    );
    assert!(!format!("{checkpoint:?}").contains("private"));
}

#[test]
fn persisted_record_rejects_malformed_oversized_version_and_corruption() {
    assert_eq!(
        PersistedProviderOperationCheckpoint::from_bytes(b"not-a-checkpoint")
            .expect_err("malformed record rejects")
            .kind(),
        ProviderOperationCheckpointFailureKind::InvalidEncoding
    );
    assert_eq!(
        PersistedProviderOperationCheckpoint::from_bytes(vec![0; 20 * 1024 + 1])
            .expect_err("oversized record rejects")
            .kind(),
        ProviderOperationCheckpointFailureKind::Oversized
    );

    let mut unsupported = fixture_record();
    unsupported[16..18].copy_from_slice(&2u16.to_be_bytes());
    assert_eq!(
        PersistedProviderOperationCheckpoint::from_bytes(unsupported)
            .expect_err("unsupported version rejects")
            .kind(),
        ProviderOperationCheckpointFailureKind::UnsupportedVersion
    );

    let mut corrupted = fixture_record();
    corrupted[28] ^= 1;
    assert_eq!(
        PersistedProviderOperationCheckpoint::from_bytes(corrupted)
            .expect_err("corruption rejects")
            .kind(),
        ProviderOperationCheckpointFailureKind::IntegrityMismatch
    );
}

#[test]
fn persisted_record_default_formatting_is_opaque() {
    let record = PersistedProviderOperationCheckpoint::from_bytes(fixture_record())
        .expect("fixture record is valid");
    assert_eq!(
        format!("{record:?}"),
        "PersistedProviderOperationCheckpoint(<opaque>)"
    );
}

fn fixture_record() -> Vec<u8> {
    let fields: [&[u8]; 4] = [
        b"provider-session",
        b"runtime-turn",
        b"provider-turn",
        b"cursor",
    ];
    let mut payload = Vec::new();
    payload.extend_from_slice(b"SWST-OP-CHECKPT\0");
    payload.extend_from_slice(&1u16.to_be_bytes());
    for field in fields {
        payload.extend_from_slice(&(field.len() as u16).to_be_bytes());
        payload.extend_from_slice(field);
    }
    payload.extend_from_slice(&[7; 32]);
    let digest = Sha256::digest(&payload);
    payload.extend_from_slice(&digest);
    payload
}
