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
        InterfaceVersionScheme::Integer => version
            .as_str()
            .parse::<u64>()
            .map(|_| ())
            .map_err(|_| InvalidInterfaceCompatibilityClaim::new("Integer version is invalid")),
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
        InterfaceVersionScheme::Integer => Ok(left
            .as_str()
            .parse::<u64>()?
            .cmp(&right.as_str().parse::<u64>()?)),
        InterfaceVersionScheme::CalendarDate => {
            Ok(parse_calendar_date(left)?.cmp(&parse_calendar_date(right)?))
        }
        InterfaceVersionScheme::Opaque => Ok(left.cmp(right)),
    }
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
