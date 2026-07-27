# Packaged Provider Session Lifecycle Checkpoint

Date: 2026-07-27
Card: `../roadmaps/g02/batch-cards/059-packaged-provider-session-lifecycle-proof.md`

## Package Topology Repair

The first transient candidate stopped while packaging Claude Agent. Its new
remote ACP lifecycle test depends on `swallowtail-transport-acp-remote`, but
the package assembler patched only the six older internal crates into the
local registry view.

The package set now owns one centralized seven-crate internal patch inventory.
Package assembly, extracted provider tests, lifecycle tests, and packaged
consumer tests all render their workspace patches from that inventory.

## Lifecycle Gate

A separate extracted-package gate now proves:

- the source-bundled route and lifecycle matrices
- exactly three adapters declare provider-session management
- nine focused lifecycle suites across testkit, Codex, Claude Agent, remote
  ACP, and OpenCode
- every supported, unsupported, and not-applicable route posture
- compile-time absence of a fabricated management role from Kimi, Gemini, and
  the seventeen not-applicable routes
- checksummed evidence with no credential or provider call

The nine suites ran 33 tests.

## Transient Candidate Evidence

One clean synthetic snapshot exercised the full candidate pipeline without
retaining or replacing release evidence:

- all 23 package archives assembled
- source bundle and parent verified
- regenerated package and package-file hashes matched byte-for-byte
- 20 provider-facade suites passed across all 22 routes
- all 33 focused lifecycle tests passed
- Nucleus passed 15 deterministic checks; two live checks stayed ignored
- Soundcheck passed six deterministic checks; one live check stayed ignored
- packaged Codex passed 105 tests

No credential, provider call, publication, push, tag, release, registry,
workflow, or owner mutation occurred.

## Provenance Gate

The held candidate remains unchanged. A synthetic snapshot is useful test
evidence but cannot replace the existing canonical-history candidate.

Card 059 remains active. After the current Swallowtail source enters canonical
history, rerun the same full gate against clean `HEAD`, retain the candidate,
verify its evidence, then advance to card 060.
