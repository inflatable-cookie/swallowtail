use swallowtail_adapter_kimi::{kimi_acp_claim, kimi_code_binding};

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
    let binding = kimi_code_binding(version.trim()).expect("Kimi emits one semantic version");
    assert!(
        kimi_acp_claim().permits(binding.version()),
        "installed stable Kimi remains qualified or visibly unverified newer"
    );
}
