use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation};
use swallowtail_runtime::{
    BoxFuture, DebugObservationKind, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, RuntimeFailure,
    validate_installed_executable_discovery_services,
};

use crate::selection::{DEEPSEEK_HARNESS_RELEASE_VERSION, validate_target_payload};
use crate::{
    DeepSeekHarnessJsonRpcDriver, deepseek_harness_jsonrpc_claim, deepseek_harness_release_binding,
};

impl DiscoveryDriver for DeepSeekHarnessJsonRpcDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.deepseek_harness.discovery_target_required",
                "DeepSeek Harness discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(async move { classify_pinned_payload(request, services) })
    }
}

fn classify_pinned_payload(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_target_payload(request.target().executable().as_host_value())?;
    validate_installed_executable_discovery_services(&request, &services)?;
    let claim = deepseek_harness_jsonrpc_claim();
    if request.target().version_axis() != claim.axis() {
        return Err(crate::failure::failure(
            "swallowtail.deepseek_harness.discovery_axis_mismatch",
            "DeepSeek Harness discovery target uses a different version axis",
        ));
    }
    if request.cancellation().is_requested() {
        return Ok(DiscoveryOutcome::new(
            DiscoveryStatus::Cancelled,
            Some(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.deepseek_harness.discovery_cancelled",
                "DeepSeek Harness installed discovery was cancelled",
            )),
        ));
    }
    let binding = deepseek_harness_release_binding(DEEPSEEK_HARNESS_RELEASE_VERSION)
        .expect("static DeepSeek Harness release binds");
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &claim,
    )
    .map_err(|_| {
        let message = "DeepSeek Harness version observation could not be classified";
        services.emit_failure_debug(
            DebugObservationKind::InterfaceVersion,
            "DeepSeek Harness",
            "installed_discovery.classify",
            "swallowtail.deepseek_harness.discovery_classification_failed",
            message,
        );
        crate::failure::failure(
            "swallowtail.deepseek_harness.discovery_classification_failed",
            message,
        )
    })?;
    Ok(DiscoveryOutcome::installed_executable(observation))
}

#[cfg(test)]
mod tests {
    use super::deepseek_harness_release_binding;
    use crate::DEEPSEEK_HARNESS_RELEASE_VERSION;

    #[test]
    fn pin_text_is_the_discovery_version_not_cli_output() {
        assert_eq!(
            deepseek_harness_release_binding(DEEPSEEK_HARNESS_RELEASE_VERSION)
                .expect("exact runtime version binds")
                .version()
                .as_str(),
            "0.1.0rc6"
        );
        for rejected in [
            "",
            "0.1.0rc5",
            "0.1.0rc7",
            "dsh-jsonrpc-agent 0.1.0rc6",
            "0.1.0rc6 ",
            "0.1.0rc6\n",
        ] {
            assert!(
                deepseek_harness_release_binding(rejected).is_none(),
                "{rejected:?}"
            );
        }
    }
}
