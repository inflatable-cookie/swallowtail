#[cfg(test)]
mod tests {
    use super::{complete_before_deadline, parse_page};
    use futures_executor::block_on;
    use std::future::poll_fn;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::task::Poll;
    use swallowtail_runtime::{Deadline, MonotonicInstant, RuntimeFailure};
    use swallowtail_testkit::RecordingHostServices;

    #[test]
    fn official_deployable_model_shape_preserves_identity_and_pagination() {
        let page = parse_page(
            br#"{"request_id":"fixture","output":{"page_no":1,"page_size":100,"total":1,"models":[{"model_name":"qwen3-8b","plans":[{"plan":"mu"},{"plan":"lora"}]}]}}"#,
            1,
        )
        .expect("page parses");
        assert_eq!(page.models[0].id().as_str(), "qwen3-8b");
        assert!(page.last_page);
    }

    #[test]
    fn in_flight_deadline_requests_transport_stop_before_returning() {
        let host = RecordingHostServices::default();
        let cancelled = Arc::new(AtomicBool::new(false));
        let work_cancelled = Arc::clone(&cancelled);
        let error = block_on(complete_before_deadline(
            poll_fn(move |_| {
                if work_cancelled.load(Ordering::SeqCst) {
                    Poll::Ready(Ok::<(), RuntimeFailure>(()))
                } else {
                    Poll::Pending
                }
            }),
            Some(Deadline::at(MonotonicInstant::from_ticks(18))),
            host.services(),
            Arc::clone(&cancelled),
        ))
        .expect_err("deadline wins");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.alibaba_model_studio.models.timed_out"
        );
        assert!(cancelled.load(Ordering::SeqCst));
    }
}
