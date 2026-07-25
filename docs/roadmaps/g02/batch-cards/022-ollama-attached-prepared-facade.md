# 022 Ollama Attached Prepared Facade

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../008-representative-cross-shape-facades.md`

## Objective

Prove the facade against an attached native runtime Swallowtail does not own.

## Governing Refs

- Contracts 014, 020, 024, 029, 031, and 037
- Research 035
- Ollama native driver fixtures
- card 021

## Scope

1. Prepare one host-approved attached endpoint and exact runtime observation.
2. Bind installed catalogue, running inventory, and inference separately.
3. Preserve model artifact, selected route, and invocation-caused residency
   truth.
4. Grant no installation, pull, unload, or server lifecycle authority.
5. Keep endpoint drift and runtime-version drift visible before effects.

## Acceptance Criteria

- [x] attached server ownership remains external
- [x] installed and running inventory remain distinct observations
- [x] inference-caused residency does not imply unload authority
- [x] no model acquisition or implicit route selection occurs
- [x] low-level native driver remains usable

## Validation

- deterministic Ollama native fixtures
- attached-runtime conformance under supported topologies
- drift, cancellation, deadline, redaction, and cleanup cases

## Evidence

- `prepare_ollama_attached` binds one host-approved endpoint, exact configured
  instance, local-unauthenticated access evidence, selected route, native
  model tag, and expected manifest digest.
- Preparation observes exact runtime version, installed inventory, running
  inventory, and selected-model detail without inference, model mutation,
  credential, process, or serving effects.
- Prepared inventory and one-attempt inference remain separate typed
  operations. They delegate to the unchanged low-level native driver.
- Inference fixes `RuntimeManaged` residency. Invocation may affect residency
  but grants no pull, unload, restoration, process, or server authority.
- Local and remote-authoritative fixtures prove exact binding, cancellation
  before effects, endpoint drift, version drift, known-version exclusion, and
  visibly unverified execution on a later exact stable version.
- Contract 031 and Research 035 repair the stale hard-upper-bound wording:
  `0.14.0` through `0.32.1` remains guaranteed, exact `0.32.2` remains
  excluded, prereleases remain closed, and later stable versions may proceed
  visibly unverified.
- The complete adapter all-target suite passes 28 deterministic tests with one
  operator-gated live probe ignored. Warnings-denied Rust lint passes.
- Doctor remains at the same 19 pre-existing oversized-file findings.

## Auto-Continuation

Yes. Continue to card 023 after full repository validation.
