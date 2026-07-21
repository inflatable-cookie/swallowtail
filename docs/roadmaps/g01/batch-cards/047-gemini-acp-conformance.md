# 047 Gemini ACP Conformance

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../014-gemini-acp-proof.md`

## Objective

Prove Gemini ACP lifecycle, callbacks, topology, redaction, and cleanup.

## Scope

- long-lived ACP profile
- local and remote-authoritative host identity
- optional installed/authenticated probe when available

## Out Of Scope

- required local Gemini installation
- second ACP agent

## Acceptance Criteria

- [x] shared profile passes without weakened assertions
- [x] optional live probe is separately gated
- [x] full QA passes

## Validation

- focused conformance tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- current implementation violates promoted ACP lifecycle rules

## Auto-Continuation

No. Close roadmap 014 and confirm the attached-runtime lane.

## Evidence

- a seventh provider-neutral conformance profile covers long-lived ACP process,
  session, active-prompt cancellation, working-resource callback, redaction,
  and local/remote-authoritative topology boundaries
- deterministic Gemini driver fixtures cover successful read callbacks,
  permission observe-and-stop, native prompt cancellation, disconnect, event
  closure, and joined resource/process/task cleanup
- prompt correlation is established before the joined waiter starts, preventing
  cancellation from overtaking the provider request
- the installed Gemini `0.51.0` version probe is ignored unless
  `SWALLOWTAIL_LIVE_GEMINI_ACP=1` is set explicitly
- `effigy qa` passes with 190 tests; the Gemini and OpenCode installed probes
  remain ignored by default
- `effigy doctor` remains at the pre-existing 19 findings: 12 warnings and 7
  errors; this batch added no oversized-file debt
- the common ACP dependency tree contains only `serde_json`; provider-neutral
  protocol, core, runtime, and local-host source contain no Gemini identity
