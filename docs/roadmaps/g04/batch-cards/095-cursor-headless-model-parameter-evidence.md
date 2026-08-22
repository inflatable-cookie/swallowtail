# 095 Cursor Headless Model Parameter Evidence

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.035 Cursor Headless Model Parameters](../035-cursor-headless-model-parameters.md)
Depends on: Research 075, 077, 087, and 135

## Goal

Freeze exact secret-free evidence for Cursor headless `fast`, `context`, and
`effort`, then publish the smallest model/parameter/value allowlist that can be
implemented without a provider call.

## Method

1. Re-probe installed `cursor-agent --help` and record exact host identity;
   stop if the qualified host point moved.
2. Download the exact official macOS arm64 artifacts for
   `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, and
   `2026.08.11-e8db854` into `/tmp`; reuse the installed
   `2026.08.04-aaa8809` host for its exact specimen.
3. Record artifact URLs, archive and executable digests, and the bounded help
   excerpt that defines quoted bracket overrides.
4. Freeze current official Cursor CLI/subagent and TypeScript SDK parameter
   documentation as sibling evidence. Keep the SDK distinct from the selected
   subscription CLI route.
5. Build an exact table of model id, parameter id, value, source, qualified
   CLI versions, and disposition. A tuple is deliver-now only when official
   evidence names the value and model combination closely enough for
   fail-closed dispatch.
6. Record that `cursor-agent models` returns plain catalogue identities and no
   parameter descriptor. Do not invoke it against an authenticated account.
7. Freeze the corpus under
   `crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-headless-model-parameters-2026.07.01-2026.08.11/`.
8. Write and index promoted Research 183. Record the unchanged syntax segment,
   exact allowlist, evidence-gated tuples, and qualified-dispatch-only claim.

Do not send a prompt, authenticate, inspect account state, run the live model
catalogue, install, or update the host.

## Required Decisions

- The four exact qualified builds share one syntax segment unless the frozen
  specimens prove otherwise.
- Parameter availability remains model-dependent. A base catalogue id alone
  is not support evidence.
- Boolean or enum type shape does not qualify every value for every model.
- Unknown models, parameters, values, and combinations remain evidence-gated.
- Dispatch evidence does not prove provider acceptance or effective value.

## Acceptance Criteria

- exact artifact and documentation evidence is frozen without secrets
- the deliver-now allowlist is model-, parameter-, value-, and version-specific
- every non-qualified tuple has an explicit evidence-gated disposition
- Research 183 is promoted and indexed
- production code, claims, matrices, architecture, and changelog are unchanged
- `effigy validate:focused swallowtail-adapter-cursor` passes
- `effigy qa:northstar` passes
- `effigy qa:docs:index:research` passes
- `git diff --check` passes

Auto-continue to card 096 only when the allowlist contains a useful typed
subset and needs no live provider evidence.

## Stop Conditions

- official syntax differs across qualified builds
- exact artifact identity cannot be corroborated
- no useful tuple can be qualified without account or prompt evidence
- evidence would require treating SDK catalogue output as CLI catalogue truth
- implementation would need arbitrary string parameters

## Out Of Scope

- production binding or dispatch
- provider prompt, authentication, live catalogue, install, or update
- Cursor ACP, Cursor catalogue, sandbox, force, or ask mode
- Contract 029 ceiling changes
