use swallowtail_core::OperationShape;

use super::fixtures::{
    inference_contribution, inference_maximal_contribution, inventory_contribution,
    session_contribution,
};
use super::naming::{identities, rows};

#[test]
fn per_shape_fixtures_never_cross_operation_shapes() {
    let inventory = inventory_contribution("ollama.shape.inventory");
    let inference = inference_contribution("ollama.shape.inference");
    let session = session_contribution("ollama.shape.session");
    assert_eq!(
        inventory.applicability().driver_role(),
        swallowtail_core::DriverRole::ModelCatalog
    );
    assert_eq!(
        inference.applicability().operation_shape(),
        OperationShape::StructuredRun
    );
    assert_eq!(
        session.applicability().operation_shape(),
        OperationShape::InteractiveSession
    );
    let inventory_rows = rows(&inventory);
    let inference_rows = rows(&inference);
    let session_rows = rows(&session);
    assert!(inventory_rows.contains("feature.model-catalogue"));
    assert!(!inventory_rows.contains("feature.structured-run"));
    assert!(!inventory_rows.contains("feature.interactive-session"));
    assert!(!inventory_rows.contains("feature.activity-observation"));
    assert!(!inventory_rows.contains("control.model-selection"));
    assert!(inference_rows.contains("feature.structured-run"));
    assert!(!inference_rows.contains("feature.interactive-session"));
    assert!(!inference_rows.contains("feature.model-catalogue"));
    assert!(!inference_rows.contains("feature.cancellation-or-interruption"));
    assert!(session_rows.contains("feature.interactive-session"));
    assert!(session_rows.contains("feature.cancellation-or-interruption"));
    assert!(!session_rows.contains("feature.structured-run"));
    assert!(!session_rows.contains("feature.reasoning-selection"));
    assert!(!session_rows.contains("control.reasoning-selection"));
    assert!(!session_rows.contains("control.maximum-output-tokens"));
}

#[test]
fn twin_rows_are_keyed_by_shape_and_never_collapsed() {
    let inference = identities(&inference_maximal_contribution("ollama.twin.inference"));
    let session = identities(&session_contribution("ollama.twin.session"));
    let inference_model = inference
        .iter()
        .find(|(_, shape, semantic)| {
            *shape == "structured-run" && semantic == "control.model-selection"
        })
        .expect("structured-run model-selection");
    let session_model = session
        .iter()
        .find(|(_, shape, semantic)| {
            *shape == "interactive-session" && semantic == "control.model-selection"
        })
        .expect("interactive-session model-selection");
    assert_ne!(inference_model, session_model);
    assert!(inference.iter().any(|(_, shape, semantic)| {
        *shape == "structured-run" && semantic == "control.context-window"
    }));
    assert!(session.iter().any(|(_, shape, semantic)| {
        *shape == "interactive-session" && semantic == "control.context-window"
    }));
    assert!(
        !session
            .iter()
            .any(|(_, _, semantic)| semantic == "control.reasoning-selection")
    );
    assert!(inference.iter().any(|(_, shape, semantic)| {
        *shape == "structured-run" && semantic == "control.reasoning-selection"
    }));
}
