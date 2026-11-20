//! Host-neutral descriptors for one frozen guarded capture.
//!
//! An adapter owns the frozen provider asset — the module graph, the loader
//! source and the identity digests a route pins. A host owns file capture,
//! interpreter custody and the checked-process grant that follows from them.
//! These types are the data both sides share, so no adapter names a local-host
//! type and no host names a provider asset.
//!
//! Neither type captures bytes, verifies custody, or authorizes a launch.
//! Capturing the declared sources and deriving a recipe stay with the host
//! port that owns the filesystem.

use std::path::{Path, PathBuf};

/// One host-policy relative source path and its expected SHA-256.
///
/// The digest is an input to capture, not an attestation of vendor identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedSource {
    path: PathBuf,
    sha256: [u8; 32],
}

impl ExpectedSource {
    /// Creates one relative source specification.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, sha256: [u8; 32]) -> Self {
        Self {
            path: path.into(),
            sha256,
        }
    }

    /// Returns the manifest-relative source path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the expected digest of the captured bytes.
    #[must_use]
    pub const fn sha256(&self) -> &[u8; 32] {
        &self.sha256
    }
}

/// Frozen, host-independent inputs for one bounded guarded capture.
///
/// The manifest and the loader are the exact bytes the adapter froze for one
/// hop and platform. `vendor_sha256` and `policy_sha256` are the adapter's
/// declared identity and launch-policy inputs; the host re-derives and
/// enforces them independently, so restating them here grants nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrozenGuardedNodeCapture<'a> {
    platform: &'a str,
    manifest: &'a [u8],
    loader: &'a str,
    vendor_sha256: [u8; 32],
    policy_sha256: [u8; 32],
}

impl<'a> FrozenGuardedNodeCapture<'a> {
    /// Binds one frozen capture to its platform and identity digests.
    #[must_use]
    pub const fn new(
        platform: &'a str,
        manifest: &'a [u8],
        loader: &'a str,
        vendor_sha256: [u8; 32],
        policy_sha256: [u8; 32],
    ) -> Self {
        Self {
            platform,
            manifest,
            loader,
            vendor_sha256,
            policy_sha256,
        }
    }

    /// Returns the adapter-admitted platform label.
    #[must_use]
    pub const fn platform(self) -> &'a str {
        self.platform
    }

    /// Returns the frozen graph manifest bytes.
    #[must_use]
    pub const fn manifest(self) -> &'a [u8] {
        self.manifest
    }

    /// Returns the frozen loader source.
    #[must_use]
    pub const fn loader(self) -> &'a str {
        self.loader
    }

    /// Returns the declared vendor identity digest.
    #[must_use]
    pub const fn vendor_sha256(self) -> [u8; 32] {
        self.vendor_sha256
    }

    /// Returns the declared launch-policy digest.
    #[must_use]
    pub const fn policy_sha256(self) -> [u8; 32] {
        self.policy_sha256
    }
}

#[cfg(test)]
mod tests {
    use super::{ExpectedSource, FrozenGuardedNodeCapture};
    use std::path::Path;

    #[test]
    fn expected_source_keeps_its_relative_path_and_digest() {
        let source = ExpectedSource::new("acp/index.js", [7; 32]);
        assert_eq!(source.path(), Path::new("acp/index.js"));
        assert_eq!(source.sha256(), &[7; 32]);
    }

    #[test]
    fn frozen_capture_reports_only_the_declared_inputs() {
        let frozen = FrozenGuardedNodeCapture::new(
            "linux-x64",
            b"{\"vendor\":\"fixture\"}",
            "export {};",
            [1; 32],
            [2; 32],
        );
        assert_eq!(frozen.platform(), "linux-x64");
        assert_eq!(frozen.manifest(), b"{\"vendor\":\"fixture\"}");
        assert_eq!(frozen.loader(), "export {};");
        assert_eq!(frozen.vendor_sha256(), [1; 32]);
        assert_eq!(frozen.policy_sha256(), [2; 32]);
    }
}
