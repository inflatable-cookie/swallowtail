#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_GEMINI_ACP=1 and an installed Gemini CLI"]
fn pinned_gemini_cli_is_installed_when_live_probe_is_enabled() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_GEMINI_ACP").as_deref(),
        Ok("1"),
        "live Gemini probe requires an explicit gate"
    );
    let output = std::process::Command::new("gemini")
        .arg("--version")
        .output()
        .expect("Gemini CLI is installed");
    assert!(output.status.success(), "Gemini version probe succeeds");
    let version = String::from_utf8(output.stdout).expect("version output is UTF-8");
    assert!(
        version.contains("0.51.0"),
        "Gemini CLI version remains pinned"
    );
}
