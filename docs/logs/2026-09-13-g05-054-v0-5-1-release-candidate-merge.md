# g05.054 v0.5.1 Release Candidate Merge

Merged 2026-09-13: PR #317 landed the source-only `v0.5.1` candidate
`755a2669` (tree `d375b3227985e8e552ba9346d8f9b8631936db5f`) as merge
`e9140b46` after independent exact-head review `5649975685`
(betterthanclay, read-only: no file edited, no commit or push). Merge tree
is byte-identical to the reviewed candidate head, so no new identity was
introduced at merge.

## Accepted outcome

Coordinated version `0.5.1` carries the Desktop-qualified g05.053 source
unchanged: `git diff --exit-code 0209dd7f..merge -- crates/` is empty and
the frozen `crates/` tree `186f3ba42a4aa896c03a3bddcacb007b1afbd8cc` and
Grok adapter tree `1a6f777f84f9aef80c146badf6944811568e23cd` still resolve.
The two named `v0.5.0` baselines match immutable tag `v0.5.0` byte-for-byte;
`docs/releases/0.5.1.md` lists the true 50-route inventory including
`grok-build.catalogue`; both `[Unreleased]` entries (OpenCode HTTP
`1.18.30`, Grok `1.0.25` catalogue) promoted under `## [0.5.1]`.
`v0.5.1` is absent locally and remotely; `v0.5.0` immutable; every package
remains `publish = false`; no tag, registry publication, GitHub Release, or
other artifact.

## Review and validation at the exact head

Review `5649975685` cleared every review-oracle reject condition with
evidence: zero `crates/**` diff, patch version, fresh (not reused) release
state, prior-version changes confined to the two tag-equal blobs,
release-note inventory set-equal to the fresh `0.5.1` baseline,
provider-free prepare with all seven gates green, exact-source consumer
pass, and no implied registry availability. Exact-head hosted CI is green
twice over at `755a2669`: pull-request run `34730275986` (all PR checks
pass) and workflow-dispatch run `34730278564` (11/11 success). Merge-SHA
push run `34731113171` was still in progress at closeout; the merge changes
no tree bytes, so it cannot re-qualify the candidate.

## Deferred (non-blocking, no action)

- The worker's commit appends its own evidence-log index line to the
  reserved logs index though the manifest names that surface shared
  closeout. The line is accurate with no concurrent siblings; kept.
- Registry absence is proven structurally (`publish = false`, no publish
  step, no GitHub Release); the external crates.io query returned HTTP 403
  from the review environment, so no external-absence claim is made.

## Boundary

The exact merged identity (`e9140b46`, candidate `755a2669`) returns to Tom
for the separate exact-SHA annotated-tag authorization under Contract 036.
Desktop g02.089 stays held until the real `v0.5.1` tag exists. No task is
ready after this merge; planned tasks 035 and 039–042 await operator
promotion, disposition, or requirements.
