#[cfg(test)]
mod tests {
    use super::{complete_before_deadline, parse_response};
    use futures_executor::block_on;
    use std::future::poll_fn;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::task::Poll;
    use swallowtail_core::{CatalogObservation, ModelModality};
    use swallowtail_runtime::{Deadline, MonotonicInstant, RuntimeFailure};
    use swallowtail_testkit::RecordingHostServices;

    #[test]
    fn official_language_model_shape_preserves_identity_and_modalities() {
        let models = parse_response(br#"{"models":[{"id":"latest","fingerprint":"fp_fixture","created":1776556800,"object":"model","owned_by":"xai","version":"1.0","input_modalities":["text","image"],"output_modalities":["text"],"aliases":["grok-latest"]}]}"#)
            .expect("catalogue parses");
        assert_eq!(models[0].id().as_str(), "latest");
        let observations = models[0]
            .metadata()
            .catalog_observations()
            .expect("modalities are retained");
        assert!(
            observations
                .input_modalities()
                .expect("input modalities are retained")
                .contains(&CatalogObservation::Known(ModelModality::Image))
        );
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
            "swallowtail.xai.models.timed_out"
        );
        assert!(cancelled.load(Ordering::SeqCst));
    }
}
