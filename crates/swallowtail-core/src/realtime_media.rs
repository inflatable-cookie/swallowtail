use crate::{Capability, CapabilityConstraint, CapabilityRequirement, ModelId};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Portable realtime-media category.
pub enum MediaKind {
    /// Audio samples.
    Audio,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Direction of media relative to the provider.
pub enum MediaDirection {
    /// Media sent to the provider.
    Input,
    /// Media produced by the provider.
    Output,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Portable audio sample encoding.
pub enum AudioEncoding {
    /// Signed 16-bit little-endian PCM.
    Pcm16LittleEndian,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Exact media kind, encoding, sample rate, and channel count.
pub struct MediaFormat {
    kind: MediaKind,
    encoding: AudioEncoding,
    sample_rate_hz: NonZeroU32,
    channels: NonZeroU16,
}

impl MediaFormat {
    #[must_use]
    /// Creates an audio media format.
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
    /// Returns the media kind.
    pub const fn kind(&self) -> MediaKind {
        self.kind
    }

    #[must_use]
    /// Returns the audio encoding.
    pub const fn encoding(&self) -> AudioEncoding {
        self.encoding
    }

    #[must_use]
    /// Returns sample rate in hertz.
    pub const fn sample_rate_hz(&self) -> NonZeroU32 {
        self.sample_rate_hz
    }

    #[must_use]
    /// Returns channel count.
    pub const fn channels(&self) -> NonZeroU16 {
        self.channels
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact input/output formats and positive realtime session bounds.
pub struct RealtimeMediaConfig {
    input_format: MediaFormat,
    output_format: MediaFormat,
    maximum_chunk_bytes: NonZeroU64,
    maximum_turns: NonZeroU32,
}

impl RealtimeMediaConfig {
    #[must_use]
    /// Creates realtime-media configuration.
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
    /// Returns input media format.
    pub const fn input_format(&self) -> MediaFormat {
        self.input_format
    }

    #[must_use]
    /// Returns output media format.
    pub const fn output_format(&self) -> MediaFormat {
        self.output_format
    }

    #[must_use]
    /// Returns maximum bytes in one media chunk.
    pub const fn maximum_chunk_bytes(&self) -> NonZeroU64 {
        self.maximum_chunk_bytes
    }

    #[must_use]
    /// Returns maximum turns in the media session.
    pub const fn maximum_turns(&self) -> NonZeroU32 {
        self.maximum_turns
    }

    #[must_use]
    /// Projects configuration into an exact realtime capability requirement.
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
/// Exact model and media configuration required by a realtime session.
pub struct RealtimeMediaRequirements {
    model_id: ModelId,
    config: RealtimeMediaConfig,
}

impl RealtimeMediaRequirements {
    #[must_use]
    /// Creates requirements for one model and media configuration.
    pub const fn new(model_id: ModelId, config: RealtimeMediaConfig) -> Self {
        Self { model_id, config }
    }

    #[must_use]
    /// Returns the required model identity.
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    /// Returns exact media configuration.
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
