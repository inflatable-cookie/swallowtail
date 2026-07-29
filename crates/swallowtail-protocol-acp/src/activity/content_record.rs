use super::AcpBoundedText;

#[derive(Clone, Debug, PartialEq)]
pub enum AcpContentBlock {
    Text(AcpBoundedText),
    Image {
        data: AcpBoundedText,
        mime_type: AcpBoundedText,
        uri: Option<AcpBoundedText>,
    },
    Audio {
        data: AcpBoundedText,
        mime_type: AcpBoundedText,
    },
    ResourceLink {
        name: AcpBoundedText,
        uri: AcpBoundedText,
        description: Option<AcpBoundedText>,
        mime_type: Option<AcpBoundedText>,
        size: Option<i64>,
        title: Option<AcpBoundedText>,
    },
    EmbeddedTextResource {
        text: AcpBoundedText,
        uri: AcpBoundedText,
        mime_type: Option<AcpBoundedText>,
    },
    EmbeddedBlobResource {
        blob: AcpBoundedText,
        uri: AcpBoundedText,
        mime_type: Option<AcpBoundedText>,
    },
}
