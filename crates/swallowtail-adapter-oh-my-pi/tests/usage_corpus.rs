use serde_json::Value;

const USAGE_EVENTS: &str = include_str!("fixtures/oh-my-pi-rpc-17.2.9/usage-events.jsonl");

#[test]
fn qualified_assistant_messages_expose_disjoint_usage_components() {
    let records = USAGE_EVENTS
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("usage event parses"))
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 3);
    assert_eq!(records.last().expect("settled event")["type"], "agent_end");

    let mut totals = [0_u64; 4];
    for record in &records[..2] {
        assert_eq!(record["type"], "message_end");
        let usage = &record["message"]["usage"];
        for (total, field) in totals
            .iter_mut()
            .zip(["input", "output", "cacheRead", "cacheWrite"])
        {
            *total += usage[field]
                .as_u64()
                .expect("usage component is an integer");
        }
    }
    assert_eq!(totals, [20, 10, 4, 2]);
}
