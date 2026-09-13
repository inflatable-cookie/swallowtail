# g05.055 v0.5.1 Annotated Source Tag

Status: ready
Owner: Tom
Created: 2026-09-13
Depends on: g05.054; Contract 036; release playbook
Vision tags: source release, annotated tag, compatibility

## Outcome

Create one immutable annotated source tag `v0.5.1` at exact candidate merge
`e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`, push only that tag ref to
canonical `origin`, verify local/remote object, peel, tree, and message
agreement, wait for tag-triggered CI, then publish the reviewed documentation
closeout. Do not publish any other release object or artifact.

## Operator Authorization

Tom explicitly authorized both local annotated-tag creation and tag push on
2026-09-13 for:

- source commit: `e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`
- source tree: `d375b3227985e8e552ba9346d8f9b8631936db5f`
- reviewed candidate head: `755a2669185f4937dbccde95effc5d82dfaa0324`
- canonical branch: `main`
- remote: `git@github.com:inflatable-cookie/swallowtail.git`
- tag: `v0.5.1`
- qualifying hosted CI: push run `34731113171`, all 11 jobs green at the
  exact source commit
- no crates.io publication, GitHub Release, binary, sidecar, installer, model
  artifact, provider call, or consumer mutation

Tom approved these exact annotation bytes:

```text
Swallowtail v0.5.1

OpenCode HTTP 1.18.30 qualification and Grok Build 1.0.25 authenticated model catalogue. Canonical hosted CI run 34731113171 passed against e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27.

The source-only release contains no crates.io publication, GitHub Release object, binary, sidecar, installer, or model artifact.
```

No other release or mutation authority follows.

## Ready-State Rubric

- [x] g05.054 prepared coordinated version `0.5.1`; preparation receipt
      SHA-256 is
      `146d95d7bf0aeef8419bb3a73fc2e80fff2324f6ba59e1f5eebc8e0cb836eca7`.
- [x] PR #317 merged reviewed candidate `755a2669` as exact merge
      `e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`, tree
      `d375b3227985e8e552ba9346d8f9b8631936db5f`.
- [x] Candidate and merge trees are byte-identical.
- [x] Independent exact-head review `5649975685` accepted the candidate.
- [x] Merge-SHA push run `34731113171` completed success with all 11 jobs
      green.
- [x] `crates/` tree
      `186f3ba42a4aa896c03a3bddcacb007b1afbd8cc` and Grok subtree
      `1a6f777f84f9aef80c146badf6944811568e23cd` remain identical to
      Desktop-qualified source `0209dd7ffc19c457ca56d55683576e09f049f7ee`.
- [x] Workspace version is `0.5.1`; every package remains `publish = false`.
- [x] The candidate merge is an ancestor of canonical `main`; later closeout
      `c712b902450fad8783567ed70bba09dd240f0ebe` is not the tag target.
- [x] Local and remote `v0.5.1` are absent; `v0.5.0` remains immutable.
- [x] Tom authorized both tag mutations and the exact annotation.

## Dispatch Manifest

- **State:** ready; dispatch after this planning commit reaches pushed `main`.
- **Completion:** exact read-only preflight; one local annotated tag at the
  named candidate merge; one exact tag-ref push; verified local/remote tag
  object, peel, tree, and annotation; tag-triggered CI; source and registry/
  release evidence; one independently reviewed documentation PR; direct
  completion return to Desktop Chatterbox.
