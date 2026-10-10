//! Crate-internal exact-point qualification assessment admission.

use crate::selection::CopilotCliPlanAdmission;

pub(crate) const WRAPPER_ARCHIVE_SHA256: &str =
    "838be5537db8bd0f7c7c2e555063770ab559249cf2c3ff3c23b2efc81c854779";
pub(crate) const NATIVE_ARCHIVE_SHA256: &str =
    "95d49e3023921f0bf694e75e17591b68f8b1e59942c2606e1bc1379828d72b2f";
pub(crate) const NATIVE_EXECUTABLE_SHA256: &str =
    "35d33e040e8aa0554a02385f02648396ba3b853f026ed1db17d30d051a764e4b";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CopilotCliAssessmentArtifactIdentity {
    pub(crate) wrapper_archive_sha256: String,
    pub(crate) native_archive_sha256: String,
    pub(crate) native_executable_sha256: String,
}

impl CopilotCliAssessmentArtifactIdentity {
    pub(crate) fn frozen_1_0_95() -> Self {
        Self {
            wrapper_archive_sha256: WRAPPER_ARCHIVE_SHA256.to_owned(),
            native_archive_sha256: NATIVE_ARCHIVE_SHA256.to_owned(),
            native_executable_sha256: NATIVE_EXECUTABLE_SHA256.to_owned(),
        }
    }

    pub(crate) fn matches_frozen_target(&self) -> bool {
        self.wrapper_archive_sha256 == WRAPPER_ARCHIVE_SHA256
            && self.native_archive_sha256 == NATIVE_ARCHIVE_SHA256
            && self.native_executable_sha256 == NATIVE_EXECUTABLE_SHA256
    }
}

pub(crate) const fn admission() -> CopilotCliPlanAdmission {
    CopilotCliPlanAdmission::PrivateAssessment095
}
