# 038 Ollama Native Attached Runtime Proof

Status: completed
Owner: Tom
Updated: 2026-07-23

## Purpose

Prove one attach-only Ollama native route and the first non-singleton
interface-version compatibility window.

## Authority

- Research 024: Post-Continuation Coverage And Ollama Selection
- Contract 007: Model Artifact And Serving Boundary
- Contract 020: Model Catalogue Observation And Availability Boundary
- Contract 029: Interface Version Qualification And Compatibility
- Contract 031: Attached Native Runtime Version And Residency

## Goals

- [x] Realize provider-neutral attached-runtime version and residency records.
- [x] Freeze the `0.14.0` through `0.32.1` native text subset at baseline,
      latest, and intermediate qualification points.
- [x] Implement exact version, installed, running, and selected-model
      observation without runtime or model mutation.
- [x] Implement one text-only native NDJSON structured run with explicit
      runtime-managed residency acceptance.
- [x] Prove the unchanged attached-self-hosted profile under local and
      remote-authoritative host identities.

## Execution Plan

- [x] Attached-runtime records and compatibility assertions: card 106.
- [x] Multi-release Ollama corpus and native codec: card 107.
- [x] Production attached driver: card 108.
- [x] Portability conformance and closeout: card 109.

## Cards

- `batch-cards/106-attached-runtime-version-and-residency-records.md` — completed
- `batch-cards/107-ollama-multi-release-native-corpus.md` — completed
- `batch-cards/108-ollama-native-attached-driver.md` — completed
- `batch-cards/109-ollama-portability-conformance-and-closeout.md` — completed

## Boundaries

- attach-only external runtime
- local unauthenticated native endpoint
- text-only direct structured run
- no install, update, sign-in, cloud, model acquisition, model mutation,
  server lifecycle, container, or Monkey ownership
- no tools, thinking, vision, compatible facade, retry, or fallback
- no live inference in default QA

## Completion Gate

The roadmap closes only when one descriptor carries the tested maintained
window, exact runtime preflight fails closed, native catalogue and streaming
fixtures pass, both host topologies pass the unchanged attached-self-hosted
profile, and cleanup makes no unload or restoration claim.
