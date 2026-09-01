use crate::failure::failure;
use std::num::NonZeroU32;
use swallowtail_runtime::RuntimeFailure;

/// Exact Claude Code versions Research 226 probed for `--max-turns`.
///
/// This is the deliver-now version set, not the route's qualified window. The
/// window is a semantic range and its compatibility claim also permits later
/// stable points as `UnverifiedNewer`. Neither is evidence for this feature.
///
/// `2.1.230` is inside the semantic range and is deliberately absent: it was
/// never published to npm, so no artifact exists and nothing about its parser,
/// loop guard, or terminal shape was proved. A host reporting it is rejected
/// for a maximum-turn selection like any other unprobed point.
pub(crate) const ADMITTED_VERSIONS: [&str; 21] = [
    "2.1.220", "2.1.221", "2.1.222", "2.1.223", "2.1.224", "2.1.225", "2.1.226", "2.1.227",
    "2.1.228", "2.1.229", "2.1.231", "2.1.232", "2.1.233", "2.1.234", "2.1.235", "2.1.236",
    "2.1.237", "2.1.238", "2.1.239", "2.1.240", "2.1.241",
];

/// Reports whether one observed interface version admits a maximum-turn bound.
///
/// The version must sit on the headless axis, assess as `Qualified` rather
/// than provisionally permitted `UnverifiedNewer`, and be one of the exact
/// points Research 226 probed. The last condition is the operative one: the
/// route's compatibility claim covers a semantic range that contains at least
/// one version no artifact exists for, and it permits later stable points the
/// research never saw.
pub(crate) fn admits(binding: &swallowtail_core::InterfaceVersionBinding) -> bool {
    let claim = crate::claude_code_headless_claim();
    binding.axis() == claim.axis()
        && matches!(
            claim.assess(binding.version()),
            swallowtail_core::InterfaceCompatibilityAssessment::Qualified(_)
        )
        && ADMITTED_VERSIONS.contains(&binding.version().as_str())
}

/// Exact admitted `--max-turns` selection for native Claude Code headless runs.
///
/// This is adapter-local Claude Code configuration. It is not a portable agent
/// budget, an output-token limit, a tool-call budget, a cost cap, a wall-time
/// deadline, a context bound, or a retry count. One counted turn is one
/// tool-use round trip; a final text-only response is not counted.
///
/// Research 226 closes the domain to positive integers because the native
/// parser does not. Exact `2.1.220..=2.1.241` coerces the argument with
/// `Number` and rejects only `NaN`, so zero, negatives, fractions, `Infinity`,
/// exponent and hexadecimal forms, grouped digits, and the empty string all
/// pass parsing. The agent loop then guards with `maxTurns && next > maxTurns`,
/// a truthiness test under which a resolved `0` disables enforcement outright
/// and a negative value stops after the first tool-use turn. Only a positive
/// integer produces the documented bound, so only a positive integer is
/// admitted here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClaudeCodeMaximumTurns(NonZeroU32);

impl ClaudeCodeMaximumTurns {
    /// Creates a maximum-turn selection from one admitted positive integer.
    #[must_use]
    pub const fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    /// Validates one caller-supplied integer before preparation.
    ///
    /// Zero and any value above [`u32::MAX`] fail closed. Every admitted value
    /// is exactly representable as an IEEE-754 double, so the native `Number`
    /// coercion round-trips the dispatched decimal literal without loss.
    pub fn from_u64(value: u64) -> Result<Self, RuntimeFailure> {
        u32::try_from(value)
            .ok()
            .and_then(NonZeroU32::new)
            .map(Self)
            .ok_or_else(|| {
                failure(
                    "swallowtail.claude_code.headless.maximum_turns_invalid",
                    "Claude Code headless maximum turns must be a positive 32-bit integer",
                )
            })
    }

    /// Returns the exact positive integer emitted after `--max-turns`.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{ADMITTED_VERSIONS, ClaudeCodeMaximumTurns};
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn admitted_versions_are_qualified_and_exclude_the_unpublished_point() {
        let claim = crate::claude_code_headless_claim();
        for admitted in ADMITTED_VERSIONS {
            let version = InterfaceVersion::new(admitted).expect("version is valid");
            assert!(matches!(
                claim.assess(&version),
                InterfaceCompatibilityAssessment::Qualified(_)
            ));
        }
        // 2.1.230 sits inside the semantic window but was never published, so
        // no artifact was probed and it must not admit the feature.
        assert!(!ADMITTED_VERSIONS.contains(&"2.1.230"));
        assert!(matches!(
            claim.assess(&InterfaceVersion::new("2.1.230").expect("version is valid")),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));

        // The set is exactly the window's published points: 220..=241 less 230.
        let expected = (220..=241)
            .filter(|patch| *patch != 230)
            .map(|patch| format!("2.1.{patch}"))
            .collect::<Vec<_>>();
        assert_eq!(ADMITTED_VERSIONS.to_vec(), expected);
    }

    #[test]
    fn unprobed_and_provisional_versions_do_not_admit_the_feature() {
        for admitted in ADMITTED_VERSIONS {
            assert!(super::admits(&binding(admitted)));
        }
        for rejected in [
            "2.1.230", "2.1.242", "2.1.244", "2.1.249", "2.1.251", "2.1.252", "2.1.253", "2.1.257",
            "2.1.258", "2.1.219", "3.0.0",
        ] {
            assert!(!super::admits(&binding(rejected)));
        }
    }

    fn binding(value: &str) -> swallowtail_core::InterfaceVersionBinding {
        crate::claude_code_headless_binding(value).expect("fixture version binds")
    }

    #[test]
    fn admitted_domain_accepts_positive_integers() {
        for admitted in [1, 3, 30, u64::from(u32::MAX)] {
            assert_eq!(
                ClaudeCodeMaximumTurns::from_u64(admitted)
                    .expect("positive value is admitted")
                    .as_u32(),
                u32::try_from(admitted).expect("value fits u32")
            );
        }
    }

    #[test]
    fn zero_and_overflow_fail_closed() {
        for rejected in [0, u64::from(u32::MAX) + 1, u64::MAX] {
            let error =
                ClaudeCodeMaximumTurns::from_u64(rejected).expect_err("value is not admitted");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.claude_code.headless.maximum_turns_invalid"
            );
        }
    }
}
