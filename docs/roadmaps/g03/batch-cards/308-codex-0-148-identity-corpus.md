# 308 Codex 0.148.0 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../098-codex-0-148-useful-newer.md`
Depends on: Research 159; Research 160

## Goal

Freeze exact Codex host `0.147.0` and official npm `0.148.0` against
qualified `0.147.0`, and name the segment shape. Do not edit the production
claim in this card.

## Scope

1. Rank remaining Research 159 AllowUnverified registry-newer families;
   pick Codex CLI `0.148.0`.
2. Record npm identity, host CLI identity, generated schema, and selected
   exec/app-server surfaces through `0.148.0`.
3. Name card 309 as a compatible extension that reuses exec JSONL,
   app-server workspace-roots, lifecycle hard-delete, and thread-catalogue
   revisions.

## Out Of Scope

- editing Codex selection claims or public matrices
- mapping `fork`, `thread/fork`, or Bedrock
- Claude Agent or other 159 families
- provider prompts, live sessions, install, update, or publication

## Acceptance Criteria

- [x] exact `0.147.0` host and `0.148.0` official identity is recorded
- [x] selected exec flags and app-server methods are compared to the
      `0.147.0` corpus
- [x] the next card has an explicit compatible-extension decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt or live session
- stop if a new mapped public operation is required before the pin shape
  is named
- stop if `0.148.0` is no longer the official stable point

## Auto-Continuation

Continue to card 309 once the segment shape is named.

## Evidence

- Research 160
- `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.148.0/`
- `crates/swallowtail-adapter-codex/tests/fixtures/compatibility/codex-0-148-range.json`
- Segment decision: compatible-extension. Reuse `codex.exec.jsonl-v1`,
  `codex.app-server.v2.workspace-roots`, and
  `codex.app-server.lifecycle.v1.strict-descendant-hard-delete`. Raise
  latest qualified to `0.148.0`. Keep baseline and gaps. Synthetic later
  UnverifiedNewer is `0.148.1`. Card 309 owns the claim change.
