# 129 Negotiated Session Option Records And Kimi Range Corpus

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../043-kimi-code-capability-range.md`

## Objective

Realize Contract 034 and freeze both Kimi behavior milestones before production
dispatch changes.

## Scope

- provider-neutral negotiated reasoning setup records and assertions
- exact request, plan, capability, lifecycle, and effective-value agreement
- no generic provider option id or value in the public API
- Kimi Code `0.28.1` annotated tag and source commit
- Kimi Code `0.29.0` annotated tag and source commit
- ACP adapter `0.3.4` and `0.3.5`
- exact locked ACP SDK `0.23.0` and wire version 1
- legacy `off`/`on`, declared effort levels, always-thinking, missing option,
  ambiguity, unsupported value, provider rejection, missing confirmation, and
  effective-value drift fixtures
- exact singleton compatibility segments, lower rejection, prerelease,
  malformed, and unverified-newer fixtures
- preserve existing Kimi new, load, resume, replay, write, cancellation, and
  cleanup corpora

## Acceptance Criteria

- [x] portable reasoning selection has no provider configuration bag
- [x] both exact releases have frozen authoritative evidence
- [x] every option-shape difference has a private behavior revision
- [x] no semantic interval is inferred between published points
- [x] unsupported values never fall back to `on`, default, another model, or
      another route
- [x] production dispatch work is exact enough to compile

## Validation

- focused core, runtime, ACP, testkit, and Kimi corpus tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Auto-Continuation

Yes, after the two behavior revisions and every rejection path are frozen.

## Outcome

Runtime now exposes typed negotiated-reasoning setup and effective-confirmation
records. Setup requires exact interactive-harness, request, capability,
constraint, and new-session agreement. Load, resume, ambiguous plans, missing
selection, and effective drift reject through stable provider-neutral
diagnostics.

The ACP corpus freezes exact `0.28.1` legacy-select and `0.29.0`
declared-effort behavior revisions. Annotated tags, peeled commits, adapter
packages, locked SDK and wire versions, source digests, option shapes, exact
singleton segments, rejection points, unverified-newer behavior, and
confirmation failures are explicit.

Provider option ids and values remain in Kimi fixture evidence. The runtime and
testkit public surfaces carry only typed `ReasoningMode`.

## Evidence

- focused core, runtime, testkit, and ACP tests passed
- workspace all-target check passed
- workspace warnings-denied clippy passed
- `git diff --check` passed
