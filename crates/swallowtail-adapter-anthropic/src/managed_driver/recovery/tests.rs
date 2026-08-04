mod recovery_tests {
    use super::*;

    #[test]
    fn exact_event_sequences_preserve_active_waiting_terminal_and_unknown_truth() {
        let cases = [
            (
                crate::managed::ManagedSessionStatus::Running,
                r#"[{"id":"running","type":"session.status_running","processed_at":"2026-08-04T00:00:00Z"}]"#,
                InterruptedRunState::Active,
            ),
            (
                crate::managed::ManagedSessionStatus::Idle,
                r#"[{"id":"wait","type":"session.status_idle","processed_at":"2026-08-04T00:00:00Z","stop_reason":{"type":"requires_action","event_ids":["tool"]}}]"#,
                InterruptedRunState::WaitingForProviderInput,
            ),
            (
                crate::managed::ManagedSessionStatus::Idle,
                r#"[{"id":"failed","type":"session.status_idle","processed_at":"2026-08-04T00:00:00Z","stop_reason":{"type":"retries_exhausted"}}]"#,
                InterruptedRunState::Failed,
            ),
            (
                crate::managed::ManagedSessionStatus::Idle,
                r#"[{"id":"message","type":"user.message","processed_at":"2026-08-04T00:00:00Z","content":[{"type":"text","text":"task"}]},{"id":"interrupt","type":"user.interrupt","processed_at":"2026-08-04T00:00:01Z"},{"id":"idle","type":"session.status_idle","processed_at":"2026-08-04T00:00:02Z","stop_reason":{"type":"end_turn"}}]"#,
                InterruptedRunState::Cancelled,
            ),
            (
                crate::managed::ManagedSessionStatus::Terminated,
                r#"[{"id":"terminated","type":"session.status_terminated","processed_at":"2026-08-04T00:00:00Z"}]"#,
                InterruptedRunState::Unknown,
            ),
        ];
        for (status, data, expected) in cases {
            let history = format!(r#"{{"data":{data},"next_page":null}}"#);
            let events = crate::managed::parse_history(history.as_bytes()).expect("history parses");
            let snapshot = crate::managed::ManagedSessionSnapshot {
                id: "session".to_owned(),
                status,
                usage: None,
            };
            assert_eq!(
                classify_recovered_run(&snapshot, &events)
                    .expect("state classifies")
                    .0,
                expected
            );
        }
    }

    #[test]
    fn natural_completion_requires_the_operation_private_user_message() {
        let history = br#"{"data":[{"id":"idle","type":"session.status_idle","processed_at":"2026-08-04T00:00:00Z","stop_reason":{"type":"end_turn"}}],"next_page":null}"#;
        let events = crate::managed::parse_history(history).expect("history parses");
        let snapshot = crate::managed::ManagedSessionSnapshot {
            id: "session".to_owned(),
            status: crate::managed::ManagedSessionStatus::Idle,
            usage: None,
        };
        assert_eq!(
            classify_recovered_run(&snapshot, &events)
                .expect("state classifies")
                .0,
            InterruptedRunState::Unknown
        );
    }
}
