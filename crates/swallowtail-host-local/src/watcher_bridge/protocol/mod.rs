//! Closed HTTP/MCP decode, dispatch, and encode.

mod decode;
mod encode;

pub(super) use decode::{
    DecodedRequest, correlation_id, decode_request, decoded_request_id, recoverable_request_id,
};
pub(super) use encode::{error_http_status, error_message, jsonrpc_error};

use super::failure::{closed_failure, malformed_failure, unauthorized_failure, unknown_failure};
use super::state::{LiveLease, drive, owning_turn};
use decode::DecodedRequest as Request;
use encode::{
    completion_payload, initialize_result, jsonrpc_result, list_payload, snapshot_payload,
    stop_payload, tool_text, tools_list_result, wait_payload,
};
use serde_json::{Map, Value};
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{WatcherId, WatcherOperationData, WatcherRequester};
use swallowtail_runtime::{
    Deadline, MonotonicInstant, RuntimeFailure, WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
    WATCHER_BRIDGE_TOOL_INSPECT, WATCHER_BRIDGE_TOOL_LIST, WATCHER_BRIDGE_TOOL_START,
    WATCHER_BRIDGE_TOOL_STOP, WATCHER_BRIDGE_TOOL_WAIT, WatcherWaitOptions,
};

pub(super) fn dispatch(
    live: &Arc<LiveLease>,
    request: Request,
) -> Result<Option<String>, RuntimeFailure> {
    match request {
        Request::Initialized => {
            live.admit_initialized()?;
            Ok(None)
        }
        Request::Initialize { id } => {
            live.admit_initialize()?;
            Ok(Some(jsonrpc_result(id, initialize_result())))
        }
        Request::ToolsList { id } => {
            live.require_ready()?;
            Ok(Some(jsonrpc_result(id, tools_list_result())))
        }
        Request::ToolsCall {
            id,
            name,
            arguments,
        } => {
            live.require_ready()?;
            let result = dispatch_tool(live, &name, arguments)?;
            Ok(Some(jsonrpc_result(id, result)))
        }
    }
}

pub(super) fn authenticate(
    live: &LiveLease,
    presented: Option<&str>,
) -> Result<(), RuntimeFailure> {
    let presented = presented.ok_or_else(unauthorized_failure)?;
    if constant_time_eq(live.bearer.as_bytes(), presented.as_bytes()) {
        Ok(())
    } else {
        Err(unauthorized_failure())
    }
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0_u8, |acc, (a, b)| acc | (a ^ b))
        == 0
}

fn dispatch_tool(
    live: &Arc<LiveLease>,
    name: &str,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    match name {
        WATCHER_BRIDGE_TOOL_START => call_start(live, arguments),
        WATCHER_BRIDGE_TOOL_INSPECT => call_inspect(live, arguments),
        WATCHER_BRIDGE_TOOL_LIST => {
            require_empty_object(&arguments)?;
            live.require_not_closed()?;
            let snapshots = drive(live.watcher.list(owning_turn(&live.turn)?))?;
            Ok(tool_text(list_payload(&snapshots)))
        }
        WATCHER_BRIDGE_TOOL_WAIT => call_wait(live, arguments),
        WATCHER_BRIDGE_TOOL_STOP => call_stop(live, arguments),
        WATCHER_BRIDGE_TOOL_COMPLETION_GATE => {
            require_empty_object(&arguments)?;
            let state = live.completion_gate()?;
            Ok(tool_text(completion_payload(&state)))
        }
        _ => Err(unknown_failure()),
    }
}

fn call_start(
    live: &Arc<LiveLease>,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    let operation_data = WatcherOperationData::new(require_string(&arguments, "operation_data")?)
        .map_err(|_| malformed_failure())?;
    require_only_keys(&arguments, &["operation_data"])?;
    live.begin_create()?;
    let _creating = CreatingGuard(live);
    let snapshot = drive_until_cancel(
        live,
        live.watcher
            .accept_start(live.turn.clone(), WatcherRequester::Model, operation_data),
    )?;
    Ok(tool_text(snapshot_payload(&snapshot)))
}

fn call_inspect(
    live: &Arc<LiveLease>,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    let watcher_id = watcher_id(&arguments)?;
    require_only_keys(&arguments, &["watcher_id"])?;
    live.require_not_closed()?;
    let snapshot = drive(live.watcher.inspect(owning_turn(&live.turn)?, watcher_id))?;
    Ok(tool_text(snapshot_payload(&snapshot)))
}

fn call_wait(
    live: &Arc<LiveLease>,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    let watcher_id = watcher_id(&arguments)?;
    require_only_keys(&arguments, &["watcher_id"])?;
    live.require_not_closed()?;
    let now = live.time.now();
    let deadline = Deadline::at(MonotonicInstant::from_ticks(
        now.ticks().saturating_add(nanos(live.wait_bound)),
    ));
    let representation = drive(
        live.watcher.wait(
            owning_turn(&live.turn)?,
            watcher_id.clone(),
            WatcherWaitOptions::new()
                .with_cancellation(live.cancel.wait_requested())
                .with_deadline(live.time.wait_until(deadline)),
        ),
    )?;
    let snapshot = drive(live.watcher.inspect(owning_turn(&live.turn)?, watcher_id))?;
    Ok(tool_text(wait_payload(representation, &snapshot)))
}

fn call_stop(
    live: &Arc<LiveLease>,
    arguments: Map<String, Value>,
) -> Result<Value, RuntimeFailure> {
    let watcher_id = watcher_id(&arguments)?;
    require_only_keys(&arguments, &["watcher_id"])?;
    live.require_not_closed()?;
    let (acknowledgement, snapshot) = drive(
        live.watcher
            .request_stop(owning_turn(&live.turn)?, watcher_id),
    )?;
    Ok(tool_text(stop_payload(acknowledgement, &snapshot)))
}

fn require_empty_object(arguments: &Map<String, Value>) -> Result<(), RuntimeFailure> {
    if arguments.is_empty() {
        Ok(())
    } else {
        Err(unknown_failure())
    }
}

fn require_only_keys(
    arguments: &Map<String, Value>,
    allowed: &[&str],
) -> Result<(), RuntimeFailure> {
    if arguments.len() == allowed.len() && allowed.iter().all(|key| arguments.contains_key(*key)) {
        Ok(())
    } else {
        Err(unknown_failure())
    }
}

fn require_string(arguments: &Map<String, Value>, key: &str) -> Result<String, RuntimeFailure> {
    arguments
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(malformed_failure)
}

fn watcher_id(arguments: &Map<String, Value>) -> Result<WatcherId, RuntimeFailure> {
    WatcherId::new(require_string(arguments, "watcher_id")?).map_err(|_| malformed_failure())
}

fn drive_until_cancel<T>(
    live: &LiveLease,
    mut work: swallowtail_runtime::BoxFuture<'_, Result<T, RuntimeFailure>>,
) -> Result<T, RuntimeFailure> {
    let mut cancel = live.cancel.wait_requested();
    futures_executor::block_on(std::future::poll_fn(|context| {
        if cancel.as_mut().poll(context).is_ready() {
            return Poll::Ready(Err(closed_failure()));
        }
        work.as_mut().poll(context)
    }))
}

fn nanos(bound: std::time::Duration) -> u64 {
    u64::try_from(bound.as_nanos()).unwrap_or(u64::MAX)
}

struct CreatingGuard<'a>(&'a LiveLease);

impl Drop for CreatingGuard<'_> {
    fn drop(&mut self) {
        self.0.end_create();
    }
}
