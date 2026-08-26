use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Executable name used for automatic Cursor Agent discovery.
pub const CURSOR_AGENT_AUTOMATIC_EXECUTABLE_NAME: &str = "cursor-agent";
/// Release-date interface axis reported by Cursor Agent discovery.
pub const CURSOR_AGENT_RELEASE_AXIS: &str = "cursor-agent.release-date";
/// Oldest qualified Cursor Agent release date.
pub const CURSOR_AGENT_BASELINE_VERSION: &str = "2026-07-01";
/// Qualified build revision for [`CURSOR_AGENT_BASELINE_VERSION`].
pub const CURSOR_AGENT_BASELINE_BUILD_REVISION: &str = "41b2de7";
/// Second exact Cursor Agent milestone date.
pub const CURSOR_AGENT_JULY_23_VERSION: &str = "2026-07-23";
/// Qualified build revision for [`CURSOR_AGENT_JULY_23_VERSION`].
pub const CURSOR_AGENT_JULY_23_BUILD_REVISION: &str = "e383d2b";
/// Host-observed exact Cursor Agent milestone date.
pub const CURSOR_AGENT_AUGUST_04_VERSION: &str = "2026-08-04";
/// Qualified build revision for [`CURSOR_AGENT_AUGUST_04_VERSION`].
pub const CURSOR_AGENT_AUGUST_04_BUILD_REVISION: &str = "aaa8809";
/// Most recent qualified Cursor Agent release date.
pub const CURSOR_AGENT_LATEST_QUALIFIED_VERSION: &str = "2026-08-11";
/// Qualified build revision for [`CURSOR_AGENT_LATEST_QUALIFIED_VERSION`].
pub const CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION: &str = "e8db854";

pub(crate) const CURSOR_CATALOGUE_BEHAVIOR: &str = "cursor-agent.catalogue.calendar-release-v1";
pub(crate) const CURSOR_ACP_BEHAVIOR: &str = "cursor-agent.acp-v1.interactive-v1";
pub(crate) const CURSOR_HEADLESS_BEHAVIOR: &str = "cursor-agent.stream-json.structured-v1";

const RAW_VERSION_BYTES: usize = 18;
const BUILD_REVISION_BYTES: usize = 7;

#[must_use]
/// Parses an exact `YYYY.MM.DD-build` Cursor release into its interface binding.
pub fn cursor_agent_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.len() != RAW_VERSION_BYTES
        || value.trim() != value
        || !value.is_ascii()
        || value.chars().any(char::is_control)
    {
        return None;
    }
    let bytes = value.as_bytes();
    if bytes[4] != b'.' || bytes[7] != b'.' || bytes[10] != b'-' {
        return None;
    }
    if !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..7].iter().all(u8::is_ascii_digit)
        || !bytes[8..10].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let year = value[..4].parse::<u16>().ok()?;
    let month = value[5..7].parse::<u8>().ok()?;
    let day = value[8..10].parse::<u8>().ok()?;
    if !valid_calendar_date(year, month, day) {
        return None;
    }
    let date = format!("{}-{}-{}", &value[0..4], &value[5..7], &value[8..10]);
    let build = &value[11..];
    if build.len() != BUILD_REVISION_BYTES
        || !build
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return None;
    }
    for (qualified_date, qualified_build) in qualified_release_builds() {
        if date == qualified_date && build != qualified_build {
            return None;
        }
    }
    let version = InterfaceVersion::new(date).ok()?;
    Some(InterfaceVersionBinding::new(axis(), version))
}

const fn valid_calendar_date(year: u16, month: u8, day: u8) -> bool {
    let maximum = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            29
        }
        2 => 28,
        _ => return false,
    };
    day > 0 && day <= maximum
}

#[must_use]
/// Returns the compatibility claim for Cursor model discovery.
pub fn cursor_catalogue_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("cursor-agent.catalogue.release-window-3")
            .expect("static Cursor claim id is valid"),
        axis(),
        InterfaceVersionScheme::CalendarDate,
        InterfaceNewerVersionPosture::AllowUnverified,
        exact_milestones(CURSOR_CATALOGUE_BEHAVIOR),
        [],
    )
    .expect("static Cursor compatibility claim is valid")
}

#[must_use]
/// Returns the compatibility claim for Cursor ACP sessions.
pub fn cursor_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("cursor-agent.acp.release-window-3")
            .expect("static Cursor ACP claim id is valid"),
        axis(),
        InterfaceVersionScheme::CalendarDate,
        InterfaceNewerVersionPosture::AllowUnverified,
        exact_milestones(CURSOR_ACP_BEHAVIOR),
        [],
    )
    .expect("static Cursor ACP compatibility claim is valid")
}

#[must_use]
/// Returns the compatibility claim for Cursor stream-JSON runs.
pub fn cursor_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("cursor-agent.headless.release-window-3")
            .expect("static Cursor headless claim id is valid"),
        axis(),
        InterfaceVersionScheme::CalendarDate,
        InterfaceNewerVersionPosture::AllowUnverified,
        exact_milestones(CURSOR_HEADLESS_BEHAVIOR),
        [],
    )
    .expect("static Cursor headless compatibility claim is valid")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CursorPlanSelection;

pub(crate) fn select_cursor_acp_plan(
    plan: &PreflightPlan,
) -> Result<CursorPlanSelection, RuntimeFailure> {
    let claim = cursor_acp_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.cursor.acp.version_missing",
            "Cursor ACP plan is missing its exact release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.cursor.acp.version_ambiguous",
            "Cursor ACP plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != CURSOR_ACP_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.cursor.acp.version_incompatible",
            "Cursor release version is incompatible with the ACP driver",
        ));
    }
    Ok(CursorPlanSelection)
}

