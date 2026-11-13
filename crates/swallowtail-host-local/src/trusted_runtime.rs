//! Trusted-host runtime custody for a future checked launch.
//!
//! This pins a host-selected file; it is not a checked-process grant or an
//! operating-system guarantee against a hostile same-user replacement.

use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

const MAX_RUNTIME: u64 = 512 * 1024 * 1024;

/// A runtime file pinned by an execution host that controls its custody.
///
/// The trusted host must prevent replacement between verification and spawn.
/// This type alone grants no process authority and cannot attest a caller.
#[derive(Clone)]
pub struct LocalTrustedRuntime {
    inner: Arc<RuntimeInner>,
}

struct RuntimeInner {
    path: PathBuf,
    file: Mutex<File>,
    ancestors: Vec<(PathBuf, DirectoryIdentity)>,
    digest: [u8; 32],
    dependencies: Vec<LocalTrustedRuntime>,
    executable: bool,
    source_digest: [u8; 32],
    revoked: AtomicBool,
}

impl LocalTrustedRuntime {
    pub(crate) fn same_custody(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
    pub(crate) fn native_binding(&self) -> io::Result<serde_json::Value> {
        self.verify()?;
        let name = self
            .inner
            .path
            .to_str()
            .ok_or_else(|| invalid("non-UTF8 native path"))?;
        if !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'_' | b'-' | b'.'))
        {
            return Err(invalid("native path cannot be represented"));
        }
        let metadata = self
            .inner
            .file
            .lock()
            .map_err(|_| invalid("runtime custody unavailable"))?
            .metadata()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let ancestors:Vec<_>=self.inner.ancestors.iter().map(|(path,(dev,ino,uid,mode))|serde_json::json!({"path":path,"dev":dev.to_string(),"ino":ino.to_string(),"uid":uid,"mode":mode})).collect();
            Ok(
                serde_json::json!({"path":name,"url":format!("file://{name}"),"sha256":self.inner.source_digest.iter().map(|b| format!("{b:02x}")).collect::<String>(),"bytes":metadata.len(),"dev":metadata.dev().to_string(),"ino":metadata.ino().to_string(),"uid":metadata.uid(),"ancestors":ancestors}),
            )
        }
        #[cfg(not(unix))]
        {
            let _ = metadata;
            Err(invalid("native custody unavailable"))
        }
    }

    pub(crate) fn acquire_custody(&self) -> io::Result<std::sync::MutexGuard<'_, File>> {
        let mut held = self
            .inner
            .file
            .lock()
            .map_err(|_| invalid("runtime custody unavailable"))?;
        self.verify_held(&mut held)?;
        Ok(held)
    }

    pub(crate) fn system_ps_binding(&self, platform: &str) -> io::Result<serde_json::Value> {
        let expected = match platform {
            "darwin-arm64" => "/bin/ps",
            "linux-x64" => "/usr/bin/ps",
            _ => return Err(invalid("unclassified platform program")),
        };
        let binding = self.native_binding()?;
        if binding["path"] != expected {
            return Err(invalid("system process helper mismatch"));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if self
                .inner
                .file
                .lock()
                .map_err(|_| invalid("runtime custody unavailable"))?
                .metadata()?
                .uid()
                != 0
            {
                return Err(invalid("unowned system process helper"));
            }
        }
        Ok(binding)
    }
    /// Opens and verifies one absolute canonical runtime selected by the host.
    ///
    /// Returns an error for links, non-regular files, oversized files, or a
    /// digest mismatch. Callers must uphold the host custody precondition.
    pub fn capture(path: &Path, digest: [u8; 32]) -> io::Result<Self> {
        Self::capture_file(path, digest, true)
    }

    /// Pins the executable and the host-selected non-system loader closure.
    ///
    /// The trusted host must establish completeness and protect all selected
    /// files, including system loader dependencies, through spawn. This does
    /// not discover dependencies or establish operating-system confinement.
    pub fn capture_with_dependencies(
        path: &Path,
        digest: [u8; 32],
        dependencies: &[(PathBuf, [u8; 32])],
    ) -> io::Result<Self> {
        if dependencies.len() > 128 {
            return Err(invalid("runtime dependency limit"));
        }
        let mut runtime = Self::capture_file(path, digest, true)?;
        let mut seen = std::collections::BTreeSet::from([path.to_path_buf()]);
        let mut pins = Vec::new();
        let mut identity = Sha256::new();
        identity.update(b"swallowtail-trusted-runtime-closure-v1\0");
        identity.update(digest);
        // Identity is independent of caller ordering, but includes exact host paths.
        let mut ordered = dependencies.to_vec();
        ordered.sort_by(|a, b| a.0.cmp(&b.0));
        for (dependency, digest) in ordered {
            if !seen.insert(dependency.clone()) {
                return Err(invalid("duplicate runtime dependency"));
            }
            let pin = Self::capture_file(&dependency, digest, false)?;
            let name = dependency.as_os_str().as_encoded_bytes();
            identity.update((name.len() as u64).to_be_bytes());
            identity.update(name);
            identity.update(digest);
            pins.push(pin);
        }
        let inner = Arc::get_mut(&mut runtime.inner)
            .ok_or_else(|| invalid("runtime custody unavailable"))?;
        inner.digest = identity.finalize().into();
        inner.dependencies = pins;
        Ok(runtime)
    }

    fn capture_file(path: &Path, digest: [u8; 32], executable: bool) -> io::Result<Self> {
        if !path.is_absolute() || path.canonicalize()? != path {
            return Err(invalid("runtime path is not canonical"));
        }
        let ancestors = capture_ancestors(path)?;
        let mut file = open_no_follow(path)?;
        let metadata = file.metadata()?;
        if !metadata.is_file()
            || !trusted_file(&metadata, executable)
            || metadata.len() > MAX_RUNTIME
        {
            return Err(invalid("invalid runtime file"));
        }
        if digest_file(&mut file)? != digest {
            return Err(invalid("runtime digest mismatch"));
        }
        Ok(Self {
            inner: Arc::new(RuntimeInner {
                path: path.to_path_buf(),
                file: Mutex::new(file),
                ancestors,
                digest,
                dependencies: Vec::new(),
                executable,
                source_digest: digest,
                revoked: AtomicBool::new(false),
            }),
        })
    }

    /// Rechecks the host-held file and its current approved path.
    ///
    /// This does not itself spawn a process or close the trusted-host race.
    pub fn verify(&self) -> io::Result<()> {
        let mut held = self
            .inner
            .file
            .lock()
            .map_err(|_| invalid("runtime custody unavailable"))?;
        self.verify_held(&mut held)
    }

    fn verify_held(&self, held: &mut File) -> io::Result<()> {
        if self.inner.revoked.load(Ordering::Acquire) {
            return Err(invalid("runtime custody revoked"));
        }
        if capture_ancestors(&self.inner.path)? != self.inner.ancestors {
            return Err(invalid("runtime ancestor custody changed"));
        }
        if self.inner.path.canonicalize()? != self.inner.path {
            return Err(invalid("runtime path is not canonical"));
        }
        let current = open_no_follow(&self.inner.path)?;
        let held_metadata = held.metadata()?;
        let current_metadata = current.metadata()?;
        if !trusted_file(&held_metadata, self.inner.executable)
            || !trusted_file(&current_metadata, self.inner.executable)
            || !same_file(&held_metadata, &current_metadata)
            || digest_file(held)? != self.inner.source_digest
        {
            return Err(invalid("runtime custody changed"));
        }
        for dependency in &self.inner.dependencies {
            dependency.verify()?;
        }
        if self.inner.revoked.load(Ordering::Acquire) {
            return Err(invalid("runtime custody revoked"));
        }
        Ok(())
    }

    /// Verifies that a host launch names this runtime and enables the Node
    /// startup environment filter. Prefix and graph admission are separate.
    pub fn verify_launch(&self, launch: &crate::LocalExecutableLaunch) -> io::Result<()> {
        if launch.program() != self.inner.path || !launch.rejects_node_startup_environment() {
            return Err(invalid("runtime launch mismatch"));
        }
        self.verify()
    }

    pub(crate) fn during_spawn<T>(
        &self,
        launch: &crate::LocalExecutableLaunch,
        spawn: impl FnOnce() -> T,
    ) -> io::Result<T> {
        if launch.program() != self.inner.path || !launch.rejects_node_startup_environment() {
            return Err(invalid("runtime launch mismatch"));
        }
        let mut held = self
            .inner
            .file
            .lock()
            .map_err(|_| invalid("runtime custody unavailable"))?;
        self.verify_held(&mut held)?;
        Ok(spawn())
    }

    /// Revokes every clone of this host-held identity.
    pub fn revoke(&self) {
        let _held = self
            .inner
            .file
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        self.inner.revoked.store(true, Ordering::Release);
    }

    /// Returns the pinned runtime identity, not a process attestation.
    /// For dependency-aware capture this binds the entire supplied closure.
    #[must_use]
    pub fn digest(&self) -> &[u8; 32] {
        &self.inner.digest
    }

    /// Returns the verified executable's digest separately from its dependency identity.
    #[must_use]
    pub fn source_digest(&self) -> &[u8; 32] {
        &self.inner.source_digest
    }
}

