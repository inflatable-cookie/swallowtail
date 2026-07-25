use swallowtail_runtime::{RequestId, SessionOptions, WorkingResourceRef};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiSessionProfileInput {
    request_id: RequestId,
    working_resource: WorkingResourceRef,
    options: SessionOptions,
}

impl GeminiSessionProfileInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        options: SessionOptions,
    ) -> Self {
        Self {
            request_id,
            working_resource,
            options,
        }
    }

    pub(super) fn into_parts(self) -> (RequestId, WorkingResourceRef, SessionOptions) {
        (self.request_id, self.working_resource, self.options)
    }
}
