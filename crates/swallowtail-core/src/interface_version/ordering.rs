use super::{InterfaceVersion, InterfaceVersionScheme, InvalidInterfaceCompatibilityClaim};
use semver::Version;
use std::cmp::Ordering;

pub(super) fn validate_version(
    scheme: InterfaceVersionScheme,
    version: &InterfaceVersion,
) -> Result<(), InvalidInterfaceCompatibilityClaim> {
    match scheme {
        InterfaceVersionScheme::Semantic => Version::parse(version.as_str())
            .map(|_| ())
            .map_err(|_| InvalidInterfaceCompatibilityClaim::new("Semantic version is invalid")),
        InterfaceVersionScheme::Integer => parse_integer_version(version).map(|_| ()),
        InterfaceVersionScheme::CalendarDate => parse_calendar_date(version).map(|_| ()),
        InterfaceVersionScheme::Opaque => Ok(()),
    }
}

pub(super) fn compare_versions(
    scheme: InterfaceVersionScheme,
    left: &InterfaceVersion,
    right: &InterfaceVersion,
) -> Result<Ordering, InvalidInterfaceCompatibilityClaim> {
    match scheme {
        InterfaceVersionScheme::Semantic => {
            Ok(Version::parse(left.as_str())?.cmp(&Version::parse(right.as_str())?))
        }
        InterfaceVersionScheme::Integer => {
            Ok(parse_integer_version(left)?.cmp(&parse_integer_version(right)?))
        }
        InterfaceVersionScheme::CalendarDate => {
            Ok(parse_calendar_date(left)?.cmp(&parse_calendar_date(right)?))
        }
        InterfaceVersionScheme::Opaque => Ok(left.cmp(right)),
    }
}

/// Reports whether two spellings name the same version point.
///
/// Exclusions deny a version, so they compare at the scheme's version-point
/// granularity rather than by raw text: semver precedence ignores build
/// metadata (spec item 9), so an exclusion written `1.2.3+build1` must also
/// deny the observed `1.2.3`. Segment containment keeps the total ordering, so
/// a build-metadata spelling never widens a qualified window by itself.
pub(super) fn same_version_point(
    scheme: InterfaceVersionScheme,
    left: &InterfaceVersion,
    right: &InterfaceVersion,
) -> bool {
    if scheme == InterfaceVersionScheme::Semantic {
        return Version::parse(left.as_str())
            .ok()
            .zip(Version::parse(right.as_str()).ok())
            .is_some_and(|(left, right)| left.cmp_precedence(&right) == Ordering::Equal);
    }
    compare_versions(scheme, left, right).is_ok_and(|ordering| ordering == Ordering::Equal)
}

pub(super) fn is_semantic_prerelease(version: &InterfaceVersion) -> bool {
    Version::parse(version.as_str()).is_ok_and(|version| !version.pre.is_empty())
}

/// Parses one canonical ordered integer version.
///
/// Canonical text is ASCII digits only with no leading `+` and no redundant
/// leading zeros, so `+123` and `0123` cannot alias the stored `123`.
fn parse_integer_version(
    version: &InterfaceVersion,
) -> Result<u64, InvalidInterfaceCompatibilityClaim> {
    parse_canonical_integer(version.as_str())
}

fn parse_canonical_integer(text: &str) -> Result<u64, InvalidInterfaceCompatibilityClaim> {
    let canonical = !text.is_empty()
        && text.bytes().all(|byte| byte.is_ascii_digit())
        && (text == "0" || !text.starts_with('0'));
    if !canonical {
        return Err(InvalidInterfaceCompatibilityClaim::new(
            "Integer version is invalid",
        ));
    }
    text.parse::<u64>()
        .map_err(|_| InvalidInterfaceCompatibilityClaim::new("Integer version is invalid"))
}

fn parse_calendar_date(
    version: &InterfaceVersion,
) -> Result<(u16, u8, u8), InvalidInterfaceCompatibilityClaim> {
    let value = version.as_str().as_bytes();
    if value.len() != 10 || value[4] != b'-' || value[7] != b'-' {
        return Err(InvalidInterfaceCompatibilityClaim::new(
            "Calendar version must use YYYY-MM-DD",
        ));
    }
    // Every field is strict ASCII digits, so a signed or non-ASCII field such
    // as `2026-+1-+2` cannot alias the canonical date.
    if !value[0..4].iter().all(u8::is_ascii_digit)
        || !value[5..7].iter().all(u8::is_ascii_digit)
        || !value[8..10].iter().all(u8::is_ascii_digit)
    {
        return Err(InvalidInterfaceCompatibilityClaim::new(
            "Calendar version must use YYYY-MM-DD",
        ));
    }
    let year = version.as_str()[0..4]
        .parse::<u16>()
        .map_err(|_| InvalidInterfaceCompatibilityClaim::new("Calendar year is invalid"))?;
    let month = version.as_str()[5..7]
        .parse::<u8>()
        .map_err(|_| InvalidInterfaceCompatibilityClaim::new("Calendar month is invalid"))?;
    let day = version.as_str()[8..10]
        .parse::<u8>()
        .map_err(|_| InvalidInterfaceCompatibilityClaim::new("Calendar day is invalid"))?;
    let maximum_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            29
        }
        2 => 28,
        _ => 0,
    };
    if day == 0 || day > maximum_day {
        return Err(InvalidInterfaceCompatibilityClaim::new(
            "Calendar date is invalid",
        ));
    }
    Ok((year, month, day))
}
