use crate::{
    gemini_cli_acp_binding, gemini_cli_acp_claim, gemini_cli_headless_binding,
    gemini_cli_headless_claim,
};
use swallowtail_core::{DiscoveryOutcome, InterfaceVersionBinding};
use swallowtail_runtime::{
    BoxFuture, HostServices, InstalledExecutableDiscoveryRequest, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

const GEMINI_CLI_PROBE_CODES: swallowtail_runtime::InstalledProbeCodes =
    installed_probe_codes!("swallowtail.gemini");

#[derive(Clone, Copy)]
pub(super) enum ProbeRoute {
    Acp,
    Headless,
}

impl ProbeRoute {
    fn claim(self) -> swallowtail_core::InterfaceCompatibilityClaim {
        match self {
            Self::Acp => gemini_cli_acp_claim(),
            Self::Headless => gemini_cli_headless_claim(),
        }
    }

    fn binding(self, value: &str) -> Option<InterfaceVersionBinding> {
        match self {
            Self::Acp => gemini_cli_acp_binding(value),
            Self::Headless => gemini_cli_headless_binding(value),
        }
    }
}

pub(super) fn probe_acp(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
) -> BoxFuture<'static, Result<DiscoveryOutcome, RuntimeFailure>> {
    Box::pin(probe_installed_executable_version(
        request,
        services,
        ProbeRoute::Acp.claim(),
        move |output| parse_version(output, ProbeRoute::Acp),
        GEMINI_CLI_PROBE_CODES,
        "Gemini CLI",
    ))
}

pub(super) fn probe_headless(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
) -> BoxFuture<'static, Result<DiscoveryOutcome, RuntimeFailure>> {
    Box::pin(probe_installed_executable_version(
        request,
        services,
        ProbeRoute::Headless.claim(),
        move |output| parse_version(output, ProbeRoute::Headless),
        GEMINI_CLI_PROBE_CODES,
        "Gemini CLI",
    ))
}

fn parse_version(output: &[u8], route: ProbeRoute) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    if value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return None;
    }
    route.binding(value)
}
