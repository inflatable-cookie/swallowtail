use serde_json::Value;

const USAGE_EVENTS: &str = include_str!("fixtures/opencode-v1.14.48-v1.18.4/usage.sse");

#[test]
fn qualified_step_finish_parts_expose_disjoint_usage_components() {
    let records = USAGE_EVENTS
        .lines()
        .filter_map(|line| line.strip_prefix("data: "))
        .map(|line| serde_json::from_str::<Value>(line).expect("usage event parses"))
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 3);
    assert_eq!(records.last().expect("idle event")["type"], "session.idle");

    let mut totals = [0_u64; 5];
    for record in &records[..2] {
        assert_eq!(record["type"], "message.part.updated");
        let part = &record["properties"]["part"];
        assert_eq!(part["type"], "step-finish");
        let tokens = &part["tokens"];
        for (total, value) in totals.iter_mut().zip([
            &tokens["input"],
            &tokens["output"],
            &tokens["reasoning"],
            &tokens["cache"]["read"],
            &tokens["cache"]["write"],
        ]) {
            *total += value.as_u64().expect("usage component is an integer");
        }
    }
    assert_eq!(totals, [20, 10, 3, 4, 1]);
}
