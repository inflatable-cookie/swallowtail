use crate::failure::failure;
use swallowtail_runtime::RuntimeFailure;

const VERSION: u8 = 1;
const MAXIMUM_EPOCH_BYTES: usize = 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct KimiCursorCheckpoint {
    pub(super) seq: u64,
    pub(super) epoch: String,
}

pub(super) fn encode(seq: u64, epoch: &str) -> Result<Vec<u8>, RuntimeFailure> {
    if epoch.is_empty() || epoch.len() > MAXIMUM_EPOCH_BYTES {
        return Err(invalid());
    }
    let length = u16::try_from(epoch.len()).map_err(|_| invalid())?;
    let mut encoded = Vec::with_capacity(1 + 8 + 2 + epoch.len());
    encoded.push(VERSION);
    encoded.extend_from_slice(&seq.to_be_bytes());
    encoded.extend_from_slice(&length.to_be_bytes());
    encoded.extend_from_slice(epoch.as_bytes());
    Ok(encoded)
}

pub(super) fn decode(bytes: &[u8]) -> Result<KimiCursorCheckpoint, RuntimeFailure> {
    if bytes.len() < 12 || bytes.first().copied() != Some(VERSION) {
        return Err(invalid());
    }
    let seq = u64::from_be_bytes(bytes[1..9].try_into().map_err(|_| invalid())?);
    let length = usize::from(u16::from_be_bytes(
        bytes[9..11].try_into().map_err(|_| invalid())?,
    ));
    if length == 0 || length > MAXIMUM_EPOCH_BYTES || 11 + length != bytes.len() {
        return Err(invalid());
    }
    let epoch = std::str::from_utf8(&bytes[11..])
        .ok()
        .filter(|epoch| !epoch.is_empty())
        .ok_or_else(invalid)?;
    Ok(KimiCursorCheckpoint {
        seq,
        epoch: epoch.to_owned(),
    })
}

fn invalid() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_checkpoint_invalid",
        "Kimi local-server reconciliation checkpoint is invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::{KimiCursorCheckpoint, decode, encode};

    #[test]
    fn cursor_checkpoint_round_trips_exactly() {
        assert_eq!(
            decode(&encode(42, "fixture-epoch").expect("cursor encodes")).expect("cursor decodes"),
            KimiCursorCheckpoint {
                seq: 42,
                epoch: "fixture-epoch".to_owned(),
            }
        );
    }

    #[test]
    fn cursor_checkpoint_rejects_unknown_or_malformed_encodings() {
        assert!(decode(&[]).is_err());
        assert!(decode(&[2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, b'e']).is_err());
        assert!(decode(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, b'e']).is_err());
    }
}
