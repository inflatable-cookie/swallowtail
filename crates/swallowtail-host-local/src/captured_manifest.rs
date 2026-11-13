//! Constructs a graph from a trusted host's complete digest/format manifest.

use crate::{LocalCapturedGraph, LocalCapturedSources, LocalExpectedSource, LocalTrustedRuntime};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    vendor: String,
    entry: PathBuf,
    idle: PathBuf,
    files: BTreeMap<PathBuf, Source>,
    imports: Vec<Edge>,
    builtins: Vec<Edge>,
    require_parents: Vec<(PathBuf, PathBuf)>,
    native_parents: Vec<PathBuf>,
    native: BTreeMap<String, Native>,
    denied_dynamic: Vec<serde_json::Value>,
    source_cautions: Vec<serde_json::Value>,
    platform_programs: Vec<PlatformProgram>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    sha256: String,
    format: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Edge {
    parent: PathBuf,
    specifier: String,
    target: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Native {
    sha256: String,
    bytes: u64,
    specifier: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PlatformProgram {
    parent: PathBuf,
    name: String,
    arguments: String,
}

impl LocalCapturedGraph {
    /// Captures an exact host-approved module/format/import manifest and native.
    ///
    /// This is a host policy input, not official identity by itself. The caller
    /// must independently approve the manifest and the complete runtime loader
    /// closure. Every JavaScript/JSON member is digest checked and retained;
    /// native resolution/spawn binds one separately held executable. Unknown
    /// formats, edges, platforms, builtins and source cautions fail closed.
    pub fn capture_manifest(
        root: &Path,
        manifest: &[u8],
        platform: &str,
        native: &LocalTrustedRuntime,
        ps: Option<&LocalTrustedRuntime>,
    ) -> io::Result<Self> {
        if manifest.len() > 4 * 1024 * 1024 || !["darwin-arm64", "linux-x64"].contains(&platform) {
            return Err(invalid("invalid guarded manifest"));
        }
        let manifest: Manifest = serde_json::from_slice(manifest)?;
        if manifest
            .imports
            .len()
            .saturating_add(manifest.builtins.len())
            > 8192
            || manifest.require_parents.len() > 1024
            || manifest.native_parents.len() > 1024
            || manifest.source_cautions.len() > 4096
        {
            return Err(invalid("guarded manifest limit"));
        }
        let _ = decode(&manifest.vendor)?;
        let expected_native = manifest
            .native
            .get(platform)
            .ok_or_else(|| invalid("native platform absent"))?;
        if native.source_digest() != &decode(&expected_native.sha256)? {
            return Err(invalid("native artifact mismatch"));
        }
        let native_binding = native.native_binding()?;
        if native_binding["bytes"] != expected_native.bytes || !manifest.denied_dynamic.is_empty() {
            return Err(invalid("unadmitted graph edge"));
        }
        if manifest.source_cautions.iter().any(|row| {
            !matches!(
                row["kind"].as_str(),
                Some(
                    "identifier:Function"
                        | "identifier:createRequire"
                        | "call:createRequire-alias"
                        | "identifier:require"
                )
            )
        }) {
            return Err(invalid("unclassified source caution"));
        }
        let expected = manifest
            .files
            .iter()
            .map(|(path, source)| {
                Ok(LocalExpectedSource::new(
                    path.clone(),
                    decode(&source.sha256)?,
                ))
            })
            .collect::<io::Result<Vec<_>>>()?;
        let captured = LocalCapturedSources::capture(root, &expected)?;
        let mut files = Vec::new();
        for (path, source) in &manifest.files {
            if !["module", "commonjs", "json"].contains(&source.format.as_str()) {
                return Err(invalid("unapproved module format"));
            }
            let bytes = captured
                .source(path)
                .ok_or_else(|| invalid("missing captured module"))?;
            let text = std::str::from_utf8(bytes).map_err(|_| invalid("non-UTF8 module"))?;
            if source.format == "json" {
                let _: serde_json::Value = serde_json::from_str(text)?;
            }
            files.push(serde_json::json!({"url":url(path)?,"source":text,"format":source.format}));
        }
        if !manifest.files.contains_key(&manifest.entry)
            || manifest
                .files
                .get(&manifest.idle)
                .is_none_or(|row| row.format != "module")
        {
            return Err(invalid("missing entry or idle module"));
        }
        let entry = url(&manifest.entry)?;
        let mut edges = vec![serde_json::json!({"parent":null,"specifier":entry,"target":entry})];
        let mut seen = BTreeSet::new();
        let mut reached = BTreeSet::from([manifest.entry.clone()]);
        for edge in &manifest.imports {
            if !manifest.files.contains_key(&edge.parent)
                || !manifest.files.contains_key(Path::new(&edge.target))
                || edge.specifier.is_empty()
                || edge.specifier.len() > 4096
                || !seen.insert((edge.parent.clone(), edge.specifier.clone()))
            {
                return Err(invalid("invalid captured edge"));
            }
            edges.push(serde_json::json!({"parent":url(&edge.parent)?,"specifier":edge.specifier,"target":url(Path::new(&edge.target))?}));
        }
        loop {
            let old = reached.len();
            for edge in &manifest.imports {
                if reached.contains(&edge.parent) {
                    reached.insert(PathBuf::from(&edge.target));
                }
            }
            if reached.len() == old {
                break;
            }
        }
        if reached.len() != manifest.files.len() {
            return Err(invalid("disconnected captured source"));
        }
        const BUILTINS: &[&str] = &[
            "async_hooks",
            "buffer",
            "child_process",
            "crypto",
            "events",
            "fs",
            "fs/promises",
            "http",
            "http2",
            "https",
            "module",
            "net",
            "os",
            "path",
            "path/win32",
            "process",
            "readline",
            "stream",
            "stream/web",
            "string_decoder",
            "timers/promises",
            "url",
            "util",
            "zlib",
        ];
        for edge in &manifest.builtins {
            if !manifest.files.contains_key(&edge.parent)
                || !BUILTINS.contains(&edge.target.strip_prefix("node:").unwrap_or(""))
                || edge
                    .specifier
                    .strip_prefix("node:")
                    .unwrap_or(&edge.specifier)
                    != edge.target.strip_prefix("node:").unwrap_or("")
                || !seen.insert((edge.parent.clone(), edge.specifier.clone()))
            {
                return Err(invalid("unapproved builtin edge"));
            }
            edges.push(serde_json::json!({"parent":url(&edge.parent)?,"specifier":edge.specifier,"target":edge.target}));
        }
        let mut require_parents = Vec::new();
        let mut parent_pairs = BTreeSet::new();
        for (caller, parent) in &manifest.require_parents {
            if !manifest.files.contains_key(caller)
                || !manifest.files.contains_key(parent)
                || !parent_pairs.insert((caller.clone(), parent.clone()))
            {
                return Err(invalid("invalid require parent"));
            }
            require_parents.push(serde_json::json!([url(caller)?, url(parent)?]));
        }
        for parent in &manifest.native_parents {
            if !manifest.files.contains_key(parent)
                || !seen.insert((parent.clone(), expected_native.specifier.clone()))
            {
                return Err(invalid("invalid native resolver"));
            }
            edges.push(serde_json::json!({"parent":url(parent)?,"specifier":expected_native.specifier,"target":native_binding["url"]}));
        }
        let mut platform_programs = Vec::new();
        if manifest.platform_programs.len() > 1 {
            return Err(invalid("unclassified platform program"));
        }
        if manifest.platform_programs.is_empty() && ps.is_some() {
            return Err(invalid("unexpected platform program"));
        }
        for program in &manifest.platform_programs {
            if program.name != "ps"
                || program.arguments != "live-process-starts-v1"
                || program.parent != Path::new("acp/dist/session-index/live-registry.js")
                || !manifest.files.contains_key(&program.parent)
            {
                return Err(invalid("unclassified platform program"));
            }
            let binding = ps
                .ok_or_else(|| invalid("system process helper custody absent"))?
                .system_ps_binding(platform)?;
            platform_programs.push(
                serde_json::json!({"parent":url(&program.parent)?,"name":"ps","binding":binding}),
            );
        }
        let configuration = serde_json::to_vec(
            &serde_json::json!({"vendor":manifest.vendor,"idleUrl":url(&manifest.idle)?,"files":files,"edges":edges,"requireParents":require_parents,"native":native_binding,"platform":platform,"platformPrograms":platform_programs}),
        )?;
        let mut hash = Sha256::new();
        hash.update(b"swallowtail-captured-graph-v1\0");
        hash.update(&configuration);
        Ok(Self {
            configuration: Arc::from(configuration),
            digest: hash.finalize().into(),
            native: Some(native.clone()),
            ps: ps.cloned(),
        })
    }
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn url(path: &Path) -> io::Result<String> {
    let name = path
        .to_str()
        .ok_or_else(|| invalid("invalid manifest path"))?;
    if name.is_empty()
        || !path
            .components()
            .all(|part| matches!(part, std::path::Component::Normal(_)))
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'_' | b'-' | b'.'))
    {
        return Err(invalid("invalid manifest path"));
    }
    Ok(format!("file:///swallowtail-capture/{name}"))
}
fn decode(text: &str) -> io::Result<[u8; 32]> {
    if text.len() != 64
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid("invalid source digest"));
    }
    let mut result = [0; 32];
    for (out, pair) in result.iter_mut().zip(text.as_bytes().chunks_exact(2)) {
        let nibble = |b: u8| if b <= b'9' { b - b'0' } else { b - b'a' + 10 };
        *out = nibble(pair[0]) * 16 + nibble(pair[1]);
    }
    Ok(result)
}
