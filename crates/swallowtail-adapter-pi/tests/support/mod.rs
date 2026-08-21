// Every integration binary compiles this module tree but uses only a subset;
// unused-fixture lints fire per binary, not per module.
#[allow(dead_code)]
mod host;
#[allow(dead_code)]
mod selection;
#[allow(dead_code)]
mod sidecar_host;
#[allow(dead_code)]
mod sidecar_selection;

#[allow(unused_imports)]
pub use host::{CleanupEvent, FixtureHost, Scenario};
#[allow(unused_imports)]
pub use selection::{
    FixtureSelection, open_request, selection, selection_for_topology, turn_request,
};
#[allow(unused_imports)]
pub use selection::{run_request, run_selection_for_topology};
#[allow(unused_imports)]
pub use sidecar_host::{FIXTURE_SESSION_REF, SidecarFixtureHost, SidecarScenario};
#[allow(unused_imports)]
pub use sidecar_selection::{
    SidecarFixtureSelection, sidecar_catalogue_selection, sidecar_open_request, sidecar_selection,
    sidecar_selection_with_attachments, sidecar_selection_with_instance_versions, sidecar_versions,
};

#[allow(dead_code)]
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
