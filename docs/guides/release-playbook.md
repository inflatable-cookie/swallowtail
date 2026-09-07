# Release Playbook

Use this when preparing a Swallowtail source tag. Authority is
[Contract 036](../contracts/036-crate-release-and-compatibility-boundary.md).
Shared vocabulary: [Key Concepts](key-concepts.md). This page does not
authorize a tag.

## Lane

One prepare attempt. One qualifying hosted run: a green hosted `CI` run
triggered by `workflow_dispatch` (or a push to `main`) at the SHA to tag or
at a commit with an identical tree; pull-request runs do not qualify because
the MSRV floor skips its tests there. Tag request the moment those gates are
green. Consumer smoke runs on the tag afterwards, not as a pre-tag gate.

| Step | Who | Clock |
| --- | --- | --- |
| 1. Candidate content on clean canonical `main` | worker / merge | — |
| 2. Preseed the three `X.Y.Z` baselines from the current tree: `bash scripts/generate-public-api-baseline.sh release-baselines/public-api-X.Y.Z`; `release-baselines/production-routes-X.Y.Z.txt` (current route inventory); `release-baselines/internal-dependencies-X.Y.Z.tsv` (workspace edges at `X.Y.Z`); update root `README.md` version lines; never touch an earlier baseline | worker | minutes; the cheap `qa` gate fails immediately if any is missing |
| 3. One `effigy --json release prepare --yes --check-gates --version X.Y.Z` | worker | cheap gates; a docs-index failure dies in under a minute |
| 4. Candidate PR | worker | — |
| 5. Exact-head review and exact-SHA `CI` `workflow_dispatch` in parallel | reviewer; GitHub | hosted clippy, tests, and MSRV floor live here |
| 6. Merge on both green | coordinator | — |
| 7. If the merge SHA is not the green run's SHA and the trees differ, dispatch `CI` with `workflow_dispatch` at the merge SHA | coordinator | before the tag request |
| 8. Tag request to the operator | Chatterbox | immediate on a qualifying green run; do not wait for consumer smoke |
| 9. Local annotated tag and push | coordinator, after operator authority naming the exact SHA | — |
| 10. Source-consumer and working-application smoke | consumer-proof card | after the tag |

Cheap local gates, in order: `fmt`, `qa`, `docs`, `metadata`, `api`,
`security`, `source`. Expected local wall clock is minutes, not the previous
~30 min clippy-and-test prologue. `qa` (`effigy qa:docs` then `qa:routes`)
is the docs-index check that used to fail after that prologue.

`lint`, `lint:no-features`, `test`, and `floor` are satisfied by a green
hosted `CI` run triggered by `workflow_dispatch` (or a push to `main`) at
the SHA to tag or at a commit with an identical tree; pull-request runs do
not qualify because the MSRV floor skips its tests there. Two run ids, two jobs, no circularity: the release note (inside the tree)
names the run that proved the prepared candidate content; the tag request
names the final qualifying `workflow_dispatch` run at the SHA to tag, run
after the last docs-only commit and recorded in the PR closeout comment and
the card Result, never by another tree mutation. If no qualifying hosted run
exists, invoke the local-heavy profile in `config/release.toml` (the four
commented keys, in that order) and rerun `effigy --json release gates`.

Do not create, move, or push a tag from a green gate, changelog, or closeout.

## Tag Request

Send this the moment the SHA to tag has a qualifying green hosted
`workflow_dispatch` (or `main` push) run and accepted exact-head review. If
merge produced a new SHA whose tree is not identical to a green qualifying
run, dispatch `CI` with `workflow_dispatch` at the merge SHA first:

```text
Authorize local annotated-tag creation and tag push:

- source commit: <full SHA to tag>
- canonical branch: main
- remote: git@github.com:inflatable-cookie/swallowtail.git
- tag: vX.Y.Z
- hosted CI run: <databaseId> from `workflow_dispatch` (or push to `main`) at that SHA, or at <identical-tree SHA>
- annotated message:

Swallowtail vX.Y.Z

<source-only one-line summary>. Canonical hosted CI run <databaseId>
passed against <full SHA to tag or identical-tree SHA>.

The source-only release contains no crates.io publication, GitHub Release
object, binary, sidecar, installer, or model artifact.
```

Creating the local tag and pushing it are separate mutations unless one
approval names both.
