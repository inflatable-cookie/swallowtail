# 008 Readiness Refresh, Subject, And Updates

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: g04.007
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 006, 008, 029, 032, 047, 057
Planning state: cards 022-024 completed

## Problem

An admitted instance can hold enablement and credential references. Consumers
still cannot refresh access dimensions, observe a provider-disclosed
subject, or project an update affordance from existing Contract 029 / 032
evidence. 047 remains a snapshot with no watcher.

## Generation Runway Goal

Realize readiness refresh, authenticated-subject observation, and Contract
029 updates.

## Goals

- [x] refresh credential, entitlement, endpoint, runtime, and support
      dimensions for one admitted instance
- [x] expose optional redacted-by-default subject observation
- [x] derive instance update observation from 029 claims and 032
      observations
- [x] keep enablement and 047 `Ready` / `NotReady` independent

## Non-Goals

- overlay projection (g04.009)
- a watcher or mutation inside 047
- a second currentness system
- install, upgrade, or authenticate
- putting emails into 047 or default diagnostics
- first-proof Anthropic, Codex, or Ollama wiring
- live provider probes as a substitute for deterministic tests

## Execution Plan

### Batch 8.1 — Readiness Refresh

- [x] Execute card 022.
- [x] re-observe 006/008 access dimensions for one admitted instance
- [x] do not write enablement or invent an aggregate ready boolean

### Batch 8.2 — Subject Observation

- [x] Execute card 023 after card 022.
- [x] make `SubjectDisclosure::Absent` representable
- [x] observe email, login, or plan; redacted by default; revealable

### Batch 8.3 — Update Observation

- [x] Execute card 024 after card 023.
- [x] project an update affordance from 029 claims and 032 observations
- [x] do not install, upgrade, or authenticate

## Acceptance Criteria

- [x] refresh writes `AccessStatus` onto the admitted record, not enablement
- [x] 047 snapshots are replaced by the consumer, not mutated in place
- [x] subject observation is absent from 047 and from default diagnostics
- [x] adapters can report a field as not disclosed
- [x] update observation reuses 029/032; no second currentness system
- [x] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.007 sign-in
- this milestone: refresh, subject, updates
- next: g04.009 overlay projection
- later: first-proof routes

## Decision Gates

- Stop if refresh writes enablement or a 047 snapshot.
- Stop if subject becomes an instance id, routing key, or 047 field.
- Stop if update observation installs, authenticates, or duplicates 029.
