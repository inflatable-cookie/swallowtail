use super::protocol_failure;
use crate::local_server::interactive::session::CursorState;
use crate::local_server::interactive::websocket::resync_failure;
use crate::local_server::protocol::{PendingProviderRequest, WsEventEnvelope};
use std::sync::Mutex;
use swallowtail_runtime::RuntimeFailure;

pub(super) fn apply_cursor(
    cursor: &Mutex<CursorState>,
    envelope: &WsEventEnvelope,
) -> Result<bool, RuntimeFailure> {
    if envelope.volatile {
        return Ok(true);
    }
    let epoch = envelope.epoch.as_deref().ok_or_else(protocol_failure)?;
    let mut cursor = cursor.lock().expect("cursor lock poisoned");
    if cursor
        .epoch
        .as_deref()
        .is_some_and(|current| current != epoch)
    {
        return Err(resync_failure());
    }
    if envelope.durable_seq <= cursor.seq {
        return Ok(false);
    }
    if envelope.durable_seq != cursor.seq.saturating_add(1) {
        return Err(resync_failure());
    }
    cursor.seq = envelope.durable_seq;
    cursor.epoch = Some(epoch.to_owned());
    Ok(true)
}

pub(super) fn bind_turn(current: &mut Option<u64>, observed: u64) -> Result<(), RuntimeFailure> {
    if current.is_some_and(|current| current != observed) {
        Err(protocol_failure())
    } else {
        *current = Some(observed);
        Ok(())
    }
}

pub(super) fn validate_callback_turn(
    request: &PendingProviderRequest,
    provider_turn: &Option<u64>,
) -> Result<(), RuntimeFailure> {
    if request
        .turn_id
        .zip(*provider_turn)
        .is_some_and(|(request, active)| request != active)
    {
        Err(protocol_failure())
    } else {
        Ok(())
    }
}

pub(super) fn align_delta(offset: Option<u64>, current: &str) -> Result<bool, RuntimeFailure> {
    align_offset(offset, utf16_len(current))
}

pub(super) fn align_offset(offset: Option<u64>, current: usize) -> Result<bool, RuntimeFailure> {
    let offset =
        usize::try_from(offset.ok_or_else(protocol_failure)?).map_err(|_| protocol_failure())?;
    match offset.cmp(&current) {
        std::cmp::Ordering::Less => Ok(false),
        std::cmp::Ordering::Equal => Ok(true),
        std::cmp::Ordering::Greater => Err(resync_failure()),
    }
}

pub(super) fn utf16_len(value: &str) -> usize {
    value.encode_utf16().count()
}
