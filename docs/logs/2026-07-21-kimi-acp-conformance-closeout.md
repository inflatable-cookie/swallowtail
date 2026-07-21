# 2026-07-21 Kimi ACP Conformance Closeout

## Changed

- added a ninth provider-neutral conformance profile for persistent ACP
  lifecycle extensions
- kept the existing long-lived ACP profile unchanged for Gemini's smaller
  new/read/cancel subset
- proved one shared ACP decoder against pinned Gemini `0.51.0` and Kimi Code
  `0.28.1` corpora
- ran Kimi new, prompt, write, load, replay, resume, cancellation, disconnect,
  redaction, and cleanup fixtures under local and remote-authoritative host ids
- recorded process join before resource release and credential release
- added an ignored Kimi installed probe gated by
  `SWALLOWTAIL_LIVE_KIMI_ACP=1`

## Current State

Roadmap 018 is complete. Kimi is the second production ACP agent, but it does
not redefine the ACP baseline. Its driver composes the common long-lived ACP
shape with a distinct persistent-session extension covering bounded replay,
replay-free resume, read-write callback authority, delegated harness auth, and
explicit ambient execution. Gemini does not inherit those claims.

Callback authority remains separate from provider permission approval and
process containment. The failed optional App Sandbox proof remains negative
capability evidence; it does not block the ambient route.

## Validation

- focused ACP protocol, Kimi adapter, and testkit suites pass
- the Kimi installed probe is present but ignored by default
- full repository QA passes with 271 tests; all three installed/live probes are
  ignored by default
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors
- `git diff --check` passes

## Continuation

Roadmap 020 is active. Card 067 rechecks SDK-native embedding first because it
is the first unfinished choice in Research 005's ranked sequence. If no real
maintained Rust embedding or supported language boundary exists, the card will
select the next hosted catalogue, protocol, harness, direct-inference, or
attached-runtime route by new contract pressure. Cards 068-069 remain in bounds
only after that selection.
