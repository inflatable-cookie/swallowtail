//! Trusted-host runtime custody for a future checked launch.
//!
//! This pins a host-selected file; it is not a checked-process grant or an
//! operating-system guarantee against a hostile same-user replacement.

use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

const MAX_RUNTIME: u64 = 256 * 1024 * 1024;

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
    digest: [u8; 32],
    revoked: AtomicBool,
}

impl LocalTrustedRuntime {
    /// Opens and verifies one absolute canonical runtime selected by the host.
    ///
    /// Returns an error for links, non-regular files, oversized files, or a
    /// digest mismatch. Callers must uphold the host custody precondition.
    pub fn capture(path: &Path, digest: [u8; 32]) -> io::Result<Self> {
        if !path.is_absolute() || path.canonicalize()? != path {
            return Err(invalid("runtime path is not canonical"));
        }
        let mut file = open_no_follow(path)?;
        let metadata = file.metadata()?;
        if !metadata.is_file() || !executable(&metadata) || metadata.len() > MAX_RUNTIME {
            return Err(invalid("invalid runtime file"));
        }
        if digest_file(&mut file)? != digest {
            return Err(invalid("runtime digest mismatch"));
        }
        Ok(Self {
            inner: Arc::new(RuntimeInner {
                path: path.to_path_buf(),
                file: Mutex::new(file),
                digest,
                revoked: AtomicBool::new(false),
            }),
        })
    }

    /// Rechecks the host-held file and its current approved path.
    ///
    /// This does not itself spawn a process or close the trusted-host race.
    pub fn verify(&self) -> io::Result<()> {
        if self.inner.revoked.load(Ordering::Acquire) {
            return Err(invalid("runtime custody revoked"));
        }
        if self.inner.path.canonicalize()? != self.inner.path {
            return Err(invalid("runtime path is not canonical"));
        }
        let mut held = self
            .inner
            .file
            .lock()
            .map_err(|_| invalid("runtime custody unavailable"))?;
        let current = open_no_follow(&self.inner.path)?;
        let held_metadata = held.metadata()?;
        let current_metadata = current.metadata()?;
        if !executable(&held_metadata)
            || !executable(&current_metadata)
            || !same_file(&held_metadata, &current_metadata)
            || digest_file(&mut held)? != self.inner.digest
        {
            return Err(invalid("runtime custody changed"));
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

    /// Revokes every clone of this host-held identity.
    pub fn revoke(&self) {
        self.inner.revoked.store(true, Ordering::Release);
    }

    /// Returns the pinned content digest, not a process attestation.
    #[must_use]
    pub fn digest(&self) -> &[u8; 32] {
        &self.inner.digest
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
fn executable(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}
#[cfg(not(unix))]
fn executable(_: &std::fs::Metadata) -> bool {
    false
}
