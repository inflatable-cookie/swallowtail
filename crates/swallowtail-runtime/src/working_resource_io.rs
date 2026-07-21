use crate::{BoxFuture, InputLimitExceeded, InputValueRequired, ResourceLease, RuntimeFailure};
use std::fmt;
use std::num::NonZeroUsize;

/// Provider-supplied path passed only to a host-approved working-resource service.
#[derive(Clone, Eq, PartialEq)]
pub struct WorkingResourceLocator(String);

impl WorkingResourceLocator {
    pub fn new(value: impl Into<String>) -> Result<Self, InputValueRequired> {
        crate::input::required_text("working resource locator", value).map(Self)
    }

    /// Passes the opaque locator to the execution host.
    #[must_use]
    pub fn as_host_value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for WorkingResourceLocator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WorkingResourceLocator")
            .field(&"<opaque>")
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkingResourceReadRequest {
    locator: WorkingResourceLocator,
    line: Option<usize>,
    limit: Option<usize>,
    maximum_bytes: NonZeroUsize,
}

impl WorkingResourceReadRequest {
    #[must_use]
    pub const fn new(locator: WorkingResourceLocator, maximum_bytes: NonZeroUsize) -> Self {
        Self {
            locator,
            line: None,
            limit: None,
            maximum_bytes,
        }
    }

    #[must_use]
    pub const fn with_lines(mut self, line: Option<usize>, limit: Option<usize>) -> Self {
        self.line = line;
        self.limit = limit;
        self
    }

    #[must_use]
    pub const fn locator(&self) -> &WorkingResourceLocator {
        &self.locator
    }

    #[must_use]
    pub const fn line(&self) -> Option<usize> {
        self.line
    }

    #[must_use]
    pub const fn limit(&self) -> Option<usize> {
        self.limit
    }

    #[must_use]
    pub const fn maximum_bytes(&self) -> NonZeroUsize {
        self.maximum_bytes
    }
}

/// Bounded provider callback content. Formatting never exposes the text.
#[derive(Clone, Eq, PartialEq)]
pub struct WorkingResourceText(String);

impl WorkingResourceText {
    pub fn new(value: String, maximum_bytes: NonZeroUsize) -> Result<Self, InputLimitExceeded> {
        if value.len() > maximum_bytes.get() {
            return Err(InputLimitExceeded::new(
                "working resource text",
                maximum_bytes.get(),
                value.len(),
            ));
        }
        Ok(Self(value))
    }

    /// Passes the bounded content to the requesting driver.
    #[must_use]
    pub fn as_driver_value(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkingResourceWriteRequest {
    locator: WorkingResourceLocator,
    content: WorkingResourceText,
}

impl WorkingResourceWriteRequest {
    #[must_use]
    pub const fn new(locator: WorkingResourceLocator, content: WorkingResourceText) -> Self {
        Self { locator, content }
    }

    #[must_use]
    pub const fn locator(&self) -> &WorkingResourceLocator {
        &self.locator
    }

    #[must_use]
    pub const fn content(&self) -> &WorkingResourceText {
        &self.content
    }
}

impl fmt::Debug for WorkingResourceText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkingResourceText")
            .field("bytes", &self.0.len())
            .finish()
    }
}

pub trait WorkingResourceIoService: Send + Sync {
    fn read_text(
        &self,
        lease: &ResourceLease,
        request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>>;

    fn write_text(
        &self,
        _lease: &ResourceLease,
        _request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.working_resource_write_unsupported",
                "Execution host does not support working-resource text writes",
            )))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{WorkingResourceLocator, WorkingResourceText};
    use std::num::NonZeroUsize;

    #[test]
    fn provider_locator_and_content_are_redacted() {
        let locator = WorkingResourceLocator::new("/private/project/src/lib.rs").expect("valid");
        let content = WorkingResourceText::new(
            "private source".to_owned(),
            NonZeroUsize::new(64).expect("non-zero"),
        )
        .expect("bounded");

        assert!(!format!("{locator:?}").contains("private/project"));
        assert!(!format!("{content:?}").contains("private source"));
    }
}
