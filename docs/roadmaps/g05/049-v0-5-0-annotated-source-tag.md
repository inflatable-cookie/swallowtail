# g05.049 v0.5.0 Annotated Source Tag

Status: completed; annotated tag `v0.5.0` created and pushed at exact candidate `582d01d6` (tag object `c772c583`); local and remote agree; tag-triggered CI run 34467974791 green 11/11; documentation closeout PR pending review
Owner: Tom
Created: 2026-09-10
Depends on: g05.047; g05.048; Contract 036; release playbook
Vision tags: source release, annotated tag, compatibility

## Outcome

Create one immutable annotated source tag `v0.5.0` at exact candidate commit
`582d01d6b6890eed5195a1fcbee0ae985304a6c7`, push only that tag ref to
canonical `origin`, verify local/remote object, peel, and message agreement,
wait for the tag-triggered CI result, then publish the reviewed documentation
closeout. Do not publish any other release object or artifact.

## Operator Authorization

Tom explicitly authorized both local annotated-tag creation and tag push on
2026-09-10 for:

- source commit: `582d01d6b6890eed5195a1fcbee0ae985304a6c7`
- source tree: `eebb6978be365f79ae41436912240886d0aecf78`
- canonical branch: `main`
- remote: `git@github.com:inflatable-cookie/swallowtail.git`
- tag: `v0.5.0`
- qualifying hosted CI: push run `34464717829`, all 11 jobs green at the
  exact source commit
- no crates.io publication, GitHub Release, binary, sidecar, installer, model
  artifact, provider call, or consumer mutation

Tom separately approved these exact annotation bytes:

```text
Swallowtail v0.5.0

Registered-only Claude SDK sessions, corrected native mediation truth, and Pi RPC 0.85.1 qualification. Canonical hosted CI run 34464717829 passed against 582d01d6b6890eed5195a1fcbee0ae985304a6c7.

The source-only release contains no crates.io publication, GitHub Release object, binary, sidecar, installer, or model artifact.
```

No other release or mutation authority follows.

## Ready-State Rubric

- [x] g05.047 prepared version `0.5.0` exactly once; preparation receipt
      SHA-256 is
      `15f506d72b915a1679a8cea27d4e23578abaf439aefd7f9f778b48592ef376a1`.
- [x] g05.048 repaired the recurring pinned-MSRV deadline proof without
      changing production, workflow, version, release, or historical evidence
      surfaces.
- [x] PR #311 merged as exact candidate
      `582d01d6b6890eed5195a1fcbee0ae985304a6c7`, tree
      `eebb6978be365f79ae41436912240886d0aecf78`.
- [x] Independent exact-head review `5616849978` accepted the identical tree.
- [x] Exact-SHA push run 34464717829 completed success with all 11 jobs green,
      including Pinned MSRV floor and Pinned MSRV floor tests.
- [x] Candidate workspace version is `0.5.0`, `publish = false`, MSRV `1.95`.
- [x] The candidate is an ancestor of canonical `main`; later documentation
      closeout commit `7cff1e487a9513e070f0da34906ad6b9677c7827` is not the tag target.
- [x] Local and remote `v0.5.0` were absent at planning time; `v0.4.4` remains
      immutable at `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`.
- [x] Tom authorized both tag mutations and the exact annotation on
      2026-09-10.

## Dispatch Manifest

- **State:** ready; dispatch after this planning commit reaches pushed `main`.
- **Completion:** exact read-only preflight; one local annotated tag at the
  named candidate; one exact tag-ref push; verified local/remote tag object,
  peel, and annotation; recorded tag-triggered CI; reconciled Contract 036,
  release note/index, task/log/index, and roadmap front doors through one
  independently reviewed documentation PR.
