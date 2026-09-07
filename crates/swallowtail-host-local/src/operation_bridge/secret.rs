//! Operation-private authority material shared by both bridge profiles.

use crate::output::failure;
use swallowtail_runtime::{RuntimeFailure, WATCHER_BRIDGE_BEARER_BYTE_LEN};
use zeroize::{Zeroize, Zeroizing};

/// Creates fresh, cryptographically unguessable, operation-private material.
///
/// The material authenticates one lease generation only. It never enters
/// provider arguments, ambient environment, durable configuration, records,
/// events, diagnostics, or formatting.
pub(crate) fn generate_operation_secret() -> Result<Zeroizing<String>, RuntimeFailure> {
    let mut bytes = [0_u8; WATCHER_BRIDGE_BEARER_BYTE_LEN];
    getrandom::getrandom(&mut bytes).map_err(|_| {
        failure(
            "swallowtail.operation_bridge.entropy_failed",
            "Operation bridge could not create operation-private authority",
        )
    })?;
    let encoded = Zeroizing::new(hex_encode(&bytes));
    bytes.zeroize();
    Ok(encoded)
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