- **Owned mutable paths:** local `refs/tags/v0.5.1`; remote
  `refs/tags/v0.5.1`; this task;
  `docs/contracts/036-crate-release-and-compatibility-boundary.md`;
  `docs/releases/0.5.1.md`; `docs/releases/README.md`; one new tag log and
  `docs/logs/README.md`; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`,
  `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`.
- **Worker:** source-tag release worker; no provider access.
- **Feature freeze:** no unrelated source merge before the tag mutation. Later
  documentation-only commits do not replace the authorized candidate.
- **Acceptance evidence:** exact target/tree/version; immutable prior tags;
  pre-tag CI; single-ref push receipt; annotated tag object and annotation;
  local/remote equality; tag-triggered CI; unchanged qualified package trees;
  `publish = false`; absent GitHub Release and publication artifacts.
- **Review oracle:** reject a different target/tree, lightweight or moved tag,
  annotation drift, multi-ref or force push, missing/red tag CI, qualified-tree
  mismatch, source/workflow/version mutation, publication, provider contact,
  Desktop mutation, or unsupported registry-availability inference.
- **Stop conditions:** any dirty/divergent preflight, existing `v0.5.1`, target
  or annotation mismatch, changed prior tag, non-green/different pre-tag CI,
  or forbidden mutation. After push, retain the immutable tag and report any
  failure; never delete, move, recreate, or retry it.
- **Escalation owner:** Chatterbox for release semantics; queue coordinator for
  mechanical blockers; Tom for any mutation beyond the exact authorization.
- **Excluded:** candidate or history mutation; release prepare/execute;
  workflow edit; any tag other than `v0.5.1`; tag move, deletion, replacement,
  recreation, or force-push; crates.io; GitHub Release; binary, sidecar,
  installer, model artifact; provider or credential access; Desktop or other
  consumer mutation; source-consumer or working-application smoke.

## Work

1. Fetch canonical branch and tags without changing source. Verify the worker
   tree is clean; remote URL exact; candidate merge/tree, ancestry, version,
   release note and fresh `0.5.1` baselines exact; preparation receipt retained;
   review `5649975685` accepted the identical tree; run `34731113171` is
   exact-head success with 11 green jobs; previous tags remain immutable; local
   and remote `v0.5.1` are absent. Stop on any mismatch.
2. Create one local annotated tag `v0.5.1` at exactly
   `e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27` with exactly the approved
   three-paragraph annotation. Do not use current `HEAD` implicitly.
3. Re-read the local tag object, peeled commit, tree, type, and complete
   message. Stop before push if any identity or byte differs. Never delete or
   recreate the tag to repair a mismatch; report it.
4. Push only `refs/tags/v0.5.1` to canonical `origin`, without branch push,
   force, follow-tags, atomic multi-ref push, or another ref.
5. Fetch and verify local and remote tag object equality, exact peel to the
   candidate merge, and annotation bytes. Record the tag object SHA.
6. Identify and wait for the tag-triggered CI run whose source identity peels
   to the candidate. Record trigger, run ID, head SHA, and all 11 job results.
   A red or absent run never permits tag movement, deletion, recreation, or
   retry; retain the tag and report the release failure.
7. Reprove the tagged `crates/` and Grok subtree identities against
   `0209dd7f`; verify every package is version `0.5.1` and `publish = false`;
   verify no GitHub Release or repository artifact exists. Treat registry
   absence structurally unless an authoritative registry query succeeds.
8. Reconcile the tagged identity in Contract 036, the `0.5.1` release note and
   index, this task, one tag log/index, and roadmap front doors. Open one
   documentation PR, run `effigy qa:docs`, `effigy qa:northstar`, and
   `git diff --check`, obtain independent exact-head review, merge, and
   synchronize canonical `main`.

## Review Oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Tag target is exact | tag current documentation `HEAD` or reviewed PR head | peel is exactly `e9140b46…`; tree is `d375b322…` |
| Qualified source is preserved | tag a source-different candidate | tagged `crates/` and Grok trees equal the Desktop-qualified identities |
| Annotation is exact | paraphrase, omit CI identity, or add publication wording | full tag contents byte-match the approved three paragraphs |
| Mutation is single-ref | push a branch, `--tags`, `--follow-tags`, or another tag | push receipt names only `refs/tags/v0.5.1` |
| Tag is annotated and immutable | lightweight tag, move, delete, recreate, or force | object type `tag`; local/remote object SHAs agree; no destructive ref action |
| Release remains source-only | create GitHub Release, registry entry, or artifact | remote inspection and closeout retain every exclusion |
| Post-tag failure is honest | delete/retry tag or hide a red tag run | immutable tag retained; exact failure recorded under next-patch rule |
| Closeout does not mutate source | edit candidate, version, baseline, workflow, or runtime | documentation PR contains only owned closeout surfaces |

## Evidence

Record the authorized candidate merge and tree, reviewed head, canonical
branch/remote, tag name, exact annotation, pre-tag CI run and 11 job
conclusions, tag object SHA, local/remote ref equality, peeled commit/tree,
tag-triggered CI identity and all job results, qualified package/source tree
identity, package version and `publish = false`, registry/GitHub Release/
artifact evidence, documentation PR/head/review/merge/closeout, and every
retained non-claim. Send the complete immutable capsule directly to Desktop
Chatterbox agent `5317069e-201f-4dea-94f3-af8f0b9faff2` so g02.089 Phase B
can resume.

## Next Task

After the immutable tag and documentation closeout are verified, Desktop
g02.089 resumes against the ordinary `v0.5.1` pin. No Swallowtail consumer
mutation, provider call, registry publication, GitHub Release, or further
release follows automatically.
