# 106 v0.4.3 Candidate Preparation

Status: complete; PR 246 merged at `cbd4ddc8`
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../033-v0-4-3-release-readiness.md`
Depends on: ready on card 105's merge

## Goal

Prepare the `0.4.3` candidate exactly as card 101 did for `0.4.2`: one Effigy prepare transaction with per-gate logs, release note from card 105's result, scripts repointed, candidate PR with review and exact-SHA CI in parallel, merge, and report the candidate SHA to Chatterbox at once for the tag request.

## Readiness

Ready on the condition in the status line; the manifest in the milestone
roadmap carries the full scope, owned paths, validation, and stop rules.

## Result

Candidate prepared from clean merged PR245 base
`217de072c21bb6efe315f7eb0175554f97648ef7` under direct Tom/root authority.
Remote `main` matched that base; the feature freeze remains in force.
PR245's source head `13aca3789c872deb175ab7cd367fbf4edd694ccc` passed
independent review (GitHub comment `5561367079`) and all 11 jobs in CI run
`34052744897`.

Read-only status inferred `0.4.3` as a patch from `0.4.2`, with 11 configured
gates and no blockers. Exactly one prepare transaction ran:

```text
effigy --json release prepare --yes --check-gates --version 0.4.3
```

All gates passed: `fmt`, `lint`, `lint:no-features`, `test`, `qa`, `docs`,
`metadata`, `api`, `security`, `floor`, and `source`. Nextest passed 3164 tests
across 274 binaries, with 24 skipped. The pinned Rust `1.95.0` full Clippy/test
floor passed. The source-consumer gate used the explicitly reported synthetic
Git snapshot of the prepared tree. No retries or frozen-tree rerun occurred.

Local evidence retained in this workspace:

- `.effigy/reports/release/status-0.4.3.json`; SHA-256
  `8fc3a54df0aabb46f3b0c8928f46c479917e40ecfa7dc3f680cc787c9684460c`
- `.effigy/reports/release/prepare-0.4.3.json`; SHA-256
  `78d8662db3e94c90f0c88eb4efd6e03ef8fc7d7a7b32472c92c29f968feb77a1`
- `.effigy/reports/release/api-delta-0.4.3.json`; SHA-256
  `e8e6e4343f683dd791e50c64f3dce3621c893912b63e3cb2ebb774ec11269479`
- `.effigy/reports/release/gates/`: each named gate's full `.log` and the
  redacted `environment.json`.
- `.release-prepared.json`: successful Effigy metadata and prepared-file
  fingerprints at the source base; retained locally under the existing ignore
  rule, with no manual rewrite of its provenance.

Distinct `0.4.3` baselines contain 40 package API files plus `packages.txt`,
49 routes, and 88 internal dependency edges. Compared against the actual tagged
`v0.4.2` tree, Claude adds 18 API items and Grok adds 5, with no removals.
The existing `0.4.2` files already contain merged additive evidence; this card
preserves every existing baseline byte and creates separate `0.4.3` files.

Effigy advanced the coordinated workspace version, eight internal requirements,
and 40 workspace lock versions, and promoted the changelog. Release notes/index,
README, four gate scripts, and the consumer front door now name `0.4.3`.
The notes cover merged cards 082, 085, and 105. Card105's pinned advisories are
progress only during an active turn; valid idle advisories emit no event, actual
results remain authoritative, and malformed/unknown records fail closed.
Deterministic two-turn continuation is proved; actual live Desktop termination
cause and reply remain unproved.

Candidate PR head and exact-SHA workflow-dispatch evidence are reported on the
PR. Independent same-workspace review and merge remain root-owned. This card
claims no merge or tag. No crate source/tests, historical baselines, consumer,
provider, auth, workflow, or global tooling changed. Workspace retained through
release; no tag, publication, GitHub Release, or binaries created.
