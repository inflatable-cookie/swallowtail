# 2026-07-20 Gemini ACP Conformance

## Outcome

Roadmap 014 and cards 045-047 are complete. The pinned Gemini CLI `0.51.0`
adapter now passes provider-neutral and production-driver ACP conformance
without a required binary or credential.

## Coverage

- the seventh provider-neutral profile retains every common Contract 011
  assertion and adds owned ACP process, session, prompt cancellation,
  working-resource callback, and host-topology boundaries
- production fixtures cover success, ordered output, bounded reads, permission
  observe-and-stop, native cancellation, disconnect, event closure, redaction,
  and joined resource/process/task cleanup
- local and remote-authoritative host identities run through the same driver
  seam
- filesystem callbacks remain execution-host services and never appear as
  consumer tool callbacks
- unknown extension requests get method-not-found; extension notifications are
  ignored; unknown stable callbacks fail the scope

## Lifecycle Repair

Conformance exposed two races. Terminal turns did not close their event stream,
and immediate cancellation could reach the agent before the prompt task wrote
its request. Terminal completion now marks the event channel terminal. Prompt
registration and write now finish before the joined response waiter is
spawned.

## Validation

- `effigy qa` passes with 190 tests
- Gemini and OpenCode installed probes are ignored by default
- `effigy doctor` retains the pre-existing 19 findings: 12 warnings and 7
  errors
- new Gemini files were split until doctor returned to that baseline
- `git diff --check` passes
- the common ACP crate depends only on `serde_json` and contains no Gemini
  identity branch

## Residual Risk

- no authenticated Gemini session ran in this batch; the live installed
  version probe requires `SWALLOWTAIL_LIVE_GEMINI_ACP=1`
- the configured instance and host remain responsible for supplying an
  isolated Gemini state root whose policy disables ambient search, extensions,
  MCP, and mode widening
- interoperability with a second ACP agent is unproven and should be added only
  if it tests portable behavior or extension isolation not already covered

## Continuation

Roadmap 015 is active. Card 048 is ready to freeze an attached llama.cpp
deployment and protocol facade without adding model management or owned server
lifecycle.
