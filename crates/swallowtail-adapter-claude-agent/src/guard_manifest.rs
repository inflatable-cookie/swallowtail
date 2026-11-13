use swallowtail_host_local::LocalExpectedSource;

const INVENTORY: &str = include_str!("../sidecar/claude-agent-acp-guard-v1-inventory.json");
const HOPS: [&str; 8] = [
    "0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0", "0.106.0",
];

/// Returns the frozen static v1 wrapper source manifest for one exact hop.
///
/// This is only the relative-JavaScript wrapper inventory. It does not include
/// a complete dependency graph, establish an installed identity or authorize
/// a checked launch. Unlisted versions and malformed inventories fail closed.
#[must_use]
pub fn claude_agent_acp_guard_v1_source_manifest(
    version: &str,
) -> Option<Vec<LocalExpectedSource>> {
    if !HOPS.contains(&version) {
        return None;
    }
    let inventory: serde_json::Value = serde_json::from_str(INVENTORY).ok()?;
    let paths = inventory.get(version)?.as_object()?;
    if paths.is_empty() || paths.len() > 1024 {
        return None;
    }
    paths
        .iter()
        .map(|(path, digest)| {
            Some(LocalExpectedSource::new(
                path.as_str(),
                decode_sha256(digest.as_str()?)?,
            ))
        })
        .collect()
}

fn decode_sha256(value: &str) -> Option<[u8; 32]> {
    let bytes = value.as_bytes();
    if bytes.len() != 64 {
        return None;
    }
    let mut result = [0; 32];
    for (index, pair) in bytes.chunks_exact(2).enumerate() {
        let nibble = |byte: u8| match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        };
        result[index] = nibble(pair[0])? * 16 + nibble(pair[1])?;
    }
    Some(result)
}
