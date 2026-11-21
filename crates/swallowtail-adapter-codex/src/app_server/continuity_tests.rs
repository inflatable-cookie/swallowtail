use super::{MAXIMUM_MODEL_CATALOG_PAGES, collect_model_pages};
use crate::control_wait::{Controlled, wait_controlled_optional};
use serde_json::{Value, json};
use std::cell::Cell;
use swallowtail_runtime::{Deadline, DeadlineObservation, MonotonicInstant};

fn model_page(model: &str, next_cursor: Option<&str>) -> Value {
    let mut page = json!({
        "data": [{
            "model": model,
            "displayName": model,
            "description": "fixture model",
            "isDefault": false,
            "supportedReasoningEfforts": [{"reasoningEffort": "medium"}],
            "defaultReasoningEffort": "medium"
        }]
    });
    if let Some(cursor) = next_cursor {
        page["nextCursor"] = json!(cursor);
    }
    page
}

#[test]
fn model_pages_are_collected_until_the_cursor_ends() {
    let models = futures_executor::block_on(collect_model_pages(|cursor| async move {
        Ok(match cursor.as_deref() {
            None => model_page("first", Some("page-2")),
            Some("page-2") => model_page("second", None),
            other => panic!("unexpected cursor {other:?}"),
        })
    }))
    .expect("two bounded pages are a valid catalogue");

    let ids: Vec<&str> = models.iter().map(|entry| entry.id().as_str()).collect();
    assert_eq!(ids, ["first", "second"]);
}

#[test]
fn a_repeated_model_cursor_is_rejected_instead_of_looping() {
    let calls = Cell::new(0usize);
    let error = futures_executor::block_on(collect_model_pages(|_cursor| {
        calls.set(calls.get() + 1);
        async move { Ok(model_page("looping", Some("same-cursor"))) }
    }))
    .expect_err("a cursor the server already issued must stop the walk");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.catalog_cursor_repeated"
    );
    assert_eq!(
        calls.get(),
        2,
        "the walk stops on the first repeated cursor"
    );
}

#[test]
fn model_pagination_stops_at_the_page_limit() {
    let calls = Cell::new(0usize);
    let error = futures_executor::block_on(collect_model_pages(|_cursor| {
        let index = calls.get();
        calls.set(index + 1);
        let cursor = format!("cursor-{index}");
        async move { Ok(model_page("endless", Some(&cursor))) }
    }))
    .expect_err("an endless cursor chain must hit the page limit");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.catalog_page_limit"
    );
    assert_eq!(calls.get(), MAXIMUM_MODEL_CATALOG_PAGES);
}

#[test]
fn an_initialize_without_cancellation_is_still_bounded_by_its_deadline() {
    let observed = DeadlineObservation::new(
        Deadline::at(MonotonicInstant::from_ticks(1)),
        MonotonicInstant::from_ticks(2),
    );
    let outcome = futures_executor::block_on(wait_controlled_optional(
        std::future::pending::<()>(),
        None,
        Some(Box::pin(std::future::ready(observed))),
    ));

    assert!(
        matches!(outcome, Controlled::Deadline),
        "a handshake that never settles must yield to the caller deadline"
    );
}
