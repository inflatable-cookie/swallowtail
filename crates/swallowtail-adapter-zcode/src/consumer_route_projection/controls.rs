use crate::ZcodeAppServerMode;
use crate::prepared::projection_fixture;
use swallowtail_runtime::{
    ConsumerRouteLifecycle, ConsumerRouteOmissionSemantics, ConsumerRouteValueKind,
};

use super::fixtures::{contribution, profile};
use super::ledger::{BUILD, PLAN};
use super::naming::{admitted, row};

/// Proves both controls restate the exact prepared binding, not a default.
#[test]
fn model_selection_and_app_server_mode_come_from_the_exact_prepared_binding() {
    for (name, expected_mode) in [(PLAN, "plan"), (BUILD, "build")] {
        let run = profile(name);
        assert_eq!(run.mode().as_str(), expected_mode);
        let published = contribution(&run, "zcode.app-server.controls");
        assert_eq!(
            admitted(&published, "control.app-server-mode"),
            [expected_mode.to_owned()],
            "{name} publishes a mode the prepared evidence never bound"
        );
        assert_eq!(
            admitted(&published, "control.model-selection"),
            [projection_fixture::MODEL_ID.to_owned()]
        );
        let model = published
            .applicability()
            .model()
            .expect("the prepared run binds an exact model route");
        assert_eq!(
            model.route_id().as_str(),
            projection_fixture::MODEL_ROUTE_ID
        );
        assert_eq!(model.route_revision().as_str(), "projection-1");
        assert_eq!(model.model_id().as_str(), projection_fixture::MODEL_ID);
        assert_eq!(
            model
                .provider_id()
                .map(swallowtail_core::ProviderId::as_str),
            Some(projection_fixture::PROVIDER_ID)
        );

        let mode_row = row(&published, "control.app-server-mode");
        let mode_value = mode_row
            .control_value()
            .expect("the mode control publishes its value");
        assert_eq!(
            mode_row.lifecycle(),
            ConsumerRouteLifecycle::SessionStartOnly
        );
        assert_eq!(
            mode_value.kind(),
            ConsumerRouteValueKind::BoundedEnumeration
        );
        assert_eq!(
            mode_value.omission(),
            ConsumerRouteOmissionSemantics::Required,
            "the route constructor supplies no mode default"
        );

        let model_row = row(&published, "control.model-selection");
        let model_value = model_row
            .control_value()
            .expect("the model control publishes its value");
        assert_eq!(
            model_row.lifecycle(),
            ConsumerRouteLifecycle::SelectionSummary
        );
        assert_eq!(model_value.kind(), ConsumerRouteValueKind::ExactModelRoute);
        assert_eq!(
            model_value.omission(),
            ConsumerRouteOmissionSemantics::Required,
            "the route selection constructor supplies no model default"
        );
    }
}

/// Proves the mode domain publishes only an exactly admitted mode.
///
/// `ZcodeAppServerMode` admits `plan` and `build` alone, so no descriptor,
/// command default, or unadmitted mode can reach the projected domain.
#[test]
fn an_unadmitted_mode_never_reaches_a_projected_row() {
    assert!(ZcodeAppServerMode::new("yolo").is_none());
    let published = contribution(&profile(PLAN), "zcode.app-server.mode-guard");
    let values = admitted(&published, "control.app-server-mode");
    assert_eq!(values.len(), 1, "the mode domain admits exactly one value");
    for value in values {
        assert!(
            matches!(value.as_str(), "plan" | "build"),
            "{value} is not an admitted ZCode app-server mode"
        );
    }
}
