use super::{ResponseState, sequence_invalid};
use crate::{MediaChunk, RealtimeMediaFailure, RealtimeMediaFailureKind, RuntimeSessionId};
use swallowtail_core::{MediaDirection, RealtimeMediaConfig};

pub(super) fn validate_output(
    session_id: &RuntimeSessionId,
    config: &RealtimeMediaConfig,
    response: &mut ResponseState,
    chunk: &MediaChunk,
) -> Result<(), RealtimeMediaFailure> {
    if chunk.session_id() != session_id {
        return Err(RealtimeMediaFailure::new(
            RealtimeMediaFailureKind::SessionMismatch,
            "Realtime media output belongs to a different session",
        ));
    }
    if chunk.direction() != MediaDirection::Output || chunk.format() != config.output_format() {
        return Err(RealtimeMediaFailure::new(
            RealtimeMediaFailureKind::FormatMismatch,
            "Realtime media output has the wrong direction or format",
        ));
    }
    if let Some(stream_id) = &response.output_stream_id {
        if chunk.stream_id() != stream_id {
            return Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::StreamMismatch,
                "Realtime media output crossed stream identities",
            ));
        }
    } else {
        response.output_stream_id = Some(chunk.stream_id().clone());
    }
    if chunk.sequence().get() != response.next_output_sequence {
        return Err(sequence_invalid());
    }
    response.next_output_sequence = response.next_output_sequence.saturating_add(1);
    Ok(())
}
