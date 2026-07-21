use super::{RecordedHostCall, RecordingService};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, ResourceAccess, ResourceLease, RuntimeFailure, WorkingResourceIoService,
    WorkingResourceReadRequest, WorkingResourceText, WorkingResourceWriteRequest,
};

impl WorkingResourceIoService for RecordingService {
    fn read_text(
        &self,
        _lease: &ResourceLease,
        request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        let result = self
            .record(RecordedHostCall::WorkingResourceReadText)
            .and_then(|()| {
                WorkingResourceText::new(
                    "recording resource text".to_owned(),
                    request.maximum_bytes(),
                )
                .map_err(|_| {
                    RuntimeFailure::new(SafeDiagnostic::new(
                        "fixture.resource_read_failed",
                        "Recording resource read failed",
                    ))
                })
            });
        Box::pin(async move { result })
    }

    fn write_text(
        &self,
        lease: &ResourceLease,
        _request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let result = if lease.access() == ResourceAccess::ReadWrite {
            self.record(RecordedHostCall::WorkingResourceWriteText)
        } else {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "fixture.resource_write_denied",
                "Recording resource lease does not allow writes",
            )))
        };
        Box::pin(async move { result })
    }
}
