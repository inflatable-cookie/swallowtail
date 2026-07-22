use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use swallowtail_core::{
    AudioEncoding, CancellationScope, Capability, CapabilityConstraint, CapabilityRequirement,
    HostServiceKind, MediaFormat, RealtimeMediaConfig,
};

pub(crate) fn realtime_media_config() -> RealtimeMediaConfig {
    let format = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).expect("sample rate is nonzero"),
        NonZeroU16::new(1).expect("channel count is nonzero"),
    );
    RealtimeMediaConfig::new(
        format,
        format,
        NonZeroU64::new(32_768).expect("chunk bound is nonzero"),
        NonZeroU32::new(2).expect("turn bound is nonzero"),
    )
}

pub(super) fn common_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveResponse,
            )],
        ),
    ]
}

pub(super) fn host_services() -> [HostServiceKind; 5] {
    [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ]
}
