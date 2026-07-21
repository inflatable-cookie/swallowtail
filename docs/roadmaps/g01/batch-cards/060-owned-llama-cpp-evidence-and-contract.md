# 060 Owned llama.cpp Evidence And Contract

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../019-owned-llama-cpp-serving-proof.md`

## Objective

Select and contract the smallest current owned llama.cpp lifecycle before
runtime implementation.

## Governing References

- `../../../contracts/007-model-artifact-and-serving-boundary.md`
- `../../../contracts/009-async-operation-lifecycle.md`
- `../../../contracts/010-execution-host-services-and-inputs.md`
- `../../../contracts/011-runtime-conformance-profiles.md`
- `../../../research/005-post-tranche-coverage-evidence.md`

## Scope

- current upstream release, executable provenance, launch flags, ephemeral
  endpoint behavior, readiness, route evidence, and stop behavior
- exact artifact, endpoint, handle, and cleanup contract gaps
- attached-versus-owned and Monkey authority boundaries
- roadmap compilation from card 060

## Out Of Scope

- runtime or adapter code
- model download, conversion, license acceptance, or live inference
- persistent serving or containment-platform selection

## Acceptance Criteria

- [x] current authoritative evidence fixes one exact build and bounded launch
      shape
- [x] artifact leases remain distinct from attachments
- [x] race-free endpoint selection and safe handoff are contracted
- [x] readiness and failure cleanup order are explicit
- [x] Monkey and the existing attached driver remain outside owned authority
- [x] later execution cards need no fresh lifecycle policy decision

## Evidence

- Research 008 pins upstream `b10069`, its commit, macOS arm64 archive digest,
  single-model flags, readiness endpoints, and excluded router/tool/UI surfaces.
- Tagged source proves `--port 0` uses `bind_to_any_port` and reports the
  selected address, avoiding a host reserve-and-release port race.
- Contract 018 promotes read-only model-artifact leases, host-scoped dynamic
  endpoint handoff, readiness-before-handle, and joined cleanup ordering.
- The operator selected Kimi deferral and authorized a later deployment-owned
  containment lane; no containment platform was selected in this card.
- No executable, model, credential, server, or external inference route ran.

## Validation

- source-link and authority review
- `effigy qa:docs`
- `effigy northstar:qa`
- `git diff --check`

## Validation Result

- official tagged source and release evidence is recorded in Research 008
- docs QA, Northstar QA, formatting, workspace compile, clippy, tests, and diff
  checks pass with the card 061 batch
- doctor remains at the inherited 19 findings, including 7 errors

## Stop Conditions

- current upstream evidence cannot support a bounded owned lifecycle
- implementation would absorb model or Monkey policy

## Auto-Continuation

No. Card 061 begins runtime authority work after this evidence and contract
batch closes.
