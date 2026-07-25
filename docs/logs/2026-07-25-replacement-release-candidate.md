# Replacement Release Candidate

Date: 2026-07-25
Roadmap: g02.012
Card: 036

## Outcome

One exact provider-wide `0.1.0` candidate is active. The prior compile-only
candidate and the earlier provisional candidate remain immutable superseded
evidence. No external release state changed.

Release notes and Nucleus and Soundcheck handoffs now use the adapter-local
prepared normal path. They keep crate compatibility, qualified provider
support, permitted unverified-newer execution, access, entitlement, billing,
topology, and sandbox truth separate.

## API Classification

Comparison against superseded source
`6c0e8d9b5b05e9db0f655527491d47aa18d246f8` changes deterministic public-
declaration hashes for 20 packages: core, runtime, local host, testkit, and all
16 adapters. The ACP protocol, OpenAI-compatible chat protocol, and remote ACP
transport hashes do not change.

No Swallowtail version has been published. The replacement is therefore
candidate-breaking but remains the initial `0.1.0` baseline. No compatibility
shim or published-version migration is implied.

## Candidate Evidence

- source commit:
  `73c7f5b5b5611ef20bdcc1572deeb39ca50630e1`
- source base commit:
  `91a0774010ee83594a4565e1b4e2b0daa998db28`
- package count: 23
- package checksum-manifest digest:
  `1442fdea7f8426fd3dcd74ef8513a0945761798877208e5f9a1454720591eac5`
- evidence-manifest digest:
  `9d45e325fc7f23c28546fec107d3f0420189bdad115dd491db9899fec27a13a1`
- provider evidence digest:
  `e73a67fd06617675c9a84f4fb171409d4fdd973feaef439d91c35260bde38818`
- consumer evidence digest:
  `fe760fb4a91a273ebaff16ae7a7d3618356a6761689f4aac6a0002a56922ab07`
- prepared facade suites: 20
- production route proofs: 22
- prepared facade tests: 65 passed
- Nucleus: 14 passed, two live probes ignored
- Soundcheck: four passed, one live probe ignored
- packaged Codex: 89 passed
- credentials: absent
- provider calls: none

Rebuilding from the source bundle reproduces the package and audited file-list
checksums. The exact prior candidate is retained at
`.effigy/release-candidates/superseded/0.1.0-6c0e8d9b5b05/`.

## Validation

- package metadata, dependencies, public API, docs, MSRV, content, extraction,
  and test compilation: pass
- source-bundle regeneration and checksum comparison: pass
- packaged provider-wide facade proof: pass
- packaged Nucleus, Soundcheck, and Codex runtime proof: pass
- route inventory, repository QA, and worktree checks: pass

`effigy doctor` retains the known 19 oversized-file findings: seven errors and
12 warnings. No category or count changed.

## Publication Boundary

Registry upload, owner changes, credentials, tags, pushes, workflows, releases,
and consumer edits remain blocked. The sole next task is an operator decision
on the exact external mutation set.

## Superseded State

Roadmap g02.013 later replaced this card's parentless source snapshot with the
canonical-history candidate recorded in active evidence. This log remains
historical evidence for the first provider-wide package proof.
