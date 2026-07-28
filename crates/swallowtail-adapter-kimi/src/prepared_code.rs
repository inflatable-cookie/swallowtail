use crate::{
    KimiHeadlessPreparationInput, KimiHeadlessPreparationProbe, KimiHeadlessPreparedIntegration,
    KimiPreparationInput, KimiPreparationProbe, KimiPreparedIntegration, prepare_kimi,
    prepare_kimi_headless,
};
use swallowtail_core::{AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, HostServices, InstalledExecutableTarget,
    PreparationFailure, PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
    WorkingResourceRef,
};

/// The exact installed Kimi Code route selected before preparation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KimiCodePreparedDriver {
    Acp,
    Headless,
}

/// Shared installed Kimi Code inputs with one explicit route selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiCodePreparationInput {
    driver: KimiCodePreparedDriver,
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    state_root: Option<WorkingResourceRef>,
}

impl KimiCodePreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        driver: KimiCodePreparedDriver,
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        target: InstalledExecutableTarget,
        environment: EnvironmentRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            driver,
            instance_id,
            instance_revision,
            execution_host_id,
            target,
            environment,
            access_profile,
            access_evidence,
            state_root: None,
        }
    }

    #[must_use]
    pub const fn driver(&self) -> KimiCodePreparedDriver {
        self.driver
    }

    /// Binds the opaque Kimi state root used by ACP session-management import.
    ///
    /// Headless runs expose no reusable session authority and reject this
    /// input before executable discovery.
    #[must_use]
    pub fn with_state_root(mut self, state_root: WorkingResourceRef) -> Self {
        self.state_root = Some(state_root);
        self
    }
}

/// Caller-owned controls shared by both installed Kimi Code probes.
#[derive(Clone, Debug)]
pub struct KimiCodePreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl KimiCodePreparationProbe {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        scope_id: ScopeId,
        deadline: Deadline,
        cancellation: DiscoveryCancellation,
    ) -> Self {
        Self {
            request_id,
            scope_id,
            deadline,
            cancellation,
        }
    }
}

/// One explicitly selected installed Kimi Code route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KimiCodePreparedIntegration {
    Acp(KimiPreparedIntegration),
    Headless(KimiHeadlessPreparedIntegration),
}

impl KimiCodePreparedIntegration {
    #[must_use]
    pub const fn driver(&self) -> KimiCodePreparedDriver {
        match self {
            Self::Acp(_) => KimiCodePreparedDriver::Acp,
            Self::Headless(_) => KimiCodePreparedDriver::Headless,
        }
    }
}

/// Prepare one exact installed Kimi Code route without inferring transport.
pub async fn prepare_kimi_code(
    input: KimiCodePreparationInput,
    probe: KimiCodePreparationProbe,
    services: HostServices,
) -> Result<KimiCodePreparedIntegration, PreparationFailure> {
    let KimiCodePreparationInput {
        driver,
        instance_id,
        instance_revision,
        execution_host_id,
        target,
        environment,
        access_profile,
        access_evidence,
        state_root,
    } = input;
    let KimiCodePreparationProbe {
        request_id,
        scope_id,
        deadline,
        cancellation,
    } = probe;
    match driver {
        KimiCodePreparedDriver::Acp => {
            let mut input = KimiPreparationInput::new(
                instance_id,
                instance_revision,
                execution_host_id,
                target,
                environment,
                access_profile,
                access_evidence,
            );
            if let Some(state_root) = state_root {
                input = input.with_state_root(state_root);
            }
            prepare_kimi(
                input,
                KimiPreparationProbe::new(request_id, scope_id, deadline, cancellation),
                services,
            )
            .await
            .map(KimiCodePreparedIntegration::Acp)
        }
        KimiCodePreparedDriver::Headless => {
            if state_root.is_some() {
                return Err(PreparationFailure::new(
                    PreparationStage::TargetSelection,
                    swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.kimi.code.preparation.headless_state_root_unsupported",
                        "Kimi headless preparation does not accept ACP session-state authority",
                    )),
                ));
            }
            prepare_kimi_headless(
                KimiHeadlessPreparationInput::new(
                    instance_id,
                    instance_revision,
                    execution_host_id,
                    target,
                    environment,
                    access_profile,
                    access_evidence,
                ),
                KimiHeadlessPreparationProbe::new(request_id, scope_id, deadline, cancellation),
                services,
            )
            .await
            .map(KimiCodePreparedIntegration::Headless)
        }
    }
}
