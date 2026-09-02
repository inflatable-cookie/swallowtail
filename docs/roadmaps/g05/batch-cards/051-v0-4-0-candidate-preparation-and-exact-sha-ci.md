# 051 v0.4.0 Candidate Preparation And Exact-SHA CI

Status: planned; depends on accepted card 050 audit
Owner: Tom
Created: 2026-09-02
Milestone: `../021-v0-4-0-release-readiness.md`
Depends on: completed card 050 with accepted exact-head audit and freeze census

## Goal

Prepare, review, and freeze one coordinated `v0.4.0` source candidate, pass all
11 local gates, land it on canonical `main`, and require CI at that exact SHA.

## Scope

1. Start from the accepted card 050 head in a clean candidate worktree. Fetch
   canonical GitHub and prove exact local base, `origin/main`, tag absence, and
   no open mergeable feature/currentness PR inside the freeze.
2. Run the Effigy preparation plan, then set all 40 package versions and every
   normal/build internal compatible requirement to coordinated `0.4.0`.
   Preserve `publish = false`, Rust `1.95.0`, Apple Silicon macOS support, and
   the source-only distribution boundary.
3. Create distinct `v0.4.0` package, dependency, 48-route, and semantic API
   baselines from the accepted card 050 census. Never edit or regenerate a
   prior release baseline.
4. Promote the complete current `[Unreleased]` changelog into `0.4.0` and write
   release notes that match the audited source. Name every classified break,
   including the removed OpenAI Background `minimal` value, corrected opaque
   facade point, fail-before-effects behavior, consumer impact, coordinated
   upgrade, and exact rollback to immutable `v0.3.3`.
5. Record the 40-package source inventory, 48 production routes, release package
   order, dependency graph, API baseline, target/floor, source contents, and the
   required known limits from the milestone.
6. Prove the source tree contains no build output, generated cache, local
   release bundle, host absolute path, credential, auth state, private endpoint,
   mutable provider payload, or unreviewed live capture. Remove only
   candidate-owned generated local state before the clean-tree proof.
7. Run one final `effigy release prepare --yes --check-gates --version 0.4.0`
   on the complete candidate. Require all configured gates together: `fmt`,
   `lint`, `lint:no-features`, `test`, `qa`, `docs`, `metadata`, `api`,
   `security`, `floor`, and `source`. The earlier 2,825-test exploratory run and
   any partial rerun are not evidence.
8. Open one candidate PR, receive exact-head review, and land only the accepted
   head on canonical `main`. Record the resulting candidate SHA; no later
   closeout commit may be presented as that candidate.
9. Dispatch canonical `CI` against `main`. Select the `workflow_dispatch` run
   whose `headSha` equals the candidate SHA and require every configured job to
   pass. Reconfirm local `HEAD`, `origin/main`, and CI head identity.
10. Leave the candidate clean and immutable for card 052. Do not create a tag.

## Out Of Scope

Tag creation or push, release execution, crates.io, GitHub Release, binary or
sidecar publication, installer, provider call, live probe, consumer-repo
mutation, working-application smoke, feature/currentness work, claims beyond
the accepted audit, CI workflow edits, or papercut repair.

## Acceptance Criteria

- all 40 packages and internal requirements use coordinated `0.4.0`
- new `v0.4.0` baselines match the accepted audit; every older baseline is
  byte-for-byte unchanged
- changelog, release notes, source inventory, upgrade, rollback, route count,
  known limits, and actual source agree
- candidate worktree is clean and free of forbidden source contents
- all 11 local gates pass on one complete tree after the final synchronized
  candidate changes
- exact-head review accepts the candidate before canonical landing
- canonical CI passes at the exact candidate SHA and not a merely recent SHA
- no local or remote `v0.4.0` tag is created

## Validation

- `effigy release prepare --plan --version 0.4.0`
- `effigy release prepare --yes --check-gates --version 0.4.0`
- exact-SHA `CI` workflow query and watch from the Effigy release protocol
- `git status --porcelain`
- local `HEAD` / `origin/main` / workflow `headSha` equality
- local and remote absence of `v0.4.0`

## Review Oracle

Invariant: the reviewed candidate tree is the tree that passed every local gate
and canonical CI, and all release copy describes that exact tree.

Smallest counterexample: one dependency still requires `^0.3.3`, one prior
baseline changes, a gate passes before the last edit, CI points at another
commit, or release notes omit one audit-ledger break.

Required proof: metadata/dependency diff, old-baseline checksums, new baseline
generation record, complete source inventory, 11 named gate results, accepted
PR head, canonical candidate SHA, exact-SHA CI run, clean-tree output, and tag-
absence proof.

## Auto-Continuation

No. Card 052 also requires an explicit operator selection of the working
application and its exact smoke/rebuild authority.

## Stop Conditions

Stop on candidate drift, any failing or skipped gate, a modified historical
baseline, source contamination, overlapping mergeable feature/currentness PR,
review mismatch, non-canonical base/remote, CI from another SHA, tag presence,
or a required workflow edit.
