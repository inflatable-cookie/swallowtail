use crate::{
    GeminiHeadlessPreparationInput, GeminiHeadlessPreparationProbe,
    GeminiHeadlessPreparedIntegration, GeminiPreparationInput, GeminiPreparationProbe,
    GeminiPreparedIntegration, prepare_gemini_acp, prepare_gemini_headless,
};
use swallowtail_core::{AccessProfile, ConfiguredInstanceId, ExecutionHostId, InstanceRevision};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, HostServices, InstalledExecutableTarget,
    PreparationFailure, PreparedAccessEvidence, RequestId, ScopeId,
};

/// The exact Gemini CLI route selected before preparation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeminiCliPreparedDriver {
    Acp,
    Headless,
}

/// Shared Gemini CLI installation inputs with one explicit route selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiCliPreparationInput {
    driver: GeminiCliPreparedDriver,
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    execution_host_id: ExecutionHostId,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl GeminiCliPreparationInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        driver: GeminiCliPreparedDriver,
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
        }
    }

    #[must_use]
    pub const fn driver(&self) -> GeminiCliPreparedDriver {
        self.driver
    }
}

/// Caller-owned controls shared by both installed Gemini CLI probes.
#[derive(Clone, Debug)]
pub struct GeminiCliPreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl GeminiCliPreparationProbe {
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

/// One explicitly selected Gemini CLI route after exact executable discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GeminiCliPreparedIntegration {
    Acp(GeminiPreparedIntegration),
    Headless(GeminiHeadlessPreparedIntegration),
}

impl GeminiCliPreparedIntegration {
    #[must_use]
    pub const fn driver(&self) -> GeminiCliPreparedDriver {
        match self {
            Self::Acp(_) => GeminiCliPreparedDriver::Acp,
            Self::Headless(_) => GeminiCliPreparedDriver::Headless,
        }
    }
}

/// Prepare one exact Gemini CLI route without inferring ACP or headless intent.
pub async fn prepare_gemini_cli(
    input: GeminiCliPreparationInput,
    probe: GeminiCliPreparationProbe,
    services: HostServices,
) -> Result<GeminiCliPreparedIntegration, PreparationFailure> {
    let GeminiCliPreparationInput {
        driver,
        instance_id,
        instance_revision,
        execution_host_id,
        target,
        environment,
        access_profile,
        access_evidence,
    } = input;
    let GeminiCliPreparationProbe {
        request_id,
        scope_id,
        deadline,
        cancellation,
    } = probe;
    match driver {
        GeminiCliPreparedDriver::Acp => prepare_gemini_acp(
            GeminiPreparationInput::new(
                instance_id,
                instance_revision,
                execution_host_id,
                target,
                environment,
                access_profile,
                access_evidence,
            ),
            GeminiPreparationProbe::new(request_id, scope_id, deadline, cancellation),
            services,
        )
        .await
        .map(GeminiCliPreparedIntegration::Acp),
        GeminiCliPreparedDriver::Headless => prepare_gemini_headless(
            GeminiHeadlessPreparationInput::new(
                instance_id,
                instance_revision,
                execution_host_id,
                target,
                environment,
                access_profile,
                access_evidence,
            ),
            GeminiHeadlessPreparationProbe::new(request_id, scope_id, deadline, cancellation),
            services,
        )
        .await
        .map(GeminiCliPreparedIntegration::Headless),
    }
}
