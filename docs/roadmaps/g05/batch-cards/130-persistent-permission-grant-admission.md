# 130 Persistent Permission Grant Admission

Status: planned; backlog stub only; no dispatch authorization
Owner: coordinator
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: Contract 041; the Card129 feature-matrix cross audit

## Purpose

Build the producer seam, if the operator promotes it, for persistent
permission grants across the exact provider routes that expose a durable
permission policy. One-shot Allow/Deny exchange remains a separate capability.

## Scope

- define an explicit producer-owned grant boundary and route capability
- preserve consumer policy, provider identity, exact admission, revocation,
  expiry, and audit semantics
- qualify each route independently before changing a matrix cell

## Out Of Scope

No dispatch, runtime implementation, provider probe, consumer policy, release,
or matrix claim change is authorized by this stub. Its presence only gives a
future producer gap a durable card reference.

## Completion Conditions

- Contract 041 and the route-specific evidence settle whether a persistent
  grant is supported without widening one-shot authority.
- Each admitted route has provider-free proof and an exact-head review.
- The feature matrix changes only after the owning route evidence is frozen.

## Review Oracle

An unavailable persistent-grant cell names this card because the producer has
not built the seam; it must not be reported as a provider limitation merely
because the current producer rule withholds it.

## Dispatch Note

This is a backlog stub for Card129's ranked producer-gap output. It is not in a
dispatch manifest and does not authorize implementation.
