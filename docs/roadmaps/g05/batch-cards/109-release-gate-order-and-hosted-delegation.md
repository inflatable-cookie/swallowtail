# 109 Release Gate Order And Hosted Delegation

Status: planned; after the `v0.4.3` tag
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../034-release-lane-simplification.md`
Depends on: the g05.034 measured causes table

## Goal

Make one prepare attempt cheap: run the deterministic fast gates first, and let the exact-SHA hosted run stand as the evidence for clippy and the workspace tests instead of repeating them locally.

## Scope

1. Reorder `[release.gates]` in `config/release.toml`: `fmt`, `qa`, `docs`, `metadata`, `api`, `security`, `floor`, `source` first (all under a minute each), then `lint`, `lint:no-features`, `test`. A docs failure now costs seconds, not thirty minutes.
2. Hosted delegation: amend Contract 036 so `lint`, `lint:no-features`, and `test` are satisfied for a candidate by a green hosted run at the exact candidate SHA (the run the candidate PR already needs). The local prepare either runs them or records the run id; the release note names the run. If Effigy cannot skip a configured gate, route the request to the Effigy Chatterbox and in the meantime split the heavy gates into a separate `[release.gates]` profile invoked only when no hosted run exists.
3. `docs/guides/release-playbook.md`: one page, the lane in order with expected wall clock per step, who acts, the tag request template, and the rule that the tag request goes to the operator the moment gates are green (consumer smoke on the tag afterwards).
4. `scripts/README.md` and Contract 036 reference the playbook.

## Acceptance Criteria

- [ ] cheap gates run first; a docs failure fails in under a minute
- [ ] Contract 036 delegation clause promoted with Chatterbox co-sign
- [ ] a dry prepare on a throwaway branch proves the order and the delegation
- [ ] release playbook published and linked

## Validation

- one dry `effigy --json release prepare --check-gates` on a throwaway branch (no state written to `main`)
- `effigy qa:docs`; `git diff --check`

## Review Oracle

See the g05.034 manifest row.

## Result

PR 253. Chatterbox co-signed the Contract 036 clause on two bounded
conditions, then required this event qualifier: `floor` joins the
hosted-delegated set; evidence is a green hosted `CI` run triggered by
`workflow_dispatch` (or a push to `main`) at the SHA to tag or at a commit
with an identical tree; pull-request runs do not qualify because the MSRV
floor skips its tests there. Stopped for final delta review. No merge.

## Auto-Continuation

No. Stop for exact-head review.
