use super::{PreflightFixtureCase, RuntimePreflightFixture};
use swallowtail_core::{HarnessConfigurationPosture, OperationRequirements};

pub(super) fn instance_posture(case: PreflightFixtureCase) -> Option<HarnessConfigurationPosture> {
    match case {
        PreflightFixtureCase::HarnessConfigurationAmbient
        | PreflightFixtureCase::DirectInferenceHarnessConfiguration => {
            Some(HarnessConfigurationPosture::Ambient)
        }
        PreflightFixtureCase::HarnessConfigurationMismatch
        | PreflightFixtureCase::ProviderSuppressedWithoutVersionEvidence => {
            Some(HarnessConfigurationPosture::ProviderSuppressed)
        }
        PreflightFixtureCase::HostScopedHarnessConfiguration => {
            Some(HarnessConfigurationPosture::HostScoped)
        }
        _ => None,
    }
}

pub(super) fn bind_requirement(
    case: PreflightFixtureCase,
    requirements: OperationRequirements,
) -> OperationRequirements {
    match case {
        PreflightFixtureCase::HarnessConfigurationAmbient
        | PreflightFixtureCase::HarnessConfigurationMismatch
        | PreflightFixtureCase::DirectInferenceHarnessConfiguration => {
            requirements.with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        }
        PreflightFixtureCase::ProviderSuppressedWithoutVersionEvidence => requirements
            .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed),
        PreflightFixtureCase::HostScopedHarnessConfiguration => {
            requirements.with_harness_configuration_posture(HarnessConfigurationPosture::HostScoped)
        }
        _ => requirements,
    }
}

impl RuntimePreflightFixture {
    #[must_use]
    pub fn with_instance_harness_configuration_posture(
        mut self,
        posture: HarnessConfigurationPosture,
    ) -> Self {
        self.instance = self
            .instance
            .clone()
            .with_harness_configuration_posture(posture);
        self
    }
}
