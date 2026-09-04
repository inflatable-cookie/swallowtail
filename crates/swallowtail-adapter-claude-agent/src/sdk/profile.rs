//! Explicit tool admission and permission mode for one SDK sidecar session.
//!
//! The profile is the only place this route decides what a session may reach.
//! It is fixed before preparation, carried into the plan as an exact resource
//! access constraint, echoed by the sidecar at open, and enforced again on
//! every admission request. Nothing widens it later: a mid-session permission
//! mode change moves inside the admitted set and never adds a tool.
//!
//! Two upstream values are structurally unreachable rather than merely
//! discouraged. `bypassPermissions` and every other auto-approving mode is
//! rejected while parsing, before a plan exists, and unknown tool names are
//! rejected the same way. Neither can reach the SDK.
//!
//! The write set is admitted end to end. Contract 013 keys its consumer-tool
//! exclusion on a bounded profile's claimed filesystem boundary, and this
//! route claims none, so an ambient read-write session with consumer-mediated
//! tool calls is admissible. A write tool binds `ResourceAccess::ReadWrite`
//! into the plan, and a host that grants less fails the lease agreement at
//! open, so no write ever reaches a read-only working resource.

use super::prepared::preparation_failure;
use swallowtail_core::ResourceAccess;
use swallowtail_runtime::{PreparationFailure, PreparationStage};

/// One tool this route can admit on a prepared SDK sidecar session.
///
/// The read set is the `v0.4.0` default. The write set is admissible only on
/// a read-write working-resource lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClaudeAgentSdkTool {
    /// Read one file under the leased working resource.
    Read,
    /// Match paths under the leased working resource.
    Glob,
    /// Search content under the leased working resource.
    Grep,
    /// Replace one span in one file.
    Edit,
    /// Write one whole file.
    Write,
    /// Replace several spans in one file.
    MultiEdit,
}

/// Every tool this route admits, in the exact wire order it sends them.
const ADMISSIBLE_TOOLS: [ClaudeAgentSdkTool; 6] = [
    ClaudeAgentSdkTool::Read,
    ClaudeAgentSdkTool::Glob,
    ClaudeAgentSdkTool::Grep,
    ClaudeAgentSdkTool::Edit,
    ClaudeAgentSdkTool::Write,
    ClaudeAgentSdkTool::MultiEdit,
];

impl ClaudeAgentSdkTool {
    /// Returns the exact upstream tool name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "Read",
            Self::Glob => "Glob",
            Self::Grep => "Grep",
            Self::Edit => "Edit",
            Self::Write => "Write",
            Self::MultiEdit => "MultiEdit",
        }
    }

    /// Parses one exact upstream tool name. Any other name, including a tool
    /// this route has not admitted, returns `None`.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        ADMISSIBLE_TOOLS
            .into_iter()
            .find(|tool| tool.as_str() == name)
    }

    /// Reports whether this tool can mutate the leased working resource.
    #[must_use]
    pub const fn mutates_working_resource(self) -> bool {
        matches!(self, Self::Edit | Self::Write | Self::MultiEdit)
    }

    const fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

/// Permission mode this route admits at open and mid-session.
///
/// Upstream also declares `bypassPermissions`, `auto`, and `dontAsk`. All
/// three auto-approve tool use without asking the consumer, so none is
/// representable here and none can be parsed into this type.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ClaudeAgentSdkPermissionMode {
    /// Every admitted tool call is offered to the consumer before it runs.
    #[default]
    Default,
    /// Planning only: the provider proposes rather than acts.
    Plan,
    /// Edits to the leased working resource run without a per-call consumer
    /// decision. Every other admitted tool still goes through `canUseTool`.
    AcceptEdits,
}

impl ClaudeAgentSdkPermissionMode {
    /// Returns the exact upstream permission-mode name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Plan => "plan",
            Self::AcceptEdits => "acceptEdits",
        }
    }

    /// Parses one admitted permission-mode name.
    ///
    /// An auto-approving upstream mode fails with a distinct code from an
    /// unrecognised one, because the two are different consumer mistakes.
    pub fn parse(name: &str) -> Result<Self, PreparationFailure> {
        match name {
            "default" => Ok(Self::Default),
            "plan" => Ok(Self::Plan),
            "acceptEdits" => Ok(Self::AcceptEdits),
            "bypassPermissions" | "auto" | "dontAsk" => Err(profile_failure(
                "swallowtail.claude-agent.sdk.profile.permission_mode_rejected",
                "Claude Agent SDK preparation rejects an auto-approving permission mode",
            )),
            _ => Err(profile_failure(
                "swallowtail.claude-agent.sdk.profile.permission_mode_unknown",
                "Claude Agent SDK preparation admits only default, plan, and acceptEdits",
            )),
        }
    }

    /// Reports whether this mode lets the provider edit without a per-call
    /// consumer decision.
    #[must_use]
    pub const fn skips_edit_admission(self) -> bool {
        matches!(self, Self::AcceptEdits)
    }
}

