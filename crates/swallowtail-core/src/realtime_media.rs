use crate::{Capability, CapabilityConstraint, CapabilityRequirement, ModelId};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MediaKind {
    Audio,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MediaDirection {
    Input,
    Output,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AudioEncoding {
    Pcm16LittleEndian,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MediaFormat {
    kind: MediaKind,
    encoding: AudioEncoding,
    sample_rate_hz: NonZeroU32,
    channels: NonZeroU16,
}

impl MediaFormat {
    #[must_use]
    pub const fn audio(
        encoding: AudioEncoding,
        sample_rate_hz: NonZeroU32,
        channels: NonZeroU16,
    ) -> Self {
        Self {
            kind: MediaKind::Audio,
            encoding,
            sample_rate_hz,
            channels,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> MediaKind {
        self.kind
    }

    #[must_use]
    pub const fn encoding(&self) -> AudioEncoding {
        self.encoding
    }

    #[must_use]
    pub const fn sample_rate_hz(&self) -> NonZeroU32 {
        self.sample_rate_hz
    }

    #[must_use]
    pub const fn channels(&self) -> NonZeroU16 {
        self.channels
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealtimeMediaConfig {
    input_format: MediaFormat,
    output_format: MediaFormat,
    maximum_chunk_bytes: NonZeroU64,
    maximum_turns: NonZeroU32,
}

impl RealtimeMediaConfig {
    #[must_use]
    pub const fn new(
        input_format: MediaFormat,
        output_format: MediaFormat,
        maximum_chunk_bytes: NonZeroU64,
        maximum_turns: NonZeroU32,
    ) -> Self {
        Self {
            input_format,
            output_format,
            maximum_chunk_bytes,
            maximum_turns,
        }
    }

    #[must_use]
    pub const fn input_format(&self) -> MediaFormat {
        self.input_format
    }

    #[must_use]
    pub const fn output_format(&self) -> MediaFormat {
        self.output_format
    }

    #[must_use]
    pub const fn maximum_chunk_bytes(&self) -> NonZeroU64 {
        self.maximum_chunk_bytes
    }

    #[must_use]
    pub const fn maximum_turns(&self) -> NonZeroU32 {
        self.maximum_turns
    }

    #[must_use]
    pub fn capability_requirement(&self) -> CapabilityRequirement {
        CapabilityRequirement::new(
            Capability::RealtimeMedia,
            [
                CapabilityConstraint::RealtimeMediaFormat(MediaDirection::Input, self.input_format),
                CapabilityConstraint::RealtimeMediaFormat(
                    MediaDirection::Output,
                    self.output_format,
                ),
                CapabilityConstraint::RealtimeMediaMaximumChunkBytes(
                    self.maximum_chunk_bytes.get(),
                ),
                CapabilityConstraint::MaximumTurns(self.maximum_turns.get()),
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealtimeMediaRequirements {
    model_id: ModelId,
    config: RealtimeMediaConfig,
}

impl RealtimeMediaRequirements {
    #[must_use]
    pub const fn new(model_id: ModelId, config: RealtimeMediaConfig) -> Self {
        Self { model_id, config }
    }

    #[must_use]
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    pub const fn config(&self) -> &RealtimeMediaConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::{AudioEncoding, MediaDirection, MediaFormat, RealtimeMediaConfig};
    use crate::CapabilityConstraint;
    use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};

    #[test]
    fn exact_formats_and_bounds_become_capability_constraints() {
        let format = MediaFormat::audio(
            AudioEncoding::Pcm16LittleEndian,
            NonZeroU32::new(24_000).expect("sample rate is nonzero"),
            NonZeroU16::new(1).expect("channel count is nonzero"),
        );
        let config = RealtimeMediaConfig::new(
            format,
            format,
            NonZeroU64::new(32_768).expect("chunk bound is nonzero"),
            NonZeroU32::new(2).expect("turn bound is nonzero"),
        );
        let constraints: Vec<_> = config
            .capability_requirement()
            .constraints()
            .cloned()
            .collect();

        assert!(
            constraints.contains(&CapabilityConstraint::RealtimeMediaFormat(
                MediaDirection::Input,
                format,
            ))
        );
        assert!(
            constraints.contains(&CapabilityConstraint::RealtimeMediaFormat(
                MediaDirection::Output,
                format,
            ))
        );
        assert!(
            constraints.contains(&CapabilityConstraint::RealtimeMediaMaximumChunkBytes(
                32_768,
            ))
        );
        assert!(constraints.contains(&CapabilityConstraint::MaximumTurns(2)));
    }
}
