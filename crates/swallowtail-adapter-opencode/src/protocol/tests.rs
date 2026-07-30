#[cfg(test)]
mod tests {
    use super::{
        Event, PromptPayload, ProviderRequestKind, Response, SessionDeleteResponse, SseDecoder,
        abort, callback_response, classify_session_delete, observe_health, parse_catalog,
        parse_event, parse_session_for_version, prompt, question_request, session_create,
        session_delete,
    };
    use crate::selection::opencode_server_binding;
    use swallowtail_core::{InterfaceCompatibilityAssessment, ReasoningMode};
    use swallowtail_runtime::{
        CallbackPayload, CallbackResult, HarnessQuestionId, HarnessQuestionOptionId,
        HarnessUserInputAnswer, HarnessUserInputResponse, SchemaDocument,
        StructuredOutputDescriptor,
    };

    const ROOT: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/opencode-1.14.48"
    );
    const RANGE_ROOT: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/opencode-v1.14.48-v1.18.10"
    );


    include!("tests/catalogue_and_health.rs");
    include!("tests/events_and_requests.rs");
    include!("tests/callbacks_and_deletion.rs");

    fn fixture_response(name: &str) -> Response {
        Response {
            status: 200,
            body: std::fs::read(format!("{RANGE_ROOT}/{name}")).expect("range fixture reads"),
            next_cursor: None,
        }
    }
}