/// The admitted tool set and permission mode for one prepared session.
///
/// `read_only` is the `v0.4.0` default and is what every existing caller
/// keeps: `Read`, `Glob`, `Grep` under `default` mode on a read-only lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkSessionProfile {
    admitted: u8,
    permission_mode: ClaudeAgentSdkPermissionMode,
}

const READ_ONLY_ADMITTED: u8 = ClaudeAgentSdkTool::Read.bit()
    | ClaudeAgentSdkTool::Glob.bit()
    | ClaudeAgentSdkTool::Grep.bit();

const WRITE_ADMITTED: u8 = ClaudeAgentSdkTool::Edit.bit()
    | ClaudeAgentSdkTool::Write.bit()
    | ClaudeAgentSdkTool::MultiEdit.bit();

impl ClaudeAgentSdkSessionProfile {
    /// The unchanged `v0.4.0` profile: the read set under `default` mode.
    #[must_use]
    pub const fn read_only() -> Self {
        Self {
            admitted: READ_ONLY_ADMITTED,
            permission_mode: ClaudeAgentSdkPermissionMode::Default,
        }
    }

    /// The read set plus `Edit`, `Write`, and `MultiEdit`, which requires a
    /// read-write working-resource lease.
    #[must_use]
    pub const fn read_write(permission_mode: ClaudeAgentSdkPermissionMode) -> Self {
        Self {
            admitted: READ_ONLY_ADMITTED | WRITE_ADMITTED,
            permission_mode,
        }
    }

    /// Builds a profile from an explicit tool set and permission mode.
    ///
    /// An empty set and a repeated tool are both rejected, so the admitted
    /// set the sidecar receives is always exact and canonical.
    pub fn new(
        tools: impl IntoIterator<Item = ClaudeAgentSdkTool>,
        permission_mode: ClaudeAgentSdkPermissionMode,
    ) -> Result<Self, PreparationFailure> {
        let mut admitted = 0_u8;
        for tool in tools {
            if admitted & tool.bit() != 0 {
                return Err(profile_failure(
                    "swallowtail.claude-agent.sdk.profile.tool_repeated",
                    "Claude Agent SDK preparation admits each tool at most once",
                ));
            }
            admitted |= tool.bit();
        }
        if admitted == 0 {
            return Err(profile_failure(
                "swallowtail.claude-agent.sdk.profile.tool_set_empty",
                "Claude Agent SDK preparation requires at least one admitted tool",
            ));
        }
        Ok(Self {
            admitted,
            permission_mode,
        })
    }

    /// Builds a profile from exact upstream names.
    ///
    /// This is the surface a consumer configuration reaches: an unknown tool
    /// name and an auto-approving permission mode both fail here, before any
    /// plan, lease, process, or provider contact exists.
    pub fn from_names<'a>(
        tools: impl IntoIterator<Item = &'a str>,
        permission_mode: &str,
    ) -> Result<Self, PreparationFailure> {
        let mut parsed = Vec::new();
        for name in tools {
            parsed.push(ClaudeAgentSdkTool::parse(name).ok_or_else(|| {
                profile_failure(
                    "swallowtail.claude-agent.sdk.profile.tool_unknown",
                    "Claude Agent SDK preparation rejects a tool outside its admitted set",
                )
            })?);
        }
        Self::new(
            parsed,
            ClaudeAgentSdkPermissionMode::parse(permission_mode)?,
        )
    }

    /// Returns the admitted tools in the exact order the sidecar receives.
    pub fn tools(&self) -> impl Iterator<Item = ClaudeAgentSdkTool> + '_ {
        ADMISSIBLE_TOOLS
            .into_iter()
            .filter(|tool| self.admits(*tool))
    }

    /// Reports whether one tool is admitted on this session.
    #[must_use]
    pub const fn admits(&self, tool: ClaudeAgentSdkTool) -> bool {
        self.admitted & tool.bit() != 0
    }

    /// Reports whether any admitted tool can mutate the working resource.
    #[must_use]
    pub const fn admits_writes(&self) -> bool {
        self.admitted & WRITE_ADMITTED != 0
    }

    /// Returns the permission mode this session opens with.
    #[must_use]
    pub const fn permission_mode(&self) -> ClaudeAgentSdkPermissionMode {
        self.permission_mode
    }

    /// Returns the exact working-resource lease access this profile requires.
    ///
    /// A write tool without a read-write lease is refused: preparation binds
    /// this access into the plan, and the host's own lease must match it
    /// before the sidecar starts.
    #[must_use]
    pub const fn resource_access(&self) -> ResourceAccess {
        if self.admits_writes() {
            ResourceAccess::ReadWrite
        } else {
            ResourceAccess::Read
        }
    }

    /// Returns this profile with a different opening permission mode.
    #[must_use]
    pub const fn with_permission_mode(
        mut self,
        permission_mode: ClaudeAgentSdkPermissionMode,
    ) -> Self {
        self.permission_mode = permission_mode;
        self
    }
}

impl Default for ClaudeAgentSdkSessionProfile {
    fn default() -> Self {
        Self::read_only()
    }
}

fn profile_failure(code: &'static str, message: &'static str) -> PreparationFailure {
    preparation_failure(PreparationStage::Preflight, code, message)
}

#[cfg(test)]
mod profile_tests;
