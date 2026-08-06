# v0.1.1 Source Patch Release

Date: 2026-08-06
Roadmap: g03.044
Cards: 132-134

## Outcome

Published annotated source tag `v0.1.1` at exact CI-green release commit
`bd3f4bbdffc403897ece4499ee0904b1e8116639`.

The patch carries the Anthropic Managed Agents cancellation-precedence repair.
Release simulation also found a Kimi test-harness race: the client joined its
local observer before the independent fixture-server thread necessarily
recorded the close frame. The fixture now joins its peer before reading that
evidence; production Kimi behavior is unchanged.

## Candidate Evidence

- 40 repeated Rust 1.90 Kimi detachment runs pass
- focused Kimi validation passes 108 tests; extracted-package proof passes
- ten complete workspace rounds pass 1,464 tests each with 11 live probes
  skipped
- all 11 final-version Effigy release gates pass
- dependency refresh advances `zerocopy` and `zerocopy-derive` to `0.8.56`;
  six newer transitive versions remain outside current dependency-range or
  Rust-floor selection
- exact source-consumer proof passes locally and from a fresh tag clone

## Remote Evidence

- release commit: `bd3f4bbdffc403897ece4499ee0904b1e8116639`
- GitHub CI run: `31107478654`; all six jobs pass
- tag object: `d7cb439ef3b6808013950d209c2ffcf7930ec81a`
- peeled tag commit: `bd3f4bbdffc403897ece4499ee0904b1e8116639`
- previous tag `v0.1.0` remains unchanged

## Tool Boundary

Effigy's combined execute path cannot implement the required
release-commit-to-CI-to-tag split after prepared HEAD changes. The release used
Effigy for simulation, preparation, and all gates, then the same explicit
annotated-tag mutation used by `v0.1.0`.

The generic install verifier tried to install an `effigy` binary from this
library-only repository. Fresh-tag Git-source consumer verification is the
applicable install proof and passes.

## Exclusions

No crates.io publication, GitHub Release, binary, sidecar, installer, consumer
mutation, live provider call, or authenticated provider work ran.
