use crate::failure::failure;
use swallowtail_protocol_acp::AcpContentBlock;
use swallowtail_runtime::RuntimeFailure;

pub(super) fn text_content(content: &AcpContentBlock) -> Result<&str, RuntimeFailure> {
    match content {
        AcpContentBlock::Text(text) => Ok(text.as_str()),
        _ => Err(failure(
            "swallowtail.cline.acp.content_unsupported",
            "Cline returned unsupported ACP content",
        )),
    }
}
