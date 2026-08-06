use crate::ValueRequired;
use std::num::NonZeroU32;

macro_rules! evidence_text {
    ($name:ident, $field:literal) => {
        #[doc = concat!("Validated, non-empty ", $field, ".")]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a ", $field, " after rejecting blank text.")]
            pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(ValueRequired::for_field($field));
                }
                Ok(Self(value))
            }

            /// Returns the validated evidence text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

evidence_text!(RemoteAcpRfdRevision, "remote ACP RFD revision");
evidence_text!(
    RemoteAcpTransportSdkVersion,
    "remote ACP transport SDK version"
);
evidence_text!(RemoteAcpCoreSdkVersion, "remote ACP core SDK version");

/// Qualification status of the remote ACP transport design.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RemoteAcpRfdStatus {
    /// The selected RFD revision is active.
    Active,
}

/// Exact wire, design, and SDK version evidence for remote ACP.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteAcpVersionEvidence {
    wire_version: NonZeroU32,
    rfd_revision: RemoteAcpRfdRevision,
    rfd_status: RemoteAcpRfdStatus,
    transport_sdk_version: RemoteAcpTransportSdkVersion,
    core_sdk_version: RemoteAcpCoreSdkVersion,
}

impl RemoteAcpVersionEvidence {
    /// Creates one internally consistent version-evidence record.
    #[must_use]
    pub const fn new(
        wire_version: NonZeroU32,
        rfd_revision: RemoteAcpRfdRevision,
        rfd_status: RemoteAcpRfdStatus,
        transport_sdk_version: RemoteAcpTransportSdkVersion,
        core_sdk_version: RemoteAcpCoreSdkVersion,
    ) -> Self {
        Self {
            wire_version,
            rfd_revision,
            rfd_status,
            transport_sdk_version,
            core_sdk_version,
        }
    }

    /// Returns the negotiated ACP wire version.
    #[must_use]
    pub const fn wire_version(&self) -> NonZeroU32 {
        self.wire_version
    }

    /// Returns the selected RFD revision.
    #[must_use]
    pub const fn rfd_revision(&self) -> &RemoteAcpRfdRevision {
        &self.rfd_revision
    }

    /// Returns the RFD qualification status.
    #[must_use]
    pub const fn rfd_status(&self) -> RemoteAcpRfdStatus {
        self.rfd_status
    }

    /// Returns the remote transport SDK version.
    #[must_use]
    pub const fn transport_sdk_version(&self) -> &RemoteAcpTransportSdkVersion {
        &self.transport_sdk_version
    }

    /// Returns the core ACP SDK version.
    #[must_use]
    pub const fn core_sdk_version(&self) -> &RemoteAcpCoreSdkVersion {
        &self.core_sdk_version
    }
}
