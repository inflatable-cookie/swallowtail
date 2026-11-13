//! Host-private byte snapshots for a future checked process launch.
//!
//! Capturing bytes does not issue a checked-process grant, establish official
//! artifact identity, or attest the interpreter that will execute them.

use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

const MAX_MODULES: usize = 1024;
const MAX_MODULE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_TOTAL_BYTES: usize = 64 * 1024 * 1024;

/// One host-policy path and its expected SHA-256.
///
/// The digest is an input to capture, not an attestation of vendor identity.
#[derive(Clone)]
pub struct LocalExpectedSource {
    path: PathBuf,
    sha256: [u8; 32],
}

impl LocalExpectedSource {
    /// Creates one relative source specification.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, sha256: [u8; 32]) -> Self {
        Self {
            path: path.into(),
            sha256,
        }
    }
}

/// Immutable captured bytes for a bounded host-supplied source manifest.
///
/// A checked-launch service must separately establish the trusted manifest,
/// module graph, guard, runtime custody and policy before using these bytes.
#[derive(Clone)]
pub struct LocalCapturedSources {
    entries: BTreeMap<PathBuf, Arc<[u8]>>,
    digest: [u8; 32],
}

impl LocalCapturedSources {
    /// Reads each manifest member once and verifies its captured bytes.
    ///
    /// Rejects absolute or escaping paths, observed symlinks, aliases, duplicate
    /// names, non-files, mismatched digests and size limits. A mutable parent
    /// directory can still race the path checks. This method does not approve
    /// a launch or open or execute any interpreter.
    pub fn capture(root: &Path, expected: &[LocalExpectedSource]) -> io::Result<Self> {
        if expected.is_empty() || expected.len() > MAX_MODULES {
            return Err(invalid("invalid source count"));
        }
        if !root.is_absolute() || root.canonicalize()? != root {
            return Err(invalid("noncanonical root"));
        }
        let mut entries = BTreeMap::new();
        let mut objects = BTreeSet::new();
        let mut total = 0usize;
        for source in expected {
            if source.path.as_os_str().is_empty()
                || !source
                    .path
                    .components()
                    .all(|part| matches!(part, Component::Normal(_)))
            {
                return Err(invalid("invalid source path"));
            }
            if entries.contains_key(&source.path) {
                return Err(invalid("duplicate source path"));
            }
            let mut candidate = root.to_path_buf();
            for part in source.path.components() {
                candidate.push(part.as_os_str());
                if std::fs::symlink_metadata(&candidate)?
                    .file_type()
                    .is_symlink()
                {
                    return Err(invalid("source symlink"));
                }
            }
            let file = open_no_follow(&candidate)?;
            let metadata = file.metadata()?;
            if !metadata.is_file() || metadata.len() > MAX_MODULE_BYTES {
                return Err(invalid("invalid source file"));
            }
            if !objects.insert(file_identity(&metadata)) {
                return Err(invalid("duplicate source object"));
            }
            let mut bytes = Vec::new();
            file.take(MAX_MODULE_BYTES + 1).read_to_end(&mut bytes)?;
            let actual: [u8; 32] = Sha256::digest(&bytes).into();
            if bytes.len() as u64 > MAX_MODULE_BYTES || actual != source.sha256 {
                return Err(invalid("source digest mismatch"));
            }
            total = total
                .checked_add(bytes.len())
                .ok_or_else(|| invalid("source size overflow"))?;
            if total > MAX_TOTAL_BYTES {
                return Err(invalid("source size limit"));
            }
            entries.insert(source.path.clone(), Arc::<[u8]>::from(bytes));
        }
        let mut hash = Sha256::new();
        hash.update(b"swallowtail-captured-sources-v1\0");
        for (path, bytes) in &entries {
            let name = path
                .to_str()
                .ok_or_else(|| invalid("non-UTF8 source path"))?;
            hash.update((name.len() as u64).to_be_bytes());
            hash.update(name.as_bytes());
            hash.update((bytes.len() as u64).to_be_bytes());
            hash.update(Sha256::digest(bytes));
        }
        Ok(Self {
            entries,
            digest: hash.finalize().into(),
        })
    }

    /// Returns a captured member by its exact relative manifest path.
    #[must_use]
    pub fn source(&self, relative: &Path) -> Option<&[u8]> {
        self.entries.get(relative).map(AsRef::as_ref)
    }

    /// Returns a deterministic fingerprint of captured names and bytes.
    ///
    /// This fingerprint is not a checked-launch or interpreter attestation.
    #[must_use]
    pub const fn fingerprint(&self) -> &[u8; 32] {
        &self.digest
    }
}

impl fmt::Debug for LocalCapturedSources {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCapturedSources")
            .field("source_count", &self.entries.len())
            .finish_non_exhaustive()
    }
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(unix)]
fn open_no_follow(path: &Path) -> io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NOFOLLOW)
        .open(path)
}

#[cfg(not(unix))]
fn open_no_follow(_path: &Path) -> io::Result<std::fs::File> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "source capture is unavailable",
    ))
}

#[cfg(unix)]
fn file_identity(metadata: &std::fs::Metadata) -> (u64, u64) {
    use std::os::unix::fs::MetadataExt;
    (metadata.dev(), metadata.ino())
}

#[cfg(not(unix))]
fn file_identity(_metadata: &std::fs::Metadata) -> (u64, u64) {
    (0, 0)
}

/// One host-supplied directed import in a captured source graph.
#[derive(Clone)]
pub struct LocalCapturedImport {
    parent: PathBuf,
    specifier: String,
    target: PathBuf,
}

