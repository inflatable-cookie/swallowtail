use super::parse_version;
use crate::selection::CODEX_CLI_AXIS;
use swallowtail_core::InterfaceVersionAxis;

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
