# 244 Cursor Agent 2026.08 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../078-cursor-agent-2026-08-exact-milestones.md`
Depends on: Research 127; Research 135

## Goal

Freeze host Cursor `2026.08.04-aaa8809` and ACP-registry
`2026.08.11-e8db854` against the qualified July milestones, and name the
segment shape. Do not edit the production claim in this card.

## Scope

1. Rank remaining Research 127 AllowUnverified host-drift families; pick
   Cursor Agent.
2. Record host and registry identity, selected help flags, and prompt-free
   ACP initialize.
3. Name card 245 as exact milestones reusing the three existing route
   behaviors.

## Out Of Scope

- editing Cursor selection claims, discovery parser, or public matrices
- OpenCode, Gemini, or other 127 families
- provider prompts, install, update, or publication

## Acceptance Criteria

- [x] exact host and official package identities are recorded
- [x] selected catalogue, ACP, and headless flags are compared to the
      July command
- [x] the next card has an explicit exact-milestones decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-cursor`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a provider prompt
- stop if a new public operation is required before the pin shape is named

## Auto-Continuation

Continue to card 245 once the segment shape is named.

## Evidence

- Research 135
- `crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-2026.08.04-2026.08.11/`
- Identity decision: exact-milestones. Reuse the three route behaviors.
  Keep July points. Add `2026.08.04-aaa8809` and `2026.08.11-e8db854`.
  Do not infer the gap. Card 245 owns the claim change.
