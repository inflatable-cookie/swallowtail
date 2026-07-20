# 010 Bounded Workspace Session Access

Status: completed
Owner: Tom
Updated: 2026-07-20

## Purpose

Extend interactive sessions from the proven read-only profile to one explicit
bounded workspace-write profile required by Nucleus task execution.

## Contracts

- Contract 004: Runtime Ownership Boundary
- Contract 008: Runtime Registration and Preflight
- Contract 009: Async Operation Lifecycle
- Contract 010: Execution Host Services and Inputs
- Contract 012: Interactive Session Options and Callback Exchange
- Contract 013: Interactive Session Access Policy

## Goals

- [x] Promote independent session access-policy dimensions.
- [x] Realize policy and capability records without changing read-only defaults.
- [x] Map the bounded policy into Codex app-server and local host services.
- [x] Prove the shared runtime seam and publish the Nucleus handoff delta.

## Cards

- `batch-cards/031-interactive-session-access-contract.md` — completed
- `batch-cards/032-interactive-access-policy-records.md` — completed
- `batch-cards/033-codex-bounded-workspace-session.md` — completed
- `batch-cards/034-bounded-session-conformance-and-handoff.md` — completed

## Stop Condition

Stop if writable support changes the read-only profile, accepts arbitrary
paths, hides policy dimensions behind one flag, or imports Nucleus types.
