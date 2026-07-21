#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_KIMI_ACP=1 and an installed Kimi Code CLI"]
fn pinned_kimi_code_cli_is_installed_when_live_probe_is_enabled() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_KIMI_ACP").as_deref(),
        Ok("1"),
        "live Kimi probe requires an explicit gate"
    );
    let output = std::process::Command::new("kimi")
        .arg("--version")
        .output()
        .expect("Kimi Code CLI is installed");
    assert!(output.status.success(), "Kimi version probe succeeds");
    let version = String::from_utf8(output.stdout).expect("version output is UTF-8");
    assert!(
        version.contains("0.28.1"),
        "Kimi Code CLI version remains pinned"
    );
}
