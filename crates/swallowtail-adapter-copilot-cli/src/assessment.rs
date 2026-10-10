//! Crate-internal exact-point qualification assessment admission.

use sha2::{Digest, Sha256};
use std::fmt;
use std::io::Read;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{EnvironmentRef, ExecutableRef};

pub(crate) const WRAPPER_ARCHIVE_SHA256: &str =
    "838be5537db8bd0f7c7c2e555063770ab559249cf2c3ff3c23b2efc81c854779";
pub(crate) const NATIVE_ARCHIVE_SHA256: &str =
    "95d49e3023921f0bf694e75e17591b68f8b1e59942c2606e1bc1379828d72b2f";
pub(crate) const NATIVE_EXECUTABLE_SHA256: &str =
    "35d33e040e8aa0554a02385f02648396ba3b853f026ed1db17d30d051a764e4b";

/// Safe, exact host inputs captured before the assessment's first process.
///
/// Values are retained for binding checks but are never formatted or serialized
/// by this module. The hashes are suitable for a secret-free execution record.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct CopilotCliAssessmentHostBinding {
    pub(crate) execution_host_id: String,
    pub(crate) execution_host_id_sha256: String,
    pub(crate) executable_ref: String,
    pub(crate) executable_ref_sha256: String,
    pub(crate) environment_ref: String,
    pub(crate) environment_ref_sha256: String,
    pub(crate) executable_bytes_sha256: String,
    pub(crate) wrapper_archive_sha256: &'static str,
    pub(crate) native_archive_sha256: &'static str,
}

impl fmt::Debug for CopilotCliAssessmentHostBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CopilotCliAssessmentHostBinding")
            .field("execution_host_id_sha256", &self.execution_host_id_sha256)
            .field("executable_ref_sha256", &self.executable_ref_sha256)
            .field("environment_ref_sha256", &self.environment_ref_sha256)
            .field("executable_bytes_sha256", &self.executable_bytes_sha256)
            .finish_non_exhaustive()
    }
}

/// Hashes the executable bytes read from the host-approved executable reference.
pub(crate) fn capture_host_binding(
    execution_host_id: &ExecutionHostId,
    executable_ref: &ExecutableRef,
    environment_ref: &EnvironmentRef,
    executable: impl Read,
    expected_executable_sha256: &str,
) -> Result<CopilotCliAssessmentHostBinding, CaptureError> {
    // Snapshot the safe host-selected inputs before reading executable bytes or
    // entering the prepared discovery path.
    let execution_host_id = execution_host_id.as_str().to_owned();
    let executable_ref = executable_ref.as_host_value().to_owned();
    let environment_ref = environment_ref.as_host_value().to_owned();
    let execution_host_id_sha256 = sha256_text(&execution_host_id);
    let executable_ref_sha256 = sha256_text(&executable_ref);
    let environment_ref_sha256 = sha256_text(&environment_ref);
    let executable_bytes_sha256 = sha256_reader(executable)?;
    if executable_bytes_sha256 != expected_executable_sha256 {
        return Err(CaptureError::ExecutableDigestMismatch);
    }

    Ok(CopilotCliAssessmentHostBinding {
        execution_host_id,
        execution_host_id_sha256,
        executable_ref,
        executable_ref_sha256,
        environment_ref,
        environment_ref_sha256,
        executable_bytes_sha256,
        wrapper_archive_sha256: WRAPPER_ARCHIVE_SHA256,
        native_archive_sha256: NATIVE_ARCHIVE_SHA256,
    })
}

/// Computes a lowercase SHA-256 digest without loading the executable at once.
pub(crate) fn sha256_reader(mut reader: impl Read) -> Result<String, CaptureError> {
    let mut digest = Sha256::new();
    let mut buffer = [0; 64 * 1024];
    loop {
        let read = reader.read(&mut buffer).map_err(|_| CaptureError::Read)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(hex_digest(&digest.finalize()))
}

fn sha256_text(value: &str) -> String {
    hex_digest(&Sha256::digest(value.as_bytes()))
}

fn hex_digest(digest: &[u8]) -> String {
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut output, "{byte:02x}").expect("writing into a String cannot fail");
    }
    output
}

pub(crate) enum CaptureError {
    Read,
    ExecutableDigestMismatch,
}

impl fmt::Debug for CaptureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => formatter.write_str("CaptureError::Read(<redacted>)"),
            Self::ExecutableDigestMismatch => {
                formatter.write_str("CaptureError::ExecutableDigestMismatch")
            }
        }
    }
}

pub(crate) const fn admission() -> crate::selection::CopilotCliPlanAdmission {
    crate::selection::CopilotCliPlanAdmission::PrivateAssessment095
}

#[cfg(test)]
mod tests {
    use super::sha256_reader;

    #[test]
    fn executable_digest_is_computed_from_streamed_bytes() {
        assert_eq!(
            sha256_reader(&b"abc"[..]).expect("fixture bytes hash"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
