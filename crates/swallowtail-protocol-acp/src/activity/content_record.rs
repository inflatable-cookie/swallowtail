use super::AcpBoundedText;

/// One bounded ACP content block retained for semantic projection.
#[derive(Clone, Debug, PartialEq)]
pub enum AcpContentBlock {
    /// Plain text content.
    Text(AcpBoundedText),
    /// Inline image content.
    Image {
        /// Encoded image data.
        data: AcpBoundedText,
        /// Declared image media type.
        mime_type: AcpBoundedText,
        /// Optional source URI.
        uri: Option<AcpBoundedText>,
    },
    /// Inline audio content.
    Audio {
        /// Encoded audio data.
        data: AcpBoundedText,
        /// Declared audio media type.
        mime_type: AcpBoundedText,
    },
    /// Link to a provider- or host-owned resource.
    ResourceLink {
        /// Display name.
        name: AcpBoundedText,
        /// Resource URI.
        uri: AcpBoundedText,
        /// Optional display description.
        description: Option<AcpBoundedText>,
        /// Optional declared media type.
        mime_type: Option<AcpBoundedText>,
        /// Optional resource size in bytes.
        size: Option<i64>,
        /// Optional display title.
        title: Option<AcpBoundedText>,
    },
    /// Embedded text resource.
    EmbeddedTextResource {
        /// Embedded text.
        text: AcpBoundedText,
        /// Resource URI.
        uri: AcpBoundedText,
        /// Optional declared media type.
        mime_type: Option<AcpBoundedText>,
    },
    /// Embedded binary resource encoded as text.
    EmbeddedBlobResource {
        /// Encoded binary data.
        blob: AcpBoundedText,
        /// Resource URI.
        uri: AcpBoundedText,
        /// Optional declared media type.
        mime_type: Option<AcpBoundedText>,
    },
}
