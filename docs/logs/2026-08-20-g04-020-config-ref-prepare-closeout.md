# g04.020 Config-Ref Prepare Handoff Closeout

Date: 2026-08-20
Milestone: g04.020
Cards: 056, 057, 058
Status: complete

## Result

The six realized addable routes now build their prepare inputs from an
`AdmittedInstanceRecord`:

- Anthropic Messages and DeepSeek select stored endpoint and API-key refs.
- Codex app-server and Claude Agent ACP select stored binary-path and
  environment refs.
- Ollama attach and llama.cpp attached select stored endpoint refs while
  keeping model selections and runtime identities explicit at prepare time.

`AdmittedInstanceRecord` has exact field lookup. Core and runtime provide
opaque retyping helpers for target, executable, and environment references.
The host still resolves all values. No path, URL, environment body, or secret
bytes enter portable records, diagnostics, or 047 snapshots. Admission still
does not prepare; Contract 037 remains the exact-target boundary.

Contract 057 now records this as a durable handoff rule. The six connection-
lifecycle examples, lifecycle proofs, route guides, and realized architecture
description use the same path.

## Validation

- `effigy qa:docs:index:logs` — passed.
- `effigy validate:focused swallowtail-core swallowtail-runtime
  swallowtail-host-local` — 349 tests passed.
- `effigy validate:focused swallowtail-adapter-anthropic
  swallowtail-adapter-deepseek swallowtail-adapter-codex
  swallowtail-adapter-claude-agent` — 395 tests passed.
- `effigy validate:focused swallowtail-adapter-ollama
  swallowtail-adapter-llama-cpp swallowtail-runtime swallowtail-testkit` —
  418 tests passed.
- `cargo test` connection-lifecycle targets for all six adapters — 46 tests
  passed.
- `effigy check:examples` — passed.
- `effigy package:api` — passed; only `public-api-unreleased` changed.
- `git diff --check` — passed.

The initial `effigy doctor` orientation remains blocked by pre-existing
repository findings: 341 god-file findings and one generated-in-src warning.
No unrelated cleanup was taken into this worker lane.

## Next

g04.021 Unmarked Overlay Rows is the next roadmap card. This worker lane is
ready for review; it does not merge its PR.
