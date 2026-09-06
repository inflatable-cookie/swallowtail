# Release Playbook

Use this when preparing a Swallowtail source tag. Authority is
[Contract 036](../contracts/036-crate-release-and-compatibility-boundary.md).
Shared vocabulary: [Key Concepts](key-concepts.md). This page does not
authorize a tag.

## Lane

One prepare attempt. One hosted run at the SHA to tag, or at a commit with
an identical tree. Tag request the moment those gates are green. Consumer
smoke runs on the tag afterwards, not as a pre-tag gate.

| Step | Who | Clock |
| --- | --- | --- |
| 1. Candidate content on clean canonical `main` | worker / merge | — |
| 2. One `effigy --json release prepare --yes --check-gates --version X.Y.Z` | worker | cheap gates; a docs-index failure dies in under a minute |
| 3. Candidate PR | worker | — |
| 4. Exact-head review and exact-SHA `CI` `workflow_dispatch` in parallel | reviewer; GitHub | hosted clippy, tests, and MSRV floor live here |
| 5. Merge on both green | coordinator | — |
| 6. If the merge SHA is not the green run's SHA and the trees differ, dispatch `CI` at the merge SHA | coordinator | before the tag request |
| 7. Tag request to the operator | Chatterbox | immediate on a qualifying green run; do not wait for consumer smoke |
| 8. Local annotated tag and push | coordinator, after operator authority naming the exact SHA | — |
| 9. Source-consumer and working-application smoke | consumer-proof card | after the tag |

Cheap local gates, in order: `fmt`, `qa`, `docs`, `metadata`, `api`,
`security`, `source`. Expected local wall clock is minutes, not the previous
~30 min clippy-and-test prologue. `qa` (`effigy qa:docs` then `qa:routes`)
is the docs-index check that used to fail after that prologue.

Hosted `CI` satisfies `lint`, `lint:no-features`, `test`, and `floor`. The
run counts only at the SHA to tag or a commit with an identical tree.
Local prepare records that run id. The release note names it. If no
qualifying hosted run exists, invoke the local-heavy profile in
`config/release.toml` (the four commented keys, in that order) and rerun
`effigy --json release gates`.

Do not create, move, or push a tag from a green gate, changelog, or closeout.

## Tag Request

Send this the moment the SHA to tag has a qualifying green hosted run and
accepted exact-head review. If merge produced a new SHA whose tree is not
identical to a green run, dispatch `CI` at the merge SHA first:

```text
Authorize local annotated-tag creation and tag push:

- source commit: <full SHA to tag>
- canonical branch: main
- remote: git@github.com:inflatable-cookie/swallowtail.git
- tag: vX.Y.Z
- hosted CI run: <databaseId> at that SHA, or at <identical-tree SHA>
- annotated message:

Swallowtail vX.Y.Z

<source-only one-line summary>. Canonical hosted CI run <databaseId>
passed against <full SHA to tag or identical-tree SHA>.

The source-only release contains no crates.io publication, GitHub Release
object, binary, sidecar, installer, or model artifact.
```

Creating the local tag and pushing it are separate mutations unless one
approval names both.
