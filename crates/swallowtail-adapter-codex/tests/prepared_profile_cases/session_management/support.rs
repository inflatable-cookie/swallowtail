use super::*;

pub(super) fn lifecycle_binding(
    prepared: &swallowtail_adapter_codex::CodexPreparedIntegration,
    version: &str,
) -> ProviderSessionManagementBinding {
    let behavior = swallowtail_adapter_codex::codex_app_server_lifecycle_claim()
        .assess(&swallowtail_core::InterfaceVersion::new(version).unwrap());
    let capabilities = match behavior
        .behavior_revision()
        .expect("fixture version has lifecycle behavior")
        .as_str()
    {
        "codex.app-server.lifecycle.v1.archive-response" => {
            vec![Capability::ProviderSessionArchive]
        }
        "codex.app-server.lifecycle.v1.strict-descendant-hard-delete" => vec![
            Capability::ProviderSessionArchive,
            Capability::ProviderSessionRestore,
            Capability::ProviderSessionDelete,
        ],
        _ => vec![
            Capability::ProviderSessionArchive,
            Capability::ProviderSessionRestore,
        ],
    };
    let base = prepared.instance();
    let capability_profile = swallowtail_core::CapabilityProfile::new(
        capabilities
            .into_iter()
            .map(|capability| swallowtail_core::CapabilityRequirement::new(capability, [])),
    );
    let mut instance = swallowtail_core::ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capability_profile,
    )
    .with_interface_versions(base.interface_versions().cloned());
    if let Some(posture) = base.harness_configuration_posture() {
        instance = instance.with_harness_configuration_posture(posture);
    }
    ProviderSessionManagementBinding::from_bound_session(
        swallowtail_core::SessionRef::new("thread-provider-new").unwrap(),
        &swallowtail_adapter_codex::codex_app_server_descriptor(),
        &instance,
        prepared.access_evidence().clone(),
        Some(working_resource()),
        ProviderSessionBindingOrigin::ExplicitlyImported,
    )
    .expect("fixture management binding is valid")
}

#[derive(Default)]
pub(super) struct StagedDeadline {
    waits: std::sync::atomic::AtomicUsize,
}

impl TimeService for StagedDeadline {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let wait = self.waits.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if wait == 0 {
            Box::pin(std::future::pending())
        } else {
            Box::pin(async move {
                DeadlineObservation::new(deadline, MonotonicInstant::from_ticks(100))
            })
        }
    }
}
