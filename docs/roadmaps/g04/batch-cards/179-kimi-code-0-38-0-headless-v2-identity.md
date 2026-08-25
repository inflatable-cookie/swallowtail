# 179 Kimi Code 0.38.0 Headless V2 Identity

Status: ready
Owner: Tom
Milestone: [g04.064 Kimi Code 0.38.0 Headless V2 Useful Newer](../064-kimi-code-0-38-0-headless-v2-useful-newer.md)
Created: 2026-08-25

## Task

Freeze exact official `@moonshot-ai/kimi-code@0.38.0` agent-core-v2
headless identity and stream-json behavior. Name the segment shape. Do not edit
the production claim in this card.

## Method

1. Reconfirm npm `0.38.0`, the official tag, commit
   `0999454bdcb5ddd98f39bffee434dcf0a810f394`, package integrity, and the
   selected source hashes already recorded by Research 210.
2. Extract the official package in a disposable `/tmp` directory. Do not
   install it or alter the host executable.
3. Trace default `runPrompt` → `runV2Print` routing and freeze every selected
   agent-core-v2 renderer, writer, event, terminal, error, retry, activity,
   cancellation, resume-hint, and exit-status source blob needed by the route.
4. Compare the exact v2 JSONL grammar and stderr contract with the current
   `headless_events`, `headless_activity`, `headless_pump`, terminal, and
   prepared-headless assumptions.
5. Add a secret-free `0.38.0` v2 identity/protocol corpus and decoder specimens
   for every deterministically available success, tool, retry, failure,
   interruption, malformed, unknown, and incomplete boundary.
6. Record which semantics are source-proved, fixture-proved, withheld, or need
   a live provider prompt. Do not synthesize provider output as official
   evidence.
7. Promote Research 211 and name one decision: compatible extension,
   adapter-private v2 milestone, new public driver/facade revision, or
   incompatible stop.

No provider prompt, live session, login, credential use, install, or host
mutation.

## Expected Shape

Adapter-private milestone if exact v2 output remains representable by the
existing public structured-run lifecycle but needs a separate decoder/activity
mapping. Preserve v1 through `0.37.2`; use exact `0.38.0` as the first v2
point. A public lifecycle change or unprovable terminal/retry contract is a
stop, not permission to flatten v2 onto v1.

## Acceptance

- exact official identity and selected-source hashes are corroborated
- secret-free v2 corpus covers every deterministically provable selected shape
- Research 211 is promoted with selected protocol, remaining rank, and decision
- production compatibility claim remains unchanged
- card auto-continues to 180 only for an admitted adapter-private claim shape

## Stop Conditions

- official latest or exact artifact identity moves during the run
- actual v2 mapping needs a new public operation, driver, facade, or shared
  contract
- terminal, retry, activity, parser, interruption, or exit truth cannot be
  qualified without a provider prompt or authenticated session
- source evidence contradicts the current structured-run lifecycle

## Out Of Scope

- forcing `KIMI_CODE_LEGACY_FLAG`
- reasoning effort or other route-local feature work
- ACP, local-server, or Platform Chat changes
- production claim edits before Research 211's decision
