# Canonical-History Release Candidate

Date: 2026-07-25
Roadmap: g02.013
Cards: 037-039
Candidate: `0.1.0`

## Outcome

The active unpublished candidate comes from normal local `main` history. Its
exact source, parent, and scope are retained in
`release-candidates/0.1.0/candidate.env`. The retained bundle records complete
history and regenerates the exact 23-package archive and audited file-list set.

The former parentless provider-wide candidate moved intact to
`.effigy/release-candidates/superseded/0.1.0-73c7f5b5b561/`. The compile-only
and provisional candidates also remain immutable. Only
`.effigy/release-candidates/0.1.0/` is active.

## Provenance Boundary

Contract 036 and release topology now separate two package modes:

- ordinary local verification may freeze tracked plus nonignored untracked
  working-tree content into a deterministic root snapshot
- final candidate preparation requires a clean non-root canonical-history
  commit and records its exact parent and source scope

Candidate verification rejects root history, clones the retained source
bundle, rebuilds all archives, and compares package plus file-list checksums.
Before any upload, the exact candidate commit must be reachable from
`origin/main`. The release tag must target that same commit.

## Evidence

- source commit and parent: `release-candidates/0.1.0/candidate.env`
- package count: 23
- package and evidence manifests: `release-candidates/0.1.0/*.sha256`
- provider and consumer evidence:
  `release-candidates/0.1.0/*-validation.{env,sha256}`
- prepared facade suites: 20
- production route proofs: 22
- Nucleus: 14 passed, two live probes ignored
- Soundcheck: four passed, one installed probe ignored
- packaged Codex: 89 passed
- credentials: absent
- provider calls: none

Soundcheck changed in its own worktree during the first parallel proof. The
retained evidence names the later exact source snapshot used by the isolated
passing proof. Swallowtail did not edit Nucleus, Soundcheck,
Soundcheck Library, or Signal.

One initial parallel Soundcheck test exceeded its ten-second fake discovery
deadline under two concurrent complete package builds. It passed in 0.15
seconds after the provider proof completed and in 0.14 seconds when attached
directly to the retained candidate. This was resource contention, not a
candidate incompatibility.

## External State

Local `main` contains the candidate source commit. `origin/main` remains at its
parent. No crate, tag, push, GitHub release, workflow, owner, credential,
provider, or consumer state changed.

Publication still requires the exact crates.io owner username and explicit
authorization of the selected main push, sequential three-stage uploads,
`v0.1.0` tag push, and GitHub release.