impl LocalCapturedImport {
    /// Creates a literal import edge between captured relative paths.
    #[must_use]
    pub fn new(
        parent: impl Into<PathBuf>,
        specifier: impl Into<String>,
        target: impl Into<PathBuf>,
    ) -> Self {
        Self {
            parent: parent.into(),
            specifier: specifier.into(),
            target: target.into(),
        }
    }
}

/// Immutable loader configuration and fingerprint for captured source bytes.
///
/// The graph is only a host-provided candidate. It is not an approved launch,
/// a complete provider closure, or proof of interpreter custody.
#[derive(Clone)]
pub struct LocalCapturedGraph {
    configuration: Arc<[u8]>,
    digest: [u8; 32],
}

impl LocalCapturedGraph {
    /// Binds a captured entry, idle module and explicit literal import edges.
    ///
    /// Rejects unknown endpoints, duplicate edges, disconnected sources and
    /// URLs that the candidate loader cannot safely represent. Node builtins
    /// and other uncaptured imports are not implicitly admitted.
    pub fn new(
        sources: &LocalCapturedSources,
        entry: &Path,
        idle: &Path,
        imports: &[LocalCapturedImport],
    ) -> io::Result<Self> {
        if imports.len() > 8192
            || !sources.entries.contains_key(entry)
            || !sources.entries.contains_key(idle)
        {
            return Err(invalid("invalid captured graph"));
        }
        let mut files = Vec::with_capacity(sources.entries.len());
        for (path, bytes) in &sources.entries {
            let source = std::str::from_utf8(bytes).map_err(|_| invalid("non-UTF8 source"))?;
            files.push(serde_json::json!({ "url": virtual_url(path)?, "source": source }));
        }
        let entry_url = virtual_url(entry)?;
        let idle_url = virtual_url(idle)?;
        let mut graph = BTreeMap::<(PathBuf, String), PathBuf>::new();
        for import in imports {
            if !sources.entries.contains_key(&import.parent)
                || !sources.entries.contains_key(&import.target)
                || import.specifier.is_empty()
                || import.specifier.len() > 4096
                || import.specifier.contains('\0')
            {
                return Err(invalid("invalid captured import"));
            }
            if graph
                .insert(
                    (import.parent.clone(), import.specifier.clone()),
                    import.target.clone(),
                )
                .is_some()
            {
                return Err(invalid("duplicate captured import"));
            }
        }
        let mut reached = BTreeSet::from([entry.to_path_buf()]);
        loop {
            let old = reached.len();
            for ((parent, _), target) in &graph {
                if reached.contains(parent) {
                    reached.insert(target.clone());
                }
            }
            if reached.len() == old {
                break;
            }
        }
        if reached.len() != sources.entries.len() || !reached.contains(idle) {
            return Err(invalid("disconnected captured source"));
        }
        let mut edges = vec![
            serde_json::json!({ "parent": null, "specifier": entry_url, "target": entry_url }),
        ];
        for ((parent, specifier), target) in graph {
            edges.push(serde_json::json!({ "parent": virtual_url(&parent)?, "specifier": specifier, "target": virtual_url(&target)? }));
        }
        let configuration = serde_json::to_vec(
            &serde_json::json!({ "idleUrl": idle_url, "files": files, "edges": edges }),
        )?;
        let mut hash = Sha256::new();
        hash.update(b"swallowtail-captured-graph-v1\0");
        hash.update(&configuration);
        Ok(Self {
            configuration: Arc::from(configuration),
            digest: hash.finalize().into(),
        })
    }

    /// Returns immutable configuration bytes for the captured-source loader.
    #[must_use]
    pub fn configuration(&self) -> &[u8] {
        &self.configuration
    }

    /// Returns a deterministic graph fingerprint, not a host attestation.
    #[must_use]
    pub const fn fingerprint(&self) -> &[u8; 32] {
        &self.digest
    }
}

impl fmt::Debug for LocalCapturedGraph {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCapturedGraph")
            .finish_non_exhaustive()
    }
}

fn virtual_url(path: &Path) -> io::Result<String> {
    let name = path
        .to_str()
        .ok_or_else(|| invalid("non-UTF8 source path"))?;
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'_' | b'-' | b'.'))
    {
        return Err(invalid("source path cannot be represented"));
    }
    Ok(format!("file:///swallowtail-capture/{name}"))
}

/// Candidate digest binding for a future host-verified guarded launch.
///
/// The inputs are supplied by host policy and are not verified by this type;
/// constructing it cannot issue or authorize a checked-process reference.
#[derive(Clone)]
pub struct LocalCapturedLaunchFingerprint([u8; 32]);

impl LocalCapturedLaunchFingerprint {
    /// Binds the captured graph to vendor, shim, runtime, recipe and policy digests.
    ///
    /// The host must separately establish and enforce each digest, including
    /// the selected runtime-custody policy, before admitting a launch.
    pub fn new(
        graph: &LocalCapturedGraph,
        vendor: [u8; 32],
        shim: [u8; 32],
        runtime: [u8; 32],
        recipe: [u8; 32],
        policy: [u8; 32],
    ) -> io::Result<Self> {
        let fields = [vendor, shim, runtime, recipe, policy];
        if fields.iter().any(|digest| *digest == [0; 32]) {
            return Err(invalid("missing launch digest"));
        }
        let mut hash = Sha256::new();
        hash.update(b"swallowtail-checked-launch-candidate-v1\0");
        hash.update(graph.fingerprint());
        for field in fields {
            hash.update(field);
        }
        Ok(Self(hash.finalize().into()))
    }

    /// Returns the candidate fingerprint; it is not an attestation.
    #[must_use]
    pub const fn digest(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for LocalCapturedLaunchFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCapturedLaunchFingerprint")
            .finish_non_exhaustive()
    }
}