impl fmt::Debug for LocalTrustedRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalTrustedRuntime")
            .finish_non_exhaustive()
    }
}

fn digest_file(file: &mut File) -> io::Result<[u8; 32]> {
    file.seek(SeekFrom::Start(0))?;
    let mut hasher = Sha256::new();
    let mut count = 0u64;
    let mut buffer = [0u8; 16 * 1024];
    loop {
        let len = file.read(&mut buffer)?;
        if len == 0 {
            break;
        }
        count += len as u64;
        if count > MAX_RUNTIME {
            return Err(invalid("runtime exceeds limit"));
        }
        hasher.update(&buffer[..len]);
    }
    Ok(hasher.finalize().into())
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(unix)]
pub(crate) type DirectoryIdentity = (u64, u64, u32, u32);
#[cfg(not(unix))]
pub(crate) type DirectoryIdentity = ();

#[cfg(unix)]
pub(crate) fn capture_ancestors(path: &Path) -> io::Result<Vec<(PathBuf, DirectoryIdentity)>> {
    use std::os::unix::fs::MetadataExt;
    path.ancestors()
        .skip(1)
        .map(|directory| {
            let metadata = std::fs::symlink_metadata(directory)?;
            let mode = metadata.mode();
            if !metadata.is_dir()
                || (metadata.uid() != 0 && metadata.uid() != nix::unistd::geteuid().as_raw())
                // Root-owned sticky temporary parents protect each owned
                // child directory from other users; record them as ancestors.
                || (mode & 0o022 != 0 && !(metadata.uid() == 0 && mode & 0o1000 != 0))
            {
                return Err(invalid("untrusted runtime ancestor"));
            }
            Ok((
                directory.to_path_buf(),
                (metadata.dev(), metadata.ino(), metadata.uid(), mode),
            ))
        })
        .collect()
}

