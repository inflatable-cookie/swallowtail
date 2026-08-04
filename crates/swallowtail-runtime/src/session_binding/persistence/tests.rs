use super::{
    PersistedSessionResumeBinding, SessionResumeBindingPersistenceFailureKind, decode_record,
};
use sha2::{Digest, Sha256};

#[test]
fn record_rejects_malformed_oversized_version_and_corruption_without_detail() {
    let invalid = PersistedSessionResumeBinding::from_bytes(b"not-a-binding")
        .expect_err("malformed record rejects");
    assert_eq!(
        invalid.kind(),
        SessionResumeBindingPersistenceFailureKind::InvalidEncoding
    );

    let oversized = PersistedSessionResumeBinding::from_bytes(vec![0; 8 * 1024 + 1])
        .expect_err("oversized record rejects");
    assert_eq!(
        oversized.kind(),
        SessionResumeBindingPersistenceFailureKind::Oversized
    );

    let mut bytes = fixture_record();
    bytes[16..18].copy_from_slice(&2u16.to_be_bytes());
    let unsupported = PersistedSessionResumeBinding::from_bytes(&bytes)
        .expect_err("unsupported version rejects before integrity");
    assert_eq!(
        unsupported.kind(),
        SessionResumeBindingPersistenceFailureKind::UnsupportedVersion
    );

    let mut bytes = fixture_record();
    bytes[22] ^= 0x01;
    let corrupted =
        PersistedSessionResumeBinding::from_bytes(&bytes).expect_err("corrupted record rejects");
    assert_eq!(
        corrupted.kind(),
        SessionResumeBindingPersistenceFailureKind::IntegrityMismatch
    );
    assert!(!format!("{corrupted:?}").contains("provider/private"));
}

#[test]
fn record_default_formatting_is_opaque() {
    let record = PersistedSessionResumeBinding::from_bytes(fixture_record())
        .expect("fixture record is valid");
    assert_eq!(
        format!("{record:?}"),
        "PersistedSessionResumeBinding(<opaque>)"
    );
    assert!(!format!("{record:?}").contains("provider/private"));
    assert_eq!(
        decode_record(record.as_bytes())
            .expect("record decodes")
            .provider_session_ref,
        "provider/private"
    );
}

fn fixture_record() -> Vec<u8> {
    let provider = b"provider/private";
    let mut payload = Vec::new();
    payload.extend_from_slice(b"SWST-RESUME-BIND");
    payload.extend_from_slice(&1u16.to_be_bytes());
    payload.extend_from_slice(&(provider.len() as u16).to_be_bytes());
    payload.extend_from_slice(provider);
    payload.push(0);
    payload.extend_from_slice(&[7; 32]);
    let digest = Sha256::digest(&payload);
    payload.extend_from_slice(&digest);
    payload
}
