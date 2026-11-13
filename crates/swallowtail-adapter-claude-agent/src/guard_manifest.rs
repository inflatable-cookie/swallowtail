use sha2::{Digest, Sha256};
use swallowtail_host_local::LocalExpectedSource;

/// Candidate guarded permission behavior; declared for admission, not qualified.
/// Preserves inherited managed denial without fabricating a permission callback.
pub const CLAUDE_AGENT_ACP_GUARDED_PERMISSION_BEHAVIOR: &str =
    "claude-agent.acp.guarded-permission-inheritance-v8";

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

/// Captures the exact official guarded v1 graph under approved host roots.
///
/// `root` contains `acp`, `acpsdk`, `agentsdk`, `diff` and `zod` package trees.
/// The host must maintain custody of Node and native loader dependencies.
/// This establishes checked recipe inputs, not execution authority or version
/// qualification. Unknown versions/platforms or artifact substitutions refuse;
/// this never falls back to an ordinary stock invocation.
pub fn capture_claude_agent_acp_guarded_v1(
    version: &str,
    root: &std::path::Path,
    platform: &str,
    node: &swallowtail_host_local::LocalTrustedRuntime,
    native: &swallowtail_host_local::LocalTrustedRuntime,
    ps: Option<&swallowtail_host_local::LocalTrustedRuntime>,
) -> std::io::Result<swallowtail_host_local::LocalCapturedNodeRecipe> {
    let manifest: &[u8] = match version {
        "0.101.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.101.0.json"),
        "0.101.1" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.101.1.json"),
        "0.102.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.102.0.json"),
        "0.103.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.103.0.json"),
        "0.104.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.104.0.json"),
        "0.104.1" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.104.1.json"),
        "0.105.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.105.0.json"),
        "0.106.0" => include_bytes!("../sidecar/claude-agent-acp-guard-graphs/0.106.0.json"),
        _ => return Err(guard_identity_failure()),
    };
    if !matches!(platform, "darwin-arm64" | "linux-x64") {
        return Err(guard_identity_failure());
    }
    node.verify()?;
    let value: serde_json::Value = serde_json::from_slice(manifest)?;
    let vendor = value["vendor"]
        .as_str()
        .and_then(decode_sha256)
        .ok_or_else(guard_identity_failure)?;
    let graph = swallowtail_host_local::LocalCapturedGraph::capture_manifest(
        root, manifest, platform, native, ps,
    )?;
    swallowtail_host_local::LocalCapturedNodeRecipe::for_trusted_runtime(
        &graph,
        crate::CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
        vendor,
        node,
        Sha256::digest(
            format!("swallowtail.claude-agent-acp.guarded-v1.trusted-host-installed-node-v2\0{CLAUDE_AGENT_ACP_GUARDED_PERMISSION_BEHAVIOR}").as_bytes()
        ).into(),
    )
}

fn guard_identity_failure() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "guarded ACP identity is not admitted",
    )
}
