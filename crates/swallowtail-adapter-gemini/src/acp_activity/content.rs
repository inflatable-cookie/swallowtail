use crate::failure::malformed;
use swallowtail_core::ActivityContentStream;
use swallowtail_protocol_acp::{AcpContentBlock, AcpToolCallContent, AcpToolCallStatus};
use swallowtail_runtime::{
    ActivityContent, ActivityContentChangeKind, ActivityContentUpdate, ActivityStatus,
    OperationContent, RuntimeFailure, TerminalStatus,
};

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(super) fn text_content(content: &AcpContentBlock) -> Result<&str, RuntimeFailure> {
    match content {
        AcpContentBlock::Text(text) => Ok(text.as_str()),
        _ => Err(malformed()),
    }
}

pub(super) fn tool_content(
    title: &str,
    content: &[AcpToolCallContent],
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let mut display = title.trim().to_owned();
    for item in content {
        let text = match item {
            AcpToolCallContent::Content(AcpContentBlock::Text(text)) => text.as_str(),
            AcpToolCallContent::Diff { new_text, .. } => new_text.as_str(),
            _ => continue,
        };
        if !display.is_empty() {
            display.push('\n');
        }
        display.push_str(text);
    }
    content_update(
        &display,
        ActivityContentChangeKind::ReplacementSnapshot,
        ActivityContentStream::ProviderToolDisplay,
    )
}

pub(super) fn content_update(
    text: &str,
    change: ActivityContentChangeKind,
    stream: ActivityContentStream,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    let content = ActivityContent::new(
        OperationContent::new(text).map_err(|_| malformed())?,
        MAXIMUM_ACTIVITY_CONTENT_BYTES,
    )
    .map_err(|_| malformed())?;
    Ok(Some(ActivityContentUpdate::new(change, stream, content)))
}

pub(super) fn tool_status(status: AcpToolCallStatus) -> ActivityStatus {
    match status {
        AcpToolCallStatus::Pending => ActivityStatus::Pending,
        AcpToolCallStatus::InProgress => ActivityStatus::InProgress,
        AcpToolCallStatus::Completed => ActivityStatus::Completed,
        AcpToolCallStatus::Failed => ActivityStatus::Failed,
    }
}

pub(super) fn terminal_status(status: &TerminalStatus) -> ActivityStatus {
    match status {
        TerminalStatus::Completed => ActivityStatus::Completed,
        TerminalStatus::Cancelled
        | TerminalStatus::TimedOut
        | TerminalStatus::ProviderRequestObserved(_) => ActivityStatus::Cancelled,
        TerminalStatus::ProviderFailed(_)
        | TerminalStatus::HostFailed(_)
        | TerminalStatus::RuntimeFailed(_) => ActivityStatus::Failed,
    }
}
