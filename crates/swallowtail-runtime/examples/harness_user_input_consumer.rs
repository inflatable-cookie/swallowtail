use swallowtail_runtime::{
    CallbackId, CallbackRequest, CallbackRequestKind, CallbackResponse, CallbackResult,
    HarnessQuestionId, HarnessQuestionOptionId, HarnessUserInputAnswer, HarnessUserInputChoiceMode,
    HarnessUserInputOption, HarnessUserInputQuestion, HarnessUserInputQuestionKind,
    HarnessUserInputRequest, HarnessUserInputResponse, OperationContent, RuntimeTurnId,
};

const MAXIMUM_RESPONSE_BYTES: usize = 64 * 1024;

/// Product UI boundary. The implementation may pause a chat, render native
/// controls, and return the exact question and option ids selected by the user.
trait StudioQuestionUi {
    fn ask(&mut self, request: &HarnessUserInputRequest) -> Vec<HarnessUserInputAnswer>;
}

#[derive(Debug)]
enum ConsumerQuestionFailure {
    NotAQuestion,
    ResponseTooLarge,
    ResponseDoesNotMatchRequest,
}

/// Build the correlated response. Submit it through
/// `CallbackExchange::responder().respond(response)`.
fn answer_question<U: StudioQuestionUi>(
    ui: &mut U,
    callback: &CallbackRequest,
) -> Result<CallbackResponse, ConsumerQuestionFailure> {
    let CallbackRequestKind::HarnessUserInput(request) = callback.kind() else {
        return Err(ConsumerQuestionFailure::NotAQuestion);
    };

    let answers = ui.ask(request);
    let response =
        HarnessUserInputResponse::new(answers, request.questions().len(), MAXIMUM_RESPONSE_BYTES)
            .map_err(|_| ConsumerQuestionFailure::ResponseTooLarge)?;

    if !request.accepts(&response) {
        return Err(ConsumerQuestionFailure::ResponseDoesNotMatchRequest);
    }

    Ok(CallbackResponse::for_request(
        callback,
        CallbackResult::UserInput(response),
    ))
}

fn main() {
    let question = HarnessUserInputQuestion::new(
        HarnessQuestionId::new("scope").unwrap(),
        OperationContent::new("Scope").unwrap(),
        OperationContent::new("Choose a scope").unwrap(),
        HarnessUserInputQuestionKind::Choice {
            mode: HarnessUserInputChoiceMode::Single,
            allow_other: false,
        },
        [HarnessUserInputOption::new(
            HarnessQuestionOptionId::new("tests").unwrap(),
            OperationContent::new("Tests").unwrap(),
            None,
        )],
    )
    .unwrap();
    let input = HarnessUserInputRequest::new([question], None, 4, 8, 4096).unwrap();
    let callback = CallbackRequest::harness_user_input(
        CallbackId::new("callback-1").unwrap(),
        RuntimeTurnId::new("turn-1").unwrap(),
        1,
        None,
        input,
    );
    let response = answer_question(&mut ExampleStudioUi, &callback).unwrap();

    assert!(matches!(response.result(), CallbackResult::UserInput(_)));
}

struct ExampleStudioUi;

impl StudioQuestionUi for ExampleStudioUi {
    fn ask(&mut self, request: &HarnessUserInputRequest) -> Vec<HarnessUserInputAnswer> {
        request
            .questions()
            .map(|question| {
                // A real UI inspects header(), prompt(), kind(), and options(),
                // then returns the stable ids supplied by the request.
                match question.kind() {
                    HarnessUserInputQuestionKind::Choice { .. } => {
                        HarnessUserInputAnswer::selected(
                            question.id().clone(),
                            [question.options().next().unwrap().id().clone()],
                            None,
                        )
                    }
                    HarnessUserInputQuestionKind::Text { .. } => HarnessUserInputAnswer::selected(
                        question.id().clone(),
                        [],
                        Some(OperationContent::new("operator response").unwrap()),
                    ),
                }
            })
            .collect()
    }
}
