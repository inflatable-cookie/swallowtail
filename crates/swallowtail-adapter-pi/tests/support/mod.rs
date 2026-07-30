mod host;
mod selection;

pub use host::{CleanupEvent, FixtureHost, Scenario};
pub use selection::{
    FixtureSelection, open_request, selection, selection_for_topology, turn_request,
};
#[allow(unused_imports)]
pub use selection::{run_request, run_selection_for_topology};

pub fn allow_user_input_result(
    request: &swallowtail_runtime::CallbackRequest,
) -> swallowtail_runtime::CallbackResult {
    let swallowtail_runtime::CallbackRequestKind::HarnessUserInput(user_input) = request.kind()
    else {
        panic!("fixture callback is typed user input");
    };
    let question = user_input
        .questions()
        .next()
        .expect("fixture has one question");
    swallowtail_runtime::CallbackResult::UserInput(
        swallowtail_runtime::HarnessUserInputResponse::new(
            [swallowtail_runtime::HarnessUserInputAnswer::selected(
                question.id().clone(),
                [swallowtail_runtime::HarnessQuestionOptionId::new("Allow").unwrap()],
                None,
            )],
            1,
            64,
        )
        .unwrap(),
    )
}
