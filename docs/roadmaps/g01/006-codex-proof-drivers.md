# 006 Codex Proof Drivers

Status: completed
Owner: Tom
Updated: 2026-07-19

## Purpose

Prove the runtime kernel with two materially different Codex surfaces already
evidenced in the first consumers: one-shot `codex exec` and long-lived Codex
app-server.

Codex is the first extraction proof, not the shape of the shared runtime.

## Cards

- `batch-cards/017-local-process-host-service.md` — completed
- `batch-cards/018-codex-exec-structured-run-driver.md` — completed
- `batch-cards/019-codex-app-server-session-driver.md` — completed
- `batch-cards/020-codex-dual-driver-conformance.md` — completed
- `batch-cards/021-consumer-adoption-readiness.md` — completed

## Entry Criteria

- roadmaps 004 and 005 are complete
- Contracts 008-011 pass all synthetic profiles
- realized runtime architecture is current
- driver package boundaries are explicit

## Exit Criteria

- both Codex drivers reuse shared host and lifecycle mechanisms
- each driver passes only the capabilities it claims
- no Nucleus or Soundcheck product policy moves into Swallowtail
- Soundcheck and Nucleus have bounded downstream adoption cards
- runtime stability remains withheld pending non-Codex proofs

## Closeout

- a host-approved local process service supports both proof drivers without
  provider or consumer dependency.
- Codex exec and app-server remain separate registered drivers with distinct
  roles, transports, capabilities, and lifecycles.
- both drivers pass their selected Contract 011 profiles and reject unsupported
  inputs or cross-bound plans before provider work.
- Soundcheck and Nucleus evidence now feeds bounded roadmaps 007 and 008.
- runtime stability remains withheld until non-Codex proofs constrain the same
  public vocabulary.
