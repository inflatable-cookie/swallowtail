# 106 Attached Runtime Version And Residency Records

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../038-ollama-native-attached-runtime-proof.md`

## Objective

Realize the minimum provider-neutral records and assertions required by
Contract 031 before adding Ollama protocol code.

## Scope

- exact attached-runtime version binding separate from compatibility claim
- installed, running, and selected-model detail observation scopes
- safe model-manifest digest without raw manifest or path exposure
- explicit runtime-managed residency acceptance in preflight and request
- pure validation for instance, route, version, model, digest, topology, and
  residency agreement
- Contract 029 range fixtures covering baseline, latest, representative
  interior points, below/above range, prerelease, and stale claim
- additive attached-runtime assertion pack over the unchanged
  attached-self-hosted profile
- no Ollama crate, network client, process, credential, model, or live request

## Ordered Work

1. Inspect existing attached-self-hosted records and reuse only identities that
   preserve installed, running, routed, and artifact separation.
2. Add the smallest runtime-version, catalogue-scope, digest, and residency
   records.
3. Bind them through pure requirements and preflight validation.
4. Add reusable Contract 029 closed-window assertions.
5. Prove no existing profile or adapter claim widens.

## Acceptance Criteria

- [x] exact observation and maintained range remain independent
- [x] installed, running, selected model, route, and artifact stay separate
- [x] runtime-managed residency is explicit and grants no unload authority
- [x] stale or mismatched bindings fail before effects
- [x] range assertions cover boundaries, interior points, prerelease, and drift
- [x] public records contain no endpoint, path, manifest, prompt, or payload
- [x] no provider-specific branch enters core or the existing common profile

## Validation

- focused core, runtime, and testkit tests
- focused warnings-denied clippy
- `effigy doctor` delta review
- `git diff --check`

## Evidence

- `swallowtail-core` now carries source-scoped installed, running, and
  selected-detail observation records, a bounded native model tag, safe
  SHA-256 manifest evidence, exact runtime requirements, and the
  `RuntimeManaged` residency posture.
- pure preflight binds instance, execution host, route model, exact runtime
  version, native tag, manifest digest, external ownership, and residency
  requirements before effects.
- runtime request policy must explicitly match the preflight-bound residency
  posture; omission fails closed and grants no unload or restoration authority.
- reusable testkit assertions cover `0.14.0`, `0.32.1`, `0.18.0`, `0.30.0`,
  below-range, above-range, prerelease, mismatched observation scope, and stale
  claim revision cases.
- semantic stable ranges now reject prereleases unless a prerelease has its own
  exact compatibility segment.
- focused core, runtime, and testkit tests pass. Focused warnings-denied clippy
  passes. `git diff --check` passes.
- `effigy doctor` remains at the inherited 19 oversized-file findings: seven
  errors and twelve warnings.

## Auto-Continuation

Yes. Continue to card 107 when the shared records and assertions pass.
