use sha2::{Digest, Sha256};
use swallowtail_runtime::{ExpectedSource, FrozenGuardedNodeCapture};

/// Guarded permission behavior for the frozen ACP 0.101.1–0.106.0 points.
/// Preserves inherited managed denial without fabricating a permission callback.
pub const CLAUDE_AGENT_ACP_GUARDED_PERMISSION_BEHAVIOR: &str =
    "claude-agent.acp.guarded-permission-inheritance-v8";

const INVENTORY: &str = include_str!("../sidecar/claude-agent-acp-guard-v1-inventory.json");
const HOPS: [&str; 8] = [
    "0.101.0", "0.101.1", "0.102.0", "0.103.0", "0.104.0", "0.104.1", "0.105.0", "0.106.0",
];
/// Platforms this frozen capture was inventoried for.
const PLATFORMS: [&str; 2] = ["darwin-arm64", "linux-x64"];

pub(crate) fn requires_checked_process(version: &str) -> bool {
    HOPS.contains(&version)
}

/// Returns the frozen static v1 wrapper source manifest for one exact hop.
///
/// This is only the relative-JavaScript wrapper inventory. It does not include
/// a complete dependency graph, establish an installed identity or authorize
/// a checked launch. Unlisted versions and malformed inventories fail closed.
#[must_use]
pub fn claude_agent_acp_guard_v1_source_manifest(version: &str) -> Option<Vec<ExpectedSource>> {
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
            Some(ExpectedSource::new(
                path.as_str(),
                decode_sha256(digest.as_str()?)?,
            ))
        })
        .collect()
}

/// Returns the frozen guarded v1 capture inputs for one exact hop and platform.
///
/// The result carries only the manifest, loader and identity digests this route
/// froze. It captures no bytes, verifies no custody and authorizes no launch:
/// the host captures the graph under its own trust roots and binds the derived
/// recipe into its checked-process service. Unknown versions, unknown platforms
/// or malformed manifests return `None`, so nothing falls back to an ordinary
/// stock invocation.
#[must_use]
pub fn claude_agent_acp_guarded_v1_frozen<'a>(
    version: &str,
    platform: &'a str,
) -> Option<FrozenGuardedNodeCapture<'a>> {
    if !PLATFORMS.contains(&platform) {
        return None;
    }
    let manifest: &'static [u8] = match version {
        "0.101.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.101.0.json"),
        "0.101.1" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.101.1.json"),
        "0.102.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.102.0.json"),
        "0.103.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.103.0.json"),
        "0.104.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.104.0.json"),
        "0.104.1" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.104.1.json"),
        "0.105.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.105.0.json"),
        "0.106.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.106.0.json"),
        _ => return None,
    };
    let value: serde_json::Value = serde_json::from_slice(manifest).ok()?;
    let vendor = value["vendor"].as_str().and_then(decode_sha256)?;
    let policy: [u8; 32] = Sha256::digest(
        format!(
            "swallowtail.claude-agent-acp.guarded-v1.trusted-host-installed-node-v2\0{CLAUDE_AGENT_ACP_GUARDED_PERMISSION_BEHAVIOR}"
        )
        .as_bytes(),
    )
    .into();
    Some(FrozenGuardedNodeCapture::new(
        platform,
        manifest,
        crate::CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
        vendor,
        policy,
    ))
}

fn decode_sha256(value: &str) -> Option<[u8; 32]> {
    let bytes = value.as_bytes();
    if bytes.len() != 64 {
        return None;
    }
    let mut result = [0; 32];
    for (index, pair) in bytes.as_chunks::<2>().0.iter().enumerate() {
        let nibble = |byte: u8| match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        };
        result[index] = nibble(pair[0])? * 16 + nibble(pair[1])?;
    }
    Some(result)
}
