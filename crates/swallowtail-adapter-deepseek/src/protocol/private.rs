use super::{ProtocolFailure, ProtocolFailureKind};
use std::fmt;
use std::num::NonZeroU64;

pub(crate) struct PrivateContinuation {
    bytes: Vec<u8>,
}

#[derive(Default)]
pub(crate) struct PrivateAccumulator {
    bytes: Vec<u8>,
}

impl PrivateAccumulator {
    pub(crate) fn push(
        &mut self,
        value: &str,
        maximum_bytes: NonZeroU64,
    ) -> Result<(), ProtocolFailure> {
        if self.bytes.len().saturating_add(value.len()) as u64 > maximum_bytes.get() {
            return Err(ProtocolFailure::new(ProtocolFailureKind::BoundExceeded));
        }
        self.bytes.extend_from_slice(value.as_bytes());
        Ok(())
    }

    pub(crate) fn take(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }
}

impl Drop for PrivateAccumulator {
    fn drop(&mut self) {
        self.bytes.fill(0);
    }
}

impl PrivateContinuation {
    pub(crate) fn new(
        value: impl Into<Vec<u8>>,
        maximum_bytes: NonZeroU64,
    ) -> Result<Self, ProtocolFailure> {
        let bytes = value.into();
        if bytes.is_empty()
            || u64::try_from(bytes.len()).is_err()
            || bytes.len() as u64 > maximum_bytes.get()
            || std::str::from_utf8(&bytes).is_err()
        {
            return Err(ProtocolFailure::new(ProtocolFailureKind::BoundExceeded));
        }
        Ok(Self { bytes })
    }

    pub(crate) fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes).expect("validated private continuation remains UTF-8")
    }

    pub(crate) fn byte_len(&self) -> usize {
        self.bytes.len()
    }
}

impl Drop for PrivateContinuation {
    fn drop(&mut self) {
        self.bytes.fill(0);
    }
}

impl fmt::Debug for PrivateContinuation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PrivateContinuation")
            .field(&format_args!("<private:{} bytes>", self.bytes.len()))
            .finish()
    }
}
