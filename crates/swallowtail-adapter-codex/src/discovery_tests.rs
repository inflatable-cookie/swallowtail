use super::{CODEX_INSTALL_GUIDANCE, attach_install_guidance, parse_version};
use crate::selection::CODEX_CLI_AXIS;
use crate::{CodexAppServerDriver, CodexExecDriver};
use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus, InterfaceVersionAxis};

#[test]
fn absent_guidance_is_frozen_and_present_outcomes_have_none() {
    let absent = attach_install_guidance(DiscoveryOutcome::new(DiscoveryStatus::Absent, None));
    let guidance = absent.install_guidance().expect("absent guidance");
    assert_eq!(guidance, &CODEX_INSTALL_GUIDANCE);
    assert_eq!(CodexExecDriver::install_guidance(), CODEX_INSTALL_GUIDANCE);
    assert_eq!(
        CodexAppServerDriver::install_guidance(),
        CODEX_INSTALL_GUIDANCE
    );
    assert_eq!(guidance.display_name(), "Codex CLI");
    assert_eq!(
        guidance.command(),
        "curl -fsSL https://chatgpt.com/codex/install.sh | sh"
    );
    assert_eq!(
        guidance.documentation_url(),
        "https://github.com/openai/codex/blob/main/README.md#installing-and-running-codex-cli"
    );
    assert_eq!(guidance.frozen_on(), "2026-09-06");

    let present = attach_install_guidance(DiscoveryOutcome::new(DiscoveryStatus::Discovered, None));
    assert!(present.install_guidance().is_none());
}

#[test]
fn parser_accepts_one_exact_codex_cli_semver() {
    let binding = parse_version(
        b"codex-cli 0.145.0\n",
        InterfaceVersionAxis::new(CODEX_CLI_AXIS).unwrap(),
    )
    .expect("version parses");
    assert_eq!(binding.version().as_str(), "0.145.0");
}

#[test]
fn parser_rejects_raw_payload_variants() {
    for output in [
        b"0.145.0".as_slice(),
        b"codex-cli latest".as_slice(),
        b"codex-cli 0.145.0 extra".as_slice(),
        b"private payload".as_slice(),
    ] {
        assert!(
            parse_version(output, InterfaceVersionAxis::new(CODEX_CLI_AXIS).unwrap()).is_none()
        );
    }
}
