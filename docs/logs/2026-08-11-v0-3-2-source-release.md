# v0.3.2 Source Release

Date: 2026-08-11
Roadmap: `../roadmaps/g03/067-v0-3-2-source-patch-release.md`
Cards: 210-213

## Outcome

Published annotated source tag `v0.3.2` at exact CI-green release commit
`a859d56b47b1bc2975df7d0516ca96fd8e485b35`.

The compatible patch contains 30 independently selectable packages and 36
production routes. It adds Command Code, idioms, Codex spawn admission, and
Claude Code response-only execution while preserving Rust `1.95.0`, the
Apple Silicon macOS verified target, and source-only distribution.

## Evidence

- all 11 local release gates passed, including 1,625 workspace tests
- pre-tag CI run `31535950176` passed all five jobs at the release commit
- tag CI run `31536392314` passed all five jobs at immutable ref `v0.3.2`
- annotated tag object: `702f355631bb6fe8fe6cb098f48887df8ef8ca43`
- peeled release commit: `a859d56b47b1bc2975df7d0516ca96fd8e485b35`
- earlier tag objects `v0.1.0` through `v0.3.1` remain unchanged

## Tool Boundary

Effigy execution rejected the committed prepared state because HEAD had moved
since preparation and the expected manifest and changelog changes were already
committed. No stale override ran. The accepted explicit annotated-tag fallback
created and pushed only `v0.3.2`.

No crates.io publication, GitHub Release, binary, installer, consumer mutation,
or provider call ran.

## Next Move

Figmatic may replace its linked revision with immutable `v0.3.2` and replay the
qualified response-only unit. Swallowtail returns to the g03 evidence gate.
