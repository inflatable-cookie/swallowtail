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
    content: &[AcpToolCallContent],
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let mut display = String::new();
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
    let text = bounded_content(text);
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

fn bounded_content(text: &str) -> &str {
    let mut end = text.len().min(MAXIMUM_ACTIVITY_CONTENT_BYTES);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

pub(super) fn tool_status(status: AcpToolCallStatus) -> ActivityStatus {
    match status {
        AcpToolCallStatus::Pending => ActivityStatus::Pending,
        AcpToolCallStatus::InProgress => ActivityStatus::InProgress,
        AcpToolCallStatus::Completed => ActivityStatus::Completed,
        AcpToolCallStatus::Failed => ActivityStatus::Failed,
    }
}

pub(super) fn terminal_status(status: &TerminalStatus) -> Option<ActivityStatus> {
    match status {
        TerminalStatus::Detached => None,
        TerminalStatus::Completed => Some(ActivityStatus::Completed),
        TerminalStatus::Cancelled | TerminalStatus::TimedOut => Some(ActivityStatus::Cancelled),
        TerminalStatus::ProviderRequestObserved(_)
        | TerminalStatus::ProviderFailed(_)
        | TerminalStatus::HostFailed(_)
        | TerminalStatus::RuntimeFailed(_) => Some(ActivityStatus::Failed),
    }
}