- **Owned mutable paths:** local `refs/tags/v0.5.0`; remote
  `refs/tags/v0.5.0`; this task;
  `docs/contracts/036-crate-release-and-compatibility-boundary.md`;
  `docs/releases/0.5.0.md`; `docs/releases/README.md`; one new tag log and
  `docs/logs/README.md`; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`,
  `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`.
- **Worker:** source-tag release worker; no provider access.
- **Feature freeze:** no unrelated source merge before the tag mutation. Later
  documentation-only commits do not replace the authorized candidate.
- **Excluded:** candidate or history mutation; release prepare/execute;
  workflow edit; any tag other than `v0.5.0`; tag move, deletion, replacement,
  recreation, or force-push; crates.io; GitHub Release; binary, sidecar,
  installer, or model artifact; provider or credential access; Desktop or
  other consumer mutation; source-consumer or working-application smoke.
- **Escalation:** any preflight mismatch stops before mutation and returns to
  Chatterbox. Any post-tag failure preserves the immutable tag and enters the
  next-patch recovery rule. Tom owns later consumer or publication authority.

## Work

1. Fetch canonical branch and tags without changing source. Verify the worker
   tree is clean; remote URL exact; candidate commit/tree, ancestry, version,
   release note and fresh `0.5.0` baselines exact; preparation receipt identity
   retained in canonical evidence; review `5616849978` accepted the identical
   tree; run 34464717829 is exact-head success with 11 green jobs; previous
   tags remain immutable; local and remote `v0.5.0` are absent. Stop on any
   mismatch.
2. Create one local annotated tag `v0.5.0` at exactly
   `582d01d6b6890eed5195a1fcbee0ae985304a6c7` with exactly the approved three-
   paragraph annotation. Do not use current `HEAD` implicitly.
3. Re-read the local tag object, peeled commit, tree, type, and complete message.
   Stop before push if any identity or byte differs. Never delete or recreate
   the tag to repair a mismatch; report it.
4. Push only `refs/tags/v0.5.0` to canonical `origin`, without branch push,
   force, follow-tags, atomic multi-ref push, or another ref.
5. Fetch and verify local and remote tag object equality, exact peel to the
   candidate, and annotation bytes. Record the tag object SHA.
6. Identify and wait for the tag-triggered CI run whose source identity peels
   to the candidate. Record trigger, run ID, head SHA, and every job result. A
   red or absent run never permits tag movement, deletion, recreation, or
   retry; retain the tag and report the release failure.
7. On the current documentation branch, reconcile the tagged identity in
   Contract 036, the `0.5.0` release note/index, this task, one tag log/index,
   and roadmap front doors. Open one documentation PR, run `effigy qa:docs`,
   `effigy qa:northstar`, and `git diff --check`, obtain independent exact-head
   review, merge, and synchronize canonical `main`.

## Result

Closed 2026-09-10. One exact read-only preflight confirmed: worker tree clean
at planning commit `5ba7b11c`; remote URL exact; candidate `582d01d6` with
tree `eebb6978` and ancestor of canonical `main`; workspace version `0.5.0`,
`publish = false`, MSRV `1.95`; preparation receipt identity retained;
review `5616849978` recorded against the identical tree; merge-SHA push run
34464717829 green on all 11 jobs at the exact candidate; `v0.4.4` immutable
(tag object `41da6c1a` peeling to `49c9e3b2`); local and remote `v0.5.0`
absent.

Annotated tag `v0.5.0` was created once at exactly
`582d01d6b6890eed5195a1fcbee0ae985304a6c7` with the approved three-paragraph
annotation. Local re-read confirmed object type `tag`, peel to the exact
candidate and tree, and byte-exact message body. Push carried only
`refs/tags/v0.5.0` with no branch, force, or other ref. Post-push fetch
confirms the remote tag object equals local object
`c772c5839806b6cbf1c9d3b495049b362e4c0c52`, peeling to `582d01d6` with tree
`eebb6978`. Tag-triggered CI run 34467974791 (event `push`, head branch
`v0.5.0`, head SHA the exact candidate) completed `success` with all 11 jobs
green, including Pinned MSRV floor and Pinned MSRV floor tests. No tag
movement, deletion, recreation, GitHub Release, registry publication,
artifact, provider call, or consumer mutation occurred.

## Review Oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Tag target is exact | tag current documentation `HEAD` or PR head | tag peel is exactly `582d01d6…`; tree is `eebb6978…` |
| Annotation is exact | paraphrase, omit CI identity, or add publication wording | full tag contents byte-match the approved three paragraphs |
| Mutation is single-ref | push a branch, `--tags`, `--follow-tags`, or another tag | push receipt names only `refs/tags/v0.5.0` |
| Tag is annotated and immutable | lightweight tag, move, delete, recreate, or force | object type is `tag`; local/remote object SHAs agree; no destructive ref action |
| Release remains source-only | create GitHub Release, registry entry, or artifact | remote inspection and closeout retain every exclusion |
| Post-tag failure is honest | delete/retry tag or hide a red tag run | immutable tag retained; exact failure recorded under next-patch rule |
| Closeout does not mutate source | edit candidate, version, baselines, workflow, or runtime | documentation PR diff contains only owned closeout surfaces |

## Stop Conditions

- Stop before tag creation on dirty source, remote mismatch, missing or moved
  candidate, wrong tree/version, non-ancestor candidate, changed prior tag,
  missing review, non-green/different CI, or an existing local/remote
  `v0.5.0`.
- Stop before tag push if local type, target, tree, or annotation differs.
  Never delete or recreate the local tag under this task.
- After push, never move, delete, recreate, or force-push the tag, even if CI
  or documentation fails.
- Stop before release prepare/execute, workflow mutation, provider or consumer
  work, GitHub Release, registry publication, or artifact upload.

## Evidence

Record the authorized candidate and tree, canonical branch/remote, tag name,
exact annotation, pre-tag CI run and 11 job conclusions, tag object SHA, local
and remote ref equality, peeled commit/tree, tag-triggered CI identity and all
job results, documentation PR/head/review/merge/closeout, no forbidden
mutation, and every retained non-claim.

## Next Task

The immutable tag is verified. Tom/Chatterbox plans the source-tag consumer
and working-application lane, then relays the exact tag to Desktop so blocked
g02.051 can resume against the coordinated source. No consumer mutation,
provider call, publication, or further release follows automatically.
