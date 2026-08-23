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
