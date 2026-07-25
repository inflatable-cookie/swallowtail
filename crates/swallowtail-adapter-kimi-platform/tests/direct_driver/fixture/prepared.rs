use super::*;
use swallowtail_adapter_kimi_platform::KimiPlatformPreparationInput;
use swallowtail_runtime::PreparedAccessEvidence;

impl Fixture {
    #[allow(dead_code)]
    pub fn preparation_input(&self) -> KimiPlatformPreparationInput {
        self.preparation_input_with_metering(EntitlementMetering::PayAsYouGo)
    }

    #[allow(dead_code)]
    pub fn preparation_input_with_metering(
        &self,
        metering: EntitlementMetering,
    ) -> KimiPlatformPreparationInput {
        let access = AccessProfile::new(
            AccessProfileId::new("access.kimi-platform.prepared").expect("access id"),
            CredentialMechanism::ApiKey,
            metering,
            self.audience.clone(),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(self.credential.clone());
        let status = AccessStatus::new(
            access.id().clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        KimiPlatformPreparationInput::new(
            self.instance_id.clone(),
            InstanceRevision::new("prepared-1").expect("revision"),
            self.host_id.clone(),
            self.target.clone(),
            access,
            PreparedAccessEvidence::caller_asserted(status),
        )
    }
}
