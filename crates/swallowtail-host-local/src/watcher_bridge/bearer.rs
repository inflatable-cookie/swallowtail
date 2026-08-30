use crate::output::failure;
use swallowtail_runtime::{RuntimeFailure, WATCHER_BRIDGE_BEARER_BYTE_LEN};

pub(super) fn generate_bearer() -> Result<String, RuntimeFailure> {
    let mut bytes = [0_u8; WATCHER_BRIDGE_BEARER_BYTE_LEN];
    getrandom::getrandom(&mut bytes).map_err(|_| {
        failure(
            "swallowtail.watcher_bridge.entropy_failed",
            "Watcher bridge could not create operation-private authority",
        )
    })?;
    Ok(hex_encode(&bytes))
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}
