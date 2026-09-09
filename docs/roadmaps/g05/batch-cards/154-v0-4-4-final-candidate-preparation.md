# 154 v0.4.4 Final Candidate Preparation

Status: ready; operator authorized the consolidated final-candidate qualification batch on 2026-09-09
Owner: release-preparation worker
Created: 2026-09-09
Milestone: `../036-v0-4-4-release-readiness.md`
Depends on: Card 153 complete; clean canonical `main`; no active Swallowtail worker

## Goal

Prepare one fresh `v0.4.4` candidate tree carrying every accepted producer fix,
obtain exact-tree review and hosted CI, merge it, and return the immutable merge
SHA for the final linked Desktop acceptance gate. Do not tag.

## Scope

1. Start from clean pushed `main`. Record the base commit and tree. Keep the
   release freeze in force until the operator's tag decision.
2. Reconcile `docs/releases/0.4.4.md`, its index entry, changelog, root README,
   lockfile, and all three `0.4.4` baseline families to the current tree. Never
   alter a `0.4.3` or earlier baseline.
3. Run read-only release status first. Require inferred version `0.4.4` and a
   patch semantic-API class. Stop on another result.
4. Run exactly one new authorized successful preparation transaction:
   `effigy --json release prepare --yes --check-gates --version 0.4.4`.
   A real failure consumes the attempt and returns to Chatterbox; never retry.
5. Open one candidate PR. Run exact-head independent review and the qualifying
   hosted `CI` `workflow_dispatch` in parallel. Merge only when both accept.
   If the merge tree differs from the green reviewed head, run qualifying CI
   at the merge SHA before completion.
6. Return the accepted head, merge SHA, both tree IDs, qualifying run ID,
   preparation receipt digest, and elapsed wall clock. The merge SHA becomes
   the only source identity permitted for the dependent Desktop gate.

Owned mutable paths: root version/release surfaces; `Cargo.toml`, `Cargo.lock`,
workspace crate manifests only for the coherent `0.4.4` version transaction;
`README.md`; `CHANGELOG.md`; `docs/releases/0.4.4.md` and release index;
`release-baselines/public-api-0.4.4/**`;
`release-baselines/production-routes-0.4.4.txt`;
`release-baselines/internal-dependencies-0.4.4.tsv`; this card's `## Result`;
`PAPERCUTS.md` append only. Queue closeout owns shared roadmap/index/handoff
status surfaces.

Forbidden: Rust source or tests; provider execution; Desktop edits; dependency
range/currentness work; earlier baselines; tag creation or push; GitHub Release;
crates.io publication; binaries, installers, model artifacts, or consumer pins.

## Acceptance Criteria

- [ ] one coherent candidate tree contains every current accepted producer change
- [ ] read-only status infers patch `0.4.4`
- [ ] exactly one new prepare transaction runs and all cheap gates pass
- [ ] exact-head independent review accepts the candidate
- [ ] qualifying hosted CI passes at the merge SHA or an identical tree
- [ ] the exact merge SHA/tree and run ID are returned without a tag

## Validation

- read-only `effigy --json release status --version 0.4.4`
- exactly one `effigy --json release prepare --yes --check-gates --version 0.4.4`
- `git diff --check`
- exact-head independent review
- qualifying hosted `CI` `workflow_dispatch`

## Review Oracle

Invariant: one immutable tree supports the release surfaces, baselines, review,
hosted CI, and downstream Desktop source link. Smallest counterexample: an
earlier baseline changes, a source file changes outside the version transaction,
the hosted run proves another tree, or a tag is created.

## Stop Conditions

- the working tree or index is not clean at start;
- release status is not patch `0.4.4`;
- the prepare transaction fails;
- another Swallowtail change lands after the candidate tree freezes; or
- qualifying review/CI cannot bind to the merge tree.

Return the exact evidence and preserve all work. Do not retry preparation and
do not tag.

## Auto-Continuation

No worker continuation. Queue closeout returns the exact candidate merge SHA.
The already-authorized dependent Desktop exact-tree acceptance task then runs;
Chatterbox presents the tag request only after that task passes.

## Result

Pending.
