# 004 Claude And Gemini ACP Range Corpus

Status: ready
Owner: Tom
Created: 2026-07-31
Milestone: `../002-claude-and-gemini-acp-range-maintenance.md`
Depends on: card 003

## Goal

Freeze exact source and behavior evidence for Claude Agent `0.62.0..=0.64.0`
and Gemini CLI `0.53.0` before changing production compatibility claims.

## Scope

1. Add exact npm publication, integrity, tag-commit, package-dependency, and
   selected-source records for every candidate point.
2. Freeze Claude `0.62.0` unchanged behavior, `0.63.0` tool/subagent
   correlation, and `0.64.0` steering/form milestones.
3. Freeze Gemini's independent ACP and headless selected-source equality at
   `0.53.0`.
4. Cover each exact stable point, prerelease rejection, and one later-stable
   synthetic unverified classification.
5. Confirm stable ACP v1, schema `v1.20.0`, and package axes remain unchanged.

## Acceptance Criteria

- [ ] every candidate release has exact publication and source identity
- [ ] Claude behavior groups are explicit and route-relevant
- [ ] Gemini ACP and headless evidence stay on separate version axes
- [ ] form elicitation remains an unstable capability-gated Claude subset
- [ ] no raw provider payload, secret, path, or consumer policy enters fixtures
- [ ] no production compatibility claim changes in this card
- [ ] cards 005-006 can implement without fresh source decisions

## Validation

- focused corpus and compatibility tests for
  `swallowtail-protocol-acp`, `swallowtail-adapter-claude-agent`, and
  `swallowtail-adapter-gemini`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no broad workspace suite

## Stop Conditions

- Stop if any exact stable point lacks an authoritative artifact or tag.
- Stop if selected source requires a new shared contract.
- Do not install, authenticate, invoke a model, or change a support claim.

## Auto-Continuation

Yes. Continue to card 005 only after the corpus names every exact behavior
group and focused validation passes.
