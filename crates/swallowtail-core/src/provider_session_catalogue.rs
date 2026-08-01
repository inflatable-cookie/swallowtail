use crate::SafeDiagnostic;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

pub const MAX_PROVIDER_SESSION_PAGE_SIZE: u32 = 1_000;
pub const MAX_PROVIDER_SESSION_TOTAL_CANDIDATES: u32 = 10_000;
pub const MAX_PROVIDER_SESSION_CURSOR_BYTES: u32 = 4_096;
pub const MAX_PROVIDER_SESSION_CONTENT_BYTES: u32 = 16_384;
pub const MAX_PROVIDER_SESSION_REFERENCE_BYTES: u32 = 4_096;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionDiscoveryScope {
    WorkingResource,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionActivityState {
    Unknown,
    Inactive,
    Active,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionImportUnavailableReason {
    Active,
    Archived,
    IncompleteHistory,
    IncompatibleInterface,
    ResourceMismatch,
    ProviderReportedUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderSessionImportAvailability {
    Available,
    Unavailable(ProviderSessionImportUnavailableReason),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InvalidProviderSessionCatalogueRecordKind {
    LimitExceeded,
    InvalidBounds,
    EmptyContent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidProviderSessionCatalogueRecord {
    kind: InvalidProviderSessionCatalogueRecordKind,
    diagnostic: SafeDiagnostic,
}

impl InvalidProviderSessionCatalogueRecord {
    fn new(
        kind: InvalidProviderSessionCatalogueRecordKind,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> InvalidProviderSessionCatalogueRecordKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidProviderSessionCatalogueRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidProviderSessionCatalogueRecord {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderSessionCatalogueBounds {
    maximum_page_size: NonZeroU32,
    maximum_total_candidates: NonZeroU32,
    maximum_cursor_bytes: NonZeroU32,
    maximum_content_bytes: NonZeroU32,
    maximum_provider_reference_bytes: NonZeroU32,
}

impl ProviderSessionCatalogueBounds {
    pub fn new(
        maximum_page_size: NonZeroU32,
        maximum_total_candidates: NonZeroU32,
        maximum_cursor_bytes: NonZeroU32,
        maximum_content_bytes: NonZeroU32,
        maximum_provider_reference_bytes: NonZeroU32,
    ) -> Result<Self, InvalidProviderSessionCatalogueRecord> {
        if maximum_page_size.get() > MAX_PROVIDER_SESSION_PAGE_SIZE
            || maximum_total_candidates.get() > MAX_PROVIDER_SESSION_TOTAL_CANDIDATES
            || maximum_cursor_bytes.get() > MAX_PROVIDER_SESSION_CURSOR_BYTES
            || maximum_content_bytes.get() > MAX_PROVIDER_SESSION_CONTENT_BYTES
            || maximum_provider_reference_bytes.get() > MAX_PROVIDER_SESSION_REFERENCE_BYTES
        {
            return Err(invalid(
                InvalidProviderSessionCatalogueRecordKind::LimitExceeded,
                "swallowtail.provider_session_catalogue.limit_exceeded",
                "Provider-session catalogue bounds exceed portable limits",
            ));
        }
        if maximum_page_size > maximum_total_candidates {
            return Err(invalid(
                InvalidProviderSessionCatalogueRecordKind::InvalidBounds,
                "swallowtail.provider_session_catalogue.bounds_invalid",
                "Provider-session page bound exceeds its traversal bound",
            ));
        }
        Ok(Self {
            maximum_page_size,
            maximum_total_candidates,
            maximum_cursor_bytes,
            maximum_content_bytes,
            maximum_provider_reference_bytes,
        })
    }

    #[must_use]
    pub const fn maximum_page_size(self) -> NonZeroU32 {
        self.maximum_page_size
    }

    #[must_use]
    pub const fn maximum_total_candidates(self) -> NonZeroU32 {
        self.maximum_total_candidates
    }

    #[must_use]
    pub const fn maximum_cursor_bytes(self) -> NonZeroU32 {
        self.maximum_cursor_bytes
    }

    #[must_use]
    pub const fn maximum_content_bytes(self) -> NonZeroU32 {
        self.maximum_content_bytes
    }

    #[must_use]
    pub const fn maximum_provider_reference_bytes(self) -> NonZeroU32 {
        self.maximum_provider_reference_bytes
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ProviderSessionDisplayContent {
    title: Option<String>,
    preview: Option<String>,
}

impl ProviderSessionDisplayContent {
    pub fn new(
        title: Option<String>,
        preview: Option<String>,
    ) -> Result<Self, InvalidProviderSessionCatalogueRecord> {
        if title.as_ref().is_some_and(|value| value.trim().is_empty())
            || preview
                .as_ref()
                .is_some_and(|value| value.trim().is_empty())
        {
            return Err(invalid(
                InvalidProviderSessionCatalogueRecordKind::EmptyContent,
                "swallowtail.provider_session_catalogue.content_empty",
                "Provider-session display content must not contain empty fields",
            ));
        }
        let content = Self { title, preview };
        if content.byte_len() > MAX_PROVIDER_SESSION_CONTENT_BYTES as usize {
            return Err(invalid(
                InvalidProviderSessionCatalogueRecordKind::LimitExceeded,
                "swallowtail.provider_session_catalogue.content_limit_exceeded",
                "Provider-session display content exceeds its portable limit",
            ));
        }
        Ok(content)
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self {
            title: None,
            preview: None,
        }
    }

    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    #[must_use]
    pub fn preview(&self) -> Option<&str> {
        self.preview.as_deref()
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.title.as_ref().map_or(0, String::len) + self.preview.as_ref().map_or(0, String::len)
    }
}

impl fmt::Debug for ProviderSessionDisplayContent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderSessionDisplayContent")
            .field("title", &self.title.as_ref().map(String::len))
            .field("preview", &self.preview.as_ref().map(String::len))
            .finish()
    }
}

fn invalid(
    kind: InvalidProviderSessionCatalogueRecordKind,
    code: &'static str,
    message: &'static str,
) -> InvalidProviderSessionCatalogueRecord {
    InvalidProviderSessionCatalogueRecord::new(kind, code, message)
}

#[cfg(test)]
mod tests {
    use super::{
        InvalidProviderSessionCatalogueRecordKind, ProviderSessionCatalogueBounds,
        ProviderSessionDisplayContent,
    };
    use std::num::NonZeroU32;

    #[test]
    fn bounds_reject_oversized_and_inverted_limits() {
        let oversized = ProviderSessionCatalogueBounds::new(
            NonZeroU32::new(1_001).expect("nonzero"),
            NonZeroU32::new(10_000).expect("nonzero"),
            NonZeroU32::new(4_096).expect("nonzero"),
            NonZeroU32::new(16_384).expect("nonzero"),
            NonZeroU32::new(4_096).expect("nonzero"),
        )
        .expect_err("oversized page bound must fail");
        assert_eq!(
            oversized.kind(),
            InvalidProviderSessionCatalogueRecordKind::LimitExceeded
        );

        let inverted = ProviderSessionCatalogueBounds::new(
            NonZeroU32::new(10).expect("nonzero"),
            NonZeroU32::new(9).expect("nonzero"),
            NonZeroU32::new(64).expect("nonzero"),
            NonZeroU32::new(64).expect("nonzero"),
            NonZeroU32::new(64).expect("nonzero"),
        )
        .expect_err("page bound above traversal bound must fail");
        assert_eq!(
            inverted.kind(),
            InvalidProviderSessionCatalogueRecordKind::InvalidBounds
        );
    }

    #[test]
    fn display_content_is_available_but_redacted() {
        let content = ProviderSessionDisplayContent::new(
            Some("private title".to_owned()),
            Some("private preview".to_owned()),
        )
        .expect("content is valid");

        assert_eq!(content.title(), Some("private title"));
        assert_eq!(content.preview(), Some("private preview"));
        assert!(!format!("{content:?}").contains("private"));
    }
}
