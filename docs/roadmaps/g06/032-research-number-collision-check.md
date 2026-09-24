# g06.032 Research Number Collision Check

Owner: Tom
Created: 2026-09-24
Depends on: `scripts/check-roadmap-number-collision.py`; PAPERCUTS entries of 2026-09-24
Vision tags: docs QA, parallel lanes

## Outcome

`effigy qa:docs` and the pre-push hook refuse a research record whose number
collides with a different record on the local tree or on remote `main`.

## Why It Matters

Research numbers collided three times on 2026-09-24 (331, 342, 348) while
parallel lanes ran. Reviewers caught some; one landed and needed a
post-merge renumber. The roadmap task-number check already prevents the same
failure for task files.

## Ready-State Rubric

- [x] The roadmap check and its test suite exist to extend.
- [x] The collision cases are known from today's lanes.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- A collision is two `docs/research/NNN-*` files sharing `NNN` with different
  slugs. Same-slug companions (`.md` plus `.tsv`, `.csv`) are one record.
- Check the working tree and remote `main`, matching the roadmap check's
  transport and stale-ref behaviour.
- Grandfather the existing `328` pair by an explicit allowlist entry; do not
  renumber history.
- Wire it into `qa:docs` and the pre-push hook beside the roadmap check.

## Dispatch manifest

- **State:** ready; tooling lane; independent of g06.031.
- **Completion:** the check and its tests pass and fail on the known cases;
  independent exact-head review accepts the head.
- **Owned mutable paths:** `scripts/check-roadmap-number-collision.py` or a
  sibling research check; `scripts/tests/` for its cases; `effigy.toml` task
  wiring; `scripts/git-hooks/pre-push`; `PAPERCUTS.md` (remove the two
  resolved research-number entries); `CHANGELOG.md` `[Unreleased]` if tooling
  changes are logged there.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; renumbering any existing record; route
  code; contracts; release, tag, publication.
- **Worker evidence:** the authenticated run result.
- **Escalation:** operator via Chatterbox for scope; Queue coordinator for
  mechanical blockers.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Catches cross-lane collisions | A new record reuses a number already on remote `main` | test case fails the check |
| Companions pass | `NNN-x.md` plus `NNN-x.tsv` is flagged | test case passes |
| History tolerated | The `328` pair fails `qa:docs` | allowlist case passes |

## Stop conditions

Stop and ask if the check needs network access the roadmap check does not
already use.

## Evidence

`effigy qa:docs`, the new test script, and `git diff --check`.

## Next Task

None beyond closeout.
