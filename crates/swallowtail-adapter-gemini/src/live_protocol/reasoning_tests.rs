use super::client::ClientFrame;
use super::handle::ProviderSessionHandle;
use super::tests::expected;

#[test]
fn every_admitted_thinking_level_serializes_to_its_exact_setup_frame() {
    use crate::live_reasoning::{setup_thinking_level, thinking_level};
    use swallowtail_core::ReasoningMode;

    let mode = |value: &str| ReasoningMode::new(value).expect("mode is valid");
    assert_eq!(
        setup_thinking_level(Some(&mode("minimal"))),
        setup_thinking_level(None),
        "explicit minimal and omission dispatch the same exact level"
    );
    assert_eq!(
        ClientFrame::Setup {
            handle: None,
            thinking_level: thinking_level(&mode("minimal")).expect("minimal maps"),
            maximum_output_tokens: None,
        }
        .to_json(),
        expected("client-setup-initial.json")
    );
    for (portable, corpus) in [
        ("low", "client-setup-thinking-low.json"),
        ("medium", "client-setup-thinking-medium.json"),
        ("high", "client-setup-thinking-high.json"),
    ] {
        assert_eq!(
            ClientFrame::Setup {
                handle: None,
                thinking_level: thinking_level(&mode(portable)).expect("admitted value maps"),
                maximum_output_tokens: None,
            }
            .to_json(),
            expected(corpus)
        );
    }
    let handle = ProviderSessionHandle::new("fixture-private-handle-2".to_owned());
    assert_eq!(
        ClientFrame::Setup {
            handle: Some(&handle),
            thinking_level: thinking_level(&mode("high")).expect("high maps"),
            maximum_output_tokens: None,
        }
        .to_json(),
        expected("client-setup-resume-thinking-high.json")
    );
    for value in [
        "off", "none", "disabled", "on", "auto", "default", "dynamic", "xhigh", "max", "MINIMAL",
        "Low", "1024",
    ] {
        assert!(
            thinking_level(&mode(value)).is_none(),
            "{value} maps to no exact level"
        );
    }
}

#[test]
fn admitted_output_maxima_serialize_to_exact_setup_frames() {
    use crate::live_reasoning::OMITTED_THINKING_LEVEL;
    use crate::live_reasoning::thinking_level;
    use std::num::NonZeroU64;
    use swallowtail_core::ReasoningMode;

    let maximum = |value: u64| NonZeroU64::new(value).expect("maximum is non-zero");
    for (value, corpus) in [
        (1, "client-setup-max-1.json"),
        (1024, "client-setup-max-1024.json"),
        (65_536, "client-setup-max-65536.json"),
    ] {
        assert_eq!(
            ClientFrame::Setup {
                handle: None,
                thinking_level: OMITTED_THINKING_LEVEL,
                maximum_output_tokens: Some(maximum(value)),
            }
            .to_json(),
            expected(corpus)
        );
    }
    let handle = ProviderSessionHandle::new("fixture-private-handle-2".to_owned());
    assert_eq!(
        ClientFrame::Setup {
            handle: Some(&handle),
            thinking_level: OMITTED_THINKING_LEVEL,
            maximum_output_tokens: Some(maximum(65_536)),
        }
        .to_json(),
        expected("client-setup-resume-max-65536.json")
    );
    for (portable, corpus) in [
        ("minimal", "client-setup-max-1024.json"),
        ("low", "client-setup-max-1024-thinking-low.json"),
        ("medium", "client-setup-max-1024-thinking-medium.json"),
        ("high", "client-setup-max-1024-thinking-high.json"),
    ] {
        let mode = ReasoningMode::new(portable).expect("mode is valid");
        assert_eq!(
            ClientFrame::Setup {
                handle: None,
                thinking_level: thinking_level(&mode).expect("admitted value maps"),
                maximum_output_tokens: Some(maximum(1024)),
            }
            .to_json(),
            expected(corpus),
            "maximum 1024 composes with {portable}"
        );
    }
}
