# v0.2.0 Source Release

Date: 2026-08-06
Roadmap: g03.046
Cards: 139-142

## Outcome

Published annotated source tag `v0.2.0` at exact CI-green release commit
`0104b8948ad141f5c42ad752127203b9b1d72db5`.

The release adds the separately selectable Muse Code package and route, moves
the coordinated source set to 28 packages and 34 production routes, and raises
the unified Rust floor to `1.95.0`. The floor change is the intentional
breaking boundary. Existing package APIs and route identities remain
compatible.

## Candidate Evidence

- all 11 final-version Effigy release gates pass
- every package passes Clippy and the complete workspace suite at Rust `1.95.0`
- the external Git-source consumer resolves the exact candidate graph
- the semantic API, missing-doc, route, guide, metadata, and supply-chain gates
  pass
- Muse contributes no new error-severity structural-size finding

## Remote Evidence

- release commit: `0104b8948ad141f5c42ad752127203b9b1d72db5`
- pre-tag GitHub CI run: `31128930466`; all five jobs pass
- tag-ref GitHub CI run: `31129147745`; all five jobs pass at `v0.2.0`
- tag object: `643373ccb794c854a594297d823972dc3621fd3c`
- peeled tag commit: `0104b8948ad141f5c42ad752127203b9b1d72db5`
- previous tags `v0.1.0` and `v0.1.1` remain unchanged

## Tool Boundary

The committed candidate and its gate repairs moved `HEAD` after Effigy
preparation. Effigy correctly rejected execution because its prepared state was
stale and the expected working-tree mutations were already committed. No stale
override ran. The release used the repository runbook's explicit annotated-tag
fallback at the exact green commit.

## Exclusions

No crates.io publication, GitHub Release, binary, sidecar, installer, consumer
mutation, live provider call, or authenticated provider work ran.
