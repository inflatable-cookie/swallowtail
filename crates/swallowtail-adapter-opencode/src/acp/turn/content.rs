use crate::acp::failure::failure;
use swallowtail_protocol_acp::AcpContentBlock;
use swallowtail_runtime::RuntimeFailure;

pub(super) fn text_content(content: &AcpContentBlock) -> Result<&str, RuntimeFailure> {
    match content {
        AcpContentBlock::Text(text) => Ok(text.as_str()),
        _ => Err(failure(
            "swallowtail.opencode.acp.content_unsupported",
            "OpenCode returned unsupported ACP content",
        )),
    }
}
