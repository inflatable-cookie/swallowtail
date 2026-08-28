# 2026-08-28 g04.089c Kiro ACP Effort Evidence

Status: complete
Card: 254
Research: 251
Branch: `t3code/review-effort-evidence-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-a893e130`
PR: pending

## Boundary

Evidence only. This lane owns card 254, Research 251, this log, and optional
new Kiro-local frozen evidence. Shared planning and production stay unchanged.

## Target

Close exact model/value membership, ACP application, confirmation, failure,
lifecycle, cleanup, and omission truth.

## Result

Honest empty deliver-now set. No Kiro ACP effort row ships from this lane.

Official ACP page documents `kiro-cli acp` / optional `--agent` and lists
`session/set_model` plus `_kiro.dev/commands/*`, with zero `--effort`
occurrences. Official `--effort low|medium|high|xhigh|max` is under
`kiro-cli chat`. `/effort` membership depends on the active model and
persists to provider-owned settings. Production argv remains `["acp"]`.

Exact `2.18.1` platform archives returned HTTP 403 on ranged GET; current
stable manifest tip is `2.20.1`. Package parser bytes were not frozen.
Chat flags, unsupported `session/set_model`, and `_kiro.dev/*` were not
promoted onto ACP.

## Changed Surfaces

- `docs/research/251-kiro-acp-effort-evidence.md` — promoted empty set
- `docs/roadmaps/g04/batch-cards/254-kiro-acp-effort-evidence.md` — complete
- `crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.18.1-effort-evidence/`
  — sources + disposition

Unchanged: adapter production code, guide, identity corpus, shared milestone,
inventory, programme, triage, matrices, indexes, Next Task.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-kiro` (30 tests)
- `effigy qa:northstar`
- `git diff --check`

Inherited doctor baseline: `scan.god-files` 380 findings (334 warnings, 46
errors); generated-in-src warning; graph index stale.

## Unresolved

Reopen only with recoverable qualified-version package/source parser, closed
model/value membership without account inference, and confirmable ACP
accept/return/omission seams. No production binding from this PR. Merge not
authorised.
