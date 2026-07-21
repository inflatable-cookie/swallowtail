mod tests {
    use super::*;

    const SUCCESS: &[u8] = include_bytes!(
        "../../tests/fixtures/anthropic-2023-06-01/success.sse"
    );
    const UNKNOWN: &[u8] = include_bytes!(
        "../../tests/fixtures/anthropic-2023-06-01/unknown-event.sse"
    );
    const ERROR: &[u8] = include_bytes!(
        "../../tests/fixtures/anthropic-2023-06-01/midstream-error.sse"
    );
    const DISCONNECT: &[u8] = include_bytes!(
        "../../tests/fixtures/anthropic-2023-06-01/disconnect.sse"
    );

    #[test]
    fn production_decoder_preserves_success_order_and_usage() {
        let frames = decode(SUCCESS).expect("success decodes");
        let events: Vec<_> = frames
            .iter()
            .map(|frame| parse_event(frame).expect("event parses"))
            .collect();
        assert!(matches!(events[0], Event::MessageStart(_)));
        assert!(matches!(events[3], Event::OutputDelta(ref text) if text == "Hello"));
        assert!(matches!(events[6], Event::Usage(usage) if usage.output_tokens() == Some(3)));
        assert_eq!(events.last(), Some(&Event::MessageStop));
    }

    #[test]
    fn production_decoder_ignores_top_level_unknown_and_keeps_stream_errors() {
        let unknown = decode(UNKNOWN).expect("unknown stream decodes");
        assert_eq!(parse_event(&unknown[1]).expect("unknown parses"), Event::Unknown);

        let error = decode(ERROR).expect("error stream decodes");
        assert_eq!(
            parse_event(error.last().expect("error exists")).expect("error parses"),
            Event::ProviderFailed(ProviderErrorKind::Overloaded)
        );
    }

    #[test]
    fn production_decoder_rejects_partial_frames() {
        let mut decoder = SseDecoder::default();
        decoder.push(DISCONNECT).expect("complete prefix parses");
        let error = decoder.finish().expect_err("partial frame fails");
        assert_eq!(error.diagnostic().code(), "swallowtail.anthropic.sse_disconnected");
    }

    fn decode(bytes: &[u8]) -> Result<Vec<SseFrame>, RuntimeFailure> {
        let mut decoder = SseDecoder::default();
        let frames = decoder.push(bytes)?;
        decoder.finish()?;
        Ok(frames)
    }
}
