# 001 Route Availability And Readiness Evidence

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: completed g03
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 005-006, 008, 014, 020, 029, 032, 037, 047
Spec: 011
Planning state: cards 001-003 completed

## Problem

Consumers can already prepare routes, assemble a Contract 047 configured-instance
catalogue, and pick a model from a bound catalogue snapshot. They cannot yet
drive the earlier connection lifecycle that Poodle and T3 Code show:

- which routes can be added, grouped by hosted, installed, or local runtime
- what credential fields or sign-in actions each addable route requires
- how a new configured instance enters the accepted list
- how authentication, install, runtime reachability, and updates are observed
  after admission
- which route-specific config fields the instance exposes

Rebuilding that lifecycle in every app would duplicate provider identity,
access, discovery, and version policy. Swallowtail should supply library
components for it without becoming a server, UI, or credential store.

## Generation Runway Goal

Establish the g04 route-readiness baseline against Spec 011's settled
decisions. The inventory may refine crate placement and first-proof routes. It
must not reopen authenticated-subject, library-max sign-in, persistence-port,
or overlay policy.

## Goals

- [x] inventory every existing Swallowtail record that already covers part of
      the connection lifecycle
- [x] map Poodle and T3 Code surfaces onto those records without copying UI
      policy
- [x] classify missing, overlapping, or over-reaching surfaces against Spec 011
- [x] confirm g04.002 and g04.003 remain the next roadmaps
- [x] keep facade implementation planned until the g04.003 tag exists

## Non-Goals

- a Swallowtail-owned connection server or daemon
- raw secret storage in portable records
- putting account identifiers into Contract 047 or default diagnostics
- routing, fallback, or composer model-selection policy
- accent color and other pure UI chrome
- flattening gateway models from one provider into another connection
- live provider probes, install, login, or billing effects
- consumer repository edits, publication, or the source tag itself
- implementing the persistence port or sign-in loop in this roadmap

## Execution Plan

### Batch 1.1 — Surface Inventory

- [x] Execute card 001.
- [x] derive the exact existing record and facade inventory from canonical
      repository surfaces
- [x] map addable-route, credential, admission, readiness, config, auth, update,
      and model-list needs onto those records
- [x] rank gaps without selecting a contract shape

### Batch 1.2 — Gap Classification And Contract Fit

- [x] Execute card 002 after card 001 fixes the bounded source set.
- [x] classify reuse, amendment, new contract, or consumer-owned overlay
      using Spec 011's settled decisions
- [x] confirm no implementation card is compiled before the g04.003 tag
- [x] reopen an operator decision only if inventory contradicts it

### Batch 1.3 — Follow-On Confirmation

- [x] Execute card 003 after current evidence agrees with Spec 011.
- [x] confirm g04.002 spec closeout and g04.003 source tag as the next
      roadmaps
- [x] leave facade implementation planned until that tag exists

## Acceptance Criteria

- [x] every inventoried consumer surface identifies the Swallowtail record that
      already covers it, the gap, or the consumer overlay
- [x] Contract 047 remains a selection snapshot, not the whole lifecycle
- [x] credential, UI, routing, and secret-storage authority stay explicit
- [x] no readiness contract is promoted from this roadmap alone
- [x] only card 001 starts ready
- [x] g04.002 and g04.003 remain the compiled follow-ons

## Decision Gates

- Stop if the inventory would require storing raw secrets or becoming a server.
- Stop if a later card would flatten provider, transport, or access identity.
- Stop if a settled Spec 011 decision is silently reversed.

## Next Planning Checkpoint

After card 003, execute g04.002. After g04.002, execute g04.003. Facade
implementation stays planned until that tag exists.
