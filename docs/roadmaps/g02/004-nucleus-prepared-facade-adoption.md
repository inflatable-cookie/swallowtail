# 004 Nucleus Prepared Facade Adoption

Status: completed
Owner: Tom
Created: 2026-07-24
Depends on: g02.003
Vision tags: consumer adoption, interactive sessions, bounded workspace
Contract refs: 002, 004, 008-013, 023, 029, 032-034, 036-037
Planning state: completed through cards 011-012

## Problem

Nucleus's current repair correctly binds Codex version, configuration, and
access policy, but it carries hundreds of lines of reusable host, discovery,
and preflight assembly.

The integration should express Nucleus intent and authority, not reconstruct
Swallowtail adapter mechanics.

## Goals

- [x] Migrate model catalogue and read-only Agent Chat to the prepared
      app-server facade.
- [x] Migrate bounded task execution and smoke paths.
- [x] Preserve Nucleus tool execution, prompts, resources, receipts,
      persistence, task state, and UI.
- [x] Remove superseded host-service, discovery, preflight, and policy-copy
      helpers while retaining host-owned executable selection.
- [x] Add deterministic acceptance and retain separately gated live probes.
- [x] Record exact consumer-owned rollback.

## Non-Goals

- [ ] Do not move Nucleus product types or policy into Swallowtail.
- [ ] Do not change task admission, tools, receipts, persistence, or UI.
- [ ] Do not widen read-only Agent Chat or bounded workspace authority.
- [ ] Do not add provider/model fallback or restore direct Codex transport.
- [ ] Do not publish or release Nucleus as part of adoption.

## Execution Plan

### Batch 4.1 — Facade Migration

- [x] Execute card 011 only with consumer-repository authority.
- [x] Replace catalogue, chat, task, and smoke setup with the prepared facade.
- [x] Retain existing Nucleus request and result projections.

### Batch 4.2 — Simplification And Acceptance

- [x] Execute card 012 after functional parity.
- [x] Delete superseded integration glue and assertions.
- [x] Prove deterministic runtime preparation; keep installed/authenticated
      checks separately gated and record that they were not run.

## Acceptance Criteria

- [x] every Nucleus Codex path uses one prepared facade
- [x] no Nucleus helper manually copies exact version, ambient configuration,
      or session access into several records
- [x] read-only chat and bounded task profiles remain distinct
- [x] tools remain consumer-executed and correlated
- [x] existing product DTOs and persistence boundaries remain intact
- [x] deterministic tests fail before provider work on drift
- [x] live probes remain separately gated
- [x] rollback restores the prior pinned integration

## Risks And Mitigations

- Risk: deletion removes product policy with transport glue. Mitigation: map
  ownership before removal and retain Nucleus projections unchanged.
- Risk: current dirty repair conflicts with facade adoption. Mitigation: treat
  it as evidence, then replace cleanly rather than layering a shim.
- Risk: a live test mutates provider state. Mitigation: keep catalogue and
  session-open probes explicit, bounded, and separately authorized.

## Evidence Requirements

- before/after integration ownership map and line-count delta
- focused Nucleus adapter tests
- deterministic fake-process catalogue, chat, task, and smoke preparation
- gated exact installed-version, catalogue, read-only session, and bounded-task
  probes where authorized
- Nucleus Effigy health and QA
- consumer log and rollback instructions

## Closeout

Cards 011-012 completed under Nucleus authority. Deterministic adapter and
server parity pass, manual preparation is removed, and exact source rollback is
recorded. Authenticated Codex probes remain separately gated.

Soundcheck card 013 is ready. No commit, push, release, registry, tag, or
workflow mutation occurred.