#[cfg(not(unix))]
pub(crate) fn capture_ancestors(_: &Path) -> io::Result<Vec<(PathBuf, DirectoryIdentity)>> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "runtime custody unavailable",
    ))
}

#[cfg(unix)]
fn open_no_follow(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NOFOLLOW)
        .open(path)
}
#[cfg(not(unix))]
fn open_no_follow(_: &Path) -> io::Result<File> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "runtime custody unavailable",
    ))
}

#[cfg(unix)]
fn same_file(a: &std::fs::Metadata, b: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    a.is_file() && b.is_file() && a.dev() == b.dev() && a.ino() == b.ino()
}
#[cfg(not(unix))]
fn same_file(_: &std::fs::Metadata, _: &std::fs::Metadata) -> bool {
    false
}

#[cfg(unix)]
pub(crate) fn trusted_file(metadata: &std::fs::Metadata, executable: bool) -> bool {
    use std::os::unix::fs::MetadataExt;
    let mode = metadata.mode();
    (metadata.uid() == 0 || metadata.uid() == nix::unistd::geteuid().as_raw())
        && (!executable || mode & 0o111 != 0)
        && mode & 0o022 == 0
}
#[cfg(not(unix))]
pub(crate) fn trusted_file(_: &std::fs::Metadata, _: bool) -> bool {
    false
}