pub(crate) fn validate_cursor_headless_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = cursor_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.cursor.headless.version_missing",
            "Cursor headless plan is missing its exact release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.cursor.headless.version_ambiguous",
            "Cursor headless plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != CURSOR_HEADLESS_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.cursor.headless.version_incompatible",
            "Cursor release version is incompatible with the headless driver",
        ));
    }
    Ok(())
}

pub(crate) fn headless_release_is_exactly_qualified(binding: &InterfaceVersionBinding) -> bool {
    let claim = cursor_headless_claim();
    binding.axis() == claim.axis()
        && matches!(
            claim.assess(binding.version()),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.behavior_revision().as_str() == CURSOR_HEADLESS_BEHAVIOR
        )
}

pub(crate) fn validate_cursor_headless_ask_release(
    plan: &PreflightPlan,
) -> Result<(), RuntimeFailure> {
    let claim = cursor_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let Some(binding) = bindings.next() else {
        return Err(ask_unqualified());
    };
    if bindings.next().is_some() {
        return Err(ask_unqualified());
    }
    if claim.assess(binding.version()) != plan.assess_interface_version(binding)
        || !headless_release_is_exactly_qualified(binding)
    {
        return Err(ask_unqualified());
    }
    Ok(())
}

fn ask_unqualified() -> RuntimeFailure {
    failure(
        "swallowtail.cursor.headless.ask_mode_unqualified",
        "Cursor headless Ask mode requires an exactly qualified Cursor release",
    )
}

pub(crate) fn validate_cursor_catalogue_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = cursor_catalogue_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.cursor.catalogue.version_missing",
            "Cursor catalogue plan is missing its exact release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.cursor.catalogue.version_ambiguous",
            "Cursor catalogue plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != CURSOR_CATALOGUE_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.cursor.catalogue.version_incompatible",
            "Cursor release version is incompatible with the catalogue driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(CURSOR_AGENT_RELEASE_AXIS)
        .expect("static Cursor release axis is valid")
}

const fn qualified_release_builds() -> [(&'static str, &'static str); 4] {
    [
        (
            CURSOR_AGENT_BASELINE_VERSION,
            CURSOR_AGENT_BASELINE_BUILD_REVISION,
        ),
        (
            CURSOR_AGENT_JULY_23_VERSION,
            CURSOR_AGENT_JULY_23_BUILD_REVISION,
        ),
        (
            CURSOR_AGENT_AUGUST_04_VERSION,
            CURSOR_AGENT_AUGUST_04_BUILD_REVISION,
        ),
        (
            CURSOR_AGENT_LATEST_QUALIFIED_VERSION,
            CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION,
        ),
    ]
}

fn exact_milestones(behavior: &str) -> [InterfaceVersionSegment; 4] {
    qualified_release_builds().map(|(date, _build)| {
        InterfaceVersionSegment::exact(
            version(date).expect("static Cursor release version is valid"),
            InterfaceBehaviorRevision::new(behavior).expect("static Cursor behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )
    })
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

#[cfg(test)]
mod tests {
    use super::{
        CURSOR_AGENT_RELEASE_AXIS, cursor_acp_claim, cursor_agent_release_binding,
        cursor_catalogue_claim, cursor_headless_claim,
    };
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn exact_releases_are_qualified_without_inferring_the_calendar_gap() {
        let claim = cursor_catalogue_claim();
        assert!(claim.supports(&version("2026-07-01")));
        assert!(claim.supports(&version("2026-07-23")));
        assert!(claim.supports(&version("2026-08-04")));
        assert!(claim.supports(&version("2026-08-11")));
        assert!(!claim.permits(&version("2026-06-30")));
        assert!(!claim.permits(&version("2026-07-15")));
        assert!(!claim.permits(&version("2026-07-24")));
        assert!(!claim.permits(&version("2026-08-05")));
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("2026-08-12"))
        else {
            panic!("later Cursor release remains visibly unverified");
        };
        assert_eq!(newer.latest_qualified().as_str(), "2026-08-11");
    }

    #[test]
    fn acp_and_catalogue_keep_distinct_behavior_claims() {
        let acp = cursor_acp_claim();
        let catalogue = cursor_catalogue_claim();
        let headless = cursor_headless_claim();
        assert_ne!(acp.id(), catalogue.id());
        assert_ne!(headless.id(), catalogue.id());
        assert_ne!(headless.id(), acp.id());
        assert_ne!(
            acp.assess(&version("2026-07-01")).behavior_revision(),
            catalogue.assess(&version("2026-07-01")).behavior_revision()
        );
    }

    #[test]
    fn binding_validates_build_revision_but_orders_only_the_release_date() {
        let local = cursor_agent_release_binding("2026.07.01-41b2de7")
            .expect("installed Cursor version parses");
        assert_eq!(local.axis().as_str(), CURSOR_AGENT_RELEASE_AXIS);
        assert_eq!(local.version().as_str(), "2026-07-01");
        let registry = cursor_agent_release_binding("2026.08.11-e8db854")
            .expect("registry Cursor version parses");
        assert_eq!(registry.version().as_str(), "2026-08-11");
        let host =
            cursor_agent_release_binding("2026.08.04-aaa8809").expect("host Cursor version parses");
        assert_eq!(host.version().as_str(), "2026-08-04");

        for rejected in [
            "",
            "2026.07.01",
            "2026-07-01-41b2de7",
            "2026.07.01-41B2DE7",
            "2026.07.01-deadbee",
            "2026.07.23-deadbee",
            "2026.08.04-deadbee",
            "2026.08.11-deadbee",
            "2026.07.01-41b2de",
            "2026.02.30-41b2de7",
            " 2026.07.01-41b2de7",
        ] {
            assert!(
                cursor_agent_release_binding(rejected).is_none(),
                "{rejected}"
            );
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}
