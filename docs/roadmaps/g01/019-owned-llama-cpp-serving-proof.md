# 019 Owned llama.cpp Serving Proof

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Prove one host-owned ephemeral model-serving lifecycle without giving
Swallowtail model acquisition, persistent serving, or Monkey authority.

## Generation Runway

Advance g01 coverage expansion from attached self-hosted inference into one
owned-child lifecycle. Keep Kimi production queued until the selected native
macOS containment proof and current successor fixture delta pass. Preserve
SDK-native and persistent serving as later evidence gates.

## Contracts

- Contract 005: Integration Identity and Transport Diversity
- Contract 007: Model Artifact and Serving Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary
- Contract 018: Owned Ephemeral Model Serving Lifecycle

## Goals

- [x] Revalidate the current upstream owned-serving lifecycle and promote its
      missing contract.
- [x] Add provider-neutral artifact leases and safe owned endpoint handoff.
- [x] Implement deterministic local-host artifact authority and owned-serving
      conformance.
- [x] Implement the exact llama.cpp b10069 single-model lifecycle through the
      existing bounded facade.
- [x] Prove readiness, failure teardown, stop/join ordering, topology,
      redaction, and attached-driver non-regression.

## Execution Plan

- [x] Currentness and owned-lifecycle contract batch: card 060.
- [x] Runtime artifact and owned-binding records batch: card 061.
- [x] Local-host artifact authority and conformance batch: card 062.
- [x] llama.cpp b10069 owned driver batch: card 063.
- [x] Cross-lifecycle conformance and closeout batch: card 064.

## Cards

- `batch-cards/060-owned-llama-cpp-evidence-and-contract.md` — complete
- `batch-cards/061-model-artifact-and-owned-serving-records.md` — complete
- `batch-cards/062-local-artifact-host-and-owned-conformance.md` — complete
- `batch-cards/063-llama-cpp-owned-serving-driver.md` — complete
- `batch-cards/064-owned-serving-conformance-and-closeout.md` — complete

## Acceptance Criteria

- [x] artifact, executable, deployment, facade, route, and serving instance
      remain distinct identities
- [x] an operator-supplied artifact uses a read-only serving lease, never an
      attachment or download path
- [x] start returns only after exact build, health, and route readiness
- [x] the dynamic endpoint is loopback-only, redacted, host-scoped, and valid
      only for the owned handle lifetime
- [x] every post-spawn failure stops and joins the child before endpoint and
      artifact release
- [x] stop authority cannot reach an attached or persistent service
- [x] default QA requires no installed llama.cpp binary, model, credential, or
      live network route
- [x] the existing b9910 attached driver remains behaviorally distinct and
      passing

## Planning Checkpoint

Card 064 closed the owned-serving lane with 257 passing repository tests. The
operator selected the seamless native macOS Kimi route and required the latest
maintained Kimi successor. Card 065 subsequently froze the exact successor
artifact, ACP behavior, access, and executable packaging. Card 057 now owns the
App Sandbox helper proof; the root roadmap index holds the sole next pointer.

## Stop Condition

Stop if implementation would require model download, license acceptance,
artifact mutation, public listener authority, persistent serving ownership,
Monkey lifecycle ownership, or an unproved endpoint handoff.
