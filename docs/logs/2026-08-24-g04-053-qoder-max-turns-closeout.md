# 2026-08-24 g04.053 Qoder Maximum Turns Closeout

Status: complete; evidence stop and claim correction; merged
Owner: Tom
Milestone: g04.053
Cards: 148-150
Branch: `t3code/qoder-headless-max-turns`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-8e7b4cb9`
Base: restacked on `origin/main` after PR 52/53 (`a2ebf148` lineage)
PR: https://github.com/inflatable-cookie/swallowtail/pull/54
Merge: fast-forward to `main` at `aaf753b7` after five green hosted checks

## Result

Card 148 completed exact `@qoder-ai/qodercli@1.1.25` evidence and the
operator-directed claim correction. Research 200 admits no deliver-now
caller-decreasing row. Cards 149 and 150 stay blocked. Runtime argv still
emits historical inert `--max-turns 8`. No caller max-turns feature ships.
No install, login, credential inspection, provider prompt, or paid operation
was used.

## Claim Correction

Selected CLI headless QueryEngine factory (`entrypoint: "cli"`) hardcodes
AgentLoop `maxTurns: kN` (`1000`). CLI `--max-turns` is a raw string onto
Config `maxSessionTurns` (text-error formatter only).

Operator disposition: retain argv `8`; rewrite Research 151/200, command
comment, architecture, guide, matrices, fixtures, and tests to factory
ceiling `1000`; narrow `error_max_turns` / `limit.jsonl` to decoder-only;
rename `omit-max-turns-unbounded` to `omit-max-turns-not-route-argv`. Do not
remove the flag.

## Changed Surfaces

- Research 151 Authority; Research 200 (settled empty deliver-now)
- `command.rs` comment; guide; architecture; feature-matrix notes; triage
- fixtures/tests: historical-inert / factory-1000 / decoder-only fields;
  negative-case rename
- cards 148-150; milestone 053; this closeout

No production argv or public API change.

## Shared Closeout

- PR 54 was approved on exact head `aaf753b7` and fast-forwarded to `main`.
- A cancelled pinned-MSRV attempt was not accepted; the exact test passed
  locally and the hosted rerun completed all five checks green.
- The sole Next Task is remaining promoted per-route inventory reassessment.
- g04 remains open until explicit operator direction.

## Validation

- `effigy qa:docs:index:research|roadmaps:g04|roadmaps:batch-cards|logs`
- `effigy validate:focused swallowtail-adapter-qoder`
- `git diff --check`

## Next

Reassess the remaining promoted per-route inventory. Keep g04 open until
explicit operator direction.
