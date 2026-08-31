# Papercuts wave 23 OpenAI test-target closeout

Date: 2026-08-31
Handoff: `docs/handoffs/20260831-153924-papercuts-wave23-openai-test-targets.md`
PR: [#136](https://github.com/inflatable-cookie/swallowtail/pull/136), merged
as `af339fb65d5a249bddcf0f58abae95953d4d465a`

## Outcome

- Renamed the three explicit `swallowtail-adapter-openai` Cargo test targets
  to `catalogue_suite`, `direct_suite`, and `realtime_suite` so each matches
  its suite-root filename.
- Closed the matching `PAPERCUTS.md` entry without changing source, module
  structure, test bodies, features, dependencies, or public API.
- Accepted exact worker head `274bd2ac58ed5ca856a44300bb7edaf5b85467e4`.

## Validation

- Cargo metadata maps each new target one-to-one to its same-named suite root.
- All three stale target names fail resolution.
- Suite counts remain 7 catalogue, 35 direct, and 33 Realtime tests.
- Focused package validation passed 96 tests; all five PR checks passed.
- Old OpenAI target references outside historical handoffs, logs, and
  `PAPERCUTS.md` are confined to immutable retired `release-candidates/0.1.0`
  evidence.

## Scope and next

- The g05 roadmap Next Task remains card 024.
- Route-matrix Python bytecode cleanup is the next serial papercut lane.
- No provider, route behavior, workflow, or release evidence changed.
