# 2026-08-01 Codex Exec Queryless Navigation Lifecycle

## Result

Codex exec no longer aborts a valid structured run when Codex CLI `0.146.0`
reports a queryless completed `web_search` item whose exact action type is
`other`.

The adapter treats that shape as lifecycle-only external-search activity. It
retains the provider item and runtime activity identity, emits start and
completion, and attaches no display content.

## Exact Accepted Shape

```json
{
  "type": "item.completed",
  "item": {
    "id": "exec-36b9591e-3ba1-4a9d-b2fc-301ad691e212",
    "type": "web_search",
    "query": "",
    "action": {
      "type": "other"
    }
  }
}
```

The new rule applies only when the phase is `Completed`, display query content
is absent, and `action.type` is exactly `other`. A started queryless item
retains its existing valid content-free behavior. Any query-bearing search
retains its query as provider-tool display content.

## Fail-Closed Boundary

A completed `action.type == "search"` item without a non-empty top-level or
action query remains malformed. Missing item identity, non-text query fields,
invalid lifecycle, and other malformed JSONL remain rejected through
`swallowtail.codex.exec.malformed_jsonl`.

The structured-output parser, JSON detection, final-output retention,
terminal completion, diagnostic code, and Soundcheck schema validation were
not changed. The whole-stream fixture proves the lifecycle-only event is
followed by a valid JSON `agent_message` and `turn.completed`; both now reach
the consumer.

## Validation

- fixture-first targeted regression failed before the rule change
- query-bearing, deferred-query, lifecycle-only, and malformed actual-search
  tests pass
- focused Codex validation: 146 passed
- extracted Codex package compiled
- no live provider, authentication, consumer, installation, or publication
  effect ran

## Next

Soundcheck should rerun:

```sh
cargo test -p soundcheck-app codex_cli_returns_a_valid_structured_proposal --lib -- --ignored --nocapture
```

The rerun keeps Luna and medium reasoning unchanged. If it passes, Swallowtail
returns to the g03 evidence gate.
