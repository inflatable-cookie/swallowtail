#[test]
fn headless_model_parameters_bind_exact_rendered_model_ids() {
    let prepared = prepare(CursorPreparedDriver::Headless, host_id()).expect("prepares");
    let CursorPreparedIntegration::Headless(prepared) = prepared else {
        panic!("headless route");
    };

    let composer = prepared
        .prepare_run(parameterized_input(
            parameterized_selection("composer-2.5")
                .expect("selection")
                .with_fast(CursorHeadlessFast::Standard)
                .expect("fast"),
            ResourceAccess::ReadWrite,
        ))
        .expect("composer");
    assert_eq!(
        composer.plan().model_id().map(ModelId::as_str),
        Some("composer-2.5[fast=false]")
    );
    assert!(composer.request().policy().reasoning_mode().is_none());

    let opus5 = prepared
        .prepare_run(parameterized_input(
            parameterized_selection("claude-opus-5")
                .expect("selection")
                .with_context(CursorHeadlessContext::ThreeHundredK)
                .expect("context")
                .with_effort(ReasoningMode::new("high").expect("effort"))
                .expect("effort"),
            ResourceAccess::ReadWrite,
        ))
        .expect("opus5");
    assert_eq!(
        opus5.plan().model_id().map(ModelId::as_str),
        Some("claude-opus-5[context=300k,effort=high]")
    );
    assert_eq!(
        opus5
            .request()
            .policy()
            .reasoning_mode()
            .map(ReasoningMode::as_str),
        Some("high")
    );

    let opus48 = prepared
        .prepare_run(parameterized_input(
            parameterized_selection("claude-opus-4-8")
                .expect("selection")
                .with_context(CursorHeadlessContext::OneMillion)
                .expect("context")
                .with_effort(ReasoningMode::new("high").expect("effort"))
                .expect("effort")
                .with_fast(CursorHeadlessFast::Standard)
                .expect("fast"),
            ResourceAccess::ReadWrite,
        ))
        .expect("opus48");
    assert_eq!(
        opus48.plan().model_id().map(ModelId::as_str),
        Some("claude-opus-4-8[context=1m,effort=high,fast=false]")
    );
    assert_eq!(
        opus48
            .request()
            .policy()
            .reasoning_mode()
            .map(ReasoningMode::as_str),
        Some("high")
    );
}

#[test]
fn ask_selection_preserves_qualified_model_parameter_membership() {
    let CursorPreparedIntegration::Headless(headless) =
        prepare(CursorPreparedDriver::Headless, host_id()).expect("headless prepares")
    else {
        panic!("headless route remains explicit");
    };

    let plain = headless
        .prepare_run(parameterized_input(
            parameterized_selection("claude-opus-4-8")
                .expect("selection")
                .with_context(CursorHeadlessContext::OneMillion)
                .expect("context")
                .with_effort(ReasoningMode::new("high").expect("effort"))
                .expect("effort")
                .with_fast(CursorHeadlessFast::Standard)
                .expect("fast"),
            ResourceAccess::Read,
        ))
        .expect("parameterized read run prepares");

    let asked = headless
        .prepare_run(
            parameterized_input(
                parameterized_selection("claude-opus-4-8")
                    .expect("selection")
                    .with_context(CursorHeadlessContext::OneMillion)
                    .expect("context")
                    .with_effort(ReasoningMode::new("high").expect("effort"))
                    .expect("effort")
                    .with_fast(CursorHeadlessFast::Standard)
                    .expect("fast"),
                ResourceAccess::Read,
            )
            .with_read_mode(CursorHeadlessReadMode::Ask)
            .expect("ask selection is admitted for read authority"),
        )
        .expect("parameterized ask run prepares");

    assert_eq!(
        asked.plan().model_id().map(ModelId::as_str),
        plain.plan().model_id().map(ModelId::as_str)
    );
    assert_eq!(
        asked.request().policy().reasoning_mode(),
        plain.request().policy().reasoning_mode()
    );
    assert_eq!(asked.read_mode(), Some(CursorHeadlessReadMode::Ask));
    assert_eq!(plain.read_mode(), Some(CursorHeadlessReadMode::Plan));
}
