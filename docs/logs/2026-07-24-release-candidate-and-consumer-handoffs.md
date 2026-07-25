# Release Candidate And Consumer Handoffs

Date: 2026-07-24
Card: `../roadmaps/g02/batch-cards/004-release-candidate-and-consumer-handoffs.md`

## Result

The first non-published Swallowtail candidate freezes all 23 public packages at
coordinated version `0.1.0`.

The retained candidate contains:

- one deterministic clean source commit in a Git bundle
- all 23 `.crate` archives in the Contract 036 three-stage order
- archive and audited file-list SHA-256 checksums
- candidate environment and publication-order records
- isolated Nucleus and Soundcheck compatibility records

An independent clone of the source bundle rebuilds byte-identical package
archives and file-list evidence. The candidate source commit includes the
release notes, changelog, handoffs, gates, and completed roadmap state. Generated
textual candidate evidence is excluded from that source snapshot and copied to
`release-candidates/0.1.0/` after the candidate freezes.

## Consumer Evidence

Nucleus and Soundcheck are copied from their current tracked plus untracked
non-ignored source states into temporary directories. Each copy receives a
deterministic source-snapshot commit before dependency changes.

Both checks replace the four consumed sibling paths with exact `=0.1.0`
requirements and patch crates.io to the extracted candidate archives.

- Nucleus: `cargo check -p nucleus-agent-adapters --all-targets --locked`
- Soundcheck: `cargo check -p soundcheck-app --all-targets --locked`

Soundcheck retains its sibling Soundcheck Library and Signal sources. Its
locked `lucide-static` compile-time assets are supplied read-only from the
original worktree. The original consumer repositories, manifests, and
lockfiles do not change.

The handoffs keep crate compatibility, provider-interface ranges, access,
sandboxing, and consumer policy separate. They name exact upgrade, validation,
and rollback steps.

## Release Planning Limit

Read-only `effigy release gates` cannot resolve one workspace version from this
virtual 23-package workspace. It looks for a single package version or
`workspace.package.version`, while Swallowtail's release is an explicitly
staged package family.

The candidate scripts follow Contract 036's exact package graph and stage
order. No generic single-package release configuration was invented. A future
Effigy release model may support staged virtual workspaces; that is not
required to inspect or reproduce this candidate.

## Validation

- candidate content, size, manifest, and secret audit — passed
- extracted 23-package locked check and test compilation — passed
- source-bundle integrity and byte-for-byte package rebuild — passed
- isolated exact-package Nucleus and Soundcheck checks — passed
- package metadata and public-declaration baselines — passed
- Rust `1.93.0`, Bedrock Rust `1.94.1`, and current-stable Rust `1.97.1`
  checks — passed
- package documentation and full `effigy qa` — passed
- `effigy doctor` — unchanged inherited 19 oversized-file findings: 12
  warnings and seven errors
- `git diff --check` — passed

Apple Silicon macOS is verified. Other targets are unverified, not prohibited.

## Authority

No live provider or registry credential was read. No registry name, account,
owner, team, package, tag, branch, GitHub release, workflow, or consumer
repository was mutated.

The sole next task is an operator review of the frozen evidence and an explicit
decision on registry preflight and staged publication.
