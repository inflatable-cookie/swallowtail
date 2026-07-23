use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{
    DirectAttemptTransport, DirectContinuationConfig, DirectToolSelection,
    ProviderInferenceCachePolicy,
};

pub(crate) fn config() -> DirectContinuationConfig {
    DirectContinuationConfig::new(
        NonZeroU32::new(2).unwrap(),
        NonZeroU32::new(3).unwrap(),
        NonZeroU32::new(8).unwrap(),
        NonZeroU32::new(1).unwrap(),
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU64::new(262_144).unwrap(),
        NonZeroU64::new(1_048_576).unwrap(),
        NonZeroU32::new(4_096).unwrap(),
        NonZeroU64::new(8_192).unwrap(),
        DirectAttemptTransport::Buffered,
        DirectAttemptTransport::ServerSentEvents,
        DirectToolSelection::ProviderAutomatic,
        ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
    )
}
