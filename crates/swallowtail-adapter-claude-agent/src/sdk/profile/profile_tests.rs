use super::{ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile, ClaudeAgentSdkTool};
use swallowtail_core::ResourceAccess;

fn names(profile: &ClaudeAgentSdkSessionProfile) -> Vec<&'static str> {
    profile.tools().map(ClaudeAgentSdkTool::as_str).collect()
}

#[test]
fn the_default_profile_is_the_unchanged_read_only_set() {
    let profile = ClaudeAgentSdkSessionProfile::default();
    assert_eq!(profile, ClaudeAgentSdkSessionProfile::read_only());
    assert_eq!(names(&profile), ["Read", "Glob", "Grep"]);
    assert_eq!(
        profile.permission_mode(),
        ClaudeAgentSdkPermissionMode::Default
    );
    assert!(!profile.admits_writes());
    assert_eq!(profile.resource_access(), ResourceAccess::Read);
}

#[test]
fn a_write_tool_requires_a_read_write_lease() {
    let profile = ClaudeAgentSdkSessionProfile::read_write(ClaudeAgentSdkPermissionMode::Default);
    assert_eq!(
        names(&profile),
        ["Read", "Glob", "Grep", "Edit", "Write", "MultiEdit"]
    );
    assert!(profile.admits_writes());
    assert_eq!(profile.resource_access(), ResourceAccess::ReadWrite);
    let single = ClaudeAgentSdkSessionProfile::new(
        [ClaudeAgentSdkTool::Read, ClaudeAgentSdkTool::Edit],
        ClaudeAgentSdkPermissionMode::Default,
    )
    .expect("an explicit read plus edit set is admissible");
    assert_eq!(single.resource_access(), ResourceAccess::ReadWrite);
}

#[test]
fn an_auto_approving_permission_mode_cannot_be_constructed() {
    for rejected in ["bypassPermissions", "auto", "dontAsk"] {
        let failure = ClaudeAgentSdkPermissionMode::parse(rejected)
            .expect_err("an auto-approving mode is rejected");
        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.claude-agent.sdk.profile.permission_mode_rejected"
        );
    }
    let unknown =
        ClaudeAgentSdkPermissionMode::parse("plan-mode").expect_err("an unknown mode is rejected");
    assert_eq!(
        unknown.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.profile.permission_mode_unknown"
    );
}

#[test]
fn an_unknown_or_repeated_tool_name_is_rejected_while_parsing() {
    let unknown = ClaudeAgentSdkSessionProfile::from_names(["Read", "Bash"], "default")
        .expect_err("Bash is outside the admitted set");
    assert_eq!(
        unknown.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.profile.tool_unknown"
    );
    let repeated = ClaudeAgentSdkSessionProfile::from_names(["Read", "Read"], "default")
        .expect_err("a repeated tool is rejected");
    assert_eq!(
        repeated.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.profile.tool_repeated"
    );
    let empty = ClaudeAgentSdkSessionProfile::from_names([], "default")
        .expect_err("an empty set is rejected");
    assert_eq!(
        empty.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.profile.tool_set_empty"
    );
}

#[test]
fn names_round_trip_through_the_admitted_vocabulary() {
    let profile =
        ClaudeAgentSdkSessionProfile::from_names(["Write", "Read", "MultiEdit"], "acceptEdits")
            .expect("an explicit write set is admissible");
    // Canonical order, not the caller's order: the sidecar echo is exact.
    assert_eq!(names(&profile), ["Read", "Write", "MultiEdit"]);
    assert!(profile.permission_mode().skips_edit_admission());
    assert!(profile.admits(ClaudeAgentSdkTool::Write));
    assert!(!profile.admits(ClaudeAgentSdkTool::Glob));
    assert!(ClaudeAgentSdkTool::Write.mutates_working_resource());
    assert!(!ClaudeAgentSdkTool::Read.mutates_working_resource());
}
