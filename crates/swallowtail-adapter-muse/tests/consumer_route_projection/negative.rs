use super::common;

#[test]
fn projection_does_not_publish_provider_effective_or_per_turn_state() {
    let run = common::prepare(common::host_id())
        .prepare_run(common::run_input(common::model(), "medium"))
        .expect("run");
    let contribution = run
        .consumer_route_projection_contribution(
            swallowtail_runtime::ConsumerRouteProjectionSourceId::new("negative").unwrap(),
        )
        .expect("contribution");
    for row in contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
    {
        assert!(!row.state_support().provider_effective());
        assert!(!row.mutation_authority().is_consumer_mediated_per_turn());
        assert_ne!(
            row.lifecycle(),
            swallowtail_runtime::ConsumerRouteLifecycle::PerTurn
        );
    }
}
