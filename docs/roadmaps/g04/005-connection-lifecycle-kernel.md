# 005 Connection Lifecycle Kernel

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: g04.004
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 006, 008, 014, 020, 036, 047, 057
Planning state: cards 013-015 completed

## Problem

Contract 057 is active and `v0.3.3` is tagged. Nothing in core, runtime, or
host-local yet holds addable-route records, overlay markers, secret
*references*, enablement, or a store. Later catalog, sign-in, and overlay
work cannot start from types that do not exist.

This is not Contract 032 planned-connection rollover. That policy replaces a
live realtime connection. This kernel admits and stores configured instances
before preparation.

## Generation Runway Goal

Realize the persistence port and optional simple adapter.

## Goals

- [x] add portable 057 records in `swallowtail-core`
- [x] add the store trait and lifecycle roles in `swallowtail-runtime`
- [x] ship optional in-memory and JSON-file adapters in
      `swallowtail-host-local`
- [x] keep 047 a snapshot and keep secrets out of portable records

## Non-Goals

- addable-route catalog assembly or production adapter descriptors
- admission API, sign-in loop, or new host ports
- readiness refresh, authenticated-subject reveal UI, or overlay projection
- first-proof Anthropic, Codex, or Ollama wiring
- a Swallowtail server, keychain, or raw-secret store
- rewriting `release-baselines/public-api-0.3.3/`
- GitHub Release, registry, or tag mutation

## Execution Plan

### Batch 5.1 — Core Records

- [x] Execute card 013.
- [x] add topology, descriptor, field, enablement, overlay-marker, and
      redacted-subject records
- [x] keep topology distinct from `ExecutionLayer`

### Batch 5.2 — Store Port

- [x] Execute card 014 after card 013.
- [x] add the runtime store trait for instance records, secret references,
      enablement, labels, and overlay markers
- [x] prove enablement is independent of access status

### Batch 5.3 — Simple Adapters

- [x] Execute card 015 after card 014.
- [x] add in-memory and JSON-file adapters
- [x] prove JSON on disk carries references, not secret bytes

## Acceptance Criteria

- [x] core records compile without host, runtime, or adapter dependencies
- [x] the store trait never requires raw secrets
- [x] enablement does not change 047 `Ready` / `NotReady`
- [x] overlay marker records cannot invent a model id
- [x] subject records default to redacted and do not enter 047
- [x] `public-api-0.3.3` stays immutable; additive API uses unreleased
      snapshots
- [x] no production adapter crate changes

## Lane Runway

- this milestone: kernel records, store, simple adapters
- next: g04.006 addable catalog, admission, and config-field wiring
- then: g04.007 sign-in loop and host ports
- later compile: readiness refresh, subject observation, overlay projection,
  then first-proof routes

## Decision Gates

- Stop if a record would carry secret bytes, paths, or emails into 047.
- Stop if the store becomes a product database or server.
- Stop if implementation reuses `PlannedConnectionRolloverPolicy`.
- Stop if `public-api-0.3.3` snapshots are rewritten.
