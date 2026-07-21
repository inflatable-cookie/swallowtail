# 064 Owned Serving Conformance And Closeout

Status: complete
Owner: Tom
Updated: 2026-07-20
Milestone: `../019-owned-llama-cpp-serving-proof.md`

## Objective

Prove Contract 018 across provider-neutral, local-host, and llama.cpp surfaces,
then close roadmap 019 without disturbing attached or Monkey authority.

## Governing References

- `../../../contracts/011-runtime-conformance-profiles.md`
- `../../../contracts/018-owned-ephemeral-model-serving-lifecycle.md`
- `063-llama-cpp-owned-serving-driver.md`

## Scope

- local and remote-authoritative owned-serving topology fixtures
- readiness, early exit, deadline, mismatch, stop, forced cleanup, and release
  ordering
- attached b9910 non-regression and no-stop assertion
- safe diagnostics and exact role/capability/ownership registration
- architecture, roadmap, front-door, and log closeout

## Out Of Scope

- live model benchmarks or authenticated tests
- Kimi containment, SDK bridges, persistent serving, or Monkey adoption

## Acceptance Criteria

- [x] the common profile passes against the production owned llama.cpp driver
- [x] local and remote-authoritative hosts preserve scope and ownership
- [x] every post-spawn terminal path joins before lease release
- [x] attached cleanup cannot stop either fixture's external server
- [x] the full repository passes with live probes still gated
- [x] one post-roadmap planning checkpoint becomes the sole next task

## Evidence

- the adapter conformance suite applies the complete owned-self-hosted profile
  and the existing attached profile without adding provider-specific branches
  to the testkit
- the production owned driver passes the same start, host-scope, ownership,
  endpoint-binding, stop, join, endpoint-release, and artifact-release checks
  under local and remote-authoritative execution-host identities
- deterministic fixtures now cover readiness timeout and exact route mismatch
  alongside malformed, duplicate, non-loopback, early-exit, build-mismatch,
  graceful-stop, and forced-stop paths
- every observed post-spawn failure joins the owned process before any endpoint
  or artifact lease release; diagnostics expose neither the observed endpoint
  nor the materialized artifact path
- the separate b9910 attached suite still proves catalogue, success, provider
  failure, cancellation, and deadline cleanup leave the external server running

## Validation Result

- 28 focused llama.cpp tests pass without a binary, model, credential, or live
  network route
- 257 repository tests pass; two installed or live probes remain skipped by
  default
- `effigy lint:rust`, formatting, repository QA, whitespace, and diff checks
  pass
- doctor remains at the inherited 19 findings, including 7 errors

## Validation

- focused runtime, testkit, host, and llama.cpp tests
- `effigy test`
- `effigy lint:rust`
- `effigy qa`
- `git diff --check`

## Stop Conditions

- full conformance exposes a missing lifecycle contract
- attached and owned behavior cannot remain distinct through public APIs

## Auto-Continuation

No. Roadmap 019 is complete. Card 065 subsequently froze the maintained Kimi
successor. The root roadmap index now points at card 057.
